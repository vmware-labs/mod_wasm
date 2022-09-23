/* Licensed to the Apache Software Foundation (ASF) under one or more
 * contributor license agreements.  See the NOTICE file distributed with
 * this work for additional information regarding copyright ownership.
 * The ASF licenses this file to You under the Apache License, Version 2.0
 * (the "License"); you may not use this file except in compliance with
 * the License.  You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

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
