//
// Copyright 2022-2023 VMware, Inc.
// SPDX-License-Identifier: Apache-2.0
//

//! `integration_tests.rs`
//!
//! This file contains different integration tests


mod common;

use std::ffi::{CString, c_char};
use std::ptr;

use wasm_runtime::c_api;


#[test]
fn wasm_config_create_general() {
    // setup
    common::setup();
    const CONFIG_ID: &'static str = "config_test_id";
    let config_id: *const c_char  = CString::new(CONFIG_ID).expect("FATAL! Can't convert &str into CString!").into_raw();

    // tests
    let create_result = c_api::wasm_config_create(config_id);

    // asserts
    assert_eq!(create_result, 0);

    // teardown
    common::teardown();
}

#[test]
fn wasm_config_create_null_or_empty() {
    // setup
    let config_id_null: *const c_char = ptr::null();
    let config_id_empty = CString::new("").expect("FATAL! Can't convert &str into CString!").into_raw();

    // tests
    let create_result_null = c_api::wasm_config_create(config_id_null);
    let create_result_empty = c_api::wasm_config_create(config_id_empty);

    // asserts
    assert_eq!(create_result_null, -1);
    assert_eq!(create_result_empty, -1);

    // teardown
    common::teardown();
}
