//
// Copyright 2022-2023 VMware, Inc.
// SPDX-License-Identifier: Apache-2.0
//

//! `c_api.rs`
//!
//! This file contains the API functions for the C language

use std::ffi::{c_int, c_char, c_uchar, c_ulong};

use crate::module::WasmModule;
use crate::config::WasmConfig;
use crate::execution_ctx::WasmExecutionCtx;
use crate::ffi_utils::*;


/// Load a Wasm Module from disk.
///
/// All successfully loaded Wasm modules are stored in a `HashMap`.
/// This implies that:
///  - The `path` (also used as module's id) must point to an existing file.
///  - The file pointed by `path` must be a valid .wasm module.
///
/// In case of error, the reason is printed to stderr and returns -1.
/// Otherwise, it returns 0.
/// 
/// Due to String management differences between C and Rust, this function uses `unsafe {}` code.
/// So `path` must be a valid pointer to a null-terminated C char array. Otherwise, code might panic.
/// In addition, `path` must contain valid ASCII chars that can be converted into UTF-8 encoding.
///
/// # Examples (C Code)
///
/// ```
/// wasm_module_load("/var/www/wasm/python3_11.wasm");
/// wasm_module_load("/var/www/wasm/php8.wasm");
/// ```
#[no_mangle]
pub extern "C" fn wasm_module_load(path: *const c_char) -> c_int {
    let path_str = const_c_char_to_str(path);

    match WasmModule::load_from_file(path_str) {
        Ok(_) => 0,
        Err(e) => {
            eprintln!("ERROR! C-API: Couldn't load Wasm module at \"{}\": {}", path_str, e);
            -1
        }
    }
}


/// Creates a new Wasm Config given an identifier.
/// The identifier must be unique.
///
/// In case of error, the reason is printed to stderr and returns -1.
/// Otherwise, it returns 0.
/// 
/// Due to String management differences between C and Rust, this function uses `unsafe {}` code.
/// So `config_id` must be a valid pointer to a null-terminated C char array. Otherwise, code might panic.
/// In addition, `config_id` must contain valid ASCII chars that can be converted into UTF-8 encoding.
///
/// # Examples (C Code)
///
/// ```
/// wasm_config_create("Drupal");
/// wasm_config_create("WordPress");
/// ```
#[no_mangle]
pub extern "C" fn wasm_config_create(config_id: *const c_char) -> c_int {
    let config_id_str = const_c_char_to_str(config_id);

    match WasmConfig::create(config_id_str) {
        Ok(_) => 0,
        Err(e) => {
            eprintln!("ERROR! C-API: Couldn't create Wasm Config \"{}\": {}", config_id_str, e);
            -1
        }
    }
}


/// Set a loaded Wasm Module to an existing Wasm Config.
///
/// In case of error, the reason is printed to stderr and returns -1.
/// Otherwise, it returns 0.
/// 
/// Due to String management differences between C and Rust, this function uses `unsafe {}` code.
/// So `config_id` and `module_id` must be a valid pointer to a null-terminated C char array. Otherwise, code might panic.
/// In addition, `config_id` and `module_id` must contain valid ASCII chars that can be converted into UTF-8 encoding.
///
/// # Examples (C Code)
///
/// ```
/// wasm_config_module_set("Drupal", "/var/www/php8.wasm");
/// wasm_config_module_set("WordPress", "/var/www/php8.wasm");
/// ```
#[no_mangle]
pub extern "C" fn wasm_config_module_set(config_id: *const c_char, module_id: *const c_char) -> c_int {
    let config_id_str = const_c_char_to_str(config_id);
    let module_id_str = const_c_char_to_str(module_id);

    match WasmConfig::set_wasm_module_for_config(config_id_str, module_id_str) {
        Ok(_) => 0,
        Err(e) => {
            eprintln!("ERROR! C-API: Couldn't set Wasm module \"{}\" for Wasm Config \"{}\": {}", module_id_str, config_id_str, e);
            -1
        }
    }
}


