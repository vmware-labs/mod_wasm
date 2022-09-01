# `mod_wasm`

Welcome to the `mod_wasm` project! This project implements an Apache
module that is able to execute WebAssembly modules.

## Development image

A container image is provided for testing convenience.

### Prerequisites

You will need `docker`, or `podman` with the `docker` alias
enabled.

### Build

You can build this image like so:

```console
$ make container-image
<snip>
Successfully tagged httpd-mod-wasm:latest
```

### Running it

This image is based off `docker.io/library/httpd`. The image has some
points of interest:

- `/usr/local/apache2/conf/httpd.conf`: the Apache
  configuration. Override this file if you want to tweak specific
  Apache settings.

- `/usr/local/apache2/wasm_modules`: this is the path where Wasm
  modules can be placed (either provided by subsequent layers or
  mounted).

By default, the Apache server will be listening on port 8080.

#### Run example

```console
$ docker run -d -p8080:8080 -v /path/to/some/local/wasm-module.wasm:/usr/local/apache2/wasm_modules/wasm-module.wasm httpd-mod-wasm:latest
```

Once that Apache is running, you can reach the WebAssembly module
built for `wasm32-wasi`, like so:

```console
$ curl -vvv http://localhost:8080/wasm-module-endpoint
> GET /wasm-module-endpoint HTTP/1.1
> Host: localhost:8080
> User-Agent: curl/7.83.1
> Accept: */*
>
* Mark bundle as not supporting multiuse
< HTTP/1.1 200 OK
< Date: Mon, 12 Sep 2022 09:08:53 GMT
< Server: Apache/2.4.54 (Unix)
< Content-Length: 14
< Content-Type: text/html
<
Hello, world!
```
