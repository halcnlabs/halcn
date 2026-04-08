#!/usr/bin/env bash
set -euo pipefail

echo "Starting local Solana validator..."

solana-test-validator \
  --reset \
  --quiet \
  --bpf-program Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS programs/halcn-core/target/deploy/halcn_core.so &

VALIDATOR_PID=$!
echo "Validator PID: $VALIDATOR_PID"

sleep 3
solana airdrop 100 --url localhost

echo "Local validator running. Press Ctrl+C to stop."
wait $VALIDATOR_PID
