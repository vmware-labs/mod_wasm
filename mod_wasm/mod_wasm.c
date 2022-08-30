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
 * Sample configuration record.  Used for both per-directory and per-server
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
    int cmode;                  /* Environment to which record applies
                                 * (directory, server, or combination).
                                 */
#define CONFIG_MODE_SERVER 1
#define CONFIG_MODE_DIRECTORY 2
#define CONFIG_MODE_COMBO 3     /* Shouldn't ever happen. */
    int local;                  /* Boolean: "Example" directive declared
                                 * here?
                                 */
    int congenital;             /* Boolean: did we inherit an "Example"? */
    char *trace;                /* Pointer to trace string. */
    char *loc;                  /* Location to which this record applies. */
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
 * Locate our directory configuration record for the current request.
 */
static x_cfg *our_dconfig(const request_rec *r)
{
    return (x_cfg *) ap_get_module_config(r->per_dir_config, &wasm_module);
}

/*
 * The following utility routines are not used in the module. Don't
 * compile them so -Wall doesn't complain about functions that are
 * defined but not used.
 */
#if 0
/*
 * Locate our server configuration record for the specified server.
 */
static x_cfg *our_sconfig(const server_rec *s)
{
    return (x_cfg *) ap_get_module_config(s->module_config, &wasm_module);
}

/*
 * Likewise for our configuration record for the specified request.
 */
static x_cfg *our_rconfig(const request_rec *r)
{
    return (x_cfg *) ap_get_module_config(r->request_config, &wasm_module);
}
#endif /* if 0 */

/*
 * Likewise for our configuration record for a connection.
 */
static x_cfg *our_cconfig(const conn_rec *c)
{
    return (x_cfg *) ap_get_module_config(c->conn_config, &wasm_module);
}

#define EXAMPLE_LOG_EACH 1

static void trace_nocontext(apr_pool_t *p, const char *file, int line,
                            const char *note)
{
#ifdef EXAMPLE_LOG_EACH
    ap_log_perror(file, line, APLOG_MODULE_INDEX, APLOG_NOTICE, 0, p, "%s", note);
#endif
}


/*
 * This function gets called to create a per-directory configuration
 * record.  This will be called for the "default" server environment, and for
 * each directory for which the parser finds any of our directives applicable.
 * If a directory doesn't have any of our directives involved (i.e., they
 * aren't in the .htaccess file, or a <Location>, <Directory>, or related
 * block), this routine will *not* be called - the configuration for the
 * closest ancestor is used.
 *
 * The return value is a pointer to the created module-specific
 * structure.
 */
static void *x_create_dir_config(apr_pool_t *p, char *dirspec)
{
    x_cfg *cfg;
    char *dname = dirspec;
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
    cfg->cmode = CONFIG_MODE_DIRECTORY;
    /*
     * Finally, add our trace to the callback list.
     */
    dname = (dname != NULL) ? dname : "";
    cfg->loc = apr_pstrcat(p, "DIR(", dname, ")", NULL);
    note = apr_psprintf(p, "x_create_dir_config(p == %pp, dirspec == %s)",
                        (void*) p, dirspec);
    trace_nocontext(NULL, __FILE__, __LINE__, note);
    return (void *) cfg;
}

/*
 * This function gets called to merge two per-directory configuration
 * records.  This is typically done to cope with things like .htaccess files
 * or <Location> directives for directories that are beneath one for which a
 * configuration record was already created.  The routine has the
 * responsibility of creating a new record and merging the contents of the
 * other two into it appropriately.  If the module doesn't declare a merge
 * routine, the record for the closest ancestor location (that has one) is
 * used exclusively.
 *
 * The routine MUST NOT modify any of its arguments!
 *
 * The return value is a pointer to the created module-specific structure
 * containing the merged values.
 */
