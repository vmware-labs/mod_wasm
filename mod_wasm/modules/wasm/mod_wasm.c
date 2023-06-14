/* Licensed to the Apache Software Foundation (ASF) under one or more
 * contributor license agreements.  See the NOTICE file distributed with
 * this work for additional information regarding copyright ownership.
 * The ASF licenses this file to You under the Apache License, Version 2.0
 * (the "License"); you may not use this file except in compliance with
 * the License.  You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
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

/*
 * Maximum number of bytes to allocate the body from an HTTP Request.
 *
 * 16KB (16*1024 = 16384)
 *
 */
#define CONFIG_HTTP_REQUEST_BODY_MAX 16384

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
    int bWasmEnableCGI;                                      /* Boolean: whether this module interfaces as if it was a CGI script */
    int bWasmMapCGIFileNames;                                /* Boolean: whether this module will adjust SCRIPT_FILENAME when enabling WasmEnableCGI according to mapped files */
    char *trace;                                             /* Pointer to trace string. */
    char *loc;                                               /* Location to which this record applies. */
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

    /*
     * Allocate the space for our record from the pool supplied.
     */
    cfg = (x_cfg *) apr_pcalloc(p, sizeof(x_cfg));
    /*
     * Now fill in the defaults.  If there are any `parent' configuration
     * records, they'll get merged as part of a separate callback.
     */
    cfg->bWasmEnableCGI = 0;
    cfg->bWasmMapCGIFileNames = 0;
    cfg->cmode = CONFIG_MODE_DIRECTORY;
    /*
     * Finally, add our trace to the callback list.
     */
    context = (context != NULL) ? context : "";
    cfg->loc = apr_pstrcat(p, "DIR(", context, ")", NULL);

    /* creates a new Wasm config for the current context */
    int ret = wasm_config_create(cfg->loc); 
    if ( ret != OK )
        ap_log_perror(APLOG_MARK, APLOG_ERR, ret, p,
            "wasm_config_create() - ERROR! Couldn't create Wasm config for context '%s' !", cfg->loc);

    return (void *) cfg;
}

/*
 * This function gets called to create a per-server configuration
 * record. It will always be called for the "default" server.
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
    cfg->bWasmEnableCGI = 0;
    cfg->bWasmMapCGIFileNames = 0;
    cfg->cmode = CONFIG_MODE_SERVER;
    /*
     * Note that we were called in the trace list.
     */
    sname = (sname != NULL) ? sname : "";
    cfg->loc = apr_pstrcat(p, "SVR(", sname, ")", NULL);

    /* creates a new Wasm config for the current context */
    int ret = wasm_config_create(cfg->loc); 
    if ( ret != OK )
        ap_log_perror(APLOG_MARK, APLOG_ERR, ret, p,
            "wasm_config_create() - ERROR! Couldn't create Wasm config for context '%s' !", cfg->loc);

    return (void *) cfg;
}

/*
 * Add the provided key to the Wasm runtime as an environment variable.
 */
static int _wasm_executionctx_env_add(void* context, const char *key, const char *value)
{
    int ret = wasm_executionctx_env_add((const char*)context, key, value);
    if ( ret != OK )
        ap_log_error(APLOG_MARK, APLOG_ERR, ret, NULL,
            "wasm_executionctx_env_add() - ERROR! Couldn't add env variable '%s = %s' to Wasm execution context!", key, value);

    return 1;
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
    int rc = DECLINED; /* return code ('DECLINED' by default) */

    /* setup the client to allow Apache to read the request body */
    if ( (rc = ap_setup_client_block(r, REQUEST_CHUNKED_ERROR)) != OK )
    {
        return rc;
    }

    /* can we read or abort? */
    if ( ap_should_client_block(r) )
    {
        char argsbuffer[CONFIG_HTTP_REQUEST_BODY_MAX];
        apr_off_t rsize, len_read, rpos = 0;
        apr_off_t length = r->remaining;

        *rbuf = (const char *) apr_pcalloc( r->pool, (apr_size_t)(length + 1) );
        *size = length;
        while ( (len_read = ap_get_client_block(r, argsbuffer, sizeof(argsbuffer))) > 0 )
        {
            if ( (rpos + len_read) > length ) {
                rsize = length - rpos;
            }
            else {
                rsize = len_read;
            }

            memcpy( (char *)*rbuf + rpos, argsbuffer, (size_t)rsize );
            rpos += rsize;
        }
    }

    return rc;
}

/*
 * Modify the SCRIPT_FILENAME variable based on the WasmMapDirs
 */