/// Add a WASI argument for the given Wasm config
///
/// Wasm config must have been previously created.
/// 
/// In case of error, the reason is printed to stderr and returns -1.
/// Otherwise, it returns 0.
/// 
/// Due to String management differences between C and Rust, this function uses `unsafe {}` code.
/// So `config_id` and `arg` must be a valid pointer to a null-terminated C char array. Otherwise, code might panic.
/// In addition, `config_id` and `arg` must contain valid ASCII chars that can be converted into UTF-8 encoding.
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
            eprintln!("ERROR! C-API: Couldn't add arg \"{}\" for Wasm config \"{}\": {}",  arg_str, config_id_str, e);
            -1
        }
    }
}


/// Add a WASI environment variable for the given Wasm config
///
/// Wasm config must have been previously created.
///
/// In case of error, the reason is printed to stderr and returns -1.
/// Otherwise, it returns 0.
/// 
/// Due to String management differences between C and Rust, this function uses `unsafe {}` code.
/// So `config_id`, `env` and `value` must be a valid pointer to a null-terminated C char array. Otherwise, code might panic.
/// In addition, `config_id`, `env` and `value` must contain valid ASCII chars that can be converted into UTF-8 encoding.
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
            eprintln!("ERROR! C-API: Couldn't add env \"{}\" for Wasm config \"{}\": {}",  env_str, config_id_str, e);
            -1
        }
    }
}


/// Add a WASI preopen dir for the Wasm module
///
/// In case of error, the reason is printed to stderr and returns -1.
/// Otherwise, it returns 0.
/// 
/// Due to String management differences between C and Rust, this function uses `unsafe {}` code.
/// So `config_id` and `dir` must be a valid pointer to a null-terminated C char array. Otherwise, code might panic.
/// In addition, `config_id` and `dir` must contain valid ASCII chars that can be converted into UTF-8 encoding.
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
            eprintln!("ERROR! C-API: Couldn't add dir \"{}\" for Wasm config \"{}\": {}",  dir_str, config_id_str, e);
            -1
        }
    }    
}

/// Add a WASI preopen dir with mapping for the Wasm module
///
/// In case of error, the reason is printed to stderr and returns -1.
/// Otherwise, it returns 0.
/// 
/// Due to String management differences between C and Rust, this function uses `unsafe {}` code.
/// So `config_id`, `map` and `dir` must be a valid pointer to a null-terminated C char array. Otherwise, code might panic.
/// In addition, `config_id`, `map` and `dir` must contain valid ASCII chars that can be converted into UTF-8 encoding.
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
            eprintln!("ERROR! C-API: Couldn't add mapdir \"{}\" \"{}\" for Wasm config \"{}\": {}", map_str, dir_str, config_id_str, e);
            -1
        }
    }  
}


/// Creates a new Wasm Execution Context for the given Wasm Config identifier.
///
/// Returns a C string (const *char) with the new generated Wasm Execution Context ID.
/// Otherwise, trace the error and returns a string explaining the error.
///
/// Due to String management differences between C and Rust, this function uses `unsafe {}` code.
/// So `config_id` must be a valid pointer to a null-terminated C char array. Otherwise, code might panic.
/// In addition, `config_id` must contain valid ASCII chars that can be converted into UTF-8 encoding.
/// 
/// Finally, the execution context itself and the returned C string's containing the execution contex are owned by Rust.
/// So, in order to avoid leaking memory, C world must invoke `wasm_executionctx_deallocate()` and `wasm_return_const_char_ownership()`
/// when the execution context and its ID are not needed anymore.
/// 
/// # Examples (C Code)
///
/// ```
/// const char* exec_ctx_id = wasm_executionctx_create_from_config("WordPress");
/// ...
/// // do some work with `exec_ctx_id`
/// ...
/// wasm_executionctx_deallocate(exec_ctx_id);
/// wasm_return_const_char_ownership(exec_ctx_id);
/// ```
#[no_mangle]
pub extern "C" fn wasm_executionctx_create_from_config(config_id: *const c_char) -> *const c_char {
    let config_id_str = const_c_char_to_str(config_id);

    let result = match WasmExecutionCtx::create_from_config(config_id_str) {
        Ok(s) => s,
        Err(e) => {
            let error_msg = format!("ERROR! C-API: Can't build new Wasm execution context from Wasm config: \'{}\'! {:?}", config_id_str, e);
            eprintln!("{}", error_msg);
            error_msg
        }
    };

    str_to_c_char(&result)
}


