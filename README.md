# `mod_wasm`

Welcome to the `mod_wasm` project! This project implements an Apache
module that is able to execute WebAssembly modules.

## Running in a container

A container image is provided for testing convenience. You can find it
as `httpd-mod-wasm:latest`.

This image is based on `docker.io/library/httpd`. The image has some
points of interest:

- `/usr/local/apache2/conf/httpd.conf`: the Apache
  configuration. Override this file if you want to tweak specific
  Apache settings.

- `/usr/local/apache2/wasm_modules`: this is the path where Wasm
  modules can be placed (either provided by subsequent layers or
  mounted).

- `/usr/local/apache2/wasm_modules/mod_wasm.conf`: the mod_wasm
  configuration file, to be added by subsequent layers or mounted.

By default, the Apache server will be listening on port 8080.

We have provided a few examples of different wasm_modules which demonstrate
what `mod_wasm.conf` could look like.

### Default example

To try ou the default wasm module just type

```console
$ docker run -d -p8080:8080 httpd-mod-wasm:latest
```

Then open a browser at `http://localhost:8080/wasm-module-endpoint` and enjoy.

### Running the different examples

This repo cointains several pre-built webassembly modules along with their
respective configurations.

To play with them you will need to create a container based on the same image
but with `examples/wasm_modules` shadowing the `/usr/local/apache2/wasm_modules`
folder that comes with the image. Here is how to do it (we'll assume the
name `mod_wasm_examples` for convenience):

```console
$ docker run --name mod_wasm_examples -d -p8080:8080 -v /path/to/this/repo/examples/wasm_modules/:/usr/local/apache2/wasm_modules/ httpd-mod-wasm:latest
```

Now to try something different from the default demo you need to:

 1. Open `examples/wasm_modules/mod_wasm.conf`, comment out the currently
    loaded example and uncomment the one you desire.
 2. Restart the container via `docker restart mod_wasm_examples`
 3. Point your browser again at `http://localhost:8080/wasm-module-endpoint`
    and see how the different example behaves.

## Building the container image

This repository contains all you need to build a local container image

### Prerequisites

To build the container you will need `docker`, or `podman` with the `docker` alias
enabled.

For convenience we've added a Makefile, so you should also have `make`.

### Build

You can build this image like so:

```console
$ make container-image
<snip>
Successfully tagged httpd-mod-wasm:latest
```

## Building the example modules

The WebAssembly modules should be built for `wasm32-wasi`.

You will find the respective source code in `examples/src`, while the compiled
ready-to-use modules and their respective configurations are in `examples/wasm_modules`.

Unless you decide to mount the whole `examples` folder in your development container,
don't forget to copy the python source files or rust wasm modules from `src` into `wasm_modules`.

### Python examples

For the python-based examples we are relying on [fermyon/wagi-python](https://github.com/fermyon/wagi-python)
which provides both the python binary (`python3.11.wasm`) and the standard python modules (`python311.zip`).

As of 2022-10-01, Fermyon claims that their binaries are based on [singlestore-labs/python-wasi](https://github.com/singlestore-labs/python-wasi).

In case you want to rebuild and replace those, you may need to configure PYTHONHOME and PYTHONPATH
appropriately. You can see how to do this any `.conf` file in `examples/wasm_modules`.

The python code itself is not compiled to `wasm32-wasi` and is interpreted on the spot.

### Rust examples

Assuming you are familiar with Rust and its development tools, to be able to
build the Rust examples you will need to add the `wasm32-wasi` target via:

```console
$ rustup target add wasm32-wasi
```

Then just go inside any source folder (e.g. `examples/src/hello_wasm` or
`examples/src/list_dir`) and run:

```console
$ cargo build --release --target=wasm32-wasi
```

After building don't forget to copy the module from the respective `target/wasm32-wasi/release/`
to the mounted `wasm_modules` folder.
