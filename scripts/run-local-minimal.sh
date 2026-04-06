#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

echo "Building local-minimal-node..."
cargo build -p local-minimal-node --offline

echo "Starting local-minimal-node on http://127.0.0.1:18090"
cargo run -p local-minimal-node --offline

