/*
 * Copyright 2022 VMware, Inc.
 * SPDX-License-Identifier: Apache-2.0
 */


#include "httpd.h"
#include "http_config.h"
#include "http_core.h"
#include "http_log.h"
#include "http_main.h"
#include "http_protocol.h"
#include "http_request.h"
#include "util_script.h"
#include "http_connection.h"
#ifdef HAVE_UNIX_SUEXEC
#include "unixd.h"
#endif
#include "scoreboard.h"
#include "mpm_common.h"

#include "apr_strings.h"

#include <stdio.h>

#include "mod_wasm.h"
#include "wasm_runtime.h"


/*--------------------------------------------------------------------------*/
/*                                                                          */
/* Data declarations.                                                       */
/*                                                                          */
/* Here are the static cells and structure declarations private to our      */
/* module.                                                                  */
/*                                                                          */
/*--------------------------------------------------------------------------*/

/**
  * Maximum number of bytes to allocate the body from an HTTP Request.
  *
  * 16KB (16*1024 = 16384)
  *
  */
#define CONFIG_HTTP_REQUEST_BODY_MAX 16384

/**
  * Maximum number of arguments specified in the static configuration.
  *
  * If the user tries to set more arguments on the Apache
  * configuration, this will raise an error. The main reason behind
  * this limitation is to avoid performing reallocations.
  *
  * TODO: remove this limitation and reallocate as more arguments are defined.
  */
#define CONFIG_DEFINED_ARGS_MAX 32

/**
  * Maximum number of environment variables specified in the static configuration.
  *
  * If the user tries to set more environment variables on the Apache
  * configuration, this will raise an error. The main reason behind
  * this limitation is to avoid performing reallocations.
  *
  * TODO: remove this limitation and reallocate as more environment
  * variables are defined.
  */
#define CONFIG_DEFINED_ENVVARS_MAX 32

typedef struct configArg {
  const char *arg;
} configArg;

typedef struct configEnvVar {
  const char *key;
  const char *value;
} configEnvVar;


/*
 * Configuration record. Used for both per-directory and per-server
 * configuration data.
 *
 * It's perfectly reasonable to have two different structures for the two
 * different environments.  The same command handlers will be called for
 * both, though, so the handlers need to be able to tell them apart.  One
 * possibility is for both structures to start with an int which is 0 for
 * one and 1 for the other.
 *
 * Note that while the per-directory and per-server configuration records are
 * available to most of the module handlers, they should be treated as
 * READ-ONLY by all except the command and merge handlers.  Sometimes handlers
 * are handed a record that applies to the current location by implication or
 * inheritance, and modifying it will change the rules for other locations.
 */
typedef struct x_cfg {
    int cmode;                                               /* Environment to which record applies
                                                              * (directory, server, or combination).
                                                              */
#define CONFIG_MODE_SERVER 1
#define CONFIG_MODE_DIRECTORY 2
#define CONFIG_MODE_COMBO 3                                  /* Shouldn't ever happen. */
    int local;                                               /* Boolean: "Example" directive declared
                                                              * here?
                                                              */
    int congenital;                                          /* Boolean: did we inherit an "Example"? */
    int bEnableCGI;                                          /* Boolean: whether this module interfaces as if it was a CGI script */
    char *trace;                                             /* Pointer to trace string. */
    char *loc;                                               /* Location to which this record applies. */

    configEnvVar *configEnvVars[CONFIG_DEFINED_ENVVARS_MAX]; /* Environment variables set in the static configuration */
    int configEnvVarCount;                                   /* Count of environment variables set in the static configuration */
    configArg *configArgs[CONFIG_DEFINED_ARGS_MAX];          /* Arguments set in the static configuration */
    int configArgCount;                                      /* Count of arguments set in the static configuration */
} x_cfg;

/*
 * String pointer to hold the startup trace. No harm working with a global until
 * the server is (may be) multi-threaded.
 */
static const char *trace = NULL;

/*
 * Declare ourselves so the configuration routines can find and know us.
 * We'll fill it in at the end of the module.
 */
