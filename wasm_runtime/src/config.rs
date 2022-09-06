
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