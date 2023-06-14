//
// Copyright 2022-2023 VMware, Inc.
// SPDX-License-Identifier: Apache-2.0
//

// execution_ctx.rs
//
// Struct to store the Wasm Execution Context

use std::collections::HashMap;
use std::sync::{Arc,RwLock};

use rand::Rng;
use once_cell::sync::Lazy;
use anyhow::Result;

use crate::module::WASM_RUNTIME_MODULES;
use crate::config::WASM_RUNTIME_CONFIGS;
use crate::wasm_engine;
use crate::apache_bindings::request_rec;


pub struct WasmExecutionCtx {
    pub id:           String,
    pub config_id:    String,
    pub module_id:    String,
    pub wasi_args:    Vec<String>,
    pub wasi_envs:    Vec<(String, String)>,
    pub wasi_dirs:    Vec<String>,
    pub wasi_mapdirs: Vec<(String, String)>,
    pub wasi_stdin:   Vec<u8>,
    pub wasi_stdout:  Arc<RwLock<Vec<u8>>>,
    pub apache_module_ids: Vec<String>,
}

impl WasmExecutionCtx {
    /// Create a new Wasm execution context (`WasmExecutionCtx`) and store it into the corresponding `HashMap`
    ///
    /// Returns Result<String, String>, with the ID for the new execution context.
    /// Or in case of invalid `config_id`, it returns a String explaing the error.
    /// 
    pub fn create_from_config(config_id: &str) -> Result<String, String> {
        
        // get read access to the WasmConfig HashMap
        let configs = WASM_RUNTIME_CONFIGS.read()
            .expect("ERROR! Poisoned RwLock WASM_RUNTIME_CONFIGS on read()");

        // check for existing config_id in the loaded configurations
        let wasm_config = match configs.get(config_id) {
            Some(c) => c,
            None => {
                let error_msg = format!("Wasm config \'{}\' not found while creating new Wasm execution context", config_id);
                return Err(error_msg);  
            }
        };

        // generate a random ID of 8 hex digits
        let hex_id = Self::generate_random_hex_id(8);

        // build the WasmExecutionCtx object based on the WasmConfig object
        let wasm_executionctx = WasmExecutionCtx {
            id:           hex_id.clone(),
            config_id:    wasm_config.id.clone(),
            module_id:    wasm_config.module_id.clone(),
            wasi_args:    wasm_config.wasi_args.clone(),
            wasi_envs:    wasm_config.wasi_envs.clone(),
            wasi_dirs:    wasm_config.wasi_dirs.clone(),
            wasi_mapdirs: wasm_config.wasi_mapdirs.clone(),
            wasi_stdin:   Vec::new(),
            wasi_stdout:  Arc::new(RwLock::new(Vec::new())),
            apache_module_ids: wasm_config.apache_module_ids.clone(),
        };

        Self::try_insert(wasm_executionctx)
    }
    

    /// Deallocates an existing Execution Context 
    ///
    /// It checks for wrong `executionctx_id`.
    /// Returns Result<(), String>, so that in case of error the String will contain the reason.
    /// 
    pub fn deallocate(executionctx_id: &str) -> Result<(), String> {
        // extracts the Execution Context and it is automatically dropped
        match Self::extract(executionctx_id) {
            Ok(_) => Ok(()),
            Err(e) => Err(e)
        }
    }


    /// Add a WASI Enviromental Variable for an existing Wasm execution context
    ///
    /// It checks for wrong `executionctx_id`.
    /// Returns Result<(), String>, so that in case of error the String will contain the reason.
    /// 
    pub fn add_wasi_env_for_executionctx(executionctx_id: &str, wasi_env: &str, wasi_value: &str) -> Result<(), String> {
    
        // get write access to the WasmExecutionCtx HashMap
        let mut executionctxs = WASM_RUNTIME_EXECUTIONCTXS.write()
            .expect("ERROR! Poisoned RwLock WASM_RUNTIME_EXECUTIONCTXS on write()");

        // get the given WasmExecutionCtx object
        let wasm_executionctx = match executionctxs.get_mut(executionctx_id) {
            Some(exectx) => exectx,
            None => {
                let error_msg = format!("Wasm execution context \'{}\' not created previously!", executionctx_id);
                return Err(error_msg); 
            }
        };

        // add WASI Env into the WasmExecutionCtx object
        wasm_executionctx.wasi_envs.push((wasi_env.to_string(), wasi_value.to_string()));

        Ok(())
    }

