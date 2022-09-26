//
// Copyright 2022 VMware, Inc.
// SPDX-License-Identifier: Apache-2.0
//

// wasmtime_shared.rs
//
// Wasmtime static mutable shared objects that will be common to different threads

use std::sync::RwLock;
use once_cell::sync::Lazy; // https://crates.io/crates/once_cell
use wasmtime::{Engine, Module};


pub struct WasmTimeSharedObjects {
    pub engine: Engine,
    pub module: Option<Module>,
}

// The following static variables are used to achieve a global, mutable and thread-safe shareable state.
// For that given purpose, it uses [Once Cell](https://crates.io/crates/once_cell).
// Any object will be protected by `once_cell::sync::Lazy` and `std::sync::{Mutex, RwLock}`.

// Stores the Wasmtime shared objects between invocations: Engine and Module
// At startup, module is 'None' since we don't know yet the Wasm file that will be loaded
pub static WASMTIME_SHARED_OBJECTS: Lazy<RwLock<WasmTimeSharedObjects>> = Lazy::new(|| {
    let data: WasmTimeSharedObjects = WasmTimeSharedObjects {
        engine: Engine::default(),
        module: None,
    };

    RwLock::new(data)
});