/// Deallocates the given Wasm execution context
///
/// Wasm execution context must have been previously created.
///
/// In case of error, the reason is printed to stderr and returns -1.
/// Otherwise, it returns 0.
/// 
/// Due to String management differences between C and Rust, this function uses `unsafe {}` code.
/// So `executionctx_id` must be a valid pointer to a null-terminated C char array. Otherwise, code might panic.
/// In addition, `executionctx_id` must contain valid ASCII chars that can be converted into UTF-8 encoding.
///
/// # Examples (C Code)
///
/// ```
/// wasm_executionctx_deallocate("12AB34DC");
/// ```
#[no_mangle]
pub extern "C" fn wasm_executionctx_deallocate(executionctx_id: *const c_char) -> c_int {
    let executionctx_id_str = const_c_char_to_str(executionctx_id);

    match WasmExecutionCtx::deallocate(executionctx_id_str) {
        Ok(_) => 0,
        Err(e) => {
            eprintln!("ERROR! C-API: Couldn't deallocate Wasm execution context \"{}\": {}", executionctx_id_str, e);
            -1
        }
    } 
}

/// Add a WASI environment variable for the given Wasm execution context
///
/// Wasm execution context must have been previously created.
///
/// In case of error, the reason is printed to stderr and returns -1.
/// Otherwise, it returns 0.
/// 
/// Due to String management differences between C and Rust, this function uses `unsafe {}` code.
/// So `executionctx_id`, `env` and `value` must be a valid pointer to a null-terminated C char array. Otherwise, code might panic.
/// In addition, `executionctx_id`, `env` and `value` must contain valid ASCII chars that can be converted into UTF-8 encoding.
///
/// # Examples (C Code)
///
/// ```
/// wasm_executionctx_env_add("12AB34DC", "TMP", "/tmp");
/// ```
#[no_mangle]
pub extern "C" fn wasm_executionctx_env_add(executionctx_id: *const c_char, env: *const c_char, value: *const c_char) -> c_int {
    let executionctx_id_str = const_c_char_to_str(executionctx_id);
    let env_str             = const_c_char_to_str(env);
    let value_str           = const_c_char_to_str(value);

    match WasmExecutionCtx::add_wasi_env_for_executionctx(executionctx_id_str, env_str, value_str) {
        Ok(_) => 0,
        Err(e) => {
            eprintln!("ERROR! C-API: Couldn't add env \"{}\"=\"{}\" for Wasm execution context \"{}\": {}", env_str, value_str, executionctx_id_str, e);
            -1
        }
    } 
}


