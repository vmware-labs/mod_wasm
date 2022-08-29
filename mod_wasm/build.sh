#!/bin/sh

echo "[Building mod_wasm]"

HTTP_SERVER_PATH=/home/ubuntu/Home/Workspace/VMware/mod_wasm/httpd
WASM_RUNTIME_PATH=/home/ubuntu/Home/Workspace/VMware/mod_wasm/wasm_runtime


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
     -I${HTTP_SERVER_PATH}/include -I${HTTP_SERVER_PATH}/os/unix \
     -I/usr/include/apr-1.0 -I/usr/include \
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
cp -v .libs/mod_wasm.so ../httpd/dist/modules/
cp -v httpd.conf ../httpd/dist/conf
