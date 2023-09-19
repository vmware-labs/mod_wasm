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
FROM $IMAGE_REPOSITORY/library/rust:1.68 as builder-wasm_runtime.so
ARG WASM_RUNTIME_PATH=/usr/src/wasm_runtime
RUN apt-get update && apt-get install make
WORKDIR $WASM_RUNTIME_PATH
COPY ./wasm_runtime ./
RUN rustup update
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
ENV PHP_7_3_33_SHA1 a1daaeebc6cf27819ab9fde7ebf8e1df2e0df832
RUN wget -P /tmp https://github.com/vmware-labs/webassembly-language-runtimes/releases/download/php%2F7.3.33%2B20221124-2159d1c/php-cgi-7.3.33.speed-optimized.wasm \
    && echo $PHP_7_3_33_SHA1 php-cgi-7.3.33.speed-optimized.wasm | sha1sum -c -

ENV PHP_7_4_32_SHA1 a8bdc8aeb5aff73c2ad8fc5a494e718850814702
RUN wget -P /tmp https://github.com/vmware-labs/webassembly-language-runtimes/releases/download/php%2F7.4.32%2B20221124-2159d1c/php-cgi-7.4.32.speed-optimized.wasm \
    && echo $PHP_7_4_32_SHA1 php-cgi-7.4.32.speed-optimized.wasm | sha1sum -c -

ENV PHP_8_2_0_SHA2 68c96239a9b8095f21dd80af1714e5f270e4e08ef516821d90290da72d829a0e
RUN wget -P /tmp https://github.com/vmware-labs/webassembly-language-runtimes/releases/download/php%2F8.2.0%2B20230418-d75a618/php-cgi-8.2.0.wasm \
    && echo $PHP_8_2_0_SHA2 php-cgi-8.2.0.wasm | sha256sum -c -

ENV PHP_8_2_0_SLIM_SHA2 9994842f3df891e77813f3fa1db1fd3630de20191ce4df6a65d5d4aa2c54c5c0
RUN wget -P /tmp https://github.com/vmware-labs/webassembly-language-runtimes/releases/download/php%2F8.2.0%2B20230418-d75a618/php-cgi-8.2.0-slim.wasm \
    && echo $PHP_8_2_0_SLIM_SHA2 php-cgi-8.2.0-slim.wasm | sha256sum -c -

# Python
ENV PYTHON_3_11_0_SHA1 89e8a27c7da1d23dd8d1a9eb14b32983ac5db0d2
RUN wget -P /tmp https://github.com/tiran/cpython-wasm-test/releases/download/v3.11.0/Python-3.11.0-wasm32-wasi-16.zip \
    && echo $PYTHON_3_11_0_SHA1 Python-3.11.0-wasm32-wasi-16.zip | sha1sum -c -
RUN unzip /tmp/Python-3.11.0-wasm32-wasi-16.zip

# Python Pygments
ENV PYGMENTS_SHA1 b064cdbb380869e4f42ae07f22a499d4a3cec099
RUN wget -P /tmp https://github.com/pygments/pygments/archive/refs/tags/2.13.0.tar.gz \
    && echo $PYGMENTS_SHA1 2.13.0.tar.gz | sha1sum -c -
RUN tar -xzf /tmp/2.13.0.tar.gz

# WordPress
ENV WORDPRESS_SHA1 9348f0757c21504d085a6c866ccbb86573b39d6f
RUN wget -P /tmp https://wordpress.org/wordpress-6.0.2.tar.gz \
    && echo $WORDPRESS_SHA1 wordpress-6.0.2.tar.gz | sha1sum -c -
RUN tar -xzf /tmp/wordpress-6.0.2.tar.gz

# Drupal 9
ENV DRUPAL_9_SHA1 e3cbf50ff86913654c54c10ebaca8084cfd5a155
RUN wget -P /tmp https://ftp.drupal.org/files/projects/drupal-9.5.9.tar.gz \
    && echo $DRUPAL_9_SHA1 drupal-9.5.9.tar.gz | sha1sum -c -
