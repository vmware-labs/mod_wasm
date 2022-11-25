/* See doc at: https://github.com/eqrion/cbindgen/blob/master/docs.md#cbindgentoml

/* Generated with cbindgen:0.24.3 */

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>
#include "version.h"

/**
 * Load a Wasm Module from disk.
 *
 * All successfully loaded Wasm modules are stored in a `HashMap`.
 * This implies that:
 *  - The `path` (also used as module's id) must point to an existing file.
 *  - The file pointed by `path` must be a valid .wasm module.
 *
 * In case of error, the reason is printed to stderr and returns -1.
 * Otherwise, it returns 0.
 *
 * Due to String management differences between C and Rust, this function uses `unsafe {}` code.
 * So `path` must be a valid pointer to a null-terminated C char array. Otherwise, code might panic.
 * In addition, `path` must contain valid ASCII chars that can be converted into UTF-8 encoding.
 *
 * # Examples (C Code)
 *
 * ```
 * wasm_module_load("/var/www/wasm/python3_11.wasm");
 * wasm_module_load("/var/www/wasm/php8.wasm");
 * ```
 */
int wasm_module_load(const char *path);

/**
 * Creates a new Wasm Config given an identifier.
 * The identifier must be unique.
 *
 * In case of error, the reason is printed to stderr and returns -1.
 * Otherwise, it returns 0.
 *
 * Due to String management differences between C and Rust, this function uses `unsafe {}` code.
 * So `config_id` must be a valid pointer to a null-terminated C char array. Otherwise, code might panic.
 * In addition, `config_id` must contain valid ASCII chars that can be converted into UTF-8 encoding.
 *
 * # Examples (C Code)
 *
 * ```
 * wasm_config_create("Drupal");
 * wasm_config_create("WordPress");
 * ```
 */
int wasm_config_create(const char *config_id);

/**
 * Set a loaded Wasm Module to an existing Wasm Config.
 *
 * In case of error, the reason is printed to stderr and returns -1.
 * Otherwise, it returns 0.
 *
 * Due to String management differences between C and Rust, this function uses `unsafe {}` code.
 * So `config_id` and `module_id` must be a valid pointer to a null-terminated C char array. Otherwise, code might panic.
 * In addition, `config_id` and `module_id` must contain valid ASCII chars that can be converted into UTF-8 encoding.
 *
 * # Examples (C Code)
 *
 * ```
 * wasm_config_set_module("Drupal", "/var/www/php8.wasm");
 * wasm_config_set_module("WordPress", "/var/www/php8.wasm");
 * ```
 */
int wasm_config_set_module(const char *config_id,
                           const char *module_id);

/**
 * Add a WASI argument for the given Wasm config
 *
 * Wasm config must have been previously created.
 *
 * In case of error, the reason is printed to stderr and returns -1.
 * Otherwise, it returns 0.
 *
 * Due to String management differences between C and Rust, this function uses `unsafe {}` code.
 * So `config_id` and `arg` must be a valid pointer to a null-terminated C char array. Otherwise, code might panic.
 * In addition, `config_id` and `arg` must contain valid ASCII chars that can be converted into UTF-8 encoding.
 *
 * # Examples (C Code)
 *
 * ```
 * wasm_config_arg_add("config_id", "--help");
 * ```
 */
int wasm_config_arg_add(const char *config_id,
                        const char *arg);

/**
 * Add a WASI environment variable for the given Wasm config
 *
 * Wasm config must have been previously created.
 *
 * In case of error, the reason is printed to stderr and returns -1.
 * Otherwise, it returns 0.
 *
 * Due to String management differences between C and Rust, this function uses `unsafe {}` code.
 * So `config_id`, `env` and `value` must be a valid pointer to a null-terminated C char array. Otherwise, code might panic.
 * In addition, `config_id`, `env` and `value` must contain valid ASCII chars that can be converted into UTF-8 encoding.
 *
 * # Examples (C Code)
 *
 * ```
 * wasm_config_env_add("config_id", "TMP", "/tmp");
 * ```
 */
int wasm_config_env_add(const char *config_id,
                        const char *env,
                        const char *value);

/**
 * Add a WASI preopen dir for the Wasm module
 *
 * In case of error, the reason is printed to stderr and returns -1.
 * Otherwise, it returns 0.
 *
 * Due to String management differences between C and Rust, this function uses `unsafe {}` code.
 * So `config_id` and `dir` must be a valid pointer to a null-terminated C char array. Otherwise, code might panic.
 * In addition, `config_id` and `dir` must contain valid ASCII chars that can be converted into UTF-8 encoding.
 *
 * # Examples (C Code)
 *
 * ```
 * wasm_config_dir_add("config_id", "/tmp");
 * ```
 */
int wasm_config_dir_add(const char *config_id,
                        const char *dir);

/**
 * Add a WASI preopen dir with mapping for the Wasm module
 *
 * In case of error, the reason is printed to stderr and returns -1.
 * Otherwise, it returns 0.
 *
 * Due to String management differences between C and Rust, this function uses `unsafe {}` code.
 * So `config_id`, `map` and `dir` must be a valid pointer to a null-terminated C char array. Otherwise, code might panic.
 * In addition, `config_id`, `map` and `dir` must contain valid ASCII chars that can be converted into UTF-8 encoding.
 *
 * # Examples (C Code)
 *
 * ```
 * wasm_config_mapdir_add("config_id", "./", ".");
 * wasm_config_mapdir_add("config_id", "/wasmhome", "/home/wasm_user");
 * wasm_config_mapdir_add("config_id", "/wasmlogs", "/var/log");
 * ```
 */
