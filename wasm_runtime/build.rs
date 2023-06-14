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
const APACHE_BINDINGS_INPUT_HEADERS: &str = "src/apache_bindings/apache_bindings.h";
const APACHE_BINDINGS_FILE: &str = "src/apache_bindings/apache_bindings.rs";

// version numbers
const VERSION: &str = env!("CARGO_PKG_VERSION");
const VERSION_MAJOR: &str = env!("CARGO_PKG_VERSION_MAJOR");
const VERSION_MINOR: &str = env!("CARGO_PKG_VERSION_MINOR");
const VERSION_PATCH: &str = env!("CARGO_PKG_VERSION_PATCH");


fn main() -> Result<(), Error> {

    // generate 'version.h' header file from Cargo Package version
    match generate_version_header() {
        Ok(()) => (),
        Err(_) => panic!("ERROR! Can't generate {} header!", VERSION_FILE)
    }

    // generate 'apache_bindings.rs' file 
    match generate_apache_bindings() {
        Ok(()) => (),
        Err(_) => panic!("ERROR! Can't generate {} bindings!", APACHE_BINDINGS_FILE)
    }

    Ok(())
}


fn generate_version_header() -> Result<(), Error> {
    let mut file = match File::create(VERSION_FILE) {
        Ok(f) => f,
        Err(e) => {
            panic!("ERROR! Can't open file {}: {}", VERSION_FILE, e);
        }
    };

    writeln!(file, "#define WASM_RUNTIME_VERSION \"{}\"", VERSION)?;
    writeln!(file, "#define WASM_RUNTIME_VERSION_MAJOR {}", VERSION_MAJOR)?;
    writeln!(file, "#define WASM_RUNTIME_VERSION_MINOR {}", VERSION_MINOR)?;
    writeln!(file, "#define WASM_RUNTIME_VERSION_PATCH {}", VERSION_PATCH)?;
    writeln!(file)?;

    Ok(())
}


fn generate_apache_bindings() -> Result<(), Error> {
    // https://github.com/studersi/apache-rs/blob/master/build.rs
    // Derived from https://rust-lang.github.io/rust-bindgen/tutorial-3.html
    let bindings = bindgen::Builder::default()
        .header(APACHE_BINDINGS_INPUT_HEADERS)
        .clang_arg("-I../httpd/srclib/apr/include")
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file(APACHE_BINDINGS_FILE)
        .expect("Couldn't write bindings!");

    Ok(())
}
