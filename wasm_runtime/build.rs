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
const APACHE_BINDINGS_OUTPUT_FILE: &str = "src/apache_bindings/apache_bindings.rs";

// version numbers
const VERSION: &str = env!("CARGO_PKG_VERSION");
const VERSION_MAJOR: &str = env!("CARGO_PKG_VERSION_MAJOR");
const VERSION_MINOR: &str = env!("CARGO_PKG_VERSION_MINOR");
const VERSION_PATCH: &str = env!("CARGO_PKG_VERSION_PATCH");


fn main() -> Result<(), Error> {
    println!("cargo:rerun-if-changed=build.rs");

    // generate 'version.h' header file from Cargo Package version
    match generate_version_header() {
        Ok(()) => (),
        Err(_) => panic!("ERROR! Can't generate {} header!", VERSION_FILE)
    }

    // generate 'apache_bindings.rs' file 
    match generate_apache_bindings() {
        Ok(()) => (),
        Err(_) => panic!("ERROR! Can't generate {} bindings!", APACHE_BINDINGS_OUTPUT_FILE)
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
    let mut apache_bindings_builder = apache_bindings_builder();

    // extend platform-specific bindings 
    cfg_if::cfg_if!{
        if #[cfg(target_os = "linux")] {
            apache_bindings_builder = extend_apache_bindings_builder_linux(apache_bindings_builder);
        } else if #[cfg(target_os = "macos")] {
            apache_bindings_builder = extend_apache_bindings_builder_macos(apache_bindings_builder);
        } else if #[cfg(target_os = "windows")] {
            apache_bindings_builder = extend_apache_bindings_builder_macos(apache_bindings_builder);
        }
    }

    let apache_bindings = match apache_bindings_builder.generate()
    {
        Ok(b) => b,
        Err(e) => panic!("ERROR! Unable to generate Apache bindings! {}", e.to_string()),
    };

    apache_bindings
        .write_to_file(APACHE_BINDINGS_OUTPUT_FILE)
        .expect("ERROR! Couldn't write Apache bindings!");

    Ok(())
}

fn apache_bindings_builder() -> bindgen::Builder {

    bindgen::Builder::default()
        .header(APACHE_BINDINGS_INPUT_HEADERS)
        .clang_arg("-I../httpd/srclib/apr/include")
        .allowlist_recursively(false)
        .allowlist_type("request_rec")
        .allowlist_type("apr_pool_t")
        .allowlist_type("conn_rec")
        .allowlist_type("conn_sense_e")
        .allowlist_type("conn_state_t")
        .allowlist_type("conn_state_e")
        .allowlist_type("hostent")
        .allowlist_type("in_addr")
        .allowlist_type("in6_addr")
        .allowlist_type("in_addr_t")
        .allowlist_type("in_port_t")
        .allowlist_type("server_rec")
        .allowlist_type("server_addr_rec")
        .allowlist_type("sa_family_t")
        .allowlist_type("sockaddr_storage")
        .allowlist_type("sockaddr_in")
        .allowlist_type("sockaddr_in6")
        .allowlist_type("sockaddr_un")
        .allowlist_type("socklen_t")
        .allowlist_type("process_rec")
        .allowlist_type("apr_time_t")
        .allowlist_type("apr_uint16_t")
        .allowlist_type("apr_int32_t")
        .allowlist_type("apr_int64_t")
        .allowlist_type("apr_uint64_t")
        .allowlist_type("ap_method_mask_t")
        .allowlist_type("apr_array_header_t")
        .allowlist_type("apr_bucket")
        .allowlist_type("apr_bucket_type_t")
        .allowlist_type("apr_bucket_brigade")
        .allowlist_type("apr_bucket_alloc_t")
        .allowlist_type("apr_size_t")
        .allowlist_type("ap_method_list_t")
        .allowlist_type("ap_init_filter_func")
        .allowlist_type("ap_filter_conn_ctx")
        .allowlist_type("ap_filter_direction")
        .allowlist_type("ap_filter_direction_e")
        .allowlist_type("ap_filter_func")
        .allowlist_type("ap_input_mode_t")
        .allowlist_type("ap_in_filter_func")
        .allowlist_type("ap_out_filter_func")
        .allowlist_type("ap_filter_type")
        .allowlist_type("ap_filter_private")
        .allowlist_type("ap_filter_provider_t")
        .allowlist_type("ap_filter_rec_t")
        .allowlist_type("ap_request_bnotes_t")
        .allowlist_type("apr_off_t")
        .allowlist_type("off_t")
        .allowlist_type("apr_table_t")
        .allowlist_type("apr_table_t")
        .allowlist_type("apr_read_type_e")
        .allowlist_type("apr_status_t")
        .allowlist_type("apr_thread_mutex_t")
        .allowlist_type("ap_conn_keepalive_e")
        .allowlist_type("ap_conf_vector_t")
        .allowlist_type("ap_errorlog_provider")
        .allowlist_type("ap_logconf")
        .allowlist_type("ap_filter_t")
        .allowlist_type("htaccess_result")
        .allowlist_type("apr_uri_t")
        .allowlist_type("apr_port_t")
        .allowlist_type("apr_finfo_t")
        .allowlist_type("apr_sockaddr_t")
        .allowlist_type("apr_socklen_t")
        .allowlist_type("apr_file_t")
        .allowlist_type("apr_fileperms_t")
        .allowlist_type("apr_filetype_e")
        .allowlist_type("apr_interval_time_t")
        .allowlist_type("apr_uid_t")
        .allowlist_type("apr_thread_t")
        .allowlist_type("uid_t")
        .allowlist_type("apr_gid_t")
        .allowlist_type("gid_t")
        .allowlist_type("apr_ino_t")
        .allowlist_type("ino_t")
        .allowlist_type("apr_dev_t")
        .allowlist_type("dev_uid_t")
        .allowlist_type("dev_t")
        .allowlist_type("socklet_t")
}


#[cfg(target_os = "linux")]
fn extend_apache_bindings_builder_linux(builder: bindgen::Builder) -> bindgen::Builder {
    builder
        .allowlist_type("__ino_t")
        .allowlist_type("__dev_t")
        .allowlist_type("__gid_t")
        .allowlist_type("__uid_t")
        .allowlist_type("__off_t")
        .allowlist_type("__socklen_t")
}

#[cfg(target_os = "macos")]
fn extend_apache_bindings_builder_macos(builder: bindgen::Builder) -> bindgen::Builder {
    builder
    .allowlist_type("__darwin_dev_t")
    .allowlist_type("__darwin_gid_t")
    .allowlist_type("__darwin_ino_t")
    .allowlist_type("__darwin_ino64_t")
    .allowlist_type("__darwin_off_t")
    .allowlist_type("__darwin_uid_t")
    .allowlist_type("__darwin_socklen_t")
    .allowlist_type("__uint8_t")
    .allowlist_type("__int8_t")
    .allowlist_type("__int16_t")
    .allowlist_type("__uint16_t")
    .allowlist_type("__uint32_t")
    .allowlist_type("__int32_t")
    .allowlist_type("__int64_t")
    .allowlist_type("__uint64_t")
}

#[cfg(target_os = "windows")]
fn extend_apache_bindings_builder_windows(builder: bindgen::Builder) -> bindgen::Builder {
    builder
        //.allowlist_type("")
}