int wasm_config_mapdir_add(const char *config_id,
                           const char *map,
                           const char *dir);

/**
 * Creates a new Wasm Execution Context for the given Wasm Config identifier.
 *
 * Returns a C string (const *char) with the the new generated Wasm Execution Context ID.
 * Otherwise, trace the error and returns a string explaining the error.
 *
 * Due to String management differences between C and Rust, this function uses `unsafe {}` code.
 * So `config_id` must be a valid pointer to a null-terminated C char array. Otherwise, code might panic.
 * In addition, `config_id` must contain valid ASCII chars that can be converted into UTF-8 encoding.
 *
 * Finally, the execution context itself and the returned C string's containing the execution contex are owneed by Rust.
 * So, in order to avoid leaking memory, C world must invoke `wasm_executionctx_deallocate()` and `wasm_return_const_char_ownership()`
 * when the execution context and its ID are not needed anymore.
 *
 * # Examples (C Code)
 *
 * ```
 * const char* exec_ctx_id = wasm_executionctx_create_from_config("WordPress");
 * ...
 * // do some work with `exec_ctx_id`
 * ...
 * wasm_executionctx_deallocate(exec_ctx_id);
 * wasm_return_const_char_ownership(exec_ctx_id);
 * ```
 */
const char *wasm_executionctx_create_from_config(const char *config_id);

/**
 * Deallocates the given Wasm execution context
 *
 * Wasm execution context must have been previously created.
 *
 * In case of error, the reason is printed to stderr and returns -1.
 * Otherwise, it returns 0.
 *
 * Due to String management differences between C and Rust, this function uses `unsafe {}` code.
 * So `executionctx_id` must be a valid pointer to a null-terminated C char array. Otherwise, code might panic.
 * In addition, `executionctx_id` must contain valid ASCII chars that can be converted into UTF-8 encoding.
 *
 * # Examples (C Code)
 *
 * ```
 * wasm_executionctx_deallocate("12AB34DC");
 * ```
 */
int wasm_executionctx_deallocate(const char *executionctx_id);

/**
 * Add a WASI environment variable for the given Wasm execution context
 *
 * Wasm execution context must have been previously created.
 *
 * In case of error, the reason is printed to stderr and returns -1.
 * Otherwise, it returns 0.
 *
 * Due to String management differences between C and Rust, this function uses `unsafe {}` code.
 * So `executionctx_id`, `env` and `value` must be a valid pointer to a null-terminated C char array. Otherwise, code might panic.
 * In addition, `executionctx_id`, `env` and `value` must contain valid ASCII chars that can be converted into UTF-8 encoding.
 *
 * # Examples (C Code)
 *
 * ```
 * wasm_executionctx_env_add("12AB34DC", "TMP", "/tmp");
 * ```
 */
int wasm_executionctx_env_add(const char *executionctx_id,
                              const char *env,
                              const char *value);

/**
 * Set the WASI stdin for the given Wasm execution context
 *
 * Wasm execution context must have been previously created.
 *
 * In case of error, the reason is printed to stderr and returns -1.
 * Otherwise, it returns 0.
 *
 * Due to String management differences between C and Rust, this function uses `unsafe {}` code.
 * So `executionctx_id` must be a valid pointer to a null-terminated C char array. Otherwise, code might panic.
 * In addition, `executionctx_id` must contain valid ASCII chars that can be converted into UTF-8 encoding.
 * Finally, this funcion can fail if data within the `buffer` is not well aligned or not in sync with `size`.
 *
 * # Examples (C Code)
 *
 * ```
 * wasm_executionctx_stdin_set("12AB34DC", buffer, buffer_size);
 * ```
 */
int wasm_executionctx_stdin_set(const char *executionctx_id,
                                const unsigned char *buffer,
                                uintptr_t buffer_size);

/**
 * Run the given Wasm execution context
 *
 * Returns a string with the stdout from the Wasm module if execution was succesfuly.
 * Otherwise, trace the error and returns a string explaining the error.
 *
 * Due to String management differences between C and Rust, this function uses `unsafe {}` code.
 * So `executionctx_id` must be a valid pointer to a null-terminated C char array. Otherwise, code might panic.
 * In addition, `executionctx_id` must contain valid ASCII chars that can be converted into UTF-8 encoding.
 *
 * Finally, the returned C string's containing the Wasm module stdout is owneed by Rust.
 * So, in order to avoid leaking memory, C world must invoke `wasm_return_const_char_ownership()`
 * when the Wasm module stdout is not needed anymore.
 *
 * # Examples (C Code)
 *
 * ```
 * const char* module_output = wasm_executionctx_run("12AB34DC");
 * ...
 * // do some work with `module_output`
 * ...
 * wasm_return_const_char_ownership(module_output);
 * ```
 */
const char *wasm_executionctx_run(const char *executionctx_id);

/**
 * Returns raw pointer's ownership
 *
 * After returning a const *char pointer from Rust-world to the C-world, when such a pointer is not going to be used any more,
 * C-world MUST invoke this function in order to Rust-world being able to deallocate the memory.
 * Otherwise, memory will leak.
 *
 */
void wasm_return_const_char_ownership(const char *ptr);
