# 🏗️ Building wasm_runtime

The steps below show how to build wasm_runtime. Rust will target the host platform, building `libwasm_runtime.so` on Linux and `wasm_runtime.dll` on Windows.

The only requirements are:

- [Rust](https://www.rust-lang.org/) 

```console
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
- [cbindgen](https://github.com/eqrion/cbindgen)

```console
cargo install cbindgen
```

Next, execute `make all` in this folder, and it will invoke `cargo` with the proper tags and parameters.
```console
make all
```

### ⚠️ Dealing with the runtime linker

During the Apache Server start up sequence, when parsing a `LoadMoudule` directive, the specified dynamic library is loaded into memory at runtime. In our case:
```apache
LoadModule wasm_module modules/mod_wasm.so
```

At that time, the OS linker indentifies that `mod_wasm.so` depends on `libwasm_runtime.so` (or `wasm_runtime.dll` on Windows) and need to load it:

- On Linux, either such `libwasm_runtime.so` library is copied into one of the known libraries locations (ie: `/usr/local/lib`, etc.) or the `LD_LIRBRARY_PATH` environment variable include a directory with its location.
- On Windows, it is enough to copy `wasm_runtime.dll` into the `C:\Apache24\modules` folder.
