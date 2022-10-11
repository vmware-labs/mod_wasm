# `mod_wasm`

Welcome to the `mod_wasm` project! This project implements an Apache
module that is able to execute WebAssembly modules. A detailed article can be found at VMware's [Wasm Labs](https://wasmlabs.dev/articles/apache-mod-wasm/) page.

To try out the default wasm module just type:

```console
docker run -p 8080:8080 projects.registry.vmware.com/wasmlabs/containers/httpd-mod-wasm:latest
```

Then open a browser at [http://localhost:8080/wasm-module-endpoint](http://localhost:8080/wasm-module-endpoint) and enjoy.

## Table of contents

* [Running in a container](#running-in-a-container)
  * [Default example](#default-example)
  * [Running development image](#running-the-dev-image)
* [Demonstrating security capabilities](#demonstrating-security-capabilities)
* [More Examples](#more-examples)
* [Building mod_wasm in your environment](#building-mod_wasm-in-your-environment)

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

## More examples

This repo cointains several pre-built WebAssembly modules along with their
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
