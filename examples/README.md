## Examples

This repo cointains several pre-built webassembly modules along with their
respective configurations.

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
[Building the container image](#building-the-container-image) section below.

To try out the default wasm module just type

```console
docker run -p 8080:8080 projects.registry.vmware.com/wasmlabs/containers/httpd-mod-wasm:latest
```

Then open a browser at [http://localhost:8080/wasm-module-endpoint]() and enjoy.

By default you will see the `cgi_prettify.py` script described below running
with a wasm python binary.

### Running the dev image

If you plan on experimenting a comparison between executing python via cgi and mod_wasm you will need to run the development container image. Refer to [Building a dev image](#building-a-dev-image).

To just run the image and see the default example (with enabled cgi access to everything) use

```console
docker run --name mod_wasm_dev_examples -p 8080:8080 httpd-mod-wasm-dev:latest
```

If you want to easily modify the examples and restart an existing container (as described above) run with

```console
docker run --name mod_wasm_dev_examples -p 8080:8080 -v ./examples/wasm_modules/:/usr/local/apache2/wasm_modules/ httpd-mod-wasm-dev:latest
```

## Demonstrating security capabilities

Run the dev image as described above. By default the loaded wasm module is python running the `cgi_prettify.py` script.

Let's examine how it behaves compared to running the script via CGI.

### 1. By default the script will list the contents of an `uploads` folder.

 - Open [http://localhost:8080/wasm-module-endpoint]() to view the listing via mod_wasm. Note the `sys.platform = wasi` information.
 - Open [http://localhost:8080/cgi-bin/cgi_prettify.py]() to view the listing via cgi. Note that `sys.platform = linux`.

### 2. Now let's prettify a file

 - Open [http://localhost:8080/wasm-module-endpoint?file=cgi_hello_python.py]() to do this via mod_wasm.
 - Open [http://localhost:8080/cgi-bin/cgi_prettify.py?file=cgi_hello_python.py]() to do this via CGI.

### 3. Let's try to hack a bit

So let's think like a hacker that wants to try and get access to any file on the server system:

1. It looks like the path to the prettified file is relative to `uploads`.
2. We know for sure that the apache httpd will have access to `/usr/local/apache2/conf/httpd.conf`.
3. Then let's just try and access this file relative to `uploads`. Starting from `../usr/local/apache2/conf/httpd.conf` with CGI we can
   easily experiment by prepending enough times with `../` until we get to a working link like [http://localhost:8080/cgi-bin/cgi_prettify.py?file=../../../../../../usr/local/apache2/conf/httpd.conf]()
5. Voila - we have access to the server's configuration and we know where root is located relative to `uploads`.

However, that will never happen with mod_wasm. Just give it a try and see that we have no access outside the `uploads` folder - [http://localhost:8080/wasm-module-endpoint?file=../../../../../../usr/local/apache2/conf/httpd.conf]()


## Running the different examples

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
