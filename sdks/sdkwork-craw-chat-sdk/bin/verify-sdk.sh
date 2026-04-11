#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

ARGS=("${SCRIPT_DIR}/verify-sdk.mjs")

while [[ $# -gt 0 ]]; do
  case "$1" in
    --language)
      if [[ $# -lt 2 ]]; then
        echo "[sdkwork-craw-chat-sdk] Missing value for --language" >&2
        exit 1
      fi
      ARGS+=("$1" "$2")
      shift 2
      ;;
    --with-dart)
      ARGS+=("$1")
      shift
      ;;
    *)
      echo "[sdkwork-craw-chat-sdk] Unknown argument: $1" >&2
      exit 1
      ;;
  esac
done

node "${ARGS[@]}"
