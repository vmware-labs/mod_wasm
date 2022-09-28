.PHONY: container-image
container-image:
	docker build -f image/Dockerfile -t httpd-mod-wasm:latest .

dev-image:
	docker build -f image/Dockerfile.dev -t httpd-mod-wasm-dev:latest .
