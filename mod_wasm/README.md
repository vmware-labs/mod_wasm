# 🏗️ Building `mod_wasm.so`

There are two different ways to build the **mod_wasm** extension module:
* Ad-hoc build using the provided `build.sh` script.
* Integrating mod_wasm into the Apache Server build script.

## Requirements

- [GNU C compiler](https://gcc.gnu.org/) (`gcc`)
- [GNU Make](https://www.gnu.org/software/make/manual/html_node/index.html) (`make`)  
- [pkg-config](https://gitlab.freedesktop.org/pkg-config/pkg-config) (`pkg-config`)
- [Libtool](https://www.gnu.org/software/libtool/manual/html_node/index.html) (`libtool-bin`)
- [libxml2](https://gitlab.gnome.org/GNOME/libxml2) (`libxml2-dev`)
- [Perl Compatible Regular Expressions](https://pcre.org/) (`libpcre2-dev`)
- [Apache Portable Runtime Project](https://apr.apache.org/) (`libapr1-dev`, `libaprutil1-dev`)
- [Apache HTTP Server Development Headers](https://httpd.apache.org/) (`apache2-dev`)
- [Subversion](https://subversion.apache.org/) (`svn`)
- [wasm_runtime](https://github.com/vmware-labs/mod_wasm/tree/main/wasm_runtime) (`libwasm_runtime`)

In a Ubuntu-like environment, you can install all dependencies by running:
```console
sudo apt install gcc make pkg-config libtool-bin libxml2-dev libpcre2-dev libapr1-dev libaprutil1-dev apache2-dev subversion
```

⚠️ Since `mod_wasm.so` depends on `libwasm_runtime.so`, make sure you build such library first by following the instructions from [wasm_runtime]([wasm_runtime](https://github.com/vmware-labs/mod_wasm/tree/main/wasm_runtime)).

## Building mod_wasm ad-hoc via `build.sh`

Just execute the script and find the final build at  `modules/wasm/.lib/mod_wasm.so`. You will need `libtool` and `pkg-config` tools installed in the system.
   ```console
   ./build.sh
   ```

## Building mod_wasm via Apache Server build script

Apache Server uses Autoconf/Automake tools to build binaries and libraries.

To build mod_wasm with such tools and integrated into the Apache Server build:
1) Clone the Apache Server repo:
   ```console
   git clone https://github.com/apache/httpd.git httpd
   ```
2) Copy mod_wasm files into the Apache Server file structure:
   ```console
   cp -Rv modules docs httpd/
   ```
3) Get the Apache Portable Runtime into the Apache Server file structure:
   ```console
   cd httpd
   svn co http://svn.apache.org/repos/asf/apr/apr/trunk srclib/apr
   ```
4) Build configure scripts:
   ```console
   ./buildconf
   ```
5) Configure Apache to include mod_wasm as an optional module:
   ```console
   ./configure --prefix=${PWD}/dist --enable-wasm --with-wasmruntime=../../wasm_runtime/
   ```
6) Build Apache Server and mod_wasm and install into `dist/`
   ```console
   make -j install
   ```

At this point, Apache Server and mod_wasm are built (see `dist/bin` and `dist/modules`). 