module AP_MODULE_DECLARE_DATA wasm_module;


/*--------------------------------------------------------------------------*/
/*                                                                          */
/* These routines are strictly internal to this module, and support its     */
/* operation.  They are not referenced by any external portion of the       */
/* server.                                                                  */
/*                                                                          */
/*--------------------------------------------------------------------------*/

static void trace_nocontext(apr_pool_t *p, const char *file, int line,
                            const char *note)
{
    ap_log_perror(file, line, APLOG_MODULE_INDEX, APLOG_NOTICE, 0, p, "%s", note);
}


/*
 * This function gets called to create a per-directory configuration
 * record. This will be called for the "default" server environment, and for
 * each directory for which the parser finds any of our directives applicable.
 * If a directory doesn't have any of our directives involved (i.e., they
 * aren't in the .htaccess file, or a <Location>, <Directory>, or related
 * block), this routine will *not* be called - the configuration for the
 * closest ancestor is used.
 *
 * The return value is a pointer to the created module-specific
 * structure.
 */
static void *create_dir_config(apr_pool_t *p, char *context)
{
    x_cfg *cfg;
    char *note;

    /*
     * Allocate the space for our record from the pool supplied.
     */
    cfg = (x_cfg *) apr_pcalloc(p, sizeof(x_cfg));
    /*
     * Now fill in the defaults.  If there are any `parent' configuration
     * records, they'll get merged as part of a separate callback.
     */
    cfg->local = 0;
    cfg->congenital = 0;
    cfg->bEnableCGI = 0;
    cfg->configEnvVarCount = 0;
    cfg->configArgCount = 0;
    cfg->cmode = CONFIG_MODE_DIRECTORY;
    /*
     * Finally, add our trace to the callback list.
     */
    context = (context != NULL) ? context : "";
    cfg->loc = apr_pstrcat(p, "DIR(", context, ")", NULL);
    note = apr_psprintf(p, "create_dir_config(p == %pp, context == %s)",
                        (void*) p, context);

    // creates a new Wasm config for the current context
    wasm_config_new(cfg->loc);

    return (void *) cfg;
}

/*
 * This function gets called to create a per-server configuration
 * record.  It will always be called for the "default" server.
 *
 * The return value is a pointer to the created module-specific
 * structure.
 */
static void *create_server_config(apr_pool_t *p, server_rec *s)
{

    x_cfg *cfg;
    char *sname = s->server_hostname;

    /*
     * As with the create_dir_config() reoutine, we allocate and fill
     * in an empty record.
     */
    cfg = (x_cfg *) apr_pcalloc(p, sizeof(x_cfg));
    cfg->local = 0;
    cfg->congenital = 0;
    cfg->bEnableCGI = 0;
    cfg->cmode = CONFIG_MODE_SERVER;
    /*
     * Note that we were called in the trace list.
     */
    sname = (sname != NULL) ? sname : "";
    cfg->loc = apr_pstrcat(p, "SVR(", sname, ")", NULL);
    trace_nocontext(NULL, __FILE__, __LINE__, sname);
    return (void *) cfg;
}

// Add the provided key to the wasmtime runtime as an environment
// variable.
static int _wasm_executionctx_env_add(void* context, const char *key, const char *value)
{
    if ( wasm_executionctx_env_add( (const char*)context, key, value) != OK )
        trace_nocontext(NULL, __FILE__, __LINE__, "_wasm_executionctx_env_add() - ERROR! Couldn't add env variable to Wasm execution context!");

    return 1;
}

// Populates the runtime with the arguments defined in the static
// configuration.
static void populate_runtime_with_config_defined_args(x_cfg *cfg)
{
    for (int i = 0; i < cfg->configArgCount; ++i) {
        wasm_config_add_arg(cfg->configArgs[i]->arg);
    }
}

// Populates the runtime with the environment variables defined in the
// static configuration.
static void populate_runtime_with_config_defined_envs(x_cfg *cfg)
{
    for (int i = 0; i < cfg->configEnvVarCount; ++i) {
        configEnvVar *envVar = cfg->configEnvVars[i];
        wasm_config_add_env(envVar->key, envVar->value);
    }
}

