# `mod_wasm`

Welcome to the `mod_wasm` project! This project implements an Apache
module that is able to execute WebAssembly modules.

To try out the default wasm module just type:

```console
$ docker run -p 8080:8080 projects.registry.vmware.com/wasmlabs/containers/httpd-mod-wasm:latest
```

Then open a browser at [http://localhost:8080/wasm-module-endpoint](http://localhost:8080/wasm-module-endpoint) and enjoy.

## Table of contents

* [Running in a container](#running-in-a-container)
  * [Default example](#default-example)
  * [Running different examples](#running-the-different-examples)
  * [Running development image](#running-the-dev-image)
* [Demonstrating security capabilities](#demonstrating-security-capabilities)
* [Examples](#examples)
  * [cgi_hello_python.conf](#cgi_hello_pythonconf)
  * [cgi_prettify.conf](#cgi_prettifyconf)
  * [cgi_python.conf](#cgi_pythonconf)
  * [cgi_search_word_count.conf](#cgi_search_word_countconf)
  * [hello_python_html.conf](#hello_python_htmlconf)
  * [rust_hello_wasm.conf](#rust_hello_wasmconf)
  * [rust_list_dir.conf](#rust_list_dirconf)
* [Building the container image](#building-the-container-image)
* [Building the examples modules](#building-the-example-modules)

## Running in a container

A container image is provided for testing convenience. You can find it
as `projects.registry.vmware.com/wasmlabs/containers/httpd-mod-wasm:latest`.

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

**Note**: If you don't have access to the `projects.registry.vmware.com/wasmlabs/containers/httpd-mod-wasm:latest` image from
a public repository, you can build it on your own by following the
[Building the container image](#Building-the-container-image) section below.

To try out the default wasm module just type

```console
$ docker run -p 8080:8080 projects.registry.vmware.com/wasmlabs/containers/httpd-mod-wasm:latest
```

Then open a browser at [http://localhost:8080/wasm-module-endpoint]() and enjoy.

By default you will see the `cgi_prettify.py` script described below running
with a wasm python binary.

### Running the different examples

This repo cointains several pre-built webassembly modules along with their
respective configurations.

To play with them you will need to create a container based on the same image
but with `examples/wasm_modules` shadowing the `/usr/local/apache2/wasm_modules`
folder that comes with the image. Here is how to do it (we'll assume the
name `mod_wasm_examples` for convenience):

```console
$ docker run --name mod_wasm_examples -p 8080:8080 -v ./examples/wasm_modules/:/usr/local/apache2/wasm_modules/ projects.registry.vmware.com/wasmlabs/containers/httpd-mod-wasm:latest
```

Now to try something different from the default demo you need to:

 1. Open `examples/wasm_modules/mod_wasm_demo.conf`, comment out the currently
    loaded example and uncomment the one you desire.
 2. Restart the container via `docker restart mod_wasm_examples`
 3. Point your browser again at [http://localhost:8080/wasm-module-endpoint]()
    and see how the different example behaves.

### Running the dev image

If you plan on experimenting a comparison between executing python via cgi and mod_wasm you will need to run the development container image. Refer to [Building a dev image](#Building-a-dev-image).

To just run the image and see the default example (with enabled cgi access to everything) use

```console
docker run --name mod_wasm_dev_examples -p 8080:8080 httpd-mod-wasm-dev:latest
```

If you want to easily modify the examples and restart an existing container (as described above) run with

```console
$ docker run --name mod_wasm_dev_examples -p 8080:8080 -v ./examples/wasm_modules/:/usr/local/apache2/wasm_modules/ httpd-mod-wasm-dev:latest
```

## Demonstrating security capabilities

Run the dev image as described above. By default the loaded wasm module is python running the `cgi_prettify.py` script.

Let's examine how it behaves compared to running the script via CGI.

### 1. By default the script will list the contents of an `uploads` folder.

 - Open [http://localhost:8080/wasm-module-endpoint]() to view the listing via mod_wasm. Note that `sys.platform = wasi` information.
 - Open [http://localhost:8080/cgi-bin/cgi_prettify.py]() to view the listing via cgi. Note that `sys.platform = linux`.

### 2. Now let's prettify a file

 - Open [http://localhost:8080/wasm-module-endpoint?file=cgi_hello_python.py]() to do this via mod_wasm.
 - Open [http://localhost:8080/cgi-bin/cgi_prettify.py?file=cgi_hello_python.py]() to do this via CGI.

### 3. Let's try to hack a bit

So let's think like a hacker that wants to try and get access to any file on the system:

1. It looks like the path to the prettified file is relative to `uploads`.
2. We know for sure that the apache httpd will have access to `/usr/local/apache2/conf/httpd.conf`.
3. Then let's just try and access this file relative to `uploads`. Starting from `../usr/local/apache2/conf/httpd.conf` with CGI we can easily go to [http://localhost:8080/cgi-bin/cgi_prettify.py?file=../../../../../../usr/local/apache2/conf/httpd.conf]()
4. Voila - we have access to the server's configuration and we know where root is located relative to `uploads`.

However, that will never happen with mod_wasm. Just give it a try and see that we have no access outside the `uploads` folder - [http://localhost:8080/wasm-module-endpoint?file=../../../../../../usr/local/apache2/conf/httpd.conf]()

## Examples

Here you can find a list of the included examples and what they do

### cgi_hello_python.conf

Runs the [cgi_hello_python.py](./examples/wasm_modules/python-scripts/cgi_hello_python.py) script. To see differences when this runs as a cgi script try [http://localhost:8080/cgi-bin/cgi_hello_python.py]()

This cgi compatible script will just print out the current environment variables.

### cgi_prettify.conf

Runs the [cgi_prettify.py](./examples/wasm_modules/python-scripts/cgi_prettify.py) script. To see differences when this runs as a cgi script try [http://localhost:8080/cgi-bin/cgi_prettify.py]()

This cgi compatible script will print out the contents of `uploads` folder. You can see in the config that `uploads` is mapped to the `/usr/local/apache2/wasm_modules/python-scripts/uploads` folder on the server.

If called with a `?file=/path/to/file` parameter it will print the given file. Running this as a cgi script will expose any file on the system, while running it in mod_wasm will automatically secure it only to the mapped folders.

### cgi_python.conf

Runs the [cgi_python.py](./examples/wasm_modules/python-scripts/cgi_python.py) script. To see differences when this runs as a cgi script try [http://localhost:8080/cgi-bin/cgi_python.py]()

This cgi compatible script includes mackdoors for listing dirs, opening files or running programs. Running this as a cgi script will expose the server system, while running it in mod_wasm will automatically allow only sandboxed access.

### cgi_search_word_count.conf

Runs the [cgi_search_word_count.py](./examples/wasm_modules/python-scripts/cgi_search_word_count.py) script. To see differences when this runs as a cgi script try [http://localhost:8080/cgi-bin/cgi_search_word_count.py]()

This cgi compatible script includes will count the occurence of a word in a file. Running this as a cgi script will expose the server system, while running it in mod_wasm will automatically allow only sandboxed access.

The file name and word are passed as custom http parameters. For example:

```console
curl -H "File: Sherlock.txt" -H "Word: elementary" http://localhost:8080/wasm-module-endpoint
curl -H "File: Sherlock.txt" -H "Word: elementary" http://localhost:8080/cgi-bin/cgi_search_word_count.py
```

### hello_python_html.conf

Runs the [hello_python_html.py](./examples/wasm_modules/python-scripts/hello_python_html.py) script. To see differences when this runs as a cgi script try [http://localhost:8080/cgi-bin/hello_python_html.py]()

This is just a python script that lists the contents of the `/home` folder. It is not cgi compatible.
You can see in the config file that `/home` is mapped to `wasm_modules` on the server.

### rust_hello_wasm.conf

Runs the `hello_wasm` binary built from [examples/rust-src/hello_wasm](./examples/rust-src/hello_wasm/src/main.rs).

This is just a simple hello world in rust, which will run in a sandboxed mod_wasm environment.

### rust_list_dir.conf

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
$ make container-image
<snip>
Successfully tagged httpd-mod-wasm:latest
```

### Building a dev image

The dev image will include all examples, along with additional tools required for future development. If you want to benchmark and compare running a python script via cgi vs via mod_wasm you will need to build this image.

```console
$ make dev-image
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

The python code itself is not compiled to a wasm module, but is interpreted on the spot.

### Rust examples

Assuming you are familiar with Rust and its development tools, to be able to
build the Rust examples you will need to add the `wasm32-wasi` target via:

```console
$ rustup target add wasm32-wasi
```

Then just go inside any source folder (e.g. `examples/rust-src/hello_wasm` or
`examples/rust-src/list_dir`) and run:

```console
$ cargo build --release --target=wasm32-wasi
```

After building don't forget to copy the module from the respective `target/wasm32-wasi/release/`
to the mounted `wasm_modules/rust-wasm` folder.
