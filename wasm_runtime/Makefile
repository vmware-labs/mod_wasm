.PHONY: install doc clean

all: target/release/libwasm_runtime.so

target/release/libwasm_runtime.so: src/wasm_runtime.h $(wildcard src/*.rs)
	cargo build --release

src/wasm_runtime.h: cbindgen.toml src/c_api.rs
	cbindgen --output src/wasm_runtime.h

install: all
	cp -v target/release/libwasm_runtime.so ../httpd/dist/modules

doc: 
	cargo doc --no-deps

clean:
	rm -rf wasm_runtime.h
	rm -rf target/release/libwasm_runtime.*

