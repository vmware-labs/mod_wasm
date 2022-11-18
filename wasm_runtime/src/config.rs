//
// Copyright 2022 VMware, Inc.
// SPDX-License-Identifier: Apache-2.0
//

// config.rs
//
// Struct to store a Wasm Configuration

use crate::module::WASM_RUNTIME_MODULES;

use std::collections::HashMap;
use std::sync::RwLock;

use once_cell::sync::Lazy;


pub struct WasmConfig {
    pub id:           String,
    pub module_id:    String,
    pub wasi_args:    Vec<String>,
    pub wasi_envs:    Vec<(String, String)>,
    pub wasi_dirs:    Vec<String>,
    pub wasi_mapdirs: Vec<(String, String)>,
}

impl WasmConfig {
    /// Adds a new configuration for a loaded module
    ///
    /// It checks for duplicated `config_id` or wrong `module_id`.
    /// Returns Result<(), String>, so that in case of error the String will contain the reason.
    /// 
    pub fn add_for_module(config_id: &str, module_id: &str) -> Result<(), String> {
                
        // get write access to the WasmConfig HashMap
        let mut configs = WASM_RUNTIME_CONFIGS.write()
            .expect("ERROR! Poisoned RwLock WASM_RUNTIME_CONFIGS on write()");

        // check for existing config_id in the loaded configurations
        if let Some(wasm_config) = configs.get(config_id) {
            // same id?
            if wasm_config.id == config_id {    // redundant but it's the WasmConfig which really owns its ID, not the Key in the HashMap
                // same module? then WasmConfig it's being added twice
                if wasm_config.module_id == module_id {
                    // TO-DO: the commented lines below should be the right behaviour.
                    // But since dry-run is not supported yet in mod_wasm.c, it's preferible to turn this check off
                    // See issue #26: https://github.com/vmware-labs/mod_wasm/issues/26
                    //                    
                    // let error_msg = format!("Wasm config \'{}\' for module \'{}\' already exists, skipping", config_id, module_id);
                    // return Err(error_msg);
                }
                // different module? then config_id is already in use by another WasmConfig 
                else {
                    let error_msg = format!("Wasm config \'{}\' is already in use for module \'{}\'", config_id, wasm_config.module_id);
                    return Err(error_msg);
                }
            }
        }

        // get read access to the WasmModule HashMap
        let modules = WASM_RUNTIME_MODULES.read()
            .expect("ERROR! Poisoned RwLock WASM_RUNTIME_MODULES on read()");
        
        // get the referring WasmModule
        let wasm_module_id = match modules.contains_key(module_id) {
            true => {
                module_id
            },
            false => {
                let error_msg = format!("Wasm module \'{}\' not loaded previously while adding Wasm config \'{}\'", module_id, config_id);
                return Err(error_msg);
            }
        };

        // build the WasmConfig object
        let wasm_config = WasmConfig {
            id:           config_id.to_string(),
            module_id:    wasm_module_id.to_string(),
            wasi_args:    Vec::new(),
            wasi_envs:    Vec::new(),
            wasi_dirs:    Vec::new(),
            wasi_mapdirs: Vec::new(),
        };

        // insert into the HasmMap
        configs.insert(wasm_config.id.clone(), wasm_config);

        Ok(())
    }
}


// The following static variable is used to achieve a global, mutable and thread-safe shareable state.
// For that given purpose, it uses [Once Cell](https://crates.io/crates/once_cell).
// Any object will be protected by `once_cell::sync::Lazy` and `std::sync::{Mutex, RwLock}`.
//

// Stores Wasm Configs 
pub static WASM_RUNTIME_CONFIGS: Lazy<RwLock<HashMap<String, WasmConfig>>> = Lazy::new(|| {
    let data: HashMap<String, WasmConfig> = HashMap::new();
    RwLock::new(data)
});