RUN tar -xzf /tmp/drupal-9.5.9.tar.gz
RUN mv /tmp/drupal-9.5.9 /tmp/drupal-9

# Drupal 10
ENV DRUPAL_10_SHA1 0c7573c2a5d1651e14fb35355999cadda2104f26
RUN wget -P /tmp https://ftp.drupal.org/files/projects/drupal-10.0.9.tar.gz \
    && echo $DRUPAL_10_SHA1 drupal-10.0.9.tar.gz | sha1sum -c -
RUN tar -xzf /tmp/drupal-10.0.9.tar.gz
RUN mv /tmp/drupal-10.0.9 /tmp/drupal-10


################################################################################
# [Final Container Image] Distribute wasm_runtime, mod_wasm and demos
################################################################################
FROM $IMAGE_REPOSITORY/library/httpd:2.4 as builder-final
LABEL org.opencontainers.image.source https://github.com/vmware-labs/mod_wasm
RUN apt-get update && apt-get install php-cli php-gd php-xml composer -y
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
COPY --from=builder-demos /tmp/php-cgi-8.2.0.wasm /usr/local/apache2/wasm_modules/php-wasm/php-cgi-8.2.0.wasm
COPY --from=builder-demos /tmp/php-cgi-8.2.0-slim.wasm /usr/local/apache2/wasm_modules/php-wasm/php-cgi-8.2.0-slim.wasm
COPY ./examples/wasm_modules/php-scripts/php-hello /usr/local/apache2/htdocs/php-hello
COPY ./examples/wasm_modules/php-scripts/php-hello-slim /usr/local/apache2/htdocs/php-hello-slim
## WordPress
RUN mkdir -p /usr/local/apache2/htdocs/wordpress
COPY --from=builder-demos /tmp/wordpress /usr/local/apache2/htdocs/wordpress
COPY ./examples/wasm_modules/php-scripts/wordpress-patch/ /usr/local/apache2/htdocs/wordpress
RUN chmod -R go+w /usr/local/apache2/htdocs/wordpress/wp-content/database
## Drupal
RUN mkdir -p /usr/local/apache2/htdocs/drupal
COPY --from=builder-demos /tmp/drupal-10 /usr/local/apache2/htdocs/drupal
COPY ./examples/wasm_modules/php-scripts/drupal-setup/ /usr/local/apache2/htdocs/drupal
RUN mkdir -p /usr/local/apache2/htdocs/drupal/sites/default/files
RUN chmod -R go+w /usr/local/apache2/htdocs/drupal/sites/default/files
## Drupal 9 Zero (install)
RUN mkdir -p /usr/local/apache2/htdocs/drupal-9-zero
COPY --from=builder-demos /tmp/drupal-9 /usr/local/apache2/htdocs/drupal-9-zero
COPY ./examples/wasm_modules/php-scripts/drupal-9-zero-setup/ /usr/local/apache2/htdocs/drupal-9-zero
RUN mkdir -p /usr/local/apache2/htdocs/drupal-9-zero/sites/default/files
RUN chmod -R go+w /usr/local/apache2/htdocs/drupal-9-zero/sites/default/files
## Drupal 10 Zero (install)
RUN mkdir -p /usr/local/apache2/htdocs/drupal-10-zero
COPY --from=builder-demos /tmp/drupal-10 /usr/local/apache2/htdocs/drupal-10-zero
COPY ./examples/wasm_modules/php-scripts/drupal-10-zero-setup/ /usr/local/apache2/htdocs/drupal-10-zero
RUN mkdir -p /usr/local/apache2/htdocs/drupal-10-zero/sites/default/files
RUN chmod -R go+w /usr/local/apache2/htdocs/drupal-10-zero/sites/default/files
