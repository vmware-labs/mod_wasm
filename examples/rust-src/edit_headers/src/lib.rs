mod apr;

#[no_mangle]
pub unsafe extern "C" fn handle_request_headers(headers_handle: u64) -> i32 {

    let op = apr::get_header(headers_handle, "operation");
    if op == "add" {
        apr::set_header(headers_handle, "added", "added_value");
    } else if op == "modify" {
        apr::set_header(headers_handle, "target", "modified_value");
    } else if op == "delete" {
        apr::delete_header(headers_handle, "target");
    }

    0
}
