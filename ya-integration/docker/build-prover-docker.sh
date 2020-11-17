#!/bin/bash
# Run from zksync/ directory

source .env

export COMPOSE_DOCKER_CLI_BUILD=1
export DOCKER_BUILDKIT=1

docker build -t zksync-prover:0.1 -f docker/prover/Dockerfile .

export COMPOSE_DOCKER_CLI_BUILD=0
export DOCKER_BUILDKIT=0

cd ya-integration
docker build -t ya-zksync-prover:0.1 -f docker/prover/Dockerfile .
