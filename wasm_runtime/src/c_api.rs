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
use crate::config::WasmConfig;
use crate::wasm_engine::run_module;


/// Load a Wasm Module from disk and assign it the given identifier.
///
/// All successfully loaded Wasm modules are stored in a `HashMap`.
/// This implies that:
///  - The `module_id` must be unique.
///  - The `path` must point to an existing file.
///  - The file must be a valid .wasm module.
///
/// In case of error, the reason is printed to stderr and returns -1.
/// Otherwise, it returns 0.
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

    match WasmModule::load_from_file(module_id_str, path_str) {
        Ok(_) => 0,
        Err(e) => {
            eprintln!("C-API: Couldn't load Wasm module \"{}\": {}", module_id_str, e);
            -1
        }
    }
}


/// Add a new Wasm Config with the given unique identifier and for an existing Wasm Module.
///
/// In order to successfully build a new Wasm Config:
///  - The `config_id` must be unique.
///  - The `module_id` must refer to a previously loaded Wasm Module id.
///
/// In case of error, the reason is printed to stderr and returns -1.
/// Otherwise, it returns 0.
///
/// # Examples (C Code)
///
/// ```
/// wasm_config_add("Drupal", "PHP");
/// wasm_config_add("WordPress", "PHP");
/// ```
#[no_mangle]
pub extern "C" fn wasm_config_add(config_id: *const c_char, module_id: *const c_char) -> c_int {
    let config_id_str = const_c_char_to_str(config_id);
    let module_id_str = const_c_char_to_str(module_id);

    match WasmConfig::add_for_module(config_id_str, module_id_str) {
        Ok(_) => 0,
        Err(e) => {
            eprintln!("C-API: Couldn't add Wasm config \"{}\" for module \"{}\": {}", config_id_str, module_id_str, e);
            -1
        }
    }
}


/// Add a WASI argument for the given Wasm config
///
/// Wasm config must has been previously created.
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
/// wasm_config_arg_add("config_id", "--help");
/// ```
#[no_mangle]
pub extern "C" fn wasm_config_arg_add(config_id: *const c_char, arg: *const c_char) -> c_int {
    let config_id_str = const_c_char_to_str(config_id);
    let arg_str = const_c_char_to_str(arg);

    match WasmConfig::add_wasi_arg_for_config(config_id_str, arg_str) {
        Ok(_) => 0,
        Err(e) => {
            eprintln!("C-API: Couldn't add arg \"{}\" for Wasm config \"{}\": {}",  arg_str, config_id_str, e);
            -1
        }
    }
}


/// Add a WASI environment variable for the given Wasm config
///
/// Wasm config must has been previously created.
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
/// wasm_config_env_add("config_id", "TMP", "/tmp");
/// ```
#[no_mangle]
pub extern "C" fn wasm_config_env_add(config_id: *const c_char, env: *const c_char, value: *const c_char) -> c_int {
    let config_id_str = const_c_char_to_str(config_id);
    let env_str       = const_c_char_to_str(env);
    let value_str     = const_c_char_to_str(value);

    match WasmConfig::add_wasi_env_for_config(config_id_str, env_str, value_str) {
        Ok(_) => 0,
        Err(e) => {
            eprintln!("C-API: Couldn't add env \"{}\" for Wasm config \"{}\": {}",  env_str, config_id_str, e);
            -1
        }
    }
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
/// wasm_config_dir_add("config_id", "/tmp");
/// ```
#[no_mangle]
pub extern "C" fn wasm_config_dir_add(config_id: *const c_char, dir: *const c_char) -> c_int {
    let config_id_str = const_c_char_to_str(config_id);
    let dir_str       = const_c_char_to_str(dir);

    match WasmConfig::add_wasi_dir_for_config(config_id_str, dir_str) {
        Ok(_) => 0,
        Err(e) => {
            eprintln!("C-API: Couldn't add dir \"{}\" for Wasm config \"{}\": {}",  dir_str, config_id_str, e);
            -1
        }
    }    
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
/// wasm_config_mapdir_add("config_id", "./", ".");
/// wasm_config_mapdir_add("config_id", "/wasmhome", "/home/wasm_user");
/// wasm_config_mapdir_add("config_id", "/wasmlogs", "/var/log");
/// ```
#[no_mangle]
pub extern "C" fn wasm_config_mapdir_add(config_id: *const c_char, map: *const c_char, dir: *const c_char) -> c_int {
    let config_id_str = const_c_char_to_str(config_id);
    let map_str       = const_c_char_to_str(map);
    let dir_str       = const_c_char_to_str(dir);

    match WasmConfig::add_wasi_mapdir_for_config(config_id_str, map_str, dir_str) {
        Ok(_) => 0,
        Err(e) => {
            eprintln!("C-API: Couldn't add mapdir \"{}\" \"{}\" for Wasm config \"{}\": {}", map_str, dir_str, config_id_str, e);
            -1
        }
    }  
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
/// wasm_config_set_stdin(body_buffer, body_size);
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
