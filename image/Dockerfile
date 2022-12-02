# Container image build for ease of testing mod_wasm.
#
# This container image provides Apache2, mod_wasm and a default
# configuration that makes easy to execute custom WebAssembly modules.

ARG IMAGE_REPOSITORY=docker.io

# Build wasm_runtime in release mode
FROM $IMAGE_REPOSITORY/library/rust:1.65.0-slim as builder-rs
ARG WASM_RUNTIME_PATH=/usr/src/wasm_runtime
WORKDIR $WASM_RUNTIME_PATH
COPY ./wasm_runtime ./
RUN cargo build --release

# Build mod_wasm Apache module
FROM $IMAGE_REPOSITORY/library/httpd:2.4 as builder-apache
ARG WASM_RUNTIME_PATH=/usr/src/wasm_runtime
ARG MOD_WASM_PATH=/usr/src/mod_wasm
ARG DIST_DIR=$MOD_WASM_PATH/dist
RUN apt-get update && apt-get install apache2-dev build-essential pkg-config libtool libapr1-dev libaprutil1-dev make gcc libtool-bin libxml2-dev libpcre2-dev subversion pkg-config -y
WORKDIR $MOD_WASM_PATH
COPY ./mod_wasm $MOD_WASM_PATH
COPY ./dist $DIST_DIR
COPY --from=builder-rs $WASM_RUNTIME_PATH/target/release/libwasm_runtime.so $WASM_RUNTIME_PATH/target/release/libwasm_runtime.so
COPY --from=builder-rs $WASM_RUNTIME_PATH/include/ $WASM_RUNTIME_PATH/include/
RUN mkdir -p $MOD_WASM_PATH/dist/conf $DIST_DIR/modules
RUN HTTPD_DIR=/usr/local/apache2 ./build.sh

# Distribute wasm_runtime, mod_wasm and a default configuration
FROM $IMAGE_REPOSITORY/library/httpd:2.4
LABEL org.opencontainers.image.source https://github.com/vmware-labs/mod_wasm
ARG WASM_RUNTIME_PATH=/usr/src/wasm_runtime
ARG MOD_WASM_PATH=/usr/src/mod_wasm
ARG DIST_DIR=$MOD_WASM_PATH/dist
RUN rm -rf /usr/local/apache2/htdocs
COPY ./examples/wasm_modules/ /usr/local/apache2/wasm_modules/
RUN mkdir -p /usr/local/apache2/htdocs/wordpress
COPY ./examples/wasm_modules/php-scripts/wp-demo/ /usr/local/apache2/htdocs/wordpress
COPY ./examples/wasm_modules/php-scripts/php-hello /usr/local/apache2/htdocs/php-hello
COPY --from=builder-rs $WASM_RUNTIME_PATH/target/release/libwasm_runtime.so /usr/lib
COPY --from=builder-apache $DIST_DIR/modules /usr/local/apache2/modules
COPY --from=builder-apache $DIST_DIR/conf/httpd.conf /usr/local/apache2/conf/
RUN chmod -R 777 /usr/local/apache2/htdocs/wordpress/wp-content/database
