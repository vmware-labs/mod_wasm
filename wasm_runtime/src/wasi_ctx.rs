//
// Copyright 2022-2023 VMware, Inc.
// SPDX-License-Identifier: Apache-2.0
//

//! `wasi_context.rs`
//!
//! 

use std::path::PathBuf;

use wasmtime_wasi::{ambient_authority, Dir, WasiCtx, WasiCtxBuilder};
use wasi_common::pipe::{ReadPipe, WritePipe};
use anyhow::Result;

use crate::module::WasmModule;
use crate::execution_ctx::WasmExecutionCtx;

/// Builds a `wasmtime_wasi::WasiCtx` for the given Wasm execution context
///
pub fn build(wasm_executionctx: &WasmExecutionCtx, wasm_module: &WasmModule) -> Result<WasiCtx, String> {
    let stdin_pipe = ReadPipe::from(wasm_executionctx.wasi_stdin.clone());
    let stdout_pipe = WritePipe::from_shared(wasm_executionctx.wasi_stdout.clone());
    let envs = wasm_executionctx.wasi_envs.clone();
    let mut args = wasm_executionctx.wasi_args.clone();

    // ensure args includes the program binary (.wasm file) as the first position in the Vec<String>
    if let Some(filename) = PathBuf::from(wasm_module.id.clone()).file_name() {
        if let Some(filename_str) = filename.to_str() {
            args.insert(0, filename_str.to_string());
        }
    }

    // use WasiCtxBuilder to setup the WASI context
    let mut wasi_builder = WasiCtxBuilder::new()
        .stdin(Box::new(stdin_pipe))
        .stdout(Box::new(stdout_pipe))
        .inherit_stderr()
        .args(&args).expect("FATAL! Wrong WASI args array Vector format!")
        .envs(&envs).expect("FATAL! Wrong WASI envs array of duples Vector format!");

    wasi_builder = add_wasi_preopen_dirs(wasm_executionctx, wasi_builder)?;

    // build the WasiCtx object
    Ok(wasi_builder.build())
}


// helper function for preopen dirs
fn add_wasi_preopen_dirs(wasm_executionctx: &WasmExecutionCtx, mut wasi_builder: WasiCtxBuilder) -> Result<WasiCtxBuilder, String> {
    let collected_preopen_dirs = collect_preopen_dirs(wasm_executionctx)?;

    for (map, dir) in collected_preopen_dirs
    {
        wasi_builder = wasi_builder.preopened_dir(dir, map)
            .expect("ERROR! Can't build WASI context due to preopen directories!");
    } 

    Ok(wasi_builder)
}

// helper function to help collecting preopen dirs checking authorized access
fn collect_preopen_dirs(wasm_executionctx: &WasmExecutionCtx) -> Result<Vec<(String, Dir)>, String> {
    let mut preopen_dirs = Vec::new();

    let dirs = wasm_executionctx.wasi_dirs.clone();
    let map_dirs = wasm_executionctx.wasi_mapdirs.clone();

    // collect preopen directories (ie: --dir /tmp)
    for dir in &dirs {
        let preopen_dir = (
            dir.clone(), 
            match Dir::open_ambient_dir(dir, ambient_authority()) {
                Ok(d) => d,
                Err(e) => {
                    let msg_err = format!("ERROR! Failed to open host directory '{}' for preopen! {}", dir.as_str(), e);
                    return Err(msg_err);
                }
            }
        );
        preopen_dirs.push(preopen_dir);
    }
    
    // collect preopen directories with mapping (ie: --mapdir /wasmhome /home/wasm_user)
    for (map, host) in &map_dirs {
        let preopen_mapdir = (
            map.clone(),
            match Dir::open_ambient_dir(host, ambient_authority()) {
                Ok(d) => d,
                Err(e) => {
                    let msg_err = format!("ERROR! Failed to open host directory '{}' for preopen with mapping to '{}'! {}", host.as_str(), map.as_str(), e);
                    return Err(msg_err);
                }
            }
        );
        preopen_dirs.push(preopen_mapdir);
    }

    Ok(preopen_dirs)
}