/*
 * This function reads the HTTP Request Body
 * 
 * r: request
 * rbuff: buffer to where the body will be allocated
 * size: size of the buffer allocated
 * 
 * More info:
 *  - https://httpd.apache.org/docs/trunk/developer/modguide.html (section: "Reading the request body into memory")
 *  - https://docstore.mik.ua/orelly/apache_mod/139.htm 
 */
static int read_http_request_body(request_rec *r, const char **rbuf, apr_off_t *size)
{
    int rc = DECLINED; // return code ('DECLINED' by default)

    // setup the client to allow Apache to read the request body
    if ( (rc = ap_setup_client_block(r, REQUEST_CHUNKED_ERROR)) != OK )
    {
        return rc;
    }


    // can we read or abort?
    if ( ap_should_client_block(r) )
    {
        char argsbuffer[CONFIG_HTTP_REQUEST_BODY_MAX];
        apr_off_t rsize, len_read, rpos = 0;
        apr_off_t length = r->remaining;

        *rbuf = (const char *) apr_pcalloc( r->pool, (apr_size_t)(length + 1) );
        *size = length;
        while ( (len_read = ap_get_client_block(r, argsbuffer, sizeof(argsbuffer))) > 0 )
        {
            if ( (rpos + len_read) > length )
            {
                rsize = length - rpos;
            }
            else
            {
                rsize = len_read;
            }

            memcpy( (char *)*rbuf + rpos, argsbuffer, (size_t)rsize );
            rpos += rsize;
        }
    }

    return rc;
}


/*
 * Content handler
 */
static int content_handler(request_rec *r)
{
    /* If it's not for us, get out as soon as possible. */
    if (strcmp(r->handler, "wasm-handler")) {
        return DECLINED;
    }

    /*
     * If we're only supposed to send header information (HEAD request), we're
     * already there.
     */
    if (r->header_only) {
        return OK;
    }

    x_cfg *dcfg = ap_get_module_config(r->per_dir_config, &wasm_module);

    // Reset initial state: to be revised with multiple
    // workers/threads.
    //
    // This reset to initial state is crucial when the module is
    // behaving as a CGI module, because arguments and environnent
    // variables might be set depending on the data of the
    // request. However, the user might have also added some static
    // arguments and environment variables to the Apache
    // configuration.
    wasm_config_clear_args();
    wasm_config_clear_envs();
    populate_runtime_with_config_defined_args(dcfg);
    populate_runtime_with_config_defined_envs(dcfg);

    if (dcfg->bEnableCGI) {
      // On CGI mode, we set the request headers as environment
      // variables with an HTTP_ prefix.
      ap_add_common_vars(r);
      ap_add_cgi_vars(r);
      apr_table_do(_wasm_executionctx_env_add, (void*)exec_ctx_id, r->subprocess_env, NULL);

      // read request body and store it as WASI Stdin
      apr_off_t body_size = 0;
      const char* body_buffer;

      if ( read_http_request_body(r, &body_buffer, &body_size) != OK ) {
        trace_nocontext(NULL, __FILE__, __LINE__, "content_handler() - ERROR! Couldn't read HTTP Request Body!");
      }
      else 
      {
        if ( wasm_executionctx_stdin_set(exec_ctx_id, body_buffer, body_size) != OK )
            trace_nocontext(NULL, __FILE__, __LINE__, "content_handler() - ERROR! Couldn't set HTTP Request Body as stdin!");
      }
    }


    // run Wasm module
    const char* module_response = wasm_runtime_run_module();

    if (dcfg->bEnableCGI) {
      // Retrieve the CGI variables and feed our own response with
      // them; write the response from the module as our own response;
      // which has the headers already stripped from it.
      const char *termch;
      int termarg;
      int ret = ap_scan_script_header_err_strs(r, NULL, &termch, &termarg, module_response, NULL);
      // ap_scan_script_header_err_strs can return either:
      //   - HTTP_OK: success
      //   - HTTP_INTERNAL_SERVER_ERROR: failure
      //   - HTTP_NOT_MODIFIED or HTTP_PRECONDITION_FAILED: script
      //     response does not meet request's conditions
      // In order to not give the external consumer more information
      // than what is needed, map all responses to a 500 error.
      if (ret != 0 && ret != HTTP_OK) {
        wasm_return_const_char_ownership(module_response);
        return HTTP_INTERNAL_SERVER_ERROR;
      }
      if (termch != NULL) {
        ap_rprintf(r, "%s", termch);
      }
    } else if (module_response != NULL) {
      ap_rprintf(r, "%s", module_response);
    }

    // return module response ownership to avoid leaking memory
    wasm_return_const_char_ownership(module_response);

    // deallocate execution context and return id ownership to avoid leaking memory
    wasm_executionctx_deallocate(exec_ctx_id);
    wasm_return_const_char_ownership(exec_ctx_id);

    return OK;
}