static void *x_merge_dir_config(apr_pool_t *p, void *parent_conf,
                                      void *newloc_conf)
{

    x_cfg *merged_config = (x_cfg *) apr_pcalloc(p, sizeof(x_cfg));
    x_cfg *pconf = (x_cfg *) parent_conf;
    x_cfg *nconf = (x_cfg *) newloc_conf;
    char *note;

    /*
     * Some things get copied directly from the more-specific record, rather
     * than getting merged.
     */
    merged_config->local = nconf->local;
    merged_config->loc = apr_pstrdup(p, nconf->loc);
    /*
     * Others, like the setting of the `congenital' flag, get ORed in.  The
     * setting of that particular flag, for instance, is TRUE if it was ever
     * true anywhere in the upstream configuration.
     */
    merged_config->congenital = (pconf->congenital | pconf->local);
    /*
     * If we're merging records for two different types of environment (server
     * and directory), mark the new record appropriately.  Otherwise, inherit
     * the current value.
     */
    merged_config->cmode =
        (pconf->cmode == nconf->cmode) ? pconf->cmode : CONFIG_MODE_COMBO;
    /*
     * Now just record our being called in the trace list.  Include the
     * locations we were asked to merge.
     */
    note = apr_psprintf(p, "x_merge_dir_config(p == %pp, parent_conf == "
                        "%pp, newloc_conf == %pp)", (void*) p,
                        (void*) parent_conf, (void*) newloc_conf);
    trace_nocontext(NULL, __FILE__, __LINE__, note);
    return (void *) merged_config;
}

/*
 * This function gets called to create a per-server configuration
 * record.  It will always be called for the "default" server.
 *
 * The return value is a pointer to the created module-specific
 * structure.
 */
static void *x_create_server_config(apr_pool_t *p, server_rec *s)
{

    x_cfg *cfg;
    char *sname = s->server_hostname;

    /*
     * As with the x_create_dir_config() reoutine, we allocate and fill
     * in an empty record.
     */
    cfg = (x_cfg *) apr_pcalloc(p, sizeof(x_cfg));
    cfg->local = 0;
    cfg->congenital = 0;
    cfg->cmode = CONFIG_MODE_SERVER;
    /*
     * Note that we were called in the trace list.
     */
    sname = (sname != NULL) ? sname : "";
    cfg->loc = apr_pstrcat(p, "SVR(", sname, ")", NULL);
    trace_nocontext(NULL, __FILE__, __LINE__, sname);
    return (void *) cfg;
}

/*
 * This function gets called to merge two per-server configuration
 * records.  This is typically done to cope with things like virtual hosts and
 * the default server configuration  The routine has the responsibility of
 * creating a new record and merging the contents of the other two into it
 * appropriately.  If the module doesn't declare a merge routine, the more
 * specific existing record is used exclusively.
 *
 * The routine MUST NOT modify any of its arguments!
 *
 * The return value is a pointer to the created module-specific structure
 * containing the merged values.
 */
static void *x_merge_server_config(apr_pool_t *p, void *server1_conf,
                                         void *server2_conf)
{

    x_cfg *merged_config = (x_cfg *) apr_pcalloc(p, sizeof(x_cfg));
    x_cfg *s1conf = (x_cfg *) server1_conf;
    x_cfg *s2conf = (x_cfg *) server2_conf;
    char *note;

    /*
     * Our inheritance rules are our own, and part of our module's semantics.
     * Basically, just note whence we came.
     */
    merged_config->cmode =
        (s1conf->cmode == s2conf->cmode) ? s1conf->cmode : CONFIG_MODE_COMBO;
    merged_config->local = s2conf->local;
    merged_config->congenital = (s1conf->congenital | s1conf->local);
    merged_config->loc = apr_pstrdup(p, s2conf->loc);
    /*
     * Trace our call, including what we were asked to merge.
     */
    note = apr_pstrcat(p, "x_merge_server_config(\"", s1conf->loc, "\",\"",
                   s2conf->loc, "\")", NULL);
    trace_nocontext(NULL, __FILE__, __LINE__, note);

    return (void *) merged_config;
}



/*
 * Content handler
 */
