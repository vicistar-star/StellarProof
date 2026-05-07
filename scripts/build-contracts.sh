#!/usr/bin/env bash
set -euo pipefail

CONTRACTS=(detection oracle reputation registry)
ROOT="$(cd "$(dirname "$0")/.." && pwd)/contracts"

for contract in "${CONTRACTS[@]}"; do
  echo "▶ Building $contract..."
  (cd "$ROOT/$contract" && stellar contract build)
  echo "✓ $contract built"
done

echo "All contracts built successfully."
