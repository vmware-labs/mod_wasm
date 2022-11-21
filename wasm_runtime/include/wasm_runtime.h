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
 *  - The file must be a valid .wasm module.
 *
 * In case of error, the reason is printed to stderr and returns -1.
 * Otherwise, it returns 0.
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
 * # Examples (C Code)
 *
 * ```
 * wasm_config_new("Drupal", "/var/www/php8.wasm");
 * wasm_config_new("WordPress", "/var/www/php8.wasm");
 * ```
 */
int wasm_config_new(const char *config_id);

/**
 * Set a loaded Wasm Module to an existing Wasm Config.
 *
 * In case of error, the reason is printed to stderr and returns -1.
 * Otherwise, it returns 0.
 *
 * # Examples (C Code)
 *
 * ```
 * wasm_config_set_module("Drupal", "/var/www/php8.wasm");
 * wasm_config_set_module("WordPress", "/var/www/php8.wasm");
 * ```
 */
int wasm_config_set_module(const char *config_id, const char *module_id);

/**
 * Add a WASI argument for the given Wasm config
 *
 * Wasm config must has been previously created.
 *
 * Due to String management differences between C and Rust, this function uses `unsafe {}` code.
 * So `arg` must be a valid pointer to a null-terminated C char array. Otherwise, code might panic.
 *
 * In addition, `arg` must contain valid ASCII chars that can be converted into UTF-8 encoding.
 * Otherwise, the root directory will be an empty string.
 *
 * # Examples (C Code)
 *
 * ```
 * wasm_config_arg_add("config_id", "--help");
 * ```
 */
int wasm_config_arg_add(const char *config_id, const char *arg);

/**
 * Add a WASI environment variable for the given Wasm config
 *
 * Wasm config must has been previously created.
 *
 * Due to String management differences between C and Rust, this function uses `unsafe {}` code.
 * So `env` and `value` must be valid pointers to a null-terminated C char array. Otherwise, code might panic.
 *
 * In addition, `env` and `value` must contain valid ASCII chars that can be converted into UTF-8 encoding.
 * Otherwise, they will trimmed to empty strings.
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
 * Due to String management differences between C and Rust, this function uses `unsafe {}` code.
 * So `dir` must be a valid pointer to a null-terminated C char array. Otherwise, code might panic.
 *
 * In addition, `dir` must contain valid ASCII chars that can be converted into UTF-8 encoding.
 * Otherwise, the root directory will be an empty string.
 *
 * # Examples (C Code)
 *
 * ```
 * wasm_config_dir_add("config_id", "/tmp");
 * ```
 */
int wasm_config_dir_add(const char *config_id, const char *dir);

/**
 * Add a WASI preopen dir with mapping for the Wasm module
 *
 * Due to String management differences between C and Rust, this function uses `unsafe {}` code.
 * So `map` and `dir` must be valid pointers to a null-terminated C char array. Otherwise, code might panic.
 *
 * In addition, `map` and `dir` must contain valid ASCII chars that can be converted into UTF-8 encoding.
 * Otherwise, they will trimmed to empty strings.
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
 * Set the WASI stdin for the Wasm module
 *
 * Due to String management differences between C and Rust, this function uses `unsafe {}` code.
 * So `filename` must be a valid pointer to a null-terminated C char array. Otherwise, code might panic.
 *
 * In addition, `filename` must contain valid ASCII chars that can be converted into UTF-8 encoding.
 * Otherwise, the root directory will be an empty string.
 *
 * # Examples (C Code)
 *
 * ```
 * wasm_config_set_stdin(body_buffer, body_size);
 * ```
 */
void wasm_config_set_stdin(const unsigned char *buffer,
                           uintptr_t size);

/**
 * Run the Wasm module
 *
 * Returns a string with the stdout from the module if execution was succesfuly.
 * Otherwise, trace the error and returns the string explaining the error.
 *
 */
const char *wasm_runtime_run_module(void);

/**
 * Returns raw pointer's ownership
 *
 * After returning a const *char pointer from Rust-world to the C-world, when such a pointer is not going to be used any more,
 * C-world MUST invoke this function in order to Rust-world being able to deallocate the memory.
 * Otherwise, memory will leak.
 *
 */
void return_const_char_ownership(const char *ptr);
