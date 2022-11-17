//
// Copyright 2022 VMware, Inc.
// SPDX-License-Identifier: Apache-2.0
//

//! `c_api.rs`
//!
//! This file contains the API functions for the C language

use std::ffi::{c_int, c_char, c_uchar};
use std::slice;


use crate::ffi_utils::*;
use crate::module::WasmModule;
use crate::config::WASM_RUNTIME_CONFIG;
use crate::wasm_engine::{run_module};


/// Load a Wasm Module from disk and assign it the given identifier.
///
/// All successfully loaded Wasm modules are stored in a `HashMap`.
/// This implies that:
///  - The `module_id` cannot be used more than once.
///  - The `path` must point to an existing file.
///  - The file must be a valid .wasm module.
///
/// In case of error, it returns a string explaining the error.
/// Otherwise, it returns an empty string.
///
/// # Examples (C Code)
///
/// ```
/// wasm_module_load("python", "/var/www/wasm/python3_11.wasm");
/// wasm_module_load("PHP", "/var/www/wasm/php8.wasm");
/// ```
#[no_mangle]
pub extern "C" fn wasm_module_load(module_id: *const c_char, path: *const c_char) -> c_int {
    let module_id_str = const_c_char_to_str(module_id);
    let path_str = const_c_char_to_str(path);

    let result: c_int = match WasmModule::from_file(module_id_str, path_str) {
        Ok(_) => {
            0
        },
        Err(e) => {
            eprintln!("C-API: Couldn't load Wasm module \"{}\": {}", module_id_str, e);
            -1
        }
    };

    result
}


/// Set the root directory for loading Wasm modules.
///
/// Due to String management differences between C and Rust, this function uses `unsafe {}` code.
/// So `path` must be a valid pointer to a null-terminated C char array. Otherwise, code might panic.
///
/// In addition, `path` must contain valid ASCII chars that can be converted into UTF-8 encoding.
/// Otherwise, the root directory will be an empty string.
///
/// # Examples (C Code)
///
/// ```
/// wasm_config_set_root("/var/www/wasm");
/// ```
#[no_mangle]
pub extern "C" fn wasm_config_set_root(path: *const c_char) {
    let path_str = const_c_char_to_str(path);

    let mut config = WASM_RUNTIME_CONFIG.write()
        .expect("ERROR! Poisoned RwLock WASM_RUNTIME_CONFIG on write()");

    config.path.clear();
    config.path.push(path_str);
}

/// Set the Wasm module filename
///
/// Due to String management differences between C and Rust, this function uses `unsafe {}` code.
/// So `filename` must be a valid pointer to a null-terminated C char array. Otherwise, code might panic.
///
/// In addition, `filename` must contain valid ASCII chars that can be converted into UTF-8 encoding.
/// Otherwise, the root directory will be an empty string.
///
/// # Examples (C Code)
///
/// ```
/// wasm_config_set_module("hello.wasm");
/// ```
#[no_mangle]
pub extern "C" fn wasm_config_set_module(filename: *const c_char) {
    let filename_str = const_c_char_to_str(filename);

    WASM_RUNTIME_CONFIG.write()
        .expect("ERROR! Poisoned RwLock WASM_RUNTIME_CONFIG on write()")
        .file
        .replace_range(.., filename_str);
}

/// Clears all WASI args for the Wasm module
#[no_mangle]
pub extern "C" fn wasm_config_clear_args() {
    WASM_RUNTIME_CONFIG.write()
        .expect("ERROR! Poisoned RwLock WASM_RUNTIME_CONFIG on write()")
        .wasi_args
        .clear();
}

/// Add a WASI arg for the Wasm module
///
/// Due to String management differences between C and Rust, this function uses `unsafe {}` code.
/// So `arg` must be a valid pointer to a null-terminated C char array. Otherwise, code might panic.
///
/// In addition, `arg` must contain valid ASCII chars that can be converted into UTF-8 encoding.
/// Otherwise, the root directory will be an empty string.
///
/// # Examples (C Code)
///
/// ```
/// wasm_config_add_arg("--help");
/// ```
#[no_mangle]
pub extern "C" fn wasm_config_add_arg(arg: *const c_char) {
    let arg_str   = const_c_char_to_str(arg);

    WASM_RUNTIME_CONFIG.write()
        .expect("ERROR! Poisoned RwLock WASM_RUNTIME_CONFIG on write()")
        .wasi_args
        .push(arg_str.to_string());
}

/// Clears all WASI environment variables for the Wasm module
#[no_mangle]
pub extern "C" fn wasm_config_clear_envs() {
    WASM_RUNTIME_CONFIG.write()
        .expect("ERROR! Poisoned RwLock WASM_RUNTIME_CONFIG on write()")
        .wasi_envs
        .clear();
}

