//
// Copyright 2022 VMware, Inc.
// SPDX-License-Identifier: Apache-2.0
//

// module.rs
//
// Struct to store Wasm Module

use std::path::PathBuf;
use std::collections::HashMap;
use std::sync::RwLock;

use once_cell::sync::Lazy;
use wasmtime::{Engine, Module};


pub struct WasmModule {
    pub id:     String,
    pub path:   PathBuf,
    pub engine: wasmtime::Engine,
    pub module: wasmtime::Module,
}

impl WasmModule {
    /// Load a Wasm Module from file into memory
    ///
    /// It checks for double loads, duplicated `module_id` or wrong file format.
    /// Returns Result<(), String>, so that in case of error the String will contain the reason.
    /// 
    pub fn load_from_file(module_id: &str, path: &str) -> Result<(), String> {

        // get write access to the WasmModule HashMap
        let mut modules = WASM_RUNTIME_MODULES.write()
            .expect("ERROR! Poisoned RwLock WASM_RUNTIME_MODULES on write()");

        // check for existing module_id in the loaded modules
        if let Some(wasm_module) = modules.get(module_id) {
            // same id?
            if wasm_module.id == module_id {    // redundant but it's the WasmModule which really owns its ID, not the Key in the HashMap
                // same path? then it's being loaded twice
                if wasm_module.path == PathBuf::from(path) {
                    // TO-DO: the commented lines below should be the right behaviour.
                    // But since dry-run is not supported yet in mod_wasm.c, it's preferible to turn this check off
                    // See issue #26: https://github.com/vmware-labs/mod_wasm/issues/26
                    //
                    // let error_msg = format!("Wasm module \'{}\' is already loaded, skipping", module_id);
                    // return Err(error_msg);
                    //
                }
                // different path? then module_id is already in use by another Wasm module
                else {
                    let error_msg = format!("Wasm module ID \'{}\' is already in use for: {}", module_id, wasm_module.path.display());
                    return Err(error_msg);
                }
            }
        }
        
        // check path is valid
        let module_path = PathBuf::from(path);
        if ! module_path.is_file() {
            let error_msg = format!("Can't find path: {}", module_path.display());
            return Err(error_msg);
        }

        // load a Wasmtime Engine with default configuration
        let module_engine = Engine::default();

        // try load module on the Wasmtime runtime
        let wasmtime_module = match Module::from_file(&module_engine, module_path.clone()) {
            Ok(m) => m,
            Err(e) => {
                let error_msg = format!("Can't load module `{}`! {}", module_id, e);
                return Err(error_msg);  
            }
        };
        
        // build the WasmModule object
        let wasm_module = WasmModule {
            id: module_id.to_string(),
            path: module_path,
            engine: module_engine,
            module: wasmtime_module,
        };

        // insert into the HasmMap
        modules.insert(wasm_module.id.clone(), wasm_module);

        Ok(())

    }
}


// The following static variable is used to achieve a global, mutable and thread-safe shareable state.
// For that given purpose, it uses [Once Cell](https://crates.io/crates/once_cell).
// Any object will be protected by `once_cell::sync::Lazy` and `std::sync::{Mutex, RwLock}`.
//

// Stores the loaded Wasm Modules 
pub static WASM_RUNTIME_MODULES: Lazy<RwLock<HashMap<String, WasmModule>>> = Lazy::new(|| {
    let data: HashMap<String, WasmModule> = HashMap::new();
    RwLock::new(data)
});
