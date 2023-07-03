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
use wasmtime_wasi::WasiCtxBuilder;

use crate::apache_bindings::request_rec;
use crate::module::{WASM_RUNTIME_MODULES, WasmModule};
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
pub fn run_wasm_module(wasm_executionctx: &WasmExecutionCtx) -> Result<(),String> {

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

    // build Instance
    let instance: Instance = match linker.instantiate(&mut store, &wasm_module.module) {
        Ok(i) => i,
        Err(e) => {
            let error_msg = format!("ERROR! Can't instantiate module! {}", e);
            return Err(error_msg);
        }
    };

    // get typed function from instance
    let function_name = "_start";
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


pub fn run_wasm_function(wasm_module: &WasmModule, function_name: &str, request: *mut request_rec) -> Result<(),String> {

    // minimum WASI context
    let wasi_ctx = WasiCtxBuilder::new()
        .inherit_stdout()
        .inherit_stderr()
        .build();

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
    match typed_function.call(&mut store, /* (*request_rec)*/ () ) {
        Ok(r) => r,
        Err(t) => {
            let error_msg = format!("ERROR! Invocation of function '{}' failed! Wasm Trap returned! {:?}", function_name, t);
            return Err(error_msg);
        }
    };

    Ok(())
}
