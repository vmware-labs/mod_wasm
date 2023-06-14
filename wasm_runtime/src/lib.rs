//
// Copyright 2022-2023 VMware, Inc.
// SPDX-License-Identifier: Apache-2.0
//

//! **`wasm_runtime`** is a Rust library intended to:
//! * Integrate with Wasm engines (such as [Wasmtime](https://github.com/bytecodealliance/wasmtime)). 
//! * Provide a thin C API for instantiating, running, and managing Wasm modules.

mod module;
mod config;
mod execution_ctx;
mod wasm_engine;
mod wasi_ctx;
mod ffi_utils;
mod apache_bindings;
pub mod c_api;
