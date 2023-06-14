//
// Copyright 2022-2023 VMware, Inc.
// SPDX-License-Identifier: Apache-2.0
//

// module.rs
//
// Struct to store Wasm Module

use std::{path::PathBuf, collections::HashSet};
use std::collections::HashMap;
use std::sync::RwLock;

use once_cell::sync::Lazy;
use wasmtime::{Engine, Module, ExternType};

pub enum WasmModuleKind {
    Regular,
    ApacheModule,
    ProxyWasm,
}

pub struct WasmModule {
    pub id:     String,
    pub kind:   WasmModuleKind,
    pub engine: wasmtime::Engine,
    pub module: wasmtime::Module,
    pub exported_functions: HashSet<String>,
}

impl WasmModule {
    /// Load a `WasmModule` from file into memory and store it in the corresponding `HashMap` (the route serves as module ID)
    ///
    /// It checks for path or wrong file format.
    /// Returns Result<(), String>, so that in case of error the String will contain the reason.
    /// 
    pub fn load_from_file(path: &str) -> Result<(), String> {

        // get write access to the WasmModule HashMap
        let mut modules = WASM_RUNTIME_MODULES.write()
            .expect("ERROR! Poisoned RwLock WASM_RUNTIME_MODULES on write()");

        // check if module was already loaded (path is used as key)
        if modules.contains_key(path) {
            // TO-DO: the commented lines below should be the right behaviour.
            // But since dry-run is not supported yet in mod_wasm.c, it's preferible to turn this check off
            // See issue #26: https://github.com/vmware-labs/mod_wasm/issues/26
            //
            // let error_msg = format!("Wasm module from \'{}\' is already loaded, skipping", path);
            // return Err(error_msg);
            return Ok(());
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
                let error_msg = format!("Can't load module from `{}`! {}", module_path.display(), e);
                return Err(error_msg);  
            }
        };
        
        // inspect the Wasm module
        let (wasm_module_kind, wasm_module_exported_functions) = Self::inspect_wasm_module(&wasmtime_module);

        // build the WasmModule object
        let wasm_module = WasmModule {
            id: path.to_string(),
            kind: wasm_module_kind,
            exported_functions: wasm_module_exported_functions,
            engine: module_engine,
            module: wasmtime_module,
        };

        // insert into the HashMap (path is used as key)
        modules.insert(path.to_string(), wasm_module);

        Ok(())
    }

    pub fn has_exported_function(&self, function_name: &str) -> bool {
        println!("  has_exported_function '{}' ??", function_name);
        self.exported_functions.contains(function_name)
    }

    // Inspect Wasm Module to collect Functions and determine is kind
    fn inspect_wasm_module(module: &Module) -> (WasmModuleKind, HashSet<String>)
    {
        let mut kind = WasmModuleKind::Regular;
        let mut exported_functions: HashSet<String> = HashSet::new();

        let exports = module.exports();
        for item in exports.into_iter() {
            // TO-DO: more elegant with a closure
            match item.ty() {
                ExternType::Func(_) => {
                    // insert function into set
                    exported_functions.insert( item.name().to_string() );

                    if item.name().starts_with("apache_abi_version_") {
                        kind = WasmModuleKind::ApacheModule;
                    }

                    if item.name().starts_with("proxy_abi_version_") {
                        kind = WasmModuleKind::ProxyWasm;
                    }
                }
                _ => {} // ignore other ExterType
            }
        }

        (kind, exported_functions)
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
