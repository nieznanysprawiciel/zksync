#!/bin/bash

PROVER_NAME=$1
exec plonk_step_by_step_prover "${PROVER_NAME}" 2>&1
