.PHONY: container-image, container-multi-arch-image, push-container-multi-arch-image, dev-image, build
container-image:
	docker build -f image/Dockerfile -t httpd-mod-wasm:latest .

container-multi-arch-image:
	docker buildx build --progress=plain --platform linux/arm64/v8,linux/amd64 -f image/Dockerfile -t ghcr.io/vmware-labs/httpd-mod-wasm:latest .

push-container-multi-arch-image:
	docker buildx build --progress=plain --platform linux/arm64/v8,linux/amd64 -f image/Dockerfile -t ghcr.io/vmware-labs/httpd-mod-wasm:latest --push .

dev-image:
	docker build -f image/Dockerfile.dev -t httpd-mod-wasm-dev:latest .

build:
	cd ./wasm_runtime && make clean_all && make all
	cd ./mod_wasm && ./build.sh
