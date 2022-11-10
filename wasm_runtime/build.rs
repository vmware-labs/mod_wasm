// Copyright 2022 VMware, Inc.
// SPDX-License-Identifier: Apache-2.0
//

//! build.rs
//!
//! Build script for Cargo

use std::fs::File;
use std::io::{Write, Error};

// file paths
const VERSION_FILE: &str = "include/version.h";

// version numbers
const VERSION: &str = env!("CARGO_PKG_VERSION");
const VERSION_MAJOR: &str = env!("CARGO_PKG_VERSION_MAJOR");
const VERSION_MINOR: &str = env!("CARGO_PKG_VERSION_MINOR");
const VERSION_PATCH: &str = env!("CARGO_PKG_VERSION_PATCH");


fn main() -> Result<(), Error> { 
    let mut file = match File::create(VERSION_FILE) {
        Ok(f) => f,
        Err(e) => panic!("ERROR! Can't open file {}: {}", VERSION_FILE, e),
    };

    write!(file, "#define WASM_RUNTIME_VERSION \"{}\"\n", VERSION)?;
    write!(file, "#define WASM_RUNTIME_VERSION_MAJOR {}\n", VERSION_MAJOR)?;
    write!(file, "#define WASM_RUNTIME_VERSION_MINOR {}\n", VERSION_MINOR)?;
    write!(file, "#define WASM_RUNTIME_VERSION_PATCH {}\n", VERSION_PATCH)?;

    Ok(())
}
