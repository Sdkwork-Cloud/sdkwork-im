#!/usr/bin/env sh
set -eu

REQUESTED_VERSION="${1-}"
BASE_URL="${BASE_URL:-http://127.0.0.1:18081}"
SCRIPT_DIR="$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)"
WORKSPACE_ROOT="$(CDPATH= cd -- "$SCRIPT_DIR/.." && pwd)"
TMP_DIR="$WORKSPACE_ROOT/.tmp"
TMP_EXPORT_PATH="$TMP_DIR/control-plane.openapi.json"
AUTHORITY_PATH="$WORKSPACE_ROOT/openapi/craw-chat-control-plane.openapi.json"
DERIVED_PATH="$WORKSPACE_ROOT/openapi/craw-chat-control-plane.sdkgen.json"
GENERATED_OUTPUT_DIR="$WORKSPACE_ROOT/sdkwork-craw-chat-sdk-admin-typescript/generated/server-openapi"

resolve_generator_root() {
  if [ -n "${SDKWORK_GENERATOR_ROOT:-}" ] && [ -d "$SDKWORK_GENERATOR_ROOT" ]; then
    printf '%s\n' "$SDKWORK_GENERATOR_ROOT"
    return 0
  fi

  current="$WORKSPACE_ROOT"
  while [ "$current" != "$(dirname "$current")" ]; do
    candidate="$current/sdk/sdkwork-sdk-generator"
    if [ -d "$candidate" ]; then
      printf '%s\n' "$candidate"
      return 0
    fi
    current="$(dirname "$current")"
  done

  echo "Unable to locate sdkwork-sdk-generator. Set SDKWORK_GENERATOR_ROOT." >&2
  return 1
}

GENERATOR_ROOT="$(resolve_generator_root)"

mkdir -p "$TMP_DIR"
cargo run -q -p control-plane-api --bin export-openapi > "$TMP_EXPORT_PATH"
node "$SCRIPT_DIR/refresh-openapi-source.mjs" --source-file "$TMP_EXPORT_PATH"
node "$SCRIPT_DIR/prepare-openapi-source.mjs" --base "$AUTHORITY_PATH" --derived "$DERIVED_PATH"

AUTHORITY_VERSION="$(node -e "const fs=require('node:fs'); const doc=JSON.parse(fs.readFileSync(process.argv[1],'utf8')); process.stdout.write(String(doc.info?.version || '0.1.0'));" "$AUTHORITY_PATH")"
RESOLVED_VERSION="$AUTHORITY_VERSION"
if [ -n "$REQUESTED_VERSION" ]; then
  RESOLVED_VERSION="$REQUESTED_VERSION"
fi

node "$GENERATOR_ROOT/bin/sdkgen.js" generate \
  --input "$DERIVED_PATH" \
  --output "$GENERATED_OUTPUT_DIR" \
  --name "sdkwork-craw-chat-sdk-admin" \
  --type "backend" \
  --language "typescript" \
  --base-url "$BASE_URL" \
  --fixed-sdk-version "$RESOLVED_VERSION" \
  --sdk-root "$WORKSPACE_ROOT" \
  --sdk-name "sdkwork-craw-chat-sdk-admin" \
  --package-name "@sdkwork/craw-chat-admin-backend-sdk"

node "$SCRIPT_DIR/normalize-generated-transport-package.mjs"
node "$SCRIPT_DIR/materialize-admin-flutter-workspace.mjs"
node "$SCRIPT_DIR/assemble-sdk.mjs"
node "$SCRIPT_DIR/verify-sdk.mjs"
