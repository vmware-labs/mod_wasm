//
// Copyright 2022 VMware, Inc.
// SPDX-License-Identifier: Apache-2.0
//

//! **wasm_runtime** is a Rust library intended to:
//! * Integrate with Wasm engines (such as [Wasmtime](https://github.com/bytecodealliance/wasmtime)). 
//! * Provide a thin C API for instantiating, running, and managing Wasm modules.

mod config;
mod module;
mod wasm_engine;
mod wasmtime_shared;
mod wasi_context;
mod stdio_buffers;
mod ffi_utils;
mod c_api;
