//! **wasm_runtime** is a Rust library intended to:
//! * Integrate with Wasm engines (such as [Wasmtime](https://github.com/bytecodealliance/wasmtime)). 
//! * Provide a thin C API for instantiating, running, and managing Wasm modules.

mod config;
mod wasmengine;
mod wasmtime_shared;
mod wasi_context;
mod ffi_utils;
mod c_api;
