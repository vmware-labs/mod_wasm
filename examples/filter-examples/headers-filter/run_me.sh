#!/usr/bin/env bash

step_no=1
step() {
    sleep 2
    echo -e "\n\n"
    echo -e "\e[33m======================================================================="
    echo -e "\e[33m   Step ${step_no} '$@':"
    echo -e "\e[33m======================================================================="
    step_no=$(expr 1 + ${step_no})
}

explain() {
    echo -e "\e[36m$@"
}

execute() {
    echo -e "\e[32m-----------------------------------------------------------------------"
    echo -e "\e[32m\$" "$@"
    echo -e "\e[32m-----------------------------------------------------------------------"
    echo -e "\e[37m"
    "$@"
    echo -e "\e[32m-----------------------------------------------------------------------"
    echo -e "\n"
}

step "Same PHP script, which prints headers is configured to serve at two locations:"
explain "Alias /sample-mod-headers /usr/local/apache2/headers-filter/php-sample"
explain "Alias /sample-mod-wasm /usr/local/apache2/headers-filter/php-sample"
explain

execute curl localhost:8080/sample-mod-headers/index.php
execute curl localhost:8080/sample-mod-wasm/index.php

step "Modifying 'target' header based on value of 'operation' header with mod_headers"
explain "[httpd.conf]"
explain
cat <<EOF
<Location /sample-mod-headers>
    SetEnvIf Operation "^add$" OPERATION_add
    RequestHeader set "added" "added_value" env=OPERATION_add

    SetEnvIf Operation "^modify$" OPERATION_modify
    RequestHeader set "target" "modified_value" env=OPERATION_modify

    SetEnvIf Operation "^delete$" OPERATION_delete
    RequestHeader unset "target" env=OPERATION_delete
</Location>
EOF

execute curl localhost:8080/sample-mod-headers/index.php --no-progress-meter -H "operation:add"
execute curl localhost:8080/sample-mod-headers/index.php --no-progress-meter -H "operation:delete" -H "target:value"
execute curl localhost:8080/sample-mod-headers/index.php --no-progress-meter -H "operation:modify" -H "target:value"


step "Modifying 'target' header based on value of 'operation' header with mod_wasm and edit_headers.wasm"
explain "[httpd.conf]"
explain
cat <<EOF
<IfModule wasm_module>
    <Location /sample-mod-wasm>
        WasmFilter /usr/local/apache2/wasm_modules/rust-wasm/edit_headers.wasm
    </Location>
</IfModule>
EOF

explain "Rust source:"
explain
cat <<EOF
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
EOF


step "Evaluating a header's value with mod_wasm and edit_headers.wasm. Can't do with mod_headers"

explain "Rust source:"
explain
cat <<EOF
fn handle_eval_header(headers_handle: u64) {
    if let Some(eval_me) = apr::get_header(headers_handle, "evaluate-me") {
        let result = match eval(eval_me.as_str()) {
            Ok(result) => result.to_string(),
            Err(e) => format!("ERROR: {}", e.to_string()),
        };
        apr::set_header(headers_handle, "evaluate-result", result.as_str());
    }
}
EOF

execute curl localhost:8080/sample-mod-wasm/index.php --no-progress-meter -H "evaluate-me:3*8-2*7"


step "Hash a header's value with mod_wasm and edit_headers.wasm. Can't do with mod_headers"

explain "Rust source:"
explain
cat <<EOF
fn handle_hash_header(headers_handle: u64) {
    if let Some(hash_me) = apr::get_header(headers_handle, "hash-me") {
        let result = compute(hash_me.as_bytes());
        apr::set_header(headers_handle, "hash-result", format!("{:x}", result).as_str());
    }
}
EOF

execute curl localhost:8080/sample-mod-wasm/index.php --no-progress-meter -H "hash-me:The quick brown fox"



step "Failures in mod_wasm and edit_headers.wasm don't affect Apache's stability. Can't do with a traditional module"

explain "Rust source:"
explain
cat <<EOF
fn handle_failure_header(headers_handle: u64) {
    if let Some(fail_me) = apr::get_header(headers_handle, "fail-me") {
        eprintln!("Simulating error '{}'...", fail_me);
        if fail_me == "division-by-zero" {
            let no_result = 1234 / 0;
            apr::set_header(headers_handle, "dummy", format!("{}", no_result).as_str());

        } else if fail_me == "filesystem-access" {
            let no_contents = fs::read_to_string("/usr/local/apache2/conf/httpd.conf").unwrap();
            apr::set_header(headers_handle, "dummy", no_contents.as_str());
        } ...
    }
}
EOF

execute curl localhost:8080/sample-mod-wasm/index.php --no-progress-meter -H "fail-me:division-by-zero"
test -f /usr/local/apache2/logs/error_log && execute tail -n 4 /usr/local/apache2/logs/error_log

execute curl localhost:8080/sample-mod-wasm/index.php --no-progress-meter -H "fail-me:filesystem-access"
test -f /usr/local/apache2/logs/error_log && execute tail -n 4 /usr/local/apache2/logs/error_log

explain "Apache is still up even with the fatal failures in the Wasm code:"
explain "ps -eo comm,etime,user | grep root | grep httpd"
ps -eo comm,etime,user | grep root | grep httpd

explain -e "\n\n ^^^^^^^^^ Scroll back through the above output. It is self descriptive! ^^^^^^^^^^\n"
