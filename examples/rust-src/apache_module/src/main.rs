use apache_bindings::request_rec;

fn main() {
    println!("[stdout] Apache Module! main()");
    eprintln!("[stderr] Apache Module! main()");
}

#[no_mangle]
pub extern "C" fn apache_abi_version_0_0_1() {
}

#[no_mangle]
pub extern "C" fn ap_hook_fixups(r: *mut request_rec) {
    println!("[stdout] Apache Module! ap_hook_fixups()");
    eprintln!("[stderr] Apache Module! ap_hook_fixups()");
}

#[no_mangle]
pub extern "C" fn ap_hook_content_handler(r: *mut request_rec) {
    println!("[stdout] Apache Module! ap_hook_content_handler()");
    eprintln!("[stderr] Apache Module! ap_hook_content_handler()");
}

#[no_mangle]
pub extern "C" fn ap_hook_log_transaction(r: *mut request_rec) {
    println!("[stdout] Apache Module! ap_hook_log_transaction()");
    eprintln!("[stderr] Apache Module! ap_hook_log_transaction()");
}

