#!/usr/bin/env sh
set -eu

SCRIPT_DIR="$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)"
WORKSPACE_DIR="$(CDPATH= cd -- "$SCRIPT_DIR/../.." && pwd)"

exec node "$WORKSPACE_DIR/bin/assemble-sdk.mjs"