/// Set a WASI environment variable for the Wasm module
///
/// Due to String management differences between C and Rust, this function uses `unsafe {}` code.
/// So `env` and `value` must be valid pointers to a null-terminated C char array. Otherwise, code might panic.
///
/// In addition, `env` and `value` must contain valid ASCII chars that can be converted into UTF-8 encoding.
/// Otherwise, they will trimmed to empty strings.
///
/// # Examples (C Code)
///
/// ```
/// wasm_config_add_env("TMP", "/tmp");
/// ```
#[no_mangle]
pub extern "C" fn wasm_config_add_env(env: *const c_char, value: *const c_char) {
    let env_str   = const_c_char_to_str(env);
    let value_str = const_c_char_to_str(value);

    WASM_RUNTIME_CONFIG.write()
        .expect("ERROR! Poisoned RwLock WASM_RUNTIME_CONFIG on write()")
        .wasi_envs
        .push((env_str.to_string(), value_str.to_string()));

}

/// Clears all WASI preopened dirs for the Wasm module
#[no_mangle]
pub extern "C" fn wasm_config_clear_dirs() {
    WASM_RUNTIME_CONFIG.write()
        .expect("ERROR! Poisoned RwLock WASM_RUNTIME_CONFIG on write()")
        .wasi_dirs
        .clear();
}

/// Add a WASI preopen dir for the Wasm module
///
/// Due to String management differences between C and Rust, this function uses `unsafe {}` code.
/// So `dir` must be a valid pointer to a null-terminated C char array. Otherwise, code might panic.
///
/// In addition, `dir` must contain valid ASCII chars that can be converted into UTF-8 encoding.
/// Otherwise, the root directory will be an empty string.
///
/// # Examples (C Code)
///
/// ```
/// wasm_config_add_dir("/tmp");
/// ```
#[no_mangle]
pub extern "C" fn wasm_config_add_dir(dir: *const c_char) {
    let dir_str   = const_c_char_to_str(dir);

    WASM_RUNTIME_CONFIG.write()
        .expect("ERROR! Poisoned RwLock WASM_RUNTIME_CONFIG on write()")
        .wasi_dirs
        .push(dir_str.to_string());
}

/// Clears all WASI propened dirs with mapping for the Wasm module
#[no_mangle]
pub extern "C" fn wasm_config_clear_mapdirs() {
    WASM_RUNTIME_CONFIG.write()
        .expect("ERROR! Poisoned RwLock WASM_RUNTIME_CONFIG on write()")
        .wasi_mapdirs
        .clear();
}

/// Add a WASI preopen dir with mapping for the Wasm module
///
/// Due to String management differences between C and Rust, this function uses `unsafe {}` code.
/// So `map` and `dir` must be valid pointers to a null-terminated C char array. Otherwise, code might panic.
///
/// In addition, `map` and `dir` must contain valid ASCII chars that can be converted into UTF-8 encoding.
/// Otherwise, they will trimmed to empty strings.
///
/// # Examples (C Code)
///
/// ```
/// wasm_config_add_mapdir("./", ".");
/// wasm_config_add_mapdir("/wasmhome", "/home/wasm_user");
/// wasm_config_add_mapdir("/wasmlogs", "/var/log");
/// ```
#[no_mangle]
pub extern "C" fn wasm_config_add_mapdir(map: *const c_char, dir: *const c_char) {
    let map_str = const_c_char_to_str(map);
    let dir_str = const_c_char_to_str(dir);

    WASM_RUNTIME_CONFIG.write()
        .expect("ERROR! Poisoned RwLock WASM_RUNTIME_CONFIG on write()")
        .wasi_mapdirs
        .push((map_str.to_string(), dir_str.to_string()));
}


/// Set the WASI stdin for the Wasm module
///
/// Due to String management differences between C and Rust, this function uses `unsafe {}` code.
/// So `filename` must be a valid pointer to a null-terminated C char array. Otherwise, code might panic.
///
/// In addition, `filename` must contain valid ASCII chars that can be converted into UTF-8 encoding.
/// Otherwise, the root directory will be an empty string.
///
/// # Examples (C Code)
///
/// ```
/// wasm_config_set_module("hello.wasm");
/// ```
#[no_mangle]
pub extern "C" fn wasm_config_set_stdin(buffer: *const c_uchar, size: usize) {
    let bytes = unsafe { slice::from_raw_parts(buffer, size) };
    let bytes_vec: Vec<u8> = Vec::from(bytes);

    WASM_RUNTIME_CONFIG.write()
        .expect("ERROR! Poisoned RwLock WASM_RUNTIME_CONFIG on write()")
        .wasi_stdin = bytes_vec;
}


/// Run the Wasm module
///
/// Returns a string with the stdout from the module if execution was succesfuly.
/// Otherwise, trace the error and returns the string explaining the error.
///
#[no_mangle]
pub extern "C" fn wasm_runtime_run_module() -> *const c_char {
    let result = match run_module() {
        Ok(s) => s,
        Err(e) => {
            let error_msg = format!("ERROR: C-API: Can't run Wasm module! {:?}", e);
            eprintln!("{}", error_msg);
            error_msg
        }
    };

    str_to_c_char(&result)
}


/// Returns raw pointer's ownership
///
/// After returning a const *char pointer from Rust-world to the C-world, when such a pointer is not going to be used any more, 
/// C-world MUST invoke this function in order to Rust-world being able to deallocate the memory.
/// Otherwise, memory will leak.
///
#[no_mangle]
pub extern "C" fn return_const_char_ownership(ptr: *const c_char) {
    deallocate_cstring(ptr);
}
