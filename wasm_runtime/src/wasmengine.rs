// wasmengine.rs
//
// Using Wasmtime from the Bytecode Alliance as the Wasm Engine
// https://github.com/bytecodealliance/wasmtime

use anyhow::Result;
use wasmtime::*;
use wasmtime_wasi::*;

use crate::wasi_context::*;

use crate::WASM_RUNTIME_CONFIG_ROOT;
use crate::WASM_RUNTIME_CONFIG_MODULE;
use crate::WASM_RUNTIME_STDOUT_SPTR;


pub fn run_module() -> Result<String> {

    let wasi = build_wasi_ctx();

    // Wasmtime Engine & Store (with WASI context)
    let engine = Engine::default();
    let mut store = Store::new(&engine, wasi);

    // Wasm module
    let modulepath = build_module_path();
    let module = Module::from_file(&engine, modulepath)
        .expect("ERROR! Wasmtime: Can't load module from file!");

    // Linker (with WASI extensions)
    let mut linker: Linker<WasiCtx> = Linker::new(&engine);
    wasmtime_wasi::add_to_linker(&mut linker, |cx| cx)
        .expect("ERROR! Wasmtime: Can't add WASI to linker!");

    // Wasm instance and entrypoint
    let instance = linker.instantiate(&mut store, &module)?;
    let entrypoint = instance.get_typed_func::<(), (), _>(&mut store, "_start")?;

    // Calling the entrypoint inside the Wasm module
    entrypoint.call(&mut store, ())?;

    // Read stdout
    let stdout_sptr = WASM_RUNTIME_STDOUT_SPTR.read()
        .expect("ERROR! Poisoned mutex WASM_RUNTIME_STDOUT_SPTR");

    let stdou_buf = stdout_sptr.read()
        .expect("ERROR! Poisoned mutex stdout_sptr");

    let out_string = match String::from_utf8((*stdou_buf).clone()) {
        Ok(s) => s,
        Err(e) => {
            let str_error_msg = format!("ERROR! Can't covert stdout to UTF-8 string! {}", e);
            eprintln!("{}", str_error_msg);
            str_error_msg
        }
    };

    Ok(out_string)
}


fn build_module_path() -> String {
    let filepath= WASM_RUNTIME_CONFIG_ROOT.read()
        .expect("ERROR! Poisoned mutex WASM_RUNTIME_CONFIG_ROOT");
    let filename= WASM_RUNTIME_CONFIG_MODULE.read()
        .expect("ERROR! Poisoned mutex WASM_RUNTIME_CONFIG_MODULE");
    let modulepath = format!("{}/{}", filepath, filename);

    modulepath
}