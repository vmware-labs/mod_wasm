#!/usr/bin/env bash

#################################################################################
# mod_wasm should be compiled in the Apache environment using Autoconf/Automake.
#
# This build.sh script is just a shortcut for development purposes.
#################################################################################

set -x

CC="${CC:-gcc}"
LIBTOOL="${LIBTOOL:-$(which libtool)}"
HTTPD_DIR="${HTTPD_DIR:-/usr/include/apache2}"

# check dependencies
if [[ ! -e "$LIBTOOL" ]]; then
    echo "libtool not found; please, install it"
    exit 1
fi

if [[ -d /usr/share/apr-1.0/build ]]; then
    export PATH=/usr/share/apr-1.0/build:$PATH
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

INCLUDE_PATHS=( "-I${HTTPD_DIR}" )
# In case HTTP_DIR is pointing to the HTTPD full repository
if [[ -d "${HTTPD_DIR}/include" ]]; then
    INCLUDE_PATHS+=( "-I${HTTPD_DIR}/include" )
fi
# In case HTTP_DIR is pointing to the HTTPD full repository
if [[ -d "${HTTPD_DIR}/dist/include" ]]; then
    INCLUDE_PATHS+=( "-I${HTTPD_DIR}/dist/include" )
fi
${LIBTOOL} --verbose --mode=compile ${CC} \
     $(pkg-config --cflags apr-1 apr-util-1) \
     ${INCLUDE_PATHS} \
     -I${WASM_RUNTIME_PATH}/include \
     -shared \
     -c mod_wasm.c && touch mod_wasm.slo

echo "[mod_wasm: linking]"
${LIBTOOL} --verbose --mode=link ${CC} \
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
