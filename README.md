# mod_wasm

**mod_wasm** is an [**Apache Server** (httpd)](https://httpd.apache.org/) extension module able to run and serve [WebAssembly](https://webassembly.org/) binaries as endpoints.

It was unveiled at the ApacheCon North America on Oct 3rd, 2022 (see the [slides](https://apachecon.com/acna2022/slides/01_Gonz%c3%a1lez_mod-wasm_Bringing_WebAssembly.pdf)). In addition, a full-detailed article can be found at VMware's [Wasm Labs](https://wasmlabs.dev/articles/apache-mod-wasm/) page. 


### ✅ Features

**mod_wasm** can be useful in the different scenarios: 
* Run existing applications from a variety of languages without modification.
* Execute untrusted third-party code in a secure environment without using containers.
* The Wasm capabilities model allows to enable/disable capabilites per HTTP request.



### ▶️ Quick Demo

1. Run the container:
```console
docker run -p 8080:8080 projects.registry.vmware.com/wasmlabs/containers/httpd-mod-wasm:latest
```

2. Open browser at:
[http://localhost:8080/wasm-module-endpoint](http://localhost:8080/wasm-module-endpoint)

More details about the ['PrettyFy' WebApp Demo](#-prettyfy-webapp-demo) below.


## 📔 Contents

* [Overview](#-overview)
* ['PrettyFy' WebApp Demo](#-prettyfy-webapp-demo)
* [Examples](#%EF%B8%8F-examples)
* [Building mod_wasm in your environment](#%EF%B8%8F-building-mod_wasm-in-your-environment)
* [Building the container image](#-building-the-container-image)
* [Troubleshooting](#%EF%B8%8F-troubleshooting)
* [Debugging](#-debugging)


## 🔭 Overview

The **mod_wasm** project is composed by two different libraries:
- `mod_wasm.so` (written in C) acts as the extension module for the [Apache Server (httpd)](https://httpd.apache.org/).
- `libwasm_runtime.so` (written in Rust) offers a very high-level C-API to manage WebAssembly modules via [Wasmtime](https://wasmtime.dev/).

![alt Architecture](https://raw.githubusercontent.com/vmware-labs/mod_wasm/main/docs/slides/architecture.png)


### New Directives

To setup and manage WebAssembly binaries, **mod_wasm** offers new directives to the `httpd.conf` configuration file:

| Directive       | Description |
| --------------- | ----------- |
| `WasmRoot`      | Set the root directory for Wasm modules. |
| `WasmModule`    | Set the Wasm module file name. |
| `WasmDir`       | Pre-open a host directory for the Wasm context. |
| `WasmMapDir`    | Pre-open a host directory for the Wasm context and mount into a given directory. |
| `WasmArg`       | Set an argument to be passed to the Wasm module context. |
| `WasmEnv`       | Set an environment variable to be passed to the Wasm module context. |
| `WasmEnableCGI` | Enable/Disable CGI emulation mode for HTTP requests. |


### Workflow

**mod_wasm** plays a role in two different stages of the Apache Server workflow:
1. The different `WasmXXX` directives are read from `httpd.conf` during the boot up sequence. Once the configuration if fully processed, mod_wasm requests to the Wasm runtime to start loading the Wasm binaries. This is by far the most expensive operation and that is why it is executed only once during the Apache boot up sequence. When completed, the Apache Sever is ready to response to incoming HTTP requests.
2. For each HTTP request, mod_wasm builds the WASI context for the already-loaded Wasm binary. Next, the Wasm module is instantiated and the entry point is executed. The `stdout` from the Wasm module is redirected to the HTTP response, and the `stderr` is appended to Apache Server's trace (usually at `<httpd_dir>/dist/logs/error_log`). 

**mod_wasm** also offers the ability to build a specific execution context per HTTP request. When setting up `WasmEnableCGI On`, mod_wasm will pass HTTP headers as environtment variables to the Wasm module (they will be prefixed as `HTTP_`). In addition, URL parameters are also passed in the environment variable `QUERY_STRING`.

![alt Workflow](https://raw.githubusercontent.com/vmware-labs/mod_wasm/main/docs/slides/workflow.png)


## ⭐ 'PrettyFy' WebApp Demo

The 'PrettyFy' demo is a simple one-script, Python-based WebApp (see [Examples](#%EF%B8%8F-examples)).
* The Python interpreter has been compiled to WebAssembly.
* Note how the system platform is identified: `sys.platform = WASI`.
* The app accepts `file=` as URL parameter to highlight a previously uploaded file:
  * [http://localhost:8080/wasm-module-endpoint?file=uploaded_text.txt](http://localhost:8080/wasm-module-endpoint?file=uploaded_text.txt)
  * [http://localhost:8080/wasm-module-endpoint?file=cgi_hello_python.py](http://localhost:8080/wasm-module-endpoint?file=cgi_hello_python.py)
* Now, if you try a basic [path traversal](https://owasp.org/www-community/attacks/Path_Traversal) attack, it won't be succesful thanks to the WebAssembly sandboxed model where the Python interpreter is running:
  * [http://localhost:8080/wasm-module-endpoint?file=../../conf/httpd.conf](http://localhost:8080/wasm-module-endpoint?file=../../conf/httpd.conf)


## 🕹️ Examples

This repo cointains several pre-built WebAssembly [examples](https://github.com/vmware-labs/mod_wasm/tree/main/examples) to play with.

Feel free to explore, modify and crash them!


## 🏗️ Building mod_wasm in your environment

### Prerequisites

- Apache Portable Runtime Project (apr)
- Apache Portable Runtime Utility Library (aprutil)
- Apache HTTP Server (development headers)
- Rust
- C compiler
- `pkg-config`
- `libtool`

For example, in an Ubuntu environment, you can install all
dependencies by running:

```console
apt install make pkg-config libtool-bin cargo libapr1-dev libaprutil1-dev apache2-dev
```

Also, [cbindgen](https://github.com/eqrion/cbindgen) is needed to generate the C bindings from Rust:

```console
cargo install cbindgen
```



### Building

```console
make build
```

After the build is complete, you can find the module and an example
Apache configuration file under the `dist` directory:

```console
$ tree dist
dist
|-- conf
|   `-- httpd.conf
`-- modules
    `-- mod_wasm.so
```

Now, you can load this module in your Apache installation.


## 📦 Building the container image

This repository contains all you need to build a local container image

### Prerequisites

To build the container you will need `docker`, or `podman` with the `docker` alias
enabled.

For convenience we've organized the build commands in a [Makefile](./Makefile), so you can use `make` if you prefer.

### Build

You can build this image like so:

```console
make container-image
```

### Building a dev image

The dev image will include all examples, along with additional tools required for future development. If you want to benchmark and compare running a python script via cgi vs via mod_wasm you will need to build this image.

```console
make dev-image
```

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