/// Set the WASI stdin for the given Wasm execution context
///
/// Wasm execution context must have been previously created.
///
/// In case of error, the reason is printed to stderr and returns -1.
/// Otherwise, it returns 0.
/// 
/// Due to String management differences between C and Rust, this function uses `unsafe {}` code.
/// So `executionctx_id` must be a valid pointer to a null-terminated C char array. Otherwise, code might panic.
/// In addition, `executionctx_id` must contain valid ASCII chars that can be converted into UTF-8 encoding.
/// Finally, this funcion can fail if data within the `buffer` is not well aligned or not in sync with `size`.
///
/// # Examples (C Code)
///
/// ```
/// wasm_executionctx_stdin_set("12AB34DC", buffer, buffer_size);
/// ```
#[no_mangle]
pub extern "C" fn wasm_executionctx_stdin_set(executionctx_id: *const c_char, buffer: *const c_uchar, buffer_size: usize) -> c_int {
    let executionctx_id_str = const_c_char_to_str(executionctx_id);
    let stdin_buffer = const_c_char_buffer_to_vec(buffer, buffer_size);

    match WasmExecutionCtx::set_wasi_stdin_for_executionctx(executionctx_id_str, stdin_buffer) {
        Ok(_) => 0,
        Err(e) => {
            eprintln!("ERROR! C-API: Couldn't set stdin for Wasm execution context \"{}\": {}", executionctx_id_str, e);
            -1
        }
    } 
}


/// Run the given Wasm execution context
///
/// In case of error, the reason is printed to stderr and returns -1.
/// Otherwise, it returns 0.
/// 
/// Parameters:
///
/// - `executionctx_id`: Wasm execution context ID. It must have been previously created.
/// - `_buffer`: It's an out-only parameter that represents a C `const char**`. Empty when calling the function.
///   On output, it points to the Wasm execution context output.
/// - `_len`: It's an out-only parameter that represents a C `unsigned long int*`. On output, it contains the buffer length.
///
/// Due to String management differences between C and Rust, this function uses `unsafe {}` code.
/// So `executionctx_id` must be a valid pointer to a null-terminated C char array. Otherwise, code might panic.
/// In addition, `executionctx_id` must contain valid ASCII chars that can be converted into UTF-8 encoding.
/// 
/// The returned `_buffer` can contain more than one NULL terminator ('\0) character (ie. binary files as .png images).
/// 
/// Finally, the memory returned in `_buffer` containing the Wasm module stdout is owned by Rust.
/// So, in order to avoid leaking memory, C world must invoke `wasm_return_const_char_ownership()`
/// when the Wasm module stdout is not needed anymore.
///
/// # Examples (C Code)
///
/// ```
/// const char* module_output = wasm_executionctx_run("12AB34DC");
/// ...
/// // do some work with `module_output`
/// ...
/// wasm_return_const_char_ownership(module_output);
/// ```
#[no_mangle]
pub extern "C" fn wasm_executionctx_run(executionctx_id: *const c_char, _buffer: &mut *const c_char, _len: &mut c_ulong) -> c_int {
    let executionctx_id_str = const_c_char_to_str(executionctx_id);

    let result: c_int = match WasmExecutionCtx::run(executionctx_id_str) {
        Ok(output) => {
            // 1) extract output length and try casting into `c_ulong` since it might not fit
            *_len = match c_ulong::try_from(output.len()) {
                Ok(l) => l,
                Err(e) => {
                    let error_msg = format!("ERROR! C-API: Couldn't cast Wasm module's output length ({}) into a C `unsigned long int` type! {:?}", output.len(), e);
                    eprintln!("{}", error_msg); 
                    return -1; // we end function execution here and ignore the output buffer
                }
            };
            
            // 2) extract output
            *_buffer = vec_u8_to_const_c_char(output);
            0
        }
        Err(e) => {
            let error_msg = format!("ERROR! C-API: Can't run Wasm execution context \'{}\'! {:?}", executionctx_id_str, e);
            eprintln!("{}", error_msg);
            -1
        }
    };

    result
}


/// Returns raw pointer's ownership
///
/// After returning a const *char pointer from Rust-world to the C-world, when such a pointer is not going to be used any more, 
/// C-world MUST invoke this function in order to Rust-world being able to deallocate the memory.
/// Otherwise, memory will leak.
///
#[no_mangle]
pub extern "C" fn wasm_return_const_char_ownership(ptr: *const c_char) {
    deallocate_cstring(ptr);
}
