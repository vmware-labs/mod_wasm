# 📦 Building the container image

This repository contains all you need to build a local container image

## Requirements

To build the container you will need `docker`, or `podman` with the `docker` alias enabled.

For convenience we have organized the build commands in a [Makefile](../Makefile), so you can use `make` from the root directory in the repo if you prefer.

## Build

You can build this image like so:

```console
make container-image
```

## Building a dev image

The dev image will include all examples, along with additional tools required for future development. If you want to benchmark and compare running a python script via cgi vs via mod_wasm you will need to build this image.

```console
make dev-image
```
