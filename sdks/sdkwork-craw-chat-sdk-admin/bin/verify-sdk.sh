#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
LANGUAGES=()
WITH_DART="false"

while [[ $# -gt 0 ]]; do
  case "$1" in
    --language)
      LANGUAGES+=("${2,,}")
      shift 2
      ;;
    --with-dart)
      WITH_DART="true"
      shift
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

VERIFY_ARGS=("${SCRIPT_DIR}/verify-sdk.mjs")
for language in "${LANGUAGES[@]}"; do
  VERIFY_ARGS+=("--language" "$language")
done
if [[ "${WITH_DART}" == "true" ]]; then
  VERIFY_ARGS+=("--with-dart")
fi

node "${VERIFY_ARGS[@]}"
