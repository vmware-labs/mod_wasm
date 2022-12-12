#!/bin/sh

#################################################################################
# mod_wasm should be compiled in the Apache environment using Autoconf/Automake.
#
# This build.sh script is just a shortcut for development purposes.
#################################################################################

env
set -x

# check dependencies
if ! which libtool; then
    echo "libtool not found; please, install it"
    exit 1
fi

if ! which pkg-config; then
    echo "pkg-config not found; please, install it"
    exit 1
fi


echo "[Building mod_wasm]"

SCRIPT_DIR=$( cd -- "$(dirname -- "$0")" &> /dev/null && pwd )
MOD_WASM_DIR=${MOD_WASM_DIR:-$(realpath "${SCRIPT_DIR}/modules/wasm")}
WASM_RUNTIME_PATH=${WASM_RUNTIME_PATH:-$(realpath "${SCRIPT_DIR}/../wasm_runtime")}
DIST_DIR=${DIST_DIR:-$(realpath "${SCRIPT_DIR}/../dist")}
if [ -z ${HTTPD_DIR+x} ]
then
    HTTPD_DIR=$(realpath ../httpd)
fi
ARCH=$(uname -m)

echo "[Deleting binaries]"

rm -fv ${MOD_WASM_DIR}/mod_wasm.o
rm -fv ${MOD_WASM_DIR}/mod_wasm.lo
rm -fv ${MOD_WASM_DIR}/mod_wasm.slo
rm -fv ${MOD_WASM_DIR}/mod_wasm.la
rm -fv ${MOD_WASM_DIR}/.libs/mod_wasm.o
rm -fv ${MOD_WASM_DIR}/.libs/mod_wasm.la
rm -fv ${MOD_WASM_DIR}/.libs/mod_wasm.lai
rm -fv ${MOD_WASM_DIR}/.libs/mod_wasm.a
rm -fv ${MOD_WASM_DIR}/.libs/mod_wasm.so

echo "[Building mod_wasm]"

echo "[mod_wasm: compiling]"
cd ${MOD_WASM_DIR}

/usr/share/apr-1.0/build/libtool --verbose --mode=compile ${ARCH}-linux-gnu-gcc \
     -I${HTTPD_DIR}/include \
     $(pkg-config --cflags apr-1 apr-util-1) \
     -I${WASM_RUNTIME_PATH}/include \
     -shared \
     -c mod_wasm.c && touch mod_wasm.slo

echo "[mod_wasm: linking]"
/usr/share/apr-1.0/build/libtool --verbose --mode=link ${ARCH}-linux-gnu-gcc \
     -L${WASM_RUNTIME_PATH}/target/release -lwasm_runtime \
     -o mod_wasm.la \
     -rpath ${HTTP_SERVER_PATH}/dist/modules \
     -module -avoid-version mod_wasm.lo

echo "[Installing module]"
mkdir -p "${DIST_DIR}/modules/" 
cp -v .libs/mod_wasm.so "${DIST_DIR}/modules/"

echo "[Installing httpd.conf]"
cd ${SCRIPT_DIR}
mkdir -p "${DIST_DIR}/conf/"
cp -v httpd.conf "${DIST_DIR}/conf/"
