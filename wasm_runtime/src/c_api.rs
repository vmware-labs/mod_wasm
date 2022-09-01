//! c_api.rs
//!
//! This file contains the API functions for the C language

use std::os::raw::c_char;

use crate::ffi_utils::*;
use crate::wasmengine::{init_module, run_module};

use crate::WASM_RUNTIME_CONFIG_ROOT;
use crate::WASM_RUNTIME_CONFIG_MODULE;
use crate::WASM_RUNTIME_CONFIG_WASI_ARGS;
use crate::WASM_RUNTIME_CONFIG_WASI_ENVS;
use crate::WASM_RUNTIME_CONFIG_WASI_DIRS;
use crate::WASM_RUNTIME_CONFIG_WASI_MAPDIRS;

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
/// wasm_set_root("/var/www/wasm");
/// ```
#[no_mangle]
pub extern "C" fn wasm_set_root(path: *const c_char) {
    let path_str = const_c_char_to_str(path);
    WASM_RUNTIME_CONFIG_ROOT.write()
        .expect("ERROR! Poisoned RwLock WASM_RUNTIME_CONFIG_ROOT on write()")
        .replace_range(.., path_str);    
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
/// wasm_set_module("hello.wasm");
/// ```
#[no_mangle]
pub extern "C" fn wasm_set_module(filename: *const c_char) {
    let filename_str = const_c_char_to_str(filename);
    WASM_RUNTIME_CONFIG_MODULE.write()
        .expect("ERROR! Poisoned RwLock WASM_RUNTIME_CONFIG_MODULE on write()")
        .replace_range(.., filename_str);    
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
/// wasm_set_arg("--help");
/// ```
#[no_mangle]
pub extern "C" fn wasm_set_arg(arg: *const c_char) {
    let arg_str   = const_c_char_to_str(arg);
    WASM_RUNTIME_CONFIG_WASI_ARGS.write()
        .expect("ERROR! Poisoned RwLock WASM_RUNTIME_CONFIG_WASI_ARGS on write()")
        .push(arg_str.to_string());
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
/// wasm_set_env("TMP", "/tmp");
/// ```
#[no_mangle]
pub extern "C" fn wasm_set_env(env: *const c_char, value: *const c_char) {
    let env_str   = const_c_char_to_str(env);
    let value_str = const_c_char_to_str(value);
    WASM_RUNTIME_CONFIG_WASI_ENVS.write()
        .expect("ERROR! Poisoned RwLock WASM_RUNTIME_CONFIG_WASI_ENVS on write()")
        .push((env_str.to_string(), value_str.to_string()));
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
/// wasm_set_dir("/tmp");
/// ```
#[no_mangle]
pub extern "C" fn wasm_set_dir(dir: *const c_char) {
    let dir_str   = const_c_char_to_str(dir);
    WASM_RUNTIME_CONFIG_WASI_DIRS.write()
        .expect("ERROR! Poisoned RwLock WASM_RUNTIME_CONFIG_WASI_DIRS on write()")
        .push(dir_str.to_string());
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
/// wasm_set_mapdir("./", ".");
/// wasm_set_mapdir("/wasmhome", "/home/wasm_user");
/// wasm_set_mapdir("/wasmlogs", "/var/log");
/// ```
#[no_mangle]
pub extern "C" fn wasm_set_mapdir(map: *const c_char, dir: *const c_char) {
    let map_str = const_c_char_to_str(map);
    let dir_str = const_c_char_to_str(dir);
    WASM_RUNTIME_CONFIG_WASI_MAPDIRS.write()
        .expect("ERROR! Poisoned RwLock WASM_RUNTIME_CONFIG_WASI_MAPDIRS on write()")
        .push((map_str.to_string(), dir_str.to_string()));
}


/// Initialize the Wasm module
///
/// Returns empty string if initialization was succesfuly.
/// Otherwise, it returns a string with the error.
/// 
#[no_mangle]
pub extern "C" fn wasm_runtime_init_module() -> *const c_char {
    
    let mut return_msg = String::new();

    match init_module() {
        true => (),
        false => {
            return_msg = format!("ERROR: C-API: Can't initialize Wasm module!");
            eprintln!("{}", return_msg);
        }
    };

    str_to_c_char(&return_msg)
}


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


#[no_mangle]
pub extern "C" fn return_const_char_ownership(ptr: *const c_char) {
    deallocate_cstring(ptr);
}