static int content_handler(request_rec *r)
{
    /* If it's not for us, get out as soon as possible. */
    if (strcmp(r->handler, "wasm-handler")) {
        //trace_nocontext(NULL, __FILE__, __LINE__, "content_handler() - DECLINED");
        return DECLINED;
    }
    else {
        trace_nocontext(NULL, __FILE__, __LINE__, "content_handler() - ACCEPTED");
    }


    /*
     * Set the Content-type header. Note that we do not actually have to send
     * the headers: this is done by the http core.
     */
    ap_set_content_type(r, "text/html");
    /*
     * If we're only supposed to send header information (HEAD request), we're
     * already there.
     */
    if (r->header_only) {
        return OK;
    }


    /*
     * Now send our actual output.  Since we tagged this as being
     * "text/html", we need to embed any HTML.
     */
    ap_rputs(DOCTYPE_HTML_3_2, r);
    ap_rputs("<HTML>\n", r);
    ap_rputs(" <HEAD>\n", r);
    ap_rputs("  <TITLE>mod_wasm demo\n", r);
    ap_rputs("  </TITLE>\n", r);
    ap_rputs(" </HEAD>\n", r);
    ap_rputs(" <BODY>\n", r);
    ap_rprintf(r, "Apache HTTP Server version: \"%s\"\n", ap_get_server_banner());
    ap_rputs("  <BR>\n", r);
    
    // invoke wasm_runtime
    const char* content = load_and_run();
    ap_rprintf(r, "%s", content);
    return_const_char_ownership(content);

    ap_rputs("  <BR>\n", r);
    ap_rputs(" </BODY>\n", r);
    ap_rputs("</HTML>\n", r);

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


#define WASM_DIRECTIVE_WASMROOT   "WasmRoot"
#define WASM_DIRECTIVE_WASMMODULE "WasmModule"
#define WASM_DIRECTIVE_WASMARG    "WasmArg"
#define WASM_DIRECTIVE_WASMENV    "WasmEnv"

static const char *wasm_directive_WasmRoot(cmd_parms *cmd, void *mconfig, const char *word1)
{
    x_cfg *cfg = (x_cfg *) mconfig;

    char message[128];
    snprintf(message, 128, "[mod_wasm]: '%s' directive set to '%s'", WASM_DIRECTIVE_WASMROOT, word1);
    trace_nocontext(NULL, __FILE__, __LINE__, message);
    wasm_set_root(word1);
    return NULL;
}


static const char *wasm_directive_WasmModule(cmd_parms *cmd, void *mconfig, const char *word1)
{
    x_cfg *cfg = (x_cfg *) mconfig;

    char message[128];
    snprintf(message, 128, "[mod_wasm]: '%s' directive set to '%s'", WASM_DIRECTIVE_WASMMODULE, word1);
    trace_nocontext(NULL, __FILE__, __LINE__, message);
    wasm_set_module(word1);
    return NULL;
}


static const char *wasm_directive_WasmArg(cmd_parms *cmd, void *mconfig, const char *word1)
{
    x_cfg *cfg = (x_cfg *) mconfig;

    char message[128];
    snprintf(message, 128, "[mod_wasm]: '%s' directive set to '%s'", WASM_DIRECTIVE_WASMARG, word1);
    trace_nocontext(NULL, __FILE__, __LINE__, message);
    wasm_set_arg(word1);
    return NULL;
}


static const char *wasm_directive_WasmEnv(cmd_parms *cmd, void *mconfig, const char *word1, const char *word2)
{
    x_cfg *cfg = (x_cfg *) mconfig;

    char message[128];
    snprintf(message, 128, "[mod_wasm]: '%s' directive set to '%s' = '%s'", WASM_DIRECTIVE_WASMENV, word1, word2);
    trace_nocontext(NULL, __FILE__, __LINE__, message);
    wasm_set_env(word1, word2);
    return NULL;
}


/*
 * List of directives specific to our module.
 */
static const command_rec directives[] =
{
    AP_INIT_TAKE1(
        WASM_DIRECTIVE_WASMROOT,                 /* directive name */
        wasm_directive_WasmRoot,                 /* config action routine */
        NULL,                                    /* argument to include in call */
        OR_OPTIONS,                              /* where available */
        "Set root directory for the Wasm file"   /* directive description */
    ),
    AP_INIT_TAKE1(
        WASM_DIRECTIVE_WASMMODULE,
        wasm_directive_WasmModule,
        NULL,
        OR_OPTIONS,      
        "Set filename for the Wasm Module"
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
    x_create_dir_config,    /* per-directory config creator */
    x_merge_dir_config,     /* dir config merger */
    x_create_server_config, /* server config creator */
    x_merge_server_config,  /* server config merger */
    directives,             /* command table */
    register_hooks,         /* set up other request processing hooks */
};
