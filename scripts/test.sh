#!/usr/bin/env bash
set -euo pipefail

echo "Running Rust tests..."
cargo test --workspace

echo ""
echo "Running SDK tests..."
cd sdk
npm test

echo ""
echo "All tests passed."
