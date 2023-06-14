fn main() {
    println!("[stdout] Apache Module! main()");
    eprintln!("[stderr] Apache Module! main()");
}

#[no_mangle]
pub extern "C" fn apache_abi_version_0_0_1() {
}

#[no_mangle]
pub extern "C" fn ap_hook_fixups(*mut request_rec) {
    println!("[stdout] Apache Module! ap_hook_fixups()");
    eprintln!("[stderr] Apache Module! ap_hook_fixups()");
}

#[no_mangle]
pub extern "C" fn ap_hook_content_handler() {
    println!("[stdout] Apache Module! ap_hook_content_handler()");
    eprintln!("[stderr] Apache Module! ap_hook_content_handler()");
}

#[no_mangle]
pub extern "C" fn ap_hook_log_transaction() {
    println!("[stdout] Apache Module! ap_hook_log_transaction()");
    eprintln!("[stderr] Apache Module! ap_hook_log_transaction()");
}

