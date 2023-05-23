mod apr;

fn handle_operation_header(headers_handle: u64) {
    if let Some(op) = apr::get_header(headers_handle, "operation") {
        if op == "add" {
            apr::set_header(headers_handle, "added", "added_value");
        } else if op == "modify" {
            apr::set_header(headers_handle, "target", "modified_value");
        } else if op == "delete" {
            apr::delete_header(headers_handle, "target");
        }
    }
}

use eval::eval;
fn handle_eval_header(headers_handle: u64) {
    if let Some(eval_me) = apr::get_header(headers_handle, "evaluate-me") {
        let result = match eval(eval_me.as_str()) {
            Ok(result) => result.to_string(),
            Err(e) => format!("ERROR: {}", e.to_string()),
        };

        apr::set_header(headers_handle, "evaluate-result", result.as_str());
    }
}

use md5::compute;
fn handle_hash_header(headers_handle: u64) {
    if let Some(hash_me) = apr::get_header(headers_handle, "hash-me") {
        let result = compute(hash_me.as_bytes());

        apr::set_header(
            headers_handle,
            "hash-result",
            format!("{:x}", result).as_str(),
        );
    }
}

use std::fs;
#[allow(unconditional_panic)]
fn handle_failure_header(headers_handle: u64) {
    if let Some(fail_me) = apr::get_header(headers_handle, "fail-me") {
        eprintln!("Simulating error '{}'...", fail_me);
        if fail_me == "division-by-zero" {
            let no_result = 1234 / 0;

            apr::set_header(headers_handle, "dummy", format!("{}", no_result).as_str());
        } else if fail_me == "filesystem-access" {
            let no_contents = fs::read_to_string("/usr/local/apache2/conf/httpd.conf").unwrap();

            apr::set_header(headers_handle, "dummy", no_contents.as_str());
        } else {
            eprintln!("Nothing to do!");
            return;
        }
        eprintln!("!!!!!!!!!!!!!!!! UNREACHABLE !!!!!!!!!!!!!!!!");
    }
}

#[no_mangle]
pub unsafe extern "C" fn handle_request_headers(headers_handle: u64) -> i32 {
    handle_operation_header(headers_handle);

    handle_eval_header(headers_handle);

    handle_hash_header(headers_handle);

    handle_failure_header(headers_handle);
    0
}