    /// Add a WASI Stdin for an existing Wasm execution context
    ///
    /// It checks for wrong `executionctx_id`.
    /// Returns Result<(), String>, so that in case of error the String will contain the reason.
    /// 
    pub fn set_wasi_stdin_for_executionctx(executionctx_id: &str, stdin: Vec<u8>) -> Result<(), String> {
    
        // get write access to the WasmExecutionCtx HashMap
        let mut executionctxs = WASM_RUNTIME_EXECUTIONCTXS.write()
            .expect("ERROR! Poisoned RwLock WASM_RUNTIME_EXECUTIONCTXS on write()");

        // get the given WasmExecutionCtx object
        let wasm_executionctx = match executionctxs.get_mut(executionctx_id) {
            Some(exectx) => exectx,
            None => {
                let error_msg = format!("Wasm execution context \'{}\' not created previously!", executionctx_id);
                return Err(error_msg); 
            }
        };

        // add WASI stdin into the WasmExecutionCtx object
        wasm_executionctx.wasi_stdin = stdin;
        
        Ok(())
    }

    /// Run the given Execution Context 
    ///
    /// It checks for wrong `executionctx_id`.
    /// Returns Result<Vec<u8>, String>, with the contents of stdout.
    /// In case something goes wrong (including invalid `conexecutionctx_id`), it returns a String explaing the error.
    /// 
    pub fn run_wasm_module(executionctx_id: &str) -> Result<Vec<u8>, String> {

        // extract Wasm execution context
        let wasm_executionctx = match Self::extract(executionctx_id) {
            Ok(exectx) => exectx,
            Err(e) => {
                let error_msg = format!("Can't run Wasm execution context \'{}\'! {}", executionctx_id, e);
                return Err(error_msg); 
            }
        };
        
        // invoke default "_start" function for the given Wasm execution context (only main WasmModule directive was defined)
        if ! wasm_executionctx.module_id.is_empty() {
            // TO-DO: check result
            wasm_engine::run_wasm_module(&wasm_executionctx)?;
        }

        // read stdout from the Wasm execution context and return it
        let wasm_module_stdout = Self::read_stdout(&wasm_executionctx);

        match Self::try_insert(wasm_executionctx) {
            Ok(_) => Ok(wasm_module_stdout),
            Err(e) => {
                let error_msg = format!("Can't insert back Wasm execution context \'{}\' after execution! {}", executionctx_id, e);
                return Err(error_msg); 
            }
        }
    }

    pub fn run_wasm_function(executionctx_id: &str, wasm_function: &str, request: *mut request_rec) -> Result<(), String> {

        // extract Wasm execution context
        let wasm_executionctx = match Self::extract(executionctx_id) {
            Ok(exectx) => exectx,
            Err(e) => {
                let error_msg = format!("Can't run Wasm execution context \'{}\'! {}", executionctx_id, e);
                return Err(error_msg); 
            }
        };
        
        // invoke the given Wasm function for the given Wasm execution context
        // TO-DO: check result

        // TO-DO: This is to ad-hoc!!
        for apache_wasm_module_id in wasm_executionctx.apache_module_ids.iter() {
            // get read access to the WasmExecutionCtx HashMap
            let modules = WASM_RUNTIME_MODULES.read()
                    .expect("ERROR! Poisoned RwLock WASM_RUNTIME_MODULES on read()");

            // get the Wasm module object referred into the execution context
            let apache_wasm_module = match modules.get(apache_wasm_module_id) {
                Some(w) => w,
                None => {
                    let error_msg = format!("ERROR! Apache Wasm Module \'{}\' referred by execution context \'{}\' not found!", wasm_executionctx.module_id, wasm_executionctx.id);
                    return Err(error_msg); 
                }
            };

            if apache_wasm_module.has_exported_function(wasm_function) {
                // TO-DO: Check result
                wasm_engine::run_wasm_function(apache_wasm_module, wasm_function, request)?;
            }
        }

        // insert back the Wasm execution context
        match Self::try_insert(wasm_executionctx) {
            Ok(_) => Ok(()),
            Err(e) => {
                let error_msg = format!("Can't insert back Wasm execution context \'{}\' after execution! {}", executionctx_id, e);
                return Err(error_msg); 
            }
        }
    }

