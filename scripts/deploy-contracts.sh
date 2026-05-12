#!/usr/bin/env bash
set -euo pipefail

NETWORK="${1:-testnet}"
CONTRACTS=(detection oracle reputation registry)
ROOT="$(cd "$(dirname "$0")/.." && pwd)/contracts"

echo "Deploying to $NETWORK..."

for contract in "${CONTRACTS[@]}"; do
  WASM="$ROOT/$contract/target/wasm32-unknown-unknown/release/$contract.wasm"
  if [[ ! -f "$WASM" ]]; then
    echo "✗ $contract WASM not found — run pnpm build:contracts first"
    exit 1
  fi
  echo "▶ Deploying $contract..."
  stellar contract deploy \
    --wasm "$WASM" \
    --network "$NETWORK"
  echo "✓ $contract deployed"
done
