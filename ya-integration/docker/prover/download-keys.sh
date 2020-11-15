#!/bin/bash
# Copy of prover-entry.sh without code for running prover.

export ZKSYNC_HOME="/"

echo SUPPORTED_BLOCK_CHUNKS_SIZES=$SUPPORTED_BLOCK_CHUNKS_SIZES
echo SUPPORTED_BLOCK_CHUNKS_SIZES_SETUP_POWERS=$SUPPORTED_BLOCK_CHUNKS_SIZES_SETUP_POWERS
echo BLOCK_CHUNK_SIZES=$BLOCK_CHUNK_SIZES

# we download only keys used in node (defined by $BLOCK_CHUNK_SIZES)
source /bin/utils.sh
REQUIRED_SETUP_POWS=`get_required_plonk_setup_powers`

if [ "$PROVER_DOWNLOAD_SETUP" == "false" ]; then
  echo Downloading setup powers $REQUIRED_SETUP_POWS
  /bin/plonk-setup download monomial $REQUIRED_SETUP_POWS
fi

/bin/verify-keys unpack

echo key download complete
