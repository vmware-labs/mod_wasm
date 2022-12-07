# mod_wasm

**mod_wasm** is an [**Apache Server** (httpd)](https://httpd.apache.org/) extension module able to run and serve [WebAssembly](https://webassembly.org/) binaries as endpoints.

The first version ([v0.1.0](https://github.com/vmware-labs/mod_wasm/blob/main/CHANGELOG.md#010-20221004)) was unveiled at the ApacheCon North America on Oct 3rd, 2022 (see the [slides](https://apachecon.com/acna2022/slides/01_Gonz%c3%a1lez_mod-wasm_Bringing_WebAssembly.pdf)). In addition, a full-detailed article was published at VMware's [Wasm Labs](https://wasmlabs.dev/articles/apache-mod-wasm/) page. 


### ✅ Features

**mod_wasm** can be useful in the different scenarios: 
* Run existing applications from a variety of languages without modification.
* Execute untrusted third-party code in a secure environment without using containers.
* The Wasm capabilities model allows to enable/disable capabilites per HTTP request.


### ▶️ Quick Demo

1. Run the container:
```console
docker run -p 8080:8080 ghcr.io/vmware-labs/httpd-mod-wasm:latest
```

2. Open browser at:

| Demo                   | Wasm Module  | URL                               |
| ---------------------- | ------------ | --------------------------------- |
| WordPress              | [PHP 7.3.33](https://github.com/vmware-labs/webassembly-language-runtimes/releases/tag/php%2F7.3.33%2B20221124-2159d1c) | [http://localhost:8080/wordpress](http://localhost:8080/wordpress) |
| HTTP Request Viewer    | [Python 3.11](https://github.com/tiran/cpython-wasm-test/releases/tag/v3.11.0) | [http://localhost:8080/http-request-viewer](http://localhost:8080/http-request-viewer) |


## 📔 Contents

- [mod\_wasm](#mod_wasm)
    - [✅ Features](#-features)
    - [▶️ Quick Demo](#️-quick-demo)
  - [📔 Contents](#-contents)
  - [🔭 Overview](#-overview)
    - [Apache Configuration](#apache-configuration)
    - [New Directives](#new-directives)
    - [Workflow](#workflow)
  - [🕹️ Examples](#️-examples)
  - [🏗️ Building mod\_wasm](#️-building-mod_wasm)
  - [📦 Building the container image](#-building-the-container-image)
  - [⚠️ Troubleshooting](#️-troubleshooting)
    - [Cannot load `modules/mod_wasm.so` into server](#cannot-load-modulesmod_wasmso-into-server)
  - [🐛 Debugging](#-debugging)


## 🔭 Overview

The **mod_wasm** project is composed by two different libraries:
- `mod_wasm.so` (written in C) acts as the extension module for the [Apache Server (httpd)](https://httpd.apache.org/).
- `libwasm_runtime.so` (written in Rust) offers a very high-level C-API to manage WebAssembly modules via [Wasmtime](https://wasmtime.dev/).

![alt Architecture](https://raw.githubusercontent.com/vmware-labs/mod_wasm/main/docs/slides/architecture.png)

### Apache Configuration

To enable **mod_wasm** in Apache, simply define your `<Location>` with the `wasm-handler` and the file path to the Wasm binary in `httpd.conf`:

```apache
LoadModule wasm_module modules/mod_wasm.so

<Location /hello-wasm>
  SetHandler wasm-handler
  WasmModule /var/www/modules/hello_wasm.wasm
</Location>
```

**mod_wasm** supports multiple `<Location>` definitions, each of them with its own configuration. In addition, multiple configurations can share the same .wasm file. **mod_wasm** will automatically cache Wasm modules and use only one instance on memory.

### New Directives

To setup and manage WebAssembly binaries and their [WASI](https://wasi.dev/) contexts, **mod_wasm** offers new directives to the `httpd.conf` configuration file:

| Directive                      | Description |
| ------------------------------ | ----------- |
| `WasmModule <path>`            | Specifies the Wasm module file path. |
| `WasmDir <dir>`                | Pre-open a host directory for the Wasm context. |
| `WasmMapDir <map> <dir>`       | Pre-open a host directory for the Wasm context and mount into a mapping directory. |
| `WasmArg <arg>`                | Set an argument to be passed to the Wasm module context. |
| `WasmEnv <env> <value>`        | Set an environment variable to be passed to the Wasm module context. |
| `WasmEnableCGI {On\|Off}`      | Enable/Disable CGI emulation mode. Default is `Off`. |


### Workflow

**mod_wasm** plays a role in two different stages of the Apache Server workflow:
1. On the boot up sequence, the different `WasmXXX` directives are read from `httpd.conf`:
   * When a `WasmModule` directive is found, the Wasm runtime tries to load the given Wasm binary from disk into memory. This is an expensive operation so that is why it is executed only once during the Apache boot up sequence. In addition, a cache is used to store the different Wasm modules, so a specific Wasm module can be shared among different configurations with only one instance loaded into memory.
   * The remaining `WasmXXX` directives define different configuration aspects. A new configuration instance is created for each `<Location>` and it will be later used during execution as a template.
2. On each incoming HTTP request, **mod_wasm** builds a new WASI context for the already-loaded Wasm binary. Next, the Wasm module is instantiated and the entry point is executed. The `stdout` from the Wasm module is redirected to the HTTP response, and the `stderr` is appended to Apache Server's trace (usually at `<httpd_dir>/dist/logs/error_log`). 

**mod_wasm** also offers the ability to build a specific execution context per HTTP request. When setting up `WasmEnableCGI On`, mod_wasm will pass the HTTP headers as environtment variables to the Wasm module (they will be prefixed as `HTTP_`). Also, URL parameters are passed in the environment variable `QUERY_STRING`. And finally, the HTTP request body is passed as the *stdin* to the module. 

![alt Workflow](https://raw.githubusercontent.com/vmware-labs/mod_wasm/main/docs/slides/workflow.png)


## 🕹️ Examples

This repo cointains several pre-built WebAssembly [examples](https://github.com/vmware-labs/mod_wasm/tree/main/examples) to play with.

Feel free to explore, modify and crash them!


## 🏗️ Building mod_wasm

<!-- ### Building `mod_wasm.so` extension module -->

As introduced in the [overview](#-overview), there are two main libraries in the **mod_wasm** project, being `libwasm_runtime.so` a dependency for `mod_wasm.so`. So, you might want to build `libwasm_runtime.so` first:

1) To build `libwasm_runtime.so`, the Wasm management and runtime library, go to [wasm_runtime](https://github.com/vmware-labs/mod_wasm/tree/main/wasm_runtime) for detailed instructions.
2) For `mod_wasm.so`, the Apache Server module extension, go to [mod_wasm](https://github.com/vmware-labs/mod_wasm/tree/main/mod_wasm).


## 📦 Building the container image

This repository contains all you need to build a local container image. Go to [image](https://github.com/vmware-labs/mod_wasm/tree/main/image) for detailed instructions.


## ⚠️ Troubleshooting

### Cannot load `modules/mod_wasm.so` into server

This is a common error related to `LD_LIBRARY_PATH`:
```
$ httpd
httpd: Syntax error on line XXX of <...>/httpd/dist/conf/httpd.conf:
Cannot load modules/mod_wasm.so into server: libwasm_runtime.so: cannot open shared object file: No such file or directory
```

Apache is loading `modules/mod_wasm.so` but during the process it cannot find `libwasm_runtime.so`. Either run Apache with `LD_LIBRARY_PATH` pointing to the directory where `libwasm_runtime.so` is located, or copy `libwasm_runtime.so` to a directory such as `/usr/lib`. 


## 🐛 Debugging

To get detailed debugging information about the Wasm execution, run the Apache Server with the following environment variables:
* `WASMTIME_BACKTRACE_DETAILS=1`
* `RUST_BACKTRACE=full`

Also, it is recommended to run Apache in debug mode (`-X` option), this means only one process, only one worker, and without detaching from the terminal.

```
WASMTIME_BACKTRACE_DETAILS=1 RUST_BACKTRACE=full ./httpd -X
```
