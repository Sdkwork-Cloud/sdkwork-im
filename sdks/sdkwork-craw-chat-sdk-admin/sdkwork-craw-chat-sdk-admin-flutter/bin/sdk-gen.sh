#!/usr/bin/env sh
set -eu

REQUESTED_VERSION="${1-}"
BASE_URL="${BASE_URL:-http://127.0.0.1:18081}"
export BASE_URL
SCRIPT_DIR="$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)"
WORKSPACE_DIR="$(CDPATH= cd -- "$SCRIPT_DIR/../.." && pwd)"

if [ -n "$REQUESTED_VERSION" ]; then
  exec "$WORKSPACE_DIR/bin/generate-sdk.sh" "$REQUESTED_VERSION"
fi

exec "$WORKSPACE_DIR/bin/generate-sdk.sh"
