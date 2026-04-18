#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
WORKSPACE_DIR="$(cd "${SCRIPT_DIR}/.." && pwd)"
AUTHORITY_SPEC="${WORKSPACE_DIR}/openapi/admin-control-plane.openapi.yaml"
DERIVED_SPEC="${WORKSPACE_DIR}/openapi/admin-control-plane.sdkgen.yaml"

LANGUAGES=()
while [[ $# -gt 0 ]]; do
  case "$1" in
    --language)
      LANGUAGES+=("${2,,}")
      shift 2
      ;;
    *)
      echo "[sdkwork-craw-chat-sdk-admin] Unknown argument: $1" >&2
      exit 1
      ;;
  esac
done

if [[ ${#LANGUAGES[@]} -eq 0 ]]; then
  LANGUAGES=("typescript" "flutter")
fi

node "${SCRIPT_DIR}/fetch-openapi-source.mjs" --authority "${AUTHORITY_SPEC}"
node "${SCRIPT_DIR}/prepare-openapi-source.mjs" --base "${AUTHORITY_SPEC}" --derived "${DERIVED_SPEC}"

VERIFY_ARGS=("${SCRIPT_DIR}/verify-sdk.mjs")
for language in "${LANGUAGES[@]}"; do
  VERIFY_ARGS+=("--language" "$language")
done

node "${VERIFY_ARGS[@]}"
