#!/usr/bin/env bash

docker rm dwaniumDev
docker image rm dwanium-dev
docker build -f devDockerFile -t dwanium-dev:latest .
docker run --env-file .env --network=dwanium_default -it --name dwaniumDev dwanium-dev
