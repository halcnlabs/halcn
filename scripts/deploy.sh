#!/usr/bin/env bash
set -euo pipefail

CLUSTER="${1:-devnet}"
PROGRAM_DIR="programs/halcn-core"

echo "Building halcn-core..."
cargo build-sbf --manifest-path "$PROGRAM_DIR/Cargo.toml"

echo "Deploying to $CLUSTER..."
solana program deploy \
  --program-id "$PROGRAM_DIR/target/deploy/halcn_core-keypair.json" \
  "$PROGRAM_DIR/target/deploy/halcn_core.so" \
  --url "$CLUSTER"

echo "Deployment complete."
