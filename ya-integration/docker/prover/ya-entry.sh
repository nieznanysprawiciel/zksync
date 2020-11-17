#!/bin/bash

#PROVER_NAME=$1
PROVER_NAME="yagna-prover"
exec yagna_prover "${PROVER_NAME}" 2>&1
