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

// config.rs
//
// Struct to store Wasm Runtime configuration (including WASI)

use std::path::PathBuf;
use std::sync::RwLock;
use once_cell::sync::Lazy; // https://crates.io/crates/once_cell

pub struct WasmRuntimeConfig {
    pub path:         PathBuf,
    pub file:         String,
    pub wasi_args:    Vec<String>,
    pub wasi_envs:    Vec<(String, String)>,
    pub wasi_dirs:    Vec<String>,
    pub wasi_mapdirs: Vec<(String, String)>,
}

// The following static variable is used to achieve a global, mutable and thread-safe shareable state.
// For that given purpose, it uses [Once Cell](https://crates.io/crates/once_cell).
// Any object will be protected by `once_cell::sync::Lazy` and `std::sync::{Mutex, RwLock}`.
//

// Stores the Wasm Runtime configuration (including WASI)
pub static WASM_RUNTIME_CONFIG: Lazy<RwLock<WasmRuntimeConfig>> = Lazy::new(|| {
    let data: WasmRuntimeConfig = WasmRuntimeConfig {
        path:         PathBuf::new(),
        file:         String::new(),
        wasi_args:    Vec::new(),
        wasi_envs:    Vec::new(),
        wasi_dirs:    Vec::new(),
        wasi_mapdirs: Vec::new(),
    };

    RwLock::new(data)
});