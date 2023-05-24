//
// Copyright 2022-2023 VMware, Inc.
// SPDX-License-Identifier: Apache-2.0
//

// config.rs
//
// Struct to store a Wasm Configuration

use crate::module::WASM_RUNTIME_MODULES;

use std::collections::HashMap;
use std::sync::RwLock;
use std::path::{Path, PathBuf};
use once_cell::sync::Lazy;
use path_clean::clean;

pub struct WasmConfig {
    pub id:           String,
    pub module_id:    String,
    pub wasi_args:    Vec<String>,
    pub wasi_envs:    Vec<(String, String)>,
    pub wasi_dirs:    Vec<String>,
    pub wasi_mapdirs: Vec<(String, String)>,
}

impl WasmConfig {
    /// Create a new Wasm configuration (`WasmConfig`) and store it into the corresponding `HashMap`
    ///
    /// It checks for duplicated `config_id`.
    /// Returns Result<(), String>, so that in case of error the String will contain the reason.
    /// 
    pub fn create(config_id: &str) -> Result<(), String> {
        // safety check for empty id
        if config_id.len() == 0 {
            return Err("ERROR! Can't create WasmConfig for an empty config_id!".to_string());
        }
                
        // get write access to the WasmConfig HashMap
        let mut configs = WASM_RUNTIME_CONFIGS.write()
            .expect("ERROR! Poisoned RwLock WASM_RUNTIME_CONFIGS on write()");

        // check for existing config_id in the loaded configurations
        if configs.contains_key(config_id) {
            // TO-DO: the commented lines below should be the right behaviour.
            // But since dry-run is not supported yet in mod_wasm.c, it's preferible to turn this check off
            // See issue #26: https://github.com/vmware-labs/mod_wasm/issues/26
            //                    
            // let error_msg = format!("Wasm config \'{}\' already exists, skipping", config_id, module_id);
            // return Err(error_msg);
        }

        // build the WasmConfig object
        let wasm_config = WasmConfig {
            id:           config_id.to_string(),
            module_id:    String::new(),
            wasi_args:    Vec::new(),
            wasi_envs:    Vec::new(),
            wasi_dirs:    Vec::new(),
            wasi_mapdirs: Vec::new(),
        };

        // insert created WasmConfig object into the HashMap
        configs.insert(wasm_config.id.clone(), wasm_config);

        Ok(())
    }


    /// Set a loaded Wasm Module to an existing Wasm config
    ///
    /// It checks for wrong `config_id` and non-loaded Wasm Modules
    /// Returns Result<(), String>, so that in case of error the String will contain the reason.
    /// 
    pub fn set_wasm_module_for_config(config_id: &str, module_id: &str) -> Result<(), String> {
        
        // get write access to the WasmConfig HashMap
        let mut configs = WASM_RUNTIME_CONFIGS.write()
            .expect("ERROR! Poisoned RwLock WASM_RUNTIME_CONFIGS on write()");

        // check for existing config_id in the loaded configurations
        let wasm_config = match configs.get_mut(config_id) {
            Some(c) => c,
            None => {
                let error_msg = format!("Wasm config \'{}\' not found while setting module \'{}\'", config_id, module_id);
                return Err(error_msg);
            }
        };

        // get read access to the WasmModule HashMap
        let modules = WASM_RUNTIME_MODULES.read()
            .expect("ERROR! Poisoned RwLock WASM_RUNTIME_MODULES on read()");
        
        // get the referring WasmModule
        let wasm_module_id = match modules.contains_key(module_id) {
            true => {
                module_id
            },
            false => {
                let error_msg = format!("Wasm module \'{}\' not loaded previously while setting to Wasm config \'{}\'", module_id, config_id);
                return Err(error_msg);
            }
        };

        // setting module in Wasm config
        wasm_config.module_id = wasm_module_id.to_string();

        Ok(())
    }


    /// Add a new WASI Arg for an existing Wasm config
    ///
    /// It checks for wrong `config_id`.
    /// Returns Result<(), String>, so that in case of error the String will contain the reason.
    /// 
    pub fn add_wasi_arg_for_config(config_id: &str, wasi_arg: &str) -> Result<(), String> {
    
        // get write access to the WasmConfig HashMap
        let mut configs = WASM_RUNTIME_CONFIGS.write()
            .expect("ERROR! Poisoned RwLock WASM_RUNTIME_CONFIGS on write()");

        // get the given WasmConfig object
        let wasm_config = match configs.get_mut(config_id) {
            Some(c) => c,
            None => {
                let error_msg = format!("Wasm config \'{}\' not created previously!", config_id);
                return Err(error_msg); 
            }
        };

        // add WASI Arg into the WasmConfig object
        wasm_config.wasi_args.push(wasi_arg.to_string());

        Ok(())
    }

