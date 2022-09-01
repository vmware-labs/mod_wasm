.PHONY: container-image
container-image:
	docker build -f image/Dockerfile -t httpd-mod-wasm:latest .
