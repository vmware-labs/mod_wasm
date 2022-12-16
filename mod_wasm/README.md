# 🏗️ Building `mod_wasm.so`

This doc shows how to build mod_wasm for [Linux](#building-for-linux) and [Windows](#building-for-windows).

⚠️ Since `mod_wasm.so` depends on `libwasm_runtime.so` (or `wasm_runtime.dll` on Windows), make sure you build such library first by following the instructions from [wasm_runtime](https://github.com/vmware-labs/mod_wasm/tree/main/wasm_runtime).

## Building for Linux

There are two different ways to build the **mod_wasm** extension module for Linux:
* Ad-hoc build using the provided `build.sh` script.
* Integrating mod_wasm into the Apache Server build script.

### Requirements

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

### Building mod_wasm ad-hoc via `build.sh`

This script is a shortcut for building mod_wasm without using the Autoconf/Automake from the Apache server project.

You will need `libtool` and `pkg-config` tools installed in the system.
   ```console
   ./build.sh
   ```

Find the final build at `modules/wasm/.lib/mod_wasm.so`. 

### Building mod_wasm via Apache Server Autoconf/Automake

Apache Server uses Autoconf/Automake tools to build binaries and libraries.

To build mod_wasm with such tools and integrated into the Apache Server build:
1) Clone the Apache Server repo:
   ```console
   git clone https://github.com/apache/httpd.git httpd
   ```
2) Copy mod_wasm files from this directory into the Apache Server file structure:
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

## Building for Windows

The only requirments for building mod_wasm on Windows are:
* [Microsoft Visual Studio](https://visualstudio.microsoft.com/).
  * Make sure you install MSVC C++ x64/x86 build tools and Windows SDK components.
* [Apache for Windows](https://www.apachelounge.com/download/VS17/binaries/httpd-2.4.54-win64-VS17.zip) (from [Apache Lounge](https://www.apachelounge.com)).
  
Next, follow the compiling and linking steps:

1) Set `APACHE` and `WASM_RUNTIME` environment variables to the corresponding routes in your system:
```console
set APACHE=C:\Apache24
set WASM_RUNTIME=C:\mod_wasm\wasm_runtime
```

2) Compile `mod_wasm.c` and get the object file (`mod_wasm.obj`):
```console
"c:\Program Files\Microsoft Visual Studio\2022\Community\VC\Tools\MSVC\14.34.31933\bin\Hostx64\x64\cl.exe" /nologo /MD /O2 /LD /W3 -DWIN32 -D_WIN32 -I%APACHE%\include -I"C:\Program Files (x86)\Windows Kits\10\Include\10.0.20348.0\um" -I"C:\Program Files (x86)\Windows Kits\10\Include\10.0.20348.0\shared" -I"C:\Program Files\Microsoft Visual Studio\2022\Community\VC\Tools\MSVC\14.34.31933\include" -I"C:\Program Files (x86)\Windows Kits\10\Include\10.0.20348.0\ucrt" -I"%WASM_RUNTIME%\include" /c /Fomod_wasm.obj mod_wasm.c
```

3) Link to get `mod_wasm.so`:
```console
"c:\Program Files\Microsoft Visual Studio\2022\Community\VC\Tools\MSVC\14.34.31933\bin\Hostx64\x64\link.exe" "C:\Program Files (x86)\Windows Kits\10\Lib\10.0.20348.0\um\x64\kernel32.lib" "%APACHE%\lib\libhttpd.lib" "%APACHE%\lib\libapr-1.lib" "%APACHE%\lib\libaprutil-1.lib" "%WASM_RUNTIME%\target\release\wasm_runtime.dll.lib" /LIBPATH:"C:\Program Files\Microsoft Visual Studio\2022\Community\VC\Tools\MSVC\14.34.31933\lib\x64" /LIBPATH:"C:\Program Files (x86)\Windows Kits\10\Lib\10.0.20348.0\ucrt\x64" /nologo /subsystem:windows /dll /out:mod_wasm.so mod_wasm.obj
```