    /// Add a WASI Enviromental Variable for an existing Wasm config
    ///
    /// It checks for wrong `config_id`.
    /// Returns Result<(), String>, so that in case of error the String will contain the reason.
    /// 
    pub fn add_wasi_env_for_config(config_id: &str, wasi_env: &str, wasi_value: &str) -> Result<(), String> {
    
        // get write access to the WasmConfig HashMap
        let mut configs = WASM_RUNTIME_CONFIGS.write()
            .expect("ERROR! Poisoned RwLock WASM_RUNTIME_CONFIGS on write()");

        // get the given WasmConfig object
        let wasm_config = match configs.get_mut(config_id) {
            Some(c) => c,
            None => {
                let error_msg = format!("Wasm config \'{}\' not created previously!", config_id);
                return Err(error_msg); 
            }
        };

        // add WASI Env into the WasmConfig object
        wasm_config.wasi_envs.push((wasi_env.to_string(), wasi_value.to_string()));
        Ok(())
    }

    /// Add a new WASI Dir for an existing Wasm config
    ///
    /// It checks for wrong `config_id`.
    /// Returns Result<(), String>, so that in case of error the String will contain the reason.
    /// 
    pub fn add_wasi_dir_for_config(config_id: &str, wasi_dir: &str) -> Result<(), String> {
    
        // get write access to the WasmConfig HashMap
        let mut configs = WASM_RUNTIME_CONFIGS.write()
            .expect("ERROR! Poisoned RwLock WASM_RUNTIME_CONFIGS on write()");

        // get the given WasmConfig object
        let wasm_config = match configs.get_mut(config_id) {
            Some(c) => c,
            None => {
                let error_msg = format!("Wasm config \'{}\' not created previously!", config_id);
                return Err(error_msg); 
            }
        };

        // add WASI Dir into the WasmConfig object
        wasm_config.wasi_dirs.push(wasi_dir.to_string());

        Ok(())
    }

    /// Add a WASI `MapDir` for an existing Wasm config
    ///
    /// It checks for wrong `config_id`.
    /// Returns Result<(), String>, so that in case of error the String will contain the reason.
    /// 
    pub fn add_wasi_mapdir_for_config(config_id: &str, wasi_map: &str, wasi_dir: &str) -> Result<(), String> {
    
        // get write access to the WasmConfig HashMap
        let mut configs = WASM_RUNTIME_CONFIGS.write()
            .expect("ERROR! Poisoned RwLock WASM_RUNTIME_CONFIGS on write()");

        // get the given WasmConfig object
        let wasm_config = match configs.get_mut(config_id) {
            Some(c) => c,
            None => {
                let error_msg = format!("Wasm config \'{}\' not created previously!", config_id);
                return Err(error_msg); 
            }
        };

        // add WASI MapDir into the WasmConfig object
        wasm_config.wasi_mapdirs.push((wasi_map.to_string(), wasi_dir.to_string()));
        Ok(())
    }
    // Returns a version of path with all slashes converted to forward
    fn path_to_unix_slashes(path: &Path) -> String {
        path.to_string_lossy().into_owned().replace("\\", "/")
    }
    fn find_longest_map(mapdirs: &[(String, String)], path: &str) -> Option<String> {
        let cleaned_path = clean(Self::path_to_unix_slashes(Path::new(path)));
        let mut ordered = mapdirs.to_vec();
        ordered.sort_by_key(|(_, from)| Path::new(from).components().count());
        let longest_map = ordered
            .iter()
            .rev()
            .find(|(_, from)| cleaned_path.starts_with(from))
            .and_then(|(to, from)| match cleaned_path.strip_prefix(from) {
                Ok(tail) => Some(PathBuf::from(to).join(tail)),
                Err(_) => None,
            });

        longest_map.map(|p| Self::path_to_unix_slashes(clean(p).as_path()))
    }

    pub fn get_mapped_path(config_id: &str, path: &str) -> Option<String> {
        let configs = match WASM_RUNTIME_CONFIGS.read() {
            Ok(c) => c,
            Err(_) => {
                return None;
            }
        };

        let wasm_config = match configs.get(config_id) {
            Some(c) => c,
            None => {
                return None;
            }
        };
        Self::find_longest_map(&wasm_config.wasi_mapdirs, path)
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