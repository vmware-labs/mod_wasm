# `mod_wasm`

`mod_wasm` is an [**Apache Server** (httpd)](https://httpd.apache.org/) extension module able to run and serve [WebAssembly](https://webassembly.org/) binaries as endpoints.

`mod_wasm` can be useful in the different scenarios: 
* Run existing applications from a variety of languages without modification.
* Execute untrusted third-party code in a secure environment without using containers.
* The Wasm capabilities model allows to enable/disable capabilites per HTTP request (*still WIP).

A full-detailed article can be found at VMware's [Wasm Labs](https://wasmlabs.dev/articles/apache-mod-wasm/) page.


## Quick Demo

1. Run the container:
```console
docker run -p 8080:8080 projects.registry.vmware.com/wasmlabs/containers/httpd-mod-wasm:latest
```

2. Open browser at:
[http://localhost:8080/wasm-module-endpoint](http://localhost:8080/wasm-module-endpoint)

More detailes about the ['PrettyFy' WebApp Demo](#prettyfy-webapp-demo) below.


## Table of contents

* ['PrettyFy' WebApp Demo](#prettyfy-webapp-demo)
* [Examples](#examples)
* [Building mod_wasm in your environment](#building-mod_wasm-in-your-environment)
* [Building the container image](#building-the-container-image)
* [Troubleshooting](#troubleshooting)
* [Debugging mod_wasm and WebAssembly](#debugging-mod_wasm-and-webassembly)


## 'PrettyFy' WebApp Demo

The 'PrettyFy' demo is a simple one-script, Python-based WebApp (see [Examples](#examples)).
* The Python interpreter has been compiled to WebAssembly.
* Note how the system platform is identified: `sys.platform = WASI`.
* The app accepts `file=` as URL parameter to highlight a previously uploaded file:
  * [http://localhost:8080/wasm-module-endpoint?file=uploaded_text.txt](http://localhost:8080/wasm-module-endpoint?file=uploaded_text.txt)
  * [http://localhost:8080/wasm-module-endpoint?file=cgi_hello_python.py](http://localhost:8080/wasm-module-endpoint?file=cgi_hello_python.py)
* Now, if you try a basic [path traversal](https://owasp.org/www-community/attacks/Path_Traversal) attack, it won't be succesful thanks to the WebAssembly sandboxed model where the Python interpreter is running:
  * [http://localhost:8080/wasm-module-endpoint?file=../../conf/httpd.conf](http://localhost:8080/wasm-module-endpoint?file=../../conf/httpd.conf)


## Examples

This repo cointains several pre-built WebAssembly modules as examples along with their
respective configurations.

Go to [examples/](https://github.com/vmware-labs/mod_wasm/tree/main/examples) for more information.


## Building mod_wasm in your environment

### Prerequisites

- Apache Portable Runtime Project (apr)
- Apache Portable Runtime Utility Library (aprutil)
- Apache HTTP Server (development headers)
- Cargo
- C compiler
- `pkg-config`

For example, in an Ubuntu environment, you can install all
dependencies by running:

```
apt install make cargo libapr1-dev libaprutil1-dev pkg-config apache2-dev
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


## Building the container image

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

## Troubleshooting

### Cannot load `modules/mod_wasm.so` into server

This is a common error related to `LD_LIBRARY_PATH`:
```
$ httpd
httpd: Syntax error on line XXX of <...>/httpd/dist/conf/httpd.conf:
Cannot load modules/mod_wasm.so into server: libwasm_runtime.so: cannot open shared object file: No such file or directory
```

Apache is loading `modules/mod_wasm.so` but during the process it cannot find `libwasm_runtime.so`. Either run Apache with `LD_LIBRARY_PATH` pointing to the directory where `libwasm_runtime.so` is located, or copy `libwasm_runtime.so` to a directory such as `/usr/lib`. 


## Debugging mod_wasm and WebAssembly

To get detailed debugging information about Wasm execution within Wasmtime, run Apache with the following environment variables:
* `WASMTIME_BACKTRACE_DETAILS=1`
* `RUST_BACKTRACE=full`

Also, it is recommended to run Apache in debug mode (`-X` option), with only one worker and without detaching from the terminal.

```
WASMTIME_BACKTRACE_DETAILS=1 RUST_BACKTRACE=full ./httpd -X
```
