// stdio_buffers.rs
//
// Static mutable shareable buffers that can be shared with different threads

use std::sync::{Arc,RwLock};

use anyhow::Result;
use once_cell::sync::Lazy; // https://crates.io/crates/once_cell


// The following static variables are used to achieve a global, mutable and thread-safe shareable state.
// For that given purpose, it uses [Once Cell](https://crates.io/crates/once_cell).
// Any object will be protected by `once_cell::sync::Lazy` and `std::sync::{Mutex, RwLock}`.

// Two different patterns co-live here:
//  1) Lazy<RwLock<T>> is the pattern for static, mutable and shareable state.
//  2) Arc<RwLock<T>> is the type required by WASI to pipe stdout.
// We need the smart pointer (Lazy<RwLock<T>>) to the stdout buffer to be allocated in the data segment as a static variable, 
// so that it can be shared between the Wasm module initialization and the module execution (function invocation)
pub static STDOUT_BUFFER_RWLOCK: Lazy<RwLock<Arc<RwLock<Vec<u8>>>>> = Lazy::new(|| {
    let data = Arc::new(RwLock::new(Vec::new()));
    RwLock::new(data)
});


pub fn clear_stdout() {
    let stdout_sptr = STDOUT_BUFFER_RWLOCK.write()
        .expect("ERROR! Poisoned RwLock STDOUT_BUFFER_RWLOCK on write()");

    let mut stdout_buf = stdout_sptr.write()
        .expect("ERROR! Poisoned RwLock stdout_sptr on write()");
        
    (*stdout_buf).clear();
}


pub fn read_stdout() -> Result<String> {
    // read stdout
    let stdout_sptr = STDOUT_BUFFER_RWLOCK.read()
        .expect("ERROR! Poisoned RwLock STDOUT_BUFFER_RWLOCK on read()");

    let stdout_buf = stdout_sptr.read()
        .expect("ERROR! Poisoned RwLock stdout_sptr on read()");
        
    let out_string = match String::from_utf8((*stdout_buf).clone()) {
        Ok(s) => s,
        Err(e) => {
            let str_error_msg = format!("ERROR! Can't covert stdout to UTF-8 string! {}", e);
            eprintln!("{}", str_error_msg);
            str_error_msg
        }
    };

    Ok(out_string)
}