/*--------------------------------------------------------------------------*/
/*                                                                          */
/* Which functions are responsible for which hooks in the server.           */
/*                                                                          */
/*--------------------------------------------------------------------------*/
/*
 * Each function our module provides to handle a particular hook is
 * specified here.  The functions are registered using
 * ap_hook_foo(name, predecessors, successors, position)
 * where foo is the name of the hook.
 *
 * The args are as follows:
 * name         -> the name of the function to call.
 * predecessors -> a list of modules whose calls to this hook must be
 *                 invoked before this module.
 * successors   -> a list of modules whose calls to this hook must be
 *                 invoked after this module.
 * position     -> The relative position of this module.  One of
 *                 APR_HOOK_FIRST, APR_HOOK_MIDDLE, or APR_HOOK_LAST.
 *                 Most modules will use APR_HOOK_MIDDLE.  If multiple
 *                 modules use the same relative position, Apache will
 *                 determine which to call first.
 *                 If your module relies on another module to run first,
 *                 or another module running after yours, use the
 *                 predecessors and/or successors.
 *
 * The number in brackets indicates the order in which the routine is called
 * during request processing.  Note that not all routines are necessarily
 * called (such as if a resource doesn't have access restrictions).
 * The actual delivery of content to the browser [9] is not handled by
 * a hook; see the handler declarations below.
 */
static void register_hooks(apr_pool_t *p)
{
    ap_hook_handler(content_handler, NULL, NULL, APR_HOOK_MIDDLE);
}


#define WASM_DIRECTIVE_WASMMODULE "WasmModule"
#define WASM_DIRECTIVE_WASMARG    "WasmArg"
#define WASM_DIRECTIVE_WASMENV    "WasmEnv"
#define WASM_DIRECTIVE_WASMDIR    "WasmDir"
#define WASM_DIRECTIVE_WASMMAPDIR "WasmMapDir"
#define WASM_DIRECTIVE_ENABLECGI  "WasmEnableCGI"

static const char *wasm_directive_WasmModule(cmd_parms *cmd, void *mconfig, const char *word1)
{
    x_cfg *cfg = (x_cfg *) mconfig;
    wasm_module_load(word1);
    wasm_config_set_module(cfg->loc, word1);
    return NULL;
}


static const char *wasm_directive_WasmArg(cmd_parms *cmd, void *mconfig, const char *word1)
{
    x_cfg *cfg = (x_cfg *) mconfig;
    if (cfg->configArgCount == CONFIG_DEFINED_ARGS_MAX) {
      return apr_psprintf(cmd->pool, "WasmArg limit reached in the httpd configuration (maximum is %d)", CONFIG_DEFINED_ARGS_MAX);
    }
    configArg *arg = malloc(sizeof(configArg));
    arg->arg = apr_pstrdup(cmd->pool, word1);
    cfg->configArgs[cfg->configArgCount] = arg;
    cfg->configArgCount++;
    wasm_config_arg_add(cfg->loc, word1);

    return NULL;
}