static void map_cgi_filenames(char *config_id, request_rec *r) {
  const char* script_filename = apr_table_get(r->subprocess_env, "SCRIPT_FILENAME");
  if (script_filename != NULL && *script_filename) {
    ap_log_rerror(APLOG_MARK, APLOG_TRACE1, 0, r,
                  "map_cgi_filenames() - TRACE Looking for remapped value for %s", script_filename);
    const char* mapped_path = wasm_config_get_mapped_path(config_id, script_filename);
    if (mapped_path != NULL) {
      ap_log_rerror(APLOG_MARK, APLOG_DEBUG, 0, r,
                    "map_cgi_filenames() - DEBUG Found remap %s => %s: Updating SCRIPT_FILENAME", script_filename, mapped_path);
      apr_table_set(r->subprocess_env, "SCRIPT_FILENAME", apr_pstrdup(r->pool, mapped_path));
      wasm_return_const_char_ownership(mapped_path);
    }
    else {
      ap_log_rerror(APLOG_MARK, APLOG_TRACE1, 0, r,
                    "map_cgi_filenames() - TRACE Could not find any mapping for %s", script_filename);
    }
  }
  else {
    ap_log_rerror(APLOG_MARK, APLOG_WARNING, 0, r,
                  "map_cgi_filenames() - WARNING! Cannot find SCRIPT_FILENAME entry");
  }
}

/*
 * generic hook handler
 */
static int handle_generic_hook(request_rec *r, const char* wasm_function_id)
{
    /* get specific configuration for the given directory/location */
    x_cfg *dcfg = ap_get_module_config(r->per_dir_config, &wasm_module);

    /* creates a new Wasm execution context */
    const char* exec_ctx_id = wasm_executionctx_create_from_config(dcfg->loc);

    int ret = OK;
    ret = wasm_executionctx_run_wasm_function(exec_ctx_id, wasm_function_id);
    if ( ret != OK ) {
        ap_log_rerror(APLOG_MARK, APLOG_ERR, ret, r,
            "handle_generic_hook() - ERROR! Couldn't run Wasm execution context '%s' for Wasm function '%s'!", exec_ctx_id, wasm_function_id);
    }

    /* deallocate execution context and return id ownership to avoid leaking memory */
    ret = wasm_executionctx_deallocate(exec_ctx_id);
    if ( ret != OK )
        ap_log_rerror(APLOG_MARK, APLOG_ERR, ret, r,
            "handle_generic_hook() - ERROR! Couldn't deallocate Wasm execution context: '%s'", exec_ctx_id);

    wasm_return_const_char_ownership(exec_ctx_id);

    return ret;
}

/*
 * Post read request hook
 */
static int hook_post_read_request(request_rec *r)
{
    return handle_generic_hook(r, "ap_hook_post_read_request");
}

/*
 * Fixups hook
 */
static int hook_fixups(request_rec *r)
{
    return handle_generic_hook(r, "ap_hook_fixups");
}

/*
 * Log transaction hook
 */
static int hook_log_transaction(request_rec *r)
{
    return handle_generic_hook(r, "ap_hook_log_transaction");
}

/*
 * Content handler hook
 */
