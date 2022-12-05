################################################################################
# Container image build for ease of testing mod_wasm.
#
# This container image provides Apache2, mod_wasm and a default
# configuration that makes easy to execute custom WebAssembly modules.
################################################################################
ARG IMAGE_REPOSITORY=docker.io


################################################################################
# [`wasm_runtime.so` Builder]
################################################################################
FROM $IMAGE_REPOSITORY/library/rust:1.65.0-slim as builder-wasm_runtime.so
ARG WASM_RUNTIME_PATH=/usr/src/wasm_runtime
RUN apt-get update && apt-get install make
WORKDIR $WASM_RUNTIME_PATH
COPY ./wasm_runtime ./
RUN cargo install cbindgen
RUN make clean
RUN make all


################################################################################
# [`mod_wasm.so` Builder]
################################################################################
FROM $IMAGE_REPOSITORY/library/httpd:2.4 as builder-mod_wasm.so
ARG WASM_RUNTIME_PATH=/usr/src/wasm_runtime
ARG MOD_WASM_PATH=/usr/src/mod_wasm
ARG DIST_DIR=$MOD_WASM_PATH/dist
RUN apt-get update && apt-get install apache2-dev build-essential pkg-config libtool libapr1-dev libaprutil1-dev make gcc libtool-bin libxml2-dev libpcre2-dev subversion pkg-config -y
WORKDIR $MOD_WASM_PATH
COPY ./mod_wasm $MOD_WASM_PATH
COPY ./dist $DIST_DIR
COPY --from=builder-wasm_runtime.so $WASM_RUNTIME_PATH/target/release/libwasm_runtime.so $WASM_RUNTIME_PATH/target/release/libwasm_runtime.so
COPY --from=builder-wasm_runtime.so $WASM_RUNTIME_PATH/include/ $WASM_RUNTIME_PATH/include/
RUN mkdir -p $MOD_WASM_PATH/dist/conf $DIST_DIR/modules
RUN HTTPD_DIR=/usr/local/apache2 ./build.sh


################################################################################
# [Runtimes and Demos Builder]
################################################################################
FROM $IMAGE_REPOSITORY/library/debian:bullseye-slim as builder-demos
RUN apt-get update && apt-get install wget unzip -y
WORKDIR /tmp
# PHP
RUN wget -P /tmp https://github.com/vmware-labs/webassembly-language-runtimes/releases/download/php%2F7.3.33%2B20221124-2159d1c/php-cgi-7.3.33.speed-optimized.wasm
RUN wget -P /tmp https://github.com/vmware-labs/webassembly-language-runtimes/releases/download/php%2F7.4.32%2B20221124-2159d1c/php-cgi-7.4.32.speed-optimized.wasm
# Python
RUN wget -P /tmp https://github.com/tiran/cpython-wasm-test/releases/download/v3.11.0/Python-3.11.0-wasm32-wasi-16.zip
RUN unzip /tmp/Python-3.11.0-wasm32-wasi-16.zip
RUN wget -P /tmp https://github.com/pygments/pygments/archive/refs/tags/2.13.0.tar.gz
RUN tar -xzf /tmp/2.13.0.tar.gz
# WordPress
ENV WORDPRESS_SHA1 9348f0757c21504d085a6c866ccbb86573b39d6f
RUN wget -P /tmp https://wordpress.org/wordpress-6.0.2.tar.gz \
    && echo $WORDPRESS_SHA1 wordpress-6.0.2.tar.gz | sha1sum -c -
RUN tar -xzf /tmp/wordpress-6.0.2.tar.gz 


################################################################################
# [Final Container Image] Distribute wasm_runtime, mod_wasm and demos
################################################################################
FROM $IMAGE_REPOSITORY/library/httpd:2.4 as builder-final
LABEL org.opencontainers.image.source https://github.com/vmware-labs/mod_wasm
ARG WASM_RUNTIME_PATH=/usr/src/wasm_runtime
ARG MOD_WASM_PATH=/usr/src/mod_wasm
ARG DIST_DIR=$MOD_WASM_PATH/dist
COPY --from=builder-wasm_runtime.so $WASM_RUNTIME_PATH/target/release/libwasm_runtime.so /usr/lib
COPY --from=builder-mod_wasm.so $DIST_DIR/modules /usr/local/apache2/modules
COPY --from=builder-mod_wasm.so $DIST_DIR/conf/httpd.conf /usr/local/apache2/conf/
COPY ./examples/wasm_modules/ /usr/local/apache2/wasm_modules/

# Python Wasm
COPY --from=builder-demos /tmp/Python-3.11.0-wasm32-wasi-16 /usr/local/apache2/wasm_modules/Python-3.11.0-wasm32-wasi-16
COPY --from=builder-demos /tmp/pygments-2.13.0/pygments /usr/local/apache2/wasm_modules/Python-3.11.0-wasm32-wasi-16/lib/python3.11/pygments

# PHP Examples
## PHP-Hello
COPY --from=builder-demos /tmp/php-cgi-7.3.33.speed-optimized.wasm /usr/local/apache2/wasm_modules/php-wasm/php-cgi-7.3.33.speed-optimized.wasm
COPY --from=builder-demos /tmp/php-cgi-7.4.32.speed-optimized.wasm /usr/local/apache2/wasm_modules/php-wasm/php-cgi-7.4.32.speed-optimized.wasm
COPY ./examples/wasm_modules/php-scripts/php-hello /usr/local/apache2/htdocs/php-hello
## WordPress
RUN mkdir -p /usr/local/apache2/htdocs/wordpress
COPY --from=builder-demos /tmp/wordpress /usr/local/apache2/htdocs/wordpress
COPY ./examples/wasm_modules/php-scripts/wordpress-patch/ /usr/local/apache2/htdocs/wordpress
RUN chmod -R 777 /usr/local/apache2/htdocs/wordpress/wp-content/database
