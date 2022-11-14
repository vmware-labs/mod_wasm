# Changelog

## Unreleased

-

## 0.8.0 (2022/11/14)

- Updated documentation.
- Added `libxml2-dev` as a dependency for building `mod_wasm.so`.
- Added version check mechanism when building `mod_wasm.so`. Now it checks `libwasm_runtime.so` version is compatible and the minimum required.
  
### `mod_wasm.so`
- Fixed directory hierarchy to be compliant with Apache Server (httpd)
  
### `libwasm_runtime.so`
- Bump version dependencies:
    - `wasmtime` to `2.0.2` (and other related Wasmtime crates)

## 0.7.0 (2022/11/03)

- Added support for sending HTTP Request body into WASI `stdin`.

## 0.6.0 (2022/10/26)

- Fixed build for aarm64.

## 0.5.0 (2022/10/21)

- Bump project version to `0.5.0` since it has been successfully tested with Python and PHP Wasm modules by other developers.

### `libwasm_runtime.so`
- Bump version dependencies:
    - `wasmtime` to `2.0.0`
    - `anyhow` to `1.0.66`
    - `once_cell` to `1.15.0`

## 0.1.0 (2022/10/04)

- Initial `mod_wasm` version!