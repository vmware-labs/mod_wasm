# Changelog

## Unreleased

-

## 0.10.1 (2022/12/12)

### `mod_wasm.so`
- All logging is now properly route to `ap_log_error()`, `ap_log_perror()` and `ap_log_rerror()`.
- Apache-2.0 extended license headers.
- Only C89 comments (/* */).

### `libwasm_runtime.so`
- Dependencies:
  - Bump version dependencies:
    - `wasmtime` to `3.0.1`.
  - Updated `cargo.lock` dependencies via `cargo update`.

## 0.10.0 (2022/11/28)

- In this version, among other improvements, we introduce two major features implementing [#6](https://github.com/vmware-labs/mod_wasm/issues/6), [#7](https://github.com/vmware-labs/mod_wasm/issues/7), and [#16](https://github.com/vmware-labs/mod_wasm/issues/16).
  1. **Wasm multi-module support:**
   
     Now you can specify different Wasm modules to be used in different routes. For instance, now it’s possible with one-single Apache instance to load simultaneously the Wasm builds for the [PHP](https://github.com/vmware-labs/webassembly-language-runtimes/releases) and [Python](https://github.com/tiran/cpython-wasm-test/releases) interpreters (and any other languages that compile to Wasm now or in the future). 

  2. **Shared Wasm modules:** 
   
     Now you can specify different per-route configurations to the same Wasm module. The Wasm binary is loaded in memory only once, and the different configurations are applied to the Wasm module per-HTTP request. Also, now each Wasm execution owns its own `stdout` buffer, so there are no interlocks between executions anymore.


  Combining all together, you can now have a more flexible configuration such as: 

    ```apache
    <Location /wordpress> 
        SetHandler wasm-handler 
        WasmModule /var/www/modules/php7.4.32.wasm 
        WasmDir    /tmp 
        … 
    </Location> 

    <Location /python-app> 
        SetHandler wasm-handler 
        WasmModule /var/www/modules/python3.11.wasm 
        WasmArg    /var/www/python-app/app.py 
        … 
    </Location> 

    <Location /python-app2> 
        SetHandler wasm-handler 
        WasmModule /var/www/modules/python3.11.wasm 
        WasmArg    /var/www/python-app2/app2.py 
        … 
    </Location> 
    ```

### `mod_wasm.so`
- Removed the `WasmRoot` directive. Now, `WasmModule` accepts the full Wasm module *filepath*.
- Removed legacy code:
  - In previous versions, given the only-one Wasm module limitation, `mod_wasm.c` implemented a mechanism to reset the WASI context for each HTTP request. This is not needed anymore.
  - Old code inherited from `mod_example_hooks.c` and not used.

### `libwasm_runtime.so`
- C-API:
  - Reorganized functions and their names to be compliant with the new features and to be more idiomatic (🚨 breaks backwards compatibility 🚨).
  - Removed any kind of logic different from C <---> Rust FFI data transformation.
  - Improved error messages with more information.
- Fixed most suggestions from `cargo clippy -- -W clippy::pedantic`.
- Dependencies:
  - Bump version dependencies:
    - `wasmtime` to `3.0.0`.
    - `once_cell` to `1.16.0`.
  - New dependency:
    - `rand` to `0.8.5`.
  - Updated `cargo.lock` dependencies via `cargo update`.

## 0.8.0 (2022/11/14)

- Updated documentation.
- Added `libxml2-dev` as a dependency for building `mod_wasm.so`.
- Added version check mechanism when building `mod_wasm.so`. Now it checks `libwasm_runtime.so` version is compatible and the minimum required.
  
### `mod_wasm.so`
- Fixed directory hierarchy to be compliant with Apache Server (httpd).
  
### `libwasm_runtime.so`
- Bump version dependencies:
    - `wasmtime` to `2.0.2` (and other related Wasmtime crates).

## 0.7.0 (2022/11/03)

- Added support for sending HTTP Request body into WASI `stdin`.

## 0.6.0 (2022/10/26)

- Fixed build for aarm64.

## 0.5.0 (2022/10/21)

- Bump project version to `0.5.0` since it has been successfully tested with Python and PHP Wasm modules by other developers.

### `libwasm_runtime.so`
- Bump version dependencies:
    - `wasmtime` to `2.0.0`.
    - `anyhow` to `1.0.66`.
    - `once_cell` to `1.15.0`.

## 0.1.0 (2022/10/04)

- Initial `mod_wasm` version!