static int hook_content_handler(request_rec *r)
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

    /* get specific configuration for the given directory/location */
    x_cfg *dcfg = ap_get_module_config(r->per_dir_config, &wasm_module);

    /* creates a new Wasm execution context */
    const char* exec_ctx_id = wasm_executionctx_create_from_config(dcfg->loc);

    int ret = OK;
    if (dcfg->bWasmEnableCGI) {
        /* On CGI mode, we set the request headers as environment variables with an HTTP_ prefix. */
        ap_add_common_vars(r);
        ap_add_cgi_vars(r);
        if (dcfg->bWasmMapCGIFileNames) {
          map_cgi_filenames(dcfg->loc, r);
        }

        apr_table_do(_wasm_executionctx_env_add, (void*)exec_ctx_id, r->subprocess_env, NULL);

        /* read HTTP Request body and set it as stdin for the Wasm module */
        apr_off_t body_size = 0;
        const char* body_buffer = NULL;

        ret = read_http_request_body(r, &body_buffer, &body_size);
        if ( ret != OK ) {
            ap_log_rerror(APLOG_MARK, APLOG_ERR, ret, r,
            "content_handler() - ERROR! Couldn't read HTTP Request Body!");
        }
        else { /* read_http_request_body() was successfull */
            ret = wasm_executionctx_stdin_set(exec_ctx_id, body_buffer, body_size);
            if ( ret != OK )
                ap_log_rerror(APLOG_MARK, APLOG_ERR, ret, r,
                    "content_handler() - ERROR! Couldn't set HTTP Request Body as stdin!");
        }
    }

    /* run Wasm execution context */
    size_t len = 0;
    const char* module_response = NULL;
    ret = wasm_executionctx_run_wasm_module(exec_ctx_id, &module_response, &len);
    if ( ret != OK ) {
        ap_log_rerror(APLOG_MARK, APLOG_ERR, ret, r,
            "content_handler() - ERROR! Couldn't run Wasm execution context '%s'!", exec_ctx_id);
    }

    if (dcfg->bWasmEnableCGI) {
        /*
        * Retrieve the CGI variables and feed our own response with
        * them; write the response from the module as our own response;
        * which has the headers already stripped from it.
        */
        char buffer[MAX_STRING_LEN] = {'\0'};
        const char *termch;
        int termarg;
        ret = ap_scan_script_header_err_strs(r, buffer, &termch, &termarg, module_response, NULL);
        /*
        * ap_scan_script_header_err_strs can return either:
        *   - HTTP_OK: success
        *   - HTTP_INTERNAL_SERVER_ERROR: failure
        *   - HTTP_NOT_MODIFIED or HTTP_PRECONDITION_FAILED: script
        *     response does not meet request's conditions
        * In order to not give the external consumer more information
        * than what is needed, map all responses to a 500 error.
        */

        if (ret != OK && ret != HTTP_OK)
        {
            if (buffer != NULL)
                ap_log_rerror(APLOG_MARK, APLOG_ERR, ret, r, "ERROR! Invalid script response. On header #%i, found character '%s': %s", termarg, termch, buffer);

            if (r->content_type == NULL)
                ap_log_rerror(APLOG_MARK, APLOG_ERR, ret, r,
                    "ERROR! Couldn't find mandatory 'Content-type' HTTP header (i.e.: \"Content-type: text/html\n\n\")");

            return HTTP_INTERNAL_SERVER_ERROR;
        }
        if (termch != NULL) {
            /*
            * After parsing the response headers in the Wasm module output with
            * `ap_scan_script_header_err_strs()`, the `termch` variable points to the
            * last parsed character. Now we need to write the remaining body into the request,
            * but we don't know its length, and thus it has to be calculated.
            * 
            * Since `termch` points to the latest char in the headers, and `module_response` points to
            * the begining of the response array, we can calculate the headers length using a simple 
            * pointer arithmetic operation: `headers_len = termch - module_response`.
            * 
            * Finally, the body length is therefore the total array length minus the headers length:
            * `body_len = len - headers_len = len - (termch - module_response)`.
            */
            ap_rwrite(termch, len - (termch - module_response), r);        
        }
    }
    else if (module_response != NULL) {
        ap_rwrite(module_response, len, r);   
    }

    /* return module response ownership to avoid leaking memory */
    wasm_return_const_char_ownership(module_response);

    /* deallocate execution context and return id ownership to avoid leaking memory */
    ret = wasm_executionctx_deallocate(exec_ctx_id);
    if ( ret != OK )
        ap_log_rerror(APLOG_MARK, APLOG_ERR, ret, r,
            "content_handler() - ERROR! Couldn't deallocate Wasm execution context: '%s'", exec_ctx_id);

    wasm_return_const_char_ownership(exec_ctx_id);

    return ret;
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
    /* These hooks are challenging to support because they are called even before the HTTP request
     * has been associated to a specific virtual host or location
     * ap_hook_post_read_request(hook_post_read_request, NULL, NULL, APR_HOOK_MIDDLE);
     */ 

    ap_hook_fixups(hook_fixups, NULL, NULL, APR_HOOK_MIDDLE);
    ap_hook_handler(hook_content_handler, NULL, NULL, APR_HOOK_MIDDLE);
    ap_hook_log_transaction(hook_log_transaction, NULL, NULL, APR_HOOK_MIDDLE);
}

#define WASM_DIRECTIVE_WASMMODULE           "WasmModule"
#define WASM_DIRECTIVE_WASMAPACHEMODULE     "WasmApacheModule"
#define WASM_DIRECTIVE_WASMARG              "WasmArg"
#define WASM_DIRECTIVE_WASMENV              "WasmEnv"
#define WASM_DIRECTIVE_WASMDIR              "WasmDir"
#define WASM_DIRECTIVE_WASMMAPDIR           "WasmMapDir"
#define WASM_DIRECTIVE_WASMENABLECGI        "WasmEnableCGI"
#define WASM_DIRECTIVE_WASMMAPCGIFILENAMES  "WasmMapCGIFileNames"

static const char *wasm_directive_WasmModule(cmd_parms *cmd, void *mconfig, const char *word1)
{
    x_cfg *cfg = (x_cfg *) mconfig;
    int ret;

    /* Wasm module is loaded and cached */
    ret = wasm_module_load(word1);
    if ( ret != OK )
        ap_log_error(APLOG_MARK, APLOG_ERR, ret, NULL,
            "wasm_directive_WasmModule() - ERROR! Couldn't load Wasm Module '%s'!", word1);

    /* Wasm config is implictly created for the current location and using the loaded module */
    ret = wasm_config_module_set(cfg->loc, word1);
    if ( ret != OK )
        ap_log_error(APLOG_MARK, APLOG_ERR, ret, NULL,
            "wasm_directive_WasmModule() - ERROR! Couldn't set Wasm Module '%s' to Wasm config '%s'!", word1, cfg->loc);

    return NULL;
}

