## Examples

This repo cointains several pre-built webassembly modules along with their
respective configurations.

1. Run the container:
```console
docker run -p 8080:8080 ghcr.io/vmware-labs/httpd-mod-wasm:latest
```

2. Try the different demos provided:

| Demo                   | Wasm Module  | URL                               | Description |
| ---------------------- | ------------ | --------------------------------- | ----------- |
| WordPress              | [PHP 7.3.33](https://github.com/vmware-labs/webassembly-language-runtimes/releases/tag/php%2F7.3.33%2B20221124-2159d1c) | [http://localhost:8080/wordpress](http://localhost:8080/wordpress) | This is WordPress running on a Wasm build of the PHP interpreter. |
| HTTP Request Viewer | [Python 3.11](https://github.com/tiran/cpython-wasm-test/releases/tag/v3.11.0) | [http://localhost:8080/http-request-viewer](http://localhost:8080/http-request-viewer) | A Python App showing WASI capabilities. Try providing a complex query with URL query parameters, headers and a body. |

3. Other examples:

| Demo                   | Wasm Module  | URL                               | Description |
| ---------------------- | ------------ | --------------------------------- | ----------- |
| Hello Wasm | [`hello_wasm.wasm`](https://github.com/vmware-labs/mod_wasm/blob/v0.10.0/examples/rust-src/hello_wasm/src/main.rs) | [http://localhost:8080/hello-wasm](http://localhost:8080/hello-wasm) | A simple *Hello Wasm* example developed in Rust that prints out to the `stdout` and `stderr` (piped into Apache's log at `/usr/local/apache2/logs/error_log`). |
| PHP Hello              | [PHP 7.4.32](https://github.com/vmware-labs/webassembly-language-runtimes/releases/tag/php%2F7.4.32%2B20221124-2159d1c) | [http://localhost:8080/php-hello/](http://localhost:8080/php-hello/) | A simple PHP App showing current time and printing `phpinfo()`. |
| PrettyFy App    | [Python 3.11](https://github.com/tiran/cpython-wasm-test/releases/tag/v3.11.0) | [http://localhost:8080/prettyfy](http://localhost:8080/prettyfy) | This demo runs on a Python Wasm build and demonstrates the WebAssembly capabilities model. Detailed info [below](#prettyfy-webapp-demo). |

## Running in a container

A container image based on `docker.io/library/httpd` is provided for testing convenience. You can find at [https://github.com/vmware-labs/mod_wasm/pkgs/container/httpd-mod-wasm](https://github.com/vmware-labs/mod_wasm/pkgs/container/httpd-mod-wasm).

By default, the Apache server will be listening on port 8080.

- `/usr/local/apache2/conf/httpd.conf`: The Apache
  configuration. Override this file to try new Wasm examples or tweak some Apache settings.

- `/usr/local/apache2/wasm_modules`: This is the path where Wasm
  modules and different examples are provided.


## 'PrettyFy' WebApp Demo

The 'PrettyFy' demo is a simple one-script Python-based WebApp:
* The Python interpreter has been compiled to WebAssembly.
* Note how the system platform is identified: `sys.platform = WASI`.
* The app accepts `file=` as a URL parameter to highlight a previously uploaded file:
  * [http://localhost:8080/prettyfy?file=uploaded_text.txt](http://localhost:8080/prettyfy?file=uploaded_text.txt)
  * [http://localhost:8080/prettyfy?file=cgi_hello_python.py](http://localhost:8080/prettyfy?file=cgi_hello_python.py)
* Now, if you try a basic [path traversal](https://owasp.org/www-community/attacks/Path_Traversal) attack, it won't be succesful thanks to the WebAssembly sandboxed model where the Python interpreter is running. In the example below, the `httpd.conf` file is there, and Apache server can obviously read it. But the Python Wasm module doesn't have engouh permission to access it.
  * [http://localhost:8080/prettyfy?file=../../conf/httpd.conf](http://localhost:8080/prettyfy?file=../../conf/httpd.conf)

