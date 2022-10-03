.PHONY: container-image
container-image:
	docker build -f image/Dockerfile -t httpd-mod-wasm:latest .

.PHONY: dev-image
dev-image:
	docker build -f image/Dockerfile.dev -t httpd-mod-wasm-dev:latest .

.PHONY: build
build:
	cd ./wasm_runtime && cargo build --release
	cd ./mod_wasm && ./build.sh
