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
 * Set the Wasm module filename
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

/**
 * Add a WASI arg for the Wasm module
 *
 * Due to String management differences between C and Rust, this funciton uses `unsafe {}` code.
 * So `arg` must be a valid pointer to a null-terminated C char array. Otherwise, code might panic.
 *
 * In addition, `arg` must contain valid ASCII chars that can be converted into UTF-8 encoding.
 * Otherwise, the root directory will be an empty string.
 *
 * # Examples (C Code)
 *
 * ```
 * wasm_set_arg("--help");
 * ```
 */
void wasm_set_arg(const char *arg);

/**
 * Set a WASI environment variable for the Wasm module
 *
 * Due to String management differences between C and Rust, this funciton uses `unsafe {}` code.
 * So `env` and `value` must be valid pointers to a null-terminated C char array. Otherwise, code might panic.
 *
 * In addition, `env` and `value` must contain valid ASCII chars that can be converted into UTF-8 encoding.
 * Otherwise, they will trimmed to empty strings.
 *
 * # Examples (C Code)
 *
 * ```
 * wasm_set_env("TMP", "/tmp");
 * ```
 */
void wasm_set_env(const char *env,
                  const char *value);

const char *load_and_run(void);

void return_const_char_ownership(const char *ptr);
