#!/usr/bin/env sh
set -eu

REQUESTED_VERSION="${1-}"
BASE_URL="${BASE_URL:-http://127.0.0.1:18081}"
SCRIPT_DIR="$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)"
WORKSPACE_DIR="$(CDPATH= cd -- "$SCRIPT_DIR/../.." && pwd)"

if [ -n "$REQUESTED_VERSION" ]; then
  pwsh -NoLogo -NoProfile -File "$WORKSPACE_DIR/bin/generate-sdk.ps1" -RequestedVersion "$REQUESTED_VERSION" -BaseUrl "$BASE_URL"
else
  pwsh -NoLogo -NoProfile -File "$WORKSPACE_DIR/bin/generate-sdk.ps1" -BaseUrl "$BASE_URL"
fi
