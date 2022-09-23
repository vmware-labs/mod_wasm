/* Licensed to the Apache Software Foundation (ASF) under one or more
 * contributor license agreements.  See the NOTICE file distributed with
 * this work for additional information regarding copyright ownership.
 * The ASF licenses this file to You under the Apache License, Version 2.0
 * (the "License"); you may not use this file except in compliance with
 * the License.  You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

//! ffi-utils.rs
//!
//! FFI stands for 'Foreign Function Interface'
//! This file contains functions needed for offering a C ABI compatible API from Rust.

use std::os::raw::c_char;
use std::ffi::CStr;
use std::ffi::CString;

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
        let cstring_to_deallocate = CString::from_raw(ptr as *mut i8);
        drop(cstring_to_deallocate); 
    };
}