    // Helper function to generate random hex IDs for the given length
    //
    // Returns String with the generated identifier.
    //  
    fn generate_random_hex_id(len: usize) -> String {
        const CHARSET: &[u8] = b"0123456789ABCDEF"; // only hex digits
        let mut rng = rand::thread_rng();
    
        let id: String = (0..len)
            .map(|_| {
                let idx = rng.gen_range(0..CHARSET.len());
                CHARSET[idx] as char
            })
            .collect();

        id
    }

    // Helper function to read stdout from the Wasm execution context
    //  
    // Returns a Vec<u8> with the stdout buffer
    //
    fn read_stdout(wasm_executionctx: &WasmExecutionCtx) -> Vec<u8> {
        let stdout_buf = wasm_executionctx.wasi_stdout.read()
            .expect("ERROR! Poisoned RwLock stdout_buf on read()");

        stdout_buf.clone()
    }

    // Helper function to insert a Wasm execution context from the HashMap
    //  
    // It checks for duplicated `executionctx_id`.
    // Returns Result<String, String>, with the inserted Wasm execution context id,
    // otherswise, in case of error the String will contain the reason.
    //
    fn try_insert(wasm_executionctx: WasmExecutionCtx) -> Result<String, String> {
        let executionctx_id = wasm_executionctx.id.clone();

        // get write access to the WasmExecutionCtx HashMap
        let mut executionctxs = WASM_RUNTIME_EXECUTIONCTXS.write()
            .expect("ERROR! Poisoned RwLock WASM_RUNTIME_EXECUTIONCTXS on write()");

        // check for existing `executionctx_id`
        match executionctxs.get(&wasm_executionctx.id) {
            Some(existing_ectx) => {
                let error_msg = format!("Wasm execution context \'{}\' already exist!", existing_ectx.id);
                Err(error_msg)
            },
            None => { // insert for non-existing `executionctx_id`
                executionctxs.insert(wasm_executionctx.id.clone(), wasm_executionctx);
                Ok(executionctx_id)
            }
        }
    }

    // Helper function to extract a Wasm execution context from the HashMap
    //  
    // It checks for wrong `executionctx_id`.
    // Returns Result<(), String>, so that in case of error the String will contain the reason.
    //
    fn extract(executionctx_id: &str) -> Result<WasmExecutionCtx, String> {
        // get write access to the WasmExecutionCtx HashMap
        let mut executionctxs = WASM_RUNTIME_EXECUTIONCTXS.write()
        .expect("ERROR! Poisoned RwLock WASM_RUNTIME_EXECUTIONCTXS on write()");

        match executionctxs.remove(executionctx_id) {
            Some(ectx) => Ok(ectx),
            None => {
                let error_msg = format!("Wasm execution context \'{}\' not found!", executionctx_id);
                Err(error_msg)
            }
        }
    }
}


// The following static variable is used to achieve a global, mutable and thread-safe shareable state.
// For that given purpose, it uses [Once Cell](https://crates.io/crates/once_cell).
// Any object will be protected by `once_cell::sync::Lazy` and `std::sync::{Mutex, RwLock}`.
//
// Stores Wasm Execution Contexts 
pub static WASM_RUNTIME_EXECUTIONCTXS: Lazy<RwLock<HashMap<String, WasmExecutionCtx>>> = Lazy::new(|| {
    let data: HashMap<String, WasmExecutionCtx> = HashMap::new();
    RwLock::new(data)
});