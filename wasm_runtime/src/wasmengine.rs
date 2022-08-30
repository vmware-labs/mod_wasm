// wasmengine.rs
//
// As Wasm engine, we will use Wasmtime from the Bytecode Alliance:
// https://github.com/bytecodealliance/wasmtime

use anyhow::Result;
use wasmtime::*;
use wasmtime_wasi::*;
use wasi_common::pipe::WritePipe;
use std::sync::{Arc, RwLock};

use crate::WASM_RUNTIME_CONFIG_ROOT;
use crate::WASM_RUNTIME_CONFIG_MODULE;
use crate::WASM_RUNTIME_CONFIG_WASI_ARGS;
use crate::WASM_RUNTIME_CONFIG_WASI_ENVS;


pub fn run_module() -> Result<String> {
    // Wasm module path
    let filepath= WASM_RUNTIME_CONFIG_ROOT.lock().unwrap();
    let filename= WASM_RUNTIME_CONFIG_MODULE.lock().unwrap();
    let modulepath = format!("{}/{}", filepath, filename);

    // WASI context
    let stdout_buf: Vec<u8> = vec![];
    let stdout_mutex = Arc::new(RwLock::new(stdout_buf));
    let stdout = WritePipe::from_shared(stdout_mutex.clone());

    let mut args = WASM_RUNTIME_CONFIG_WASI_ARGS.lock().unwrap().clone();
    args.insert(0, filename.clone()); // adding wasm filename as args[0]
    
    let envs = WASM_RUNTIME_CONFIG_WASI_ENVS.lock().unwrap();
    
    let wasi = WasiCtxBuilder::new()
        .stdout(Box::new(stdout))
        .inherit_stderr()
        .args(&args).expect("ERROR! Wrong WASI args array Vector!")
        .envs(&envs).expect("ERROR! Wrong WASI envs array of duples Vector!")
        .preopened_dir(
            Dir::open_ambient_dir(".", ambient_authority()).expect("ERROR! Can't access to host directory for preopen!"),
            "./")
            .expect("ERROR! Wrong WASI preopened directory!")
        .preopened_dir(
            Dir::open_ambient_dir("/home/ubuntu/Home/Workspace/VMware", ambient_authority()).expect("ERROR! Can't access to host directory for preopen!"),
            "/VMware")
            .expect("ERROR! Wrong WASI preopened directory!")
        .build();

    // Wasmtime Engine & Store (with WASI context)
    let engine = Engine::default();
    let mut store = Store::new(&engine, wasi);

    // Wasm module
    let module = Module::from_file(&engine, modulepath).expect("ERROR! Wasmtime: Can't load module from file!");

    // Linker (with WASI extensions)
    let mut linker: Linker<WasiCtx> = Linker::new(&engine);
    wasmtime_wasi::add_to_linker(&mut linker, |cx| cx).expect("ERROR! Wasmtime: Can't add WASI to linker!");

    // Wasm instance and entrypoint
    let instance = linker.instantiate(&mut store, &module)?;
    let entrypoint = instance.get_typed_func::<(), (), _>(&mut store, "_start")?;

    // Calling the entrypoint inside the Wasm module
    entrypoint.call(&mut store, ())?;

    // Read stdout
    let output = stdout_mutex.read().unwrap();
    let out_string = match String::from_utf8((*output).clone()) {
        Ok(s) => s,
        Err(e) => {
            let str_error_msg = format!("ERROR! Can't covert stdout to UTF-8 string! {}", e);
            eprintln!("{}", str_error_msg);
            str_error_msg
        }
    };

    Ok(out_string)
}

