//! wasi_context.rs
//!
//! 

use anyhow::Result;
use wasmtime_wasi::*;
use wasi_common::pipe::WritePipe;

use crate::WASM_RUNTIME_CONFIG_MODULE;
use crate::WASM_RUNTIME_CONFIG_WASI_ARGS;
use crate::WASM_RUNTIME_CONFIG_WASI_ENVS;
use crate::WASM_RUNTIME_CONFIG_WASI_DIRS;
use crate::WASM_RUNTIME_CONFIG_WASI_MAPDIRS;
use crate::WASM_RUNTIME_STDOUT_SPTR;


pub fn build_wasi_ctx() -> WasiCtx {
    let stdout_pipe = build_stdout_pipe();
    let args = build_wasi_args();
    let envs = build_wasi_envs();

    let mut wasi_builder = WasiCtxBuilder::new()
        .stdout(Box::new(stdout_pipe))
        .inherit_stderr()
        .args(&args).expect("ERROR! Wrong WASI args array Vector!")
        .envs(&envs).expect("ERROR! Wrong WASI envs array of duples Vector!");

    wasi_builder = add_wasi_preopen_dirs(wasi_builder);

    wasi_builder.build()
}


fn build_stdout_pipe() -> WritePipe<Vec<u8>> {
    let stdout_mutex = WASM_RUNTIME_STDOUT_SPTR.write()
        .expect("ERROR! Poisoned RwLock WASM_RUNTIME_STDOUT_SPTR on write()");
    
    WritePipe::from_shared((*stdout_mutex).clone())
}


fn build_wasi_args() -> Vec<String> {
    // filename will be used as arg[0]
    let filename= WASM_RUNTIME_CONFIG_MODULE.read()
        .expect("ERROR! Poisoned RwLock WASM_RUNTIME_CONFIG_MODULE on read()");
    
    let mut args = WASM_RUNTIME_CONFIG_WASI_ARGS.read()
        .expect("ERROR! Poisoned RwLock WASM_RUNTIME_CONFIG_WASI_ARGS on read()")
        .clone();

    args.insert(0, filename.clone()); // adding wasm filename as args[0]

    args
}


fn build_wasi_envs() -> Vec<(String, String)> {
    let envs = WASM_RUNTIME_CONFIG_WASI_ENVS.read()
        .expect("ERROR! Poisoned RwLock WASM_RUNTIME_CONFIG_WASI_ENVS on read()")
        .clone();
        
    envs
}


fn add_wasi_preopen_dirs(mut wasi_builder: WasiCtxBuilder) -> WasiCtxBuilder{

    for (map, dir) in collect_preopen_dirs()
        .expect("ERROR! Couldn't collect preopen directories!")
        .into_iter()
    {
        wasi_builder = wasi_builder.preopened_dir(dir, map)
            .expect("ERROR! Can't build WASI context due to preopen directories!");
    } 

    wasi_builder
}


fn collect_preopen_dirs() -> Result<Vec<(String, Dir)>> {
    let mut preopen_dirs = Vec::new();

    let dirs = WASM_RUNTIME_CONFIG_WASI_DIRS.read()
        .expect("ERROR! Poisoned RwLock WASM_RUNTIME_CONFIG_WASI_DIRS on read()");
    let map_dirs = WASM_RUNTIME_CONFIG_WASI_MAPDIRS.read()
        .expect("ERROR! Poisoned RwLock WASM_RUNTIME_CONFIG_WASI_MAPDIRS on read()");

    // collect preopen directories (ie: --dir /tmp)
    for dir in dirs.iter() {
        let preopen_dir = (
            dir.clone(), 
            match Dir::open_ambient_dir(dir, ambient_authority()) {
                Ok(d) => d,
                Err(e) => {
                    eprintln!("ERROR! Failed to open host directory '{}' for preopen! {}", dir.as_str(), e);
                    continue;
                }
            }
        );
        preopen_dirs.push(preopen_dir);
    }
    
    // collect preopen directories with mapping (ie: --mapdir /wasmhome /home/wasm_user)
    for (map, host) in map_dirs.iter() {
        let preopen_mapdir = (
            map.clone(),
            match Dir::open_ambient_dir(host, ambient_authority()) {
                Ok(d) => d,
                Err(e) => {
                    eprintln!("ERROR! Failed to open host directory '{}' for preopen! {}", host.as_str(), e);
                    continue;
                }
            }
        );
        preopen_dirs.push(preopen_mapdir);
    }

    Ok(preopen_dirs)
}