/* See doc at: https://github.com/eqrion/cbindgen/blob/master/docs.md#cbindgentoml

/* Generated with cbindgen:0.24.3 */

/**
 * Set the root directory for loading Wasm modules.
 *
 * Due to String management differences between C and Rust, this funciton uses `unsafe {}` code.
 * So `path` must be a valid pointer to a null-terminated C char array. Otherwise, code might panic.
 *
 * In addition, `path` must contain valid ASCII chars that can be converted into UTF-8 encoding.
 * Otherwise, the root directory will be an empty string.
 *
 * # Examples (C Code)
 *
 * ```
 * wasm_set_root("/var/www/wasm");
 * ```
 */
void wasm_set_root(const char *path);

/**
 * Set the Wasm module name
 *
 * Due to String management differences between C and Rust, this funciton uses `unsafe {}` code.
 * So `filename` must be a valid pointer to a null-terminated C char array. Otherwise, code might panic.
 *
 * In addition, `filename` must contain valid ASCII chars that can be converted into UTF-8 encoding.
 * Otherwise, the root directory will be an empty string.
 *
 * # Examples (C Code)
 *
 * ```
 * wasm_set_module("hello.wasm");
 * ```
 */
void wasm_set_module(const char *filename);

const char *load_and_run(void);

void return_const_char_ownership(const char *ptr);
