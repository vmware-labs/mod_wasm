## Examples

This repo cointains several pre-built webassembly modules along with their
respective configurations. To try them out quickly you will first have to
clone this repository.

### Running the different examples

To play with the example, when you run a container you will have to:
 - mount `examples/wasm_modules` over the `/usr/local/apache2/wasm_modules` folder that comes with the image.
 - set the WASM_MODULE_CONFIG environment variable to the example you want to try
 - Point your browser again at [http://localhost:8080/wasm-module-endpoint]()
    and see how the different example behaves.

For convenience the command is available with each example below.

### cgi_hello_python.conf

```console
docker run --rm -e WASM_MODULE_CONFIG="cgi_hello_python.conf" -p 8080:8080 -v ./examples/wasm_modules/:/usr/local/apache2/wasm_modules/ projects.registry.vmware.com/wasmlabs/containers/httpd-mod-wasm:latest
```

Runs the [cgi_hello_python.py](./examples/wasm_modules/python-scripts/cgi_hello_python.py) script.

This cgi compatible script will just print out the current environment variables.

### cgi_prettify.conf

```console
docker run --rm -e WASM_MODULE_CONFIG="cgi_prettify.conf" -p 8080:8080 -v ./examples/wasm_modules/:/usr/local/apache2/wasm_modules/ projects.registry.vmware.com/wasmlabs/containers/httpd-mod-wasm:latest
```

Runs the [cgi_prettify.py](./examples/wasm_modules/python-scripts/cgi_prettify.py) script.

This cgi compatible script will print out the contents of `uploads` folder. You can see in the config that `uploads` is mapped to the `/usr/local/apache2/wasm_modules/python-scripts/uploads` folder on the server.

If called with a `?file=/path/to/file` parameter it will print the given file. Running this as a cgi script will expose any file on the system, while running it in mod_wasm will automatically secure it only to the mapped folders.

### cgi_python.conf

```console
docker run --rm -e WASM_MODULE_CONFIG="cgi_python.conf" -p 8080:8080 -v ./examples/wasm_modules/:/usr/local/apache2/wasm_modules/ projects.registry.vmware.com/wasmlabs/containers/httpd-mod-wasm:latest
```

Runs the [cgi_python.py](./examples/wasm_modules/python-scripts/cgi_python.py) script.

This cgi compatible script includes backdoors for listing dirs, opening files or running programs. Running this as a cgi script will expose the server system, while running it in mod_wasm will automatically allow only sandboxed access.

### cgi_search_word_count.conf

```console
docker run --rm -e WASM_MODULE_CONFIG="cgi_search_word_count.conf" -p 8080:8080 -v ./examples/wasm_modules/:/usr/local/apache2/wasm_modules/ projects.registry.vmware.com/wasmlabs/containers/httpd-mod-wasm:latest
```

Runs the [cgi_search_word_count.py](./examples/wasm_modules/python-scripts/cgi_search_word_count.py) script.

This cgi compatible script will count the occurence of a word in a file. Running this as a cgi script will expose the server system, while running it in mod_wasm will automatically allow only sandboxed access.

The file name and word are passed as custom http parameters. For example:

```console
curl -H "File: Sherlock.txt" -H "Word: elementary" http://localhost:8080/wasm-module-endpoint
curl -H "File: Sherlock.txt" -H "Word: elementary" http://localhost:8080/cgi-bin/cgi_search_word_count.py
```

### hello_python_html.conf

```console
docker run --rm -e WASM_MODULE_CONFIG="hello_python_html.conf" -p 8080:8080 -v ./examples/wasm_modules/:/usr/local/apache2/wasm_modules/ projects.registry.vmware.com/wasmlabs/containers/httpd-mod-wasm:latest
```

Runs the [hello_python_html.py](./examples/wasm_modules/python-scripts/hello_python_html.py) script.

This is just a python script that lists the contents of the `/home` folder. It is not cgi compatible.
You can see in the config file that `/home` is mapped to `wasm_modules` on the server.

### rust_hello_wasm.conf

```console
docker run --rm -e WASM_MODULE_CONFIG="rust_hello_wasm.conf" -p 8080:8080 -v ./examples/wasm_modules/:/usr/local/apache2/wasm_modules/ projects.registry.vmware.com/wasmlabs/containers/httpd-mod-wasm:latest
```

Runs the `hello_wasm` binary built from [examples/rust-src/hello_wasm](./examples/rust-src/hello_wasm/src/main.rs).

This is just a simple hello world in rust, which will run in a sandboxed mod_wasm environment.

### rust_list_dir.conf

```console
docker run --rm -e WASM_MODULE_CONFIG="rust_list_dir.conf" -p 8080:8080 -v ./examples/wasm_modules/:/usr/local/apache2/wasm_modules/ projects.registry.vmware.com/wasmlabs/containers/httpd-mod-wasm:latest
```

Runs the `list_dir` binary built from [examples/rust-src/list_dir](./examples/rust-src/list_dir/src/main.rs).

This is just a simple hello world in rust, which will run in a sandboxed mod_wasm environment.

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

## Building the example modules

The WebAssembly modules should be built for `wasm32-wasi`.

You will find the respective source code in `examples/src`, while the compiled
ready-to-use modules and their respective configurations are in `examples/wasm_modules`.

Unless you decide to mount the whole `examples` folder in your development container,
don't forget to copy the python source files or rust wasm modules from `src` into `wasm_modules`.

### Python examples

For the python-based examples we are relying on [fermyon/wagi-python](https://github.com/fermyon/wagi-python)
which provides both the python binary (`python3.11.wasm`) and the standard python modules (`python311.zip`)
with [Apache License 2.0](https://github.com/fermyon/wagi-python/blob/main/LICENSE).

As of 2022-10-01, Fermyon claims that their binaries are based on [singlestore-labs/python-wasi](https://github.com/singlestore-labs/python-wasi).

In case you want to rebuild and replace those, you may need to configure PYTHONHOME and PYTHONPATH
appropriately. You can see how to do this any `.conf` file in `examples/wasm_modules`.

The python code itself is not compiled to a wasm module, but is interpreted on the spot.

The [cgi_prettify.py](examples/wasm-modules/python-scripts/cgi_prettify.py) example uses
the [pygments/pygments](https://github.com/pygments/pygments) library in `pygments.zip` with
[BSD 2-Clause "Simplified" License](https://github.com/pygments/pygments/blob/master/LICENSE)

### Rust examples

Assuming you are familiar with Rust and its development tools, to be able to
build the Rust examples you will need to add the `wasm32-wasi` target via:

```console
rustup target add wasm32-wasi
```

Then just go inside any source folder (e.g. `examples/rust-src/hello_wasm` or
`examples/rust-src/list_dir`) and run:

```console
cargo build --release --target=wasm32-wasi
```

After building don't forget to copy the module from the respective `target/wasm32-wasi/release/`
to the mounted `wasm_modules/rust-wasm` folder.