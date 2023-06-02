use std::ffi::{c_void, CStr, CString};
use std::ptr;

pub mod imports {
    use std::ffi::c_char;
    #[link(wasm_import_module = "apr")]
    extern "C" {
        pub fn get_header(headers_handle: u64, key: *const c_char) -> *const c_char;
        pub fn set_header(headers_handle: u64, key: *const c_char, value: *const c_char);
        pub fn delete_header(headers_ptr: u64, key: *const c_char);
    }
}

pub mod mem_ops {
    use std::ffi::c_void;
    use std::mem;
    #[no_mangle]
    pub extern "C" fn allocate(size: usize) -> *mut c_void {
        let mut buffer = vec![0u8; size];
        let result = buffer.as_mut_ptr() as *mut c_void;
        mem::forget(buffer);
        result
    }

    #[no_mangle]
    pub extern "C" fn deallocate(ptr: *mut c_void, size: usize) {
        let _discard_on_exit = unsafe { Vec::from_raw_parts(ptr, 0, size) };
    }
}

pub fn get_header(headers_handle: u64, key: &str) -> Option<String> {
    let key = CString::new(key).expect("Create CString for {key}");

    let op_result = unsafe { imports::get_header(headers_handle, key.as_ptr()) };
    if op_result == ptr::null() {
        return None;
    }

    let op_ptr = unsafe { CStr::from_ptr(op_result as *const i8) };

    // TODO - construct with String::from_raw_parts to get ownership and skip the copy and deallocate.
    // Note! - doing this properly may require modification of the ABI.
    let result = op_ptr.to_str().expect("UTF-8 string").to_string();

    mem_ops::deallocate(op_ptr.as_ptr() as *mut c_void, result.len());

    Some(result)
}

pub fn set_header(headers_handle: u64, key: &str, value: &str) {
    let key = CString::new(key).expect("CString::new failed");
    let value = CString::new(value).expect("CString::new failed");
    unsafe { imports::set_header(headers_handle, key.as_ptr(), value.as_ptr()) };
}

pub fn delete_header(headers_handle: u64, key: &str) {
    let key = CString::new(key).expect("CString::new failed");
    unsafe { imports::delete_header(headers_handle, key.as_ptr()) };
}
