#!/bin/sh

env
set -x

if ! which pkg-config; then
    echo "pkg-config not found; please, install it"
    exit 1
fi

echo "[Building mod_wasm]"

SCRIPT_DIR=$( cd -- "$(dirname -- "${BASH_SOURCE[0]}")" &> /dev/null && pwd )
WASM_RUNTIME_PATH=${WASM_RUNTIME_PATH:-$(realpath "${SCRIPT_DIR}/../wasm_runtime")}
DIST_DIR=${DIST_DIR:-$(realpath "${SCRIPT_DIR}/../dist")}

echo "[Deleting binaries]"

rm -fv modules/wasm/mod_wasm.o
rm -fv modules/wasm/mod_wasm.lo
rm -fv modules/wasm/mod_wasm.slo
rm -fv modules/wasm/mod_wasm.la
rm -fv modules/wasm/.libs/mod_wasm.o
rm -fv modules/wasm/.libs/mod_wasm.la
rm -fv modules/wasm/.libs/mod_wasm.lai
rm -fv modules/wasm/.libs/mod_wasm.a
rm -fv modules/wasm/.libs/mod_wasm.so

echo "[Building mod_wasm]"

echo "[mod_wasm: compiling]"
/usr/share/apr-1.0/build/libtool --verbose --mode=compile x86_64-linux-gnu-gcc -DLINUX -D_REENTRANT -D_GNU_SOURCE \
     -I/usr/include/apache2 \
     $(pkg-config --cflags apr-1 apr-util-1) \
     -I/usr/include \
     -I/usr/include/mod_wasm \
     -I${WASM_RUNTIME_PATH}/src \
     -shared \
     -c mod_wasm.c && touch mod_wasm.slo

echo "[mod_wasm: linking]"
/usr/share/apr-1.0/build/libtool --verbose --mode=link x86_64-linux-gnu-gcc \
     -L${WASM_RUNTIME_PATH}/target/release -lwasm_runtime \
     -o mod_wasm.la \
     -rpath ${HTTP_SERVER_PATH}/dist/modules \
     -module -avoid-version mod_wasm.lo

echo "[Installing module]"
mkdir -p "${DIST_DIR}/modules/" "${DIST_DIR}/conf/"
cp -v .libs/mod_wasm.so "${DIST_DIR}/modules/"
cp -v httpd.conf "${DIST_DIR}/conf/"
