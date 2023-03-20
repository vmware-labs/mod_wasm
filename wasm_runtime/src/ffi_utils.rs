//
// Copyright 2022 VMware, Inc.
// SPDX-License-Identifier: Apache-2.0
//

//! ffi-utils.rs
//!
//! FFI stands for 'Foreign Function Interface'
//! This file contains functions needed for offering a C ABI compatible API from Rust.

use std::ffi::{CString, CStr, c_char, c_uchar};
use std::slice;


// Coverts a `const char*` from C into a safe Rust string literal `&str``
// Two steps:
//   1) From c_char to CStr
//   2) From CStr to &str
pub fn const_c_char_to_str(const_c_char: *const c_char) -> &'static str {
    // unsafe conversion from C const char* to a safe CStr
    let safe_cstr = unsafe {
        CStr::from_ptr(const_c_char)
    };

    // safe conversion from CStr to &str
    let str_literal = match safe_cstr.to_str() {
        Ok(s) => s,
        Err(e) => {
            eprintln!("ERROR: Can't parse '{:?}' due to UTF-8 encoding error! {:?}", safe_cstr, e);
            ""
        }
    };

    str_literal
}

// Coverts a Rust String slice into a null-terminated C `const char*`
// Two steps:
//   1) From &str to CString (ensuring null-termination)
//   2) From CString into *const c_char (C char pointer) via CString::into_raw()
//      This second step will transfer ownership of the pointer to the C world.
//      C must callback Rust to deallocate such a CString raw pointer via CString::from_raw.
//      Otherwise, the CString will leak the memory used.
//      More info at: https://doc.rust-lang.org/alloc/ffi/struct.CString.html#method.from_raw
pub fn str_to_c_char(string: &str) -> *const c_char {
    let safe_cstring = match CString::new(string) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("ERROR: Can't convert String into CString due to a NULL character found in the String type! {:?}", e);
            CString::new("").unwrap()
        }
    };

    safe_cstring.into_raw()
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

// Converts a `c_uchar` buffer into a Vec<u8>
// 
// This funcion is unsafe and can fail if data within the buffer is not well aligned.
// See more information at: https://doc.rust-lang.org/std/slice/fn.from_raw_parts.html
pub fn const_c_char_buffer_to_vec(buffer: *const c_uchar, size: usize) -> Vec<u8> {
    let bytes = unsafe { slice::from_raw_parts(buffer, size) };
    let bytes_vec: Vec<u8> = Vec::from(bytes);

    bytes_vec
}

// Converts a Vec<u8> into a null-terminated C `const char*`
// 
// See more information at: https://doc.rust-lang.org/std/ffi/struct.CString.html#method.from_vec_unchecked
// 
// NOTE: As a reminder, C strings use the NULL terminator convention ('\0' character).
// The Vec<u8> input buffer might contain more than one NULL terminator in the middle that we must honor since they might 
// be part of a binary content (ie.: a .png file). That is why we use `from_vec_unchecked()` to ensure all bytes are part
// of the new CString.
// To work in C with the returning `const char* buffer`, make sure you also get the Vec<u8> size and DO NOT use C NULL-terminated 
// related functions such as `printf` (which will truncate the output), and use size-based functions like `fwrite()` instead.
pub fn vec_u8_to_const_c_char(buffer: Vec<u8>) -> *const c_char {
    let safe_cstring = unsafe { CString::from_vec_unchecked(buffer) };

    safe_cstring.into_raw()
}
    