static const char *wasm_directive_WasmEnv(cmd_parms *cmd, void *mconfig, const char *word1, const char *word2)
{
    x_cfg *cfg = (x_cfg *) mconfig;
    if (cfg->configEnvVarCount == CONFIG_DEFINED_ENVVARS_MAX) {
      return apr_psprintf(cmd->pool, "WasmEnv limit reached in the httpd configuration (maximum is %d)", CONFIG_DEFINED_ENVVARS_MAX);
    }
    configEnvVar *env = malloc(sizeof(configEnvVar));
    env->key = apr_pstrdup(cmd->pool, word1);
    env->value = apr_pstrdup(cmd->pool, word2);
    cfg->configEnvVars[cfg->configEnvVarCount] = env;
    cfg->configEnvVarCount++;
    wasm_config_env_add(cfg->loc, word1, word2);

    return NULL;
}


static const char *wasm_directive_WasmDir(cmd_parms *cmd, void *mconfig, const char *word1)
{
    x_cfg *cfg = (x_cfg *) mconfig;
    wasm_config_dir_add(cfg->loc, word1);
    return NULL;
}


static const char *wasm_directive_WasmMapDir(cmd_parms *cmd, void *mconfig, const char *word1, const char *word2)
{
    x_cfg *cfg = (x_cfg *) mconfig;
    wasm_config_mapdir_add(cfg->loc, word1, word2);
    return NULL;
}


static const char *wasm_directive_WasmEnableCGI(cmd_parms *cmd, void *mconfig, int arg)
{
    x_cfg *cfg = (x_cfg *) mconfig;
    cfg->bEnableCGI = arg;
    return NULL;
}


/*
 * List of directives specific to our module.
 */
static const command_rec directives[] =
{
    AP_INIT_TAKE1(
        WASM_DIRECTIVE_WASMMODULE,       /* directive name */
        wasm_directive_WasmModule,       /* config action routine */
        NULL,                            /* argument to include in call */
        OR_OPTIONS,                      /* where available */
        "Load a Wasm Module from disk"   /* directive description */
    ),
    AP_INIT_TAKE1(
        WASM_DIRECTIVE_WASMARG,
        wasm_directive_WasmArg,
        NULL,
        OR_OPTIONS,
        "Add arg context for the Wasm Module"
    ),
    AP_INIT_TAKE2(
        WASM_DIRECTIVE_WASMENV,
        wasm_directive_WasmEnv,
        NULL,
        OR_OPTIONS,
        "Set environtment variable for the Wasm Module"
    ),
    AP_INIT_TAKE1(
        WASM_DIRECTIVE_WASMDIR,
        wasm_directive_WasmDir,
        NULL,
        OR_OPTIONS,
        "Preopen Dir for the Wasm Module"
    ),
    AP_INIT_TAKE2(
        WASM_DIRECTIVE_WASMMAPDIR,
        wasm_directive_WasmMapDir,
        NULL,
        OR_OPTIONS,
        "Preopen Dir with Mapping for the Wasm Module"
    ),
    AP_INIT_FLAG(
        WASM_DIRECTIVE_ENABLECGI,
        wasm_directive_WasmEnableCGI,
        NULL,
        OR_OPTIONS,
        "Whether this WebAssembly module behaves as a CGI"
    ),
    {NULL}
};



/*--------------------------------------------------------------------------*/
/*                                                                          */
/* Finally, the list of callback routines and data structures that provide  */
/* the static hooks into our module from the other parts of the server.     */
/*                                                                          */
/*--------------------------------------------------------------------------*/
/*
 * Module definition for configuration.  If a particular callback is not
 * needed, replace its routine name below with the word NULL.
 */
AP_DECLARE_MODULE(wasm) =
{
    STANDARD20_MODULE_STUFF,
    create_dir_config,      /* per-directory config creator */
    NULL,                   /* dir config merger */
    create_server_config,   /* server config creator */
    NULL,                   /* server config merger */
    directives,             /* command table */
    register_hooks,         /* set up other request processing hooks */
};
