# 🏗️ Building wasm_runtime

The steps below show how to build **wasm_runtime**.

Rust will automatically target the host platform, building `libwasm_runtime.so` on Linux and `wasm_runtime.dll` on Windows.

### Requirements

- [Rust](https://www.rust-lang.org/): Go to the [getting started](https://www.rust-lang.org/learn/get-started) section and follow instructions for your platform.
- [cbindgen](https://github.com/eqrion/cbindgen): Execute `cargo install cbindgen` once Rust is installed.

### Build
Execute `make all` in this folder, and it will invoke `cargo` with the proper tags and parameters.

### ⚠️ Dealing with the runtime linker

During the Apache Server start up sequence, when parsing a `LoadMoudule` directive, the specified dynamic library is loaded into memory at runtime. In our case:
```apache
LoadModule wasm_module modules/mod_wasm.so
```

At that time, the OS linker indentifies that `mod_wasm.so` depends on `libwasm_runtime.so` (or `wasm_runtime.dll` on Windows) and need to load it:

- On Linux, either such `libwasm_runtime.so` library is copied into one of the known libraries locations (ie: `/usr/local/lib`, etc.) or the `LD_LIRBRARY_PATH` environment variable include a directory with its location.
- On Windows, it is enough to copy `wasm_runtime.dll` into the `C:\Apache24\modules` folder.
