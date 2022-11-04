# 🏗️ Building `libwasm_runtime.so`

## Requirements

- [Rust](https://www.rust-lang.org/) 

```console
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
- [cbindgen](https://github.com/eqrion/cbindgen)

```console
cargo install cbindgen
```

## Building `libwasm_runtime.so`

Just execute `make` in this folder, and it will invoke `cargo` with the proper tags and parameters.
   ```console
      make
   ```

## Dealing with the runtime linker (`ld`)

During the Apache Server start up sequence, when parsing a `LoadMoudule` directive, the specified dynamic library is loaded into memory at runtime. In our case:
```apache
LoadModule wasm_module modules/mod_wasm.so
```

At that time, the OS linker indentifies that `mod_wasm.so` depends on `libwasm_runtime.so`. So either `libwasm_runtime.so` is copied into one of the known libraries locations (ie: `/usr/local/lib`, etc.) or the LD_LIRBRARY_PATH environment variable include a directory with its location.
