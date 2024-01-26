use std::ffi::{CStr, CString};

fn main() {
    print!("Content-Type: text/plain\r\n\r\n");
    println!("Hello, Wasm! @stdout");
    eprintln!("Hello, Wasm! @stderr");

    println!("Environment:");

    for (key, value) in std::env::vars_os() {
        println!("{key:?}: {value:?}");
    }

    println!("Invoking host functions!");

    // -----------------------------------------
    println!("1) say_hello:()");
    unsafe {
        host_functions::say_hello();
    }

    // -----------------------------------------
    println!("2a) negate_number:(10) --> {}",
        unsafe {
            host_functions::negate_number(10)
        }
    );
    println!("2b) negate_number:(-10) --> {}",
        unsafe {
            host_functions::negate_number(-10)
        }
    );
    println!("2c) negate_number:(0) --> {}",
        unsafe {
            host_functions::negate_number(0)
        }
    );

    // -----------------------------------------
    let const_c_char = unsafe {
        let input = CString::new("¡Hola! ¿Cómo va?").unwrap().into_raw(); 
        host_functions::upper_case(input)
    };

    let safe_cstr = unsafe { CStr::from_ptr(const_c_char) };
    println!("3) upper_case:(\"¡Hola! ¿Cómo va?\") --> {}", safe_cstr.to_str().unwrap());
}

pub mod host_functions {

    use std::ffi::c_char;

    #[link(wasm_import_module = "host_functions_demo")]
    extern "C" {
        pub fn say_hello();
        pub fn negate_number(number: isize) -> isize;
        pub fn upper_case(input: *const c_char) -> *const c_char;
    }
}

pub mod mem_ops {
    use std::ffi::c_void;
    use std::mem;
    #[no_mangle]
    pub extern "C" fn allocate(size: usize) -> *mut c_void {
        let mut buffer = vec![0u8; size];
        let ptr = buffer.as_mut_ptr() as *mut c_void;
        mem::forget(buffer);
        ptr
    }

    #[no_mangle]
    pub extern "C" fn deallocate(ptr: *mut c_void, size: usize) {
        let _discard_on_exit = unsafe { Vec::from_raw_parts(ptr, 0, size) };
    }
}
