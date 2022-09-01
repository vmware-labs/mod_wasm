// wasmengine.rs
//
// Using Wasmtime from the Bytecode Alliance as the Wasm Engine
// https://github.com/bytecodealliance/wasmtime

use anyhow::Result;

use crate::WASM_RUNTIME_STDOUT_SPTR;
use crate::WASM_RUNTIME_STORE;
use crate::WASM_RUNTIME_INSTANCE;


/// Initialize the Wasm Module and all the Wasmtime needed objects to later call a function.
///
/// Due to the Wasmtime object's depency graph, by initializing an Instance, will automatically
/// trigger all the other lazy initializations.
/// 
/// See below the Wasmtime object's dependcy graph:
/// Instance  ---> Module, Linker, Store
///    Module ---> Engine
///    Linker ---> Engine
///    Store  ---> Engine, WasiCtx
///    Engine ---> 0
///   WasiCtx ---> 0
///
pub fn init_module() -> bool {
    let loaded_instance = match WASM_RUNTIME_INSTANCE.read() {
        Ok(_) => true,
        Err(_) => {
            eprintln!("ERROR! Poisoned RwLock WASM_RUNTIME_STORE on write()");
            false
        }
    };
    
    loaded_instance
}


pub fn run_module() -> Result<String> {        
    clear_stdout();
    invoke_function("_start");

    let output = read_stdout()
        .expect("ERROR! Couldn't read stdout after invoking function!");

    Ok(output)
}


fn clear_stdout() {
    let stdout_sptr = WASM_RUNTIME_STDOUT_SPTR.write()
        .expect("ERROR! Poisoned RwLock WASM_RUNTIME_STDOUT_SPTR on write()");

    let mut stdout_buf = stdout_sptr.write()
        .expect("ERROR! Poisoned RwLock stdout_sptr on write()");
        
    (*stdout_buf).clear();
}


fn invoke_function(function_name: &str) {
    // get store and instance from statics
    let mut store = WASM_RUNTIME_STORE.write()
        .expect("ERROR! Poisoned RwLock WASM_RUNTIME_STORE on write()");

    let instance = WASM_RUNTIME_INSTANCE.read()
        .expect("ERROR! Poisoned RwLock WASM_RUNTIME_STORE on write()");
        
    // get typed function from instance
    let typed_function = instance.get_typed_func::<(), (), _>(&mut *store, function_name)
        .expect("ERROR! Can't get typed function from instance!");

    // invoke function    
    typed_function.call(&mut (*store), ())
        .expect("ERROR! Can't invoke typed function!");
}


fn read_stdout() -> Result<String> {
    // read stdout
    let stdout_sptr = WASM_RUNTIME_STDOUT_SPTR.read()
        .expect("ERROR! Poisoned RwLock WASM_RUNTIME_STDOUT_SPTR on read()");

    let stdout_buf = stdout_sptr.read()
        .expect("ERROR! Poisoned RwLock stdout_sptr on read()");
        
    let out_string = match String::from_utf8((*stdout_buf).clone()) {
        Ok(s) => s,
        Err(e) => {
            let str_error_msg = format!("ERROR! Can't covert stdout to UTF-8 string! {}", e);
            eprintln!("{}", str_error_msg);
            str_error_msg
        }
    };

    Ok(out_string)
}
