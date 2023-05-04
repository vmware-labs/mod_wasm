.PHONY: install doc clean clean_all

all: target/release/libwasm_runtime.so

target/release/libwasm_runtime.so: include/wasm_runtime.h $(wildcard src/*.rs)
	cargo build --release

include/wasm_runtime.h: cbindgen.toml src/c_api.rs
	cbindgen --output include/wasm_runtime.h

install: all
	cp -v target/release/libwasm_runtime.so ../httpd/dist/modules

doc: 
	cargo doc --no-deps

clean:
	rm -rf target/release/libwasm_runtime.*

clean_all: clean
	cargo clean
	cargo clean --release
	rm -rf include/wasm_runtime.h
	rm -rf include/version.h
