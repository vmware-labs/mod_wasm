#!/bin/bash

docker build -f Dockerfile.archive_tar -t archive_tar:0.1 .
docker run -ti archive_tar:0.1 bash