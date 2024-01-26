//
// Copyright 2022-2023 VMware, Inc.
// SPDX-License-Identifier: Apache-2.0
//

// wasm_engine.rs
//
// Using Wasmtime from the Bytecode Alliance as the Wasm Engine
// https://github.com/bytecodealliance/wasmtime

use anyhow::Result;
use wasi_common::WasiCtx;
use wasmtime::{Store, Linker, Instance, TypedFunc};

use crate::{module::WASM_RUNTIME_MODULES, ffi_utils};
use crate::execution_ctx::WasmExecutionCtx;
use crate::wasi_ctx;


/// Invoke the requested Wasm function for the given Wasm execution context
///
// 
// `invoke_wasm_function()` will extract Module and and Engine from the Wasm execution context,
// and it will create WasiCtx, Linker, Store, Instance and Typed_Function.
// 
// See below the Wasmtime object's dependcy graph:
// Typed_Funct ---> Store, Instance
//       Store ---> Engine, WasiCtx
//    Instance ---> Module, Linker, Store
//      Module ---> Engine
//      Linker ---> Engine
//      Engine ---> 0
//     WasiCtx ---> 0
//
// Steps to load a Wasm module and invoking a function:
// 1) Get a wasmtime::Engine object.
//    Engine::default() will return an Engine with default setup (will be likely good enough).
// 2) A wasmtime::Module can be loaded now into memory for the previous Engine.
//    This is probably them most time-consuming step, so we will execute it at the init stage.
//    Module::from_file(&engine, modulepath)
// 3) Get a wasmtime::Store object. 
//    A new Store requires a reference to the previous Engine and optionally a WASI context (stdio, envs, args, preopen dirs, etc.)
// 4) Get a mutable wasmtime::Linker object.
//    Optionally, add WASI extension to the Linker via wasmtime_wasi::add_to_linker()
// 5) Request the Linker to instantiate the Module for the given Store. That would return an Instance.
//    linker.instantiate(&mut store, &module)
// 6) Obtain the function to invoke from the Instance and passing the Store.
//    instance.get_typed_func::<(), (), _>(&mut *store, "_start")
//
pub fn invoke_wasm_function(wasm_executionctx: &WasmExecutionCtx, function_name: &str) -> Result<(),String> {

    // get read access to the WasmExecutionCtx HashMap
    let modules = WASM_RUNTIME_MODULES.read()
        .expect("ERROR! Poisoned RwLock WASM_RUNTIME_MODULES on read()");

    // get the Wasm module object referred into the execution context
    let wasm_module = match modules.get(&wasm_executionctx.module_id) {
        Some(w) => w,
        None => {
            let error_msg = format!("ERROR! Wasm Module \'{}\' referred by execution context \'{}\' not found!", wasm_executionctx.module_id, wasm_executionctx.id);
            return Err(error_msg); 
        }
    };

    // build WasiCtx
    let wasi_ctx = match wasi_ctx::build(wasm_executionctx, wasm_module) {
        Ok(ctx) => ctx,
        Err(e) => {
            let error_msg = format!("ERROR! Couldn't build WASI Context for \'{}\'! {}", wasm_executionctx.config_id, e);
            return Err(error_msg); 
        }
    };

    // build Store
    let mut store: Store<WasiCtx> = Store::new(&wasm_module.engine, wasi_ctx);

    // build Linker (with WASI extensions)
    let mut linker: Linker<WasiCtx> = Linker::new(&wasm_module.engine);
    match wasmtime_wasi::add_to_linker(&mut linker, |cx| cx) {
        Ok(_) => (),
        Err(e) => {
            let error_msg = format!("ERROR! Can't add WASI extensions to Wasmtime::Linker! {}", e);
            return Err(error_msg);
        }
    };

    // add host functions
    register_host_functions(&mut linker);

    // build Instance
    let instance: Instance = match linker.instantiate(&mut store, &wasm_module.module) {
        Ok(i) => i,
        Err(e) => {
            let error_msg = format!("ERROR! Can't instantiate module! {}", e);
            return Err(error_msg);
        }
    };

    // get typed function from instance
    let typed_function: TypedFunc<(), ()> = match instance.get_typed_func::<(), ()>(&mut store, function_name) {
        Ok(tp) => tp,
        Err(e) => {
            let error_msg = format!("ERROR! Can't get typed function '{}' from instance! {}", function_name, e);
            return Err(error_msg);
        }
    };

    // invoke function    
    match typed_function.call(&mut store, ()) {
        Ok(r) => r,
        Err(t) => {
            let error_msg = format!("ERROR! Invocation of function '{}' failed! Wasm Trap returned! {:?}", function_name, t);
            return Err(error_msg);
        }
    };

    Ok(())
}

use std::ffi::{c_char};
use wasmtime::Caller;

// https://docs.wasmtime.dev/api/wasmtime/struct.Linker.html#method.func_wrap
// https://docs.wasmtime.dev/api/wasmtime/struct.Func.html#method.wrap

fn register_host_functions(linker: &mut Linker<WasiCtx>) {
    // pub fn host_say_hello();
    linker.func_wrap(
        "host_functions_demo",
        "say_hello",
        || {
            host_say_hello();
        }
    ).unwrap();

    linker.func_wrap(
        "host_functions_demo",
        "negate_number",
        |number: i32| -> i32 {
            host_negate_number(number as isize) as i32
        }
    ).unwrap();

    linker.func_wrap(
        "host_functions_demo",
        "upper_case",
        |mut caller: Caller<'_, WasiCtx>, input_ptr: i32| -> i32 {
            // get input parameter from input_ptr (need to read from linear memory)
            let memory = caller.get_export("memory").unwrap().into_memory().unwrap();
            let input_str = unsafe { memory.data_ptr(&caller).offset(input_ptr as isize) };

            // execute the inner host function
            let output = host_upper_case(input_str as *const c_char);
            let output_bytes = ffi_utils::const_c_char_to_bytes(output);

            // allocate and write the result into linear memory
            let allocate_func = caller.get_export("allocate")
                                                        .unwrap()
                                                        .into_func()
                                                        .unwrap()
                                                        .typed::<i32, i32>(&caller)
                                                        .unwrap();
            let ptr = allocate_func.call(&mut caller, output_bytes.len() as i32).unwrap();
            memory.write(&mut caller, ptr as usize, output_bytes).ok();

            ptr as i32
        },
    ).ok();
}


fn host_say_hello() {
    println!("[Host@stdout] Hello!");
    eprintln!("[Host@stderr] Hello!");
}

fn host_negate_number(number: isize) -> isize { 
    number * -1
}

fn host_upper_case(input: *const c_char) -> *const c_char {
    let input_str = ffi_utils::const_c_char_to_str(input);
    let output = input_str.to_uppercase();
    let output_str = ffi_utils::str_to_c_char(output.as_str());

    output_str
}