static const char *wasm_directive_WasmApacheModule(cmd_parms *cmd, void *mconfig, const char *word1)
{
    x_cfg *cfg = (x_cfg *) mconfig;
    int ret;

    /* Wasm Apache module is loaded and cached */
    ret = wasm_module_load(word1);
    if ( ret != OK )
        ap_log_error(APLOG_MARK, APLOG_ERR, ret, NULL,
            "wasm_directive_WasmApacheModule() - ERROR! Couldn't load Wasm Apache Module '%s'!", word1);

    /* Wasm config is implictly created for the current location and the loaded Apache module is added */
    ret = wasm_config_apache_module_add(cfg->loc, word1);
    if ( ret != OK )
        ap_log_error(APLOG_MARK, APLOG_ERR, ret, NULL,
            "wasm_directive_WasmApacheModule() - ERROR! Couldn't add Wasm Apache Module '%s' to Wasm config '%s'!", word1, cfg->loc);

    return NULL;
}

static const char *wasm_directive_WasmArg(cmd_parms *cmd, void *mconfig, const char *word1)
{
    x_cfg *cfg = (x_cfg *) mconfig;
    
    int ret = wasm_config_arg_add(cfg->loc, word1);
    if ( ret != OK )
        ap_log_error(APLOG_MARK, APLOG_ERR, ret, NULL,
            "wasm_directive_WasmArg() - ERROR! Couldn't add arg '%s' to Wasm config '%s'!", word1, cfg->loc);

    return NULL;
}

static const char *wasm_directive_WasmEnv(cmd_parms *cmd, void *mconfig, const char *word1, const char *word2)
{
    x_cfg *cfg = (x_cfg *) mconfig;

    int ret = wasm_config_env_add(cfg->loc, word1, word2);
    if ( ret != OK )
        ap_log_error(APLOG_MARK, APLOG_ERR, ret, NULL,
            "wasm_directive_WasmEnv() - ERROR! Couldn't add env var '%s=%s' to Wasm config '%s'!", word1, word2, cfg->loc);

    return NULL;
}

static const char *wasm_directive_WasmDir(cmd_parms *cmd, void *mconfig, const char *word1)
{
    x_cfg *cfg = (x_cfg *) mconfig;

    int ret = wasm_config_dir_add(cfg->loc, word1);
    if ( ret != OK )
        ap_log_error(APLOG_MARK, APLOG_ERR, ret, NULL,
            "wasm_directive_WasmDir() - ERROR! Couldn't preopen dir '%s' for Wasm config '%s'!", word1, cfg->loc);

    return NULL;
}

static const char *wasm_directive_WasmMapDir(cmd_parms *cmd, void *mconfig, const char *word1, const char *word2)
{
    x_cfg *cfg = (x_cfg *) mconfig;

    int ret = wasm_config_mapdir_add(cfg->loc, word1, word2); 
    if ( ret != OK )
        ap_log_error(APLOG_MARK, APLOG_ERR, ret, NULL,
            "wasm_directive_WasmMapDir() - ERROR! Couldn't preopen dir '%s' with mapping to '%s' for Wasm config '%s'!", word2, word1, cfg->loc);            

    return NULL;
}

static const char *wasm_directive_WasmEnableCGI(cmd_parms *cmd, void *mconfig, int arg)
{
    x_cfg *cfg = (x_cfg *) mconfig;
    cfg->bWasmEnableCGI = arg;
    return NULL;
}

static const char *wasm_directive_WasmMapCGIFileNames(cmd_parms *cmd, void *mconfig, int arg)
{
    x_cfg *cfg = (x_cfg *) mconfig;
    cfg->bWasmMapCGIFileNames = arg;
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
        WASM_DIRECTIVE_WASMAPACHEMODULE,
        wasm_directive_WasmApacheModule,
        NULL,
        OR_OPTIONS,
        "Load a Wasm module and hook its exported funcitons"
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
        WASM_DIRECTIVE_WASMENABLECGI,
        wasm_directive_WasmEnableCGI,
        NULL,
        OR_OPTIONS,
        "Whether this WebAssembly module behaves as a CGI"
    ),
    AP_INIT_FLAG(
        WASM_DIRECTIVE_WASMMAPCGIFILENAMES,
        wasm_directive_WasmMapCGIFileNames,
        NULL,
        OR_OPTIONS,
        "Whether SCRIPT_FILENAME should be mapped based on WasmMapDir mounts when running as a CGI"
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
