//! **wasm_runtime** is a Rust library intended to:
//! * Integrate with Wasm engines (such as [Wasmtime](https://github.com/bytecodealliance/wasmtime)). 
//! * Provide a thin C API for instantiating, running, and managing Wasm modules.

use std::sync::{Arc, RwLock, Mutex};
use once_cell::sync::Lazy; // https://crates.io/crates/once_cell

use wasmtime::{Engine, Store, Linker, Instance, Module};
use wasi_common::WasiCtx;

use crate::config::WASM_RUNTIME_CONFIG;
use crate::wasi_context::build_wasi_ctx;

// modules
mod config;
mod wasmengine;
mod wasi_context;
mod ffi_utils;
mod c_api;

// The following static variables are used to achieve a global, mutable and thread-safe shareable state.
// For that given purpose, it uses [Once Cell](https://crates.io/crates/once_cell).
// Any object will be protected by `once_cell::sync::Lazy` and `std::sync::{Mutex, RwLock}`.
//
//


// Two different patterns co-live here:
//  1) Lazy<RwLock<T>> is the pattern for static, mutable and shareable state.
//  2) Arc<RwLock<T>> is the type required by WASI to pipe stdout.
// We need stdout buf to be allocated in the data segment as a static variable, 
// so that it can be shared between the Wasmtime initialization plus the Wasm instantiation process,
// and the execution of a Wasm function.
static WASM_RUNTIME_STDOUT_SPTR: Lazy<RwLock<Arc<RwLock<Vec<u8>>>>> = Lazy::new(|| {
    let data = Arc::new(RwLock::new(Vec::new()));
    RwLock::new(data)
});

// Stores the Wasmtime Engine
static WASM_RUNTIME_ENGINE: Lazy<RwLock<Engine>> = Lazy::new(|| {
    let data: Engine = Engine::default();
    RwLock::new(data)
});

// Stores the Wasmtime Store
static WASM_RUNTIME_STORE: Lazy<RwLock<Store<WasiCtx>>> = Lazy::new(|| {
    let engine = WASM_RUNTIME_ENGINE.read()
        .expect("ERROR! Poisoned RwLock WASM_RUNTIME_ENGINE on read()");

    let wasi = build_wasi_ctx();

    let data: Store<WasiCtx> = Store::new(&engine, wasi);
    RwLock::new(data)
});

// Stores the Wasmtime Module Instance
//
// Steps to load a Wasm module and invoking a function:
// 1) Get a wasmtime::Engine object.
//    Engine::default() will return an Engine with default setup (will be likely good enough).
// 2) A wasmtime::Module can be loaded now into memory for the previous Engine.
//    Module::from_file(&engine, modulepath)
// 3) Get a wasmtime::Store object. 
//    A new Store requires a reference to the previous Engine and optionally a WASI context (stdio, envs, args, preopen dirs, etc.)
// 4) Get a mutable wasmtime::Linker object.
//    Optionally, add WASI extension to the Linker via wasmtime_wasi::add_to_linker()
// 5) Request the Linker to instantiate the Module for the given Store. That would return an Instance.
//    linker.instantiate(&mut store, &module)
// 6) Obtain the function to invoke from the Instance and passing the Store.
//    instance.get_typed_func::<(), (), _>(&mut *store, "_start")
static WASM_RUNTIME_INSTANCE: Lazy<RwLock<Instance>> = Lazy::new(|| {
    let engine = WASM_RUNTIME_ENGINE.read()
        .expect("ERROR! Poisoned RwLock WASM_RUNTIME_ENGINE on read()");

    // read Wasm module from file
    let modulepath = build_module_path();
    let module = Module::from_file(&engine, modulepath)
        .expect("ERROR! Wasmtime: Can't load module from file!");

    let mut store = WASM_RUNTIME_STORE.write()
        .expect("ERROR! Poisoned RwLock WASM_RUNTIME_STORE on write()");
    
    // Linker (with WASI extensions)
    let mut linker: Linker<WasiCtx> = Linker::new(&engine);
    wasmtime_wasi::add_to_linker(&mut linker, |cx| cx)
        .expect("ERROR! Wasmtime: Can't add WASI to linker!");

    let data: Instance = linker.instantiate(&mut *store, &module)
        .expect("ERROR! Wasmtime: Can't instantiate module!");

    RwLock::new(data)
});

// Lock for Wasm function invocation 
static WASM_RUNTIME_INVOCATION: Lazy<Mutex<()>> = Lazy::new(|| {
    let data = ();
    Mutex::new(data)
});

fn build_module_path() -> String {
    let wasm_runtime_config = WASM_RUNTIME_CONFIG.read()
        .expect("ERROR! Poisoned RwLock WASM_RUNTIME_CONFIG on read()");

    // module_path = path + "/" + file
    format!("{}/{}", wasm_runtime_config.path, wasm_runtime_config.file)
}