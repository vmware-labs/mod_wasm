//
// Copyright 2022 VMware, Inc.
// SPDX-License-Identifier: Apache-2.0
//

//! ffi-utils.rs
//!
//! FFI stands for 'Foreign Function Interface'
//! This file contains functions needed for offering a C ABI compatible API from Rust.

use std::ffi::{CString, CStr, c_char};

// Coverts a `const char*` from C into a safe Rust string literal `&str``
// Two steps:
//   1) From c_char to CStr
//   2) From CStr to &str
pub fn const_c_char_to_str(const_c_char: *const c_char) -> &'static str {
    // unsafe conversion from C const char* to a safe CStr
    let cstr = unsafe {
        CStr::from_ptr(const_c_char)
    };

    // safe conversion from CStr to &str
    let str = match cstr.to_str() {
        Ok(s) => s,
        Err(e) => {
            eprintln!("ERROR: Can't parse '{:?}' due to UTF-8 encoding error! {:?}", cstr, e);
            ""
        }
    };

    str
}

// Coverts a Rust String slice into a null-terminated C `const char*`
// Two steps:
//   1) From &str to CString (ensuring null-termination)
//   2) From CString into *const c_char (C char pointer) via CString::into_raw()
//      This sencond step will trasnfer ownership of the pointer to the C world.
//      C must callback Rust to deallocate such a CString raw pointer via CString::from_raw.
//      Otherwise, the CString will leak the memory used.
//      More info at: https://doc.rust-lang.org/alloc/ffi/struct.CString.html#method.from_raw
pub fn str_to_c_char(string: &str) -> *const c_char {
    let cstring = match CString::new(string) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("ERROR: Can't convert String into CString due to a NULL character found in the String type! {:?}", e);
            CString::new("").unwrap()
        }
    };

    cstring.into_raw()
}


// Once a const char* has been used by C-side with CString::intro_raw(), and it's not needed anymore,
// Rust needs to recover ownership and deallocate it. So CString::from_raw() is used.
// Later, we explicitly drop it (it's not really needed) to emphasize the fact that this CString is not needed anymore.
// More info at: https://doc.rust-lang.org/alloc/ffi/struct.CString.html#method.from_raw
pub fn deallocate_cstring(ptr: *const c_char) {
    unsafe {
        let cstring_to_deallocate = CString::from_raw(ptr as *mut c_char);
        drop(cstring_to_deallocate);
    };
}
