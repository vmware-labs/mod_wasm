// Copyright 2022 VMware, Inc.
// SPDX-License-Identifier: Apache-2.0
//

//! build.rs
//!
//! Build script for Cargo

use std::fs::File;
use std::io::{Write, Error};

// file paths
const VERSION_H_FILE: &str = "include/version.h";

// version numbers
const VERSION: &str = env!("CARGO_PKG_VERSION");
const VERSION_MAJOR: &str = env!("CARGO_PKG_VERSION_MAJOR");
const VERSION_MINOR: &str = env!("CARGO_PKG_VERSION_MINOR");
const VERSION_PATCH: &str = env!("CARGO_PKG_VERSION_PATCH");
const VERSION_PRE: &str = env!("CARGO_PKG_VERSION_PRE");


fn main() -> Result<(), Error> { 
    let mut file = match File::create(VERSION_H_FILE) {
        Ok(f) => f,
        Err(e) => panic!("ERROR! Can't open file {}: {}", VERSION_H_FILE, e),
    };

    writeln!(file, "#define WASM_RUNTIME_VERSION \"{}\"", VERSION)?;
    writeln!(file, "#define WASM_RUNTIME_VERSION_MAJOR {}", VERSION_MAJOR)?;
    writeln!(file, "#define WASM_RUNTIME_VERSION_MINOR {}", VERSION_MINOR)?;
    writeln!(file, "#define WASM_RUNTIME_VERSION_PATCH {}", VERSION_PATCH)?;
    writeln!(file, "#define WASM_RUNTIME_VERSION_PRE {}", VERSION_PRE)?;
    writeln!(file)?;

    Ok(())
}
