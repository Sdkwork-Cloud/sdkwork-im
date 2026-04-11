#!/usr/bin/env bash
set -euo pipefail

show_help() {
  cat <<'EOF'
Usage: bash bin/list-runtime-backups-local.sh [--profile <local-minimal|local-default>] [--runtime-dir <path>] [--json] [--release]

List managed local runtime-dir backup snapshots for the selected local-minimal/local-default profile with readiness preview through the local-minimal-node catalog entrypoint.
EOF
}

# Resolves CRAW_CHAT_RUNTIME_DIR from the selected profile config before preferring target/debug/local-minimal-node or target/release/local-minimal-node.
profile_name="local-minimal"
runtime_dir=""
json_output=0
prefer_release=0

while [[ $# -gt 0 ]]; do
  case "$1" in
    --profile)
      if [[ $# -lt 2 ]]; then
        echo "--profile requires a value" >&2
        exit 1
      fi
      profile_name="$2"
      shift 2
      ;;
    --runtime-dir)
      if [[ $# -lt 2 ]]; then
        echo "--runtime-dir requires a value" >&2
        exit 1
      fi
      runtime_dir="$2"
      shift 2
      ;;
    --json)
      json_output=1
      shift
      ;;
    --release)
      prefer_release=1
      shift
      ;;
    -h|--help)
      show_help
      exit 0
      ;;
    *)
      echo "Unknown argument: $1" >&2
      show_help >&2
      exit 1
      ;;
  esac
done

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
RUNTIME_PROFILE_HELPER="${ROOT_DIR}/bin/_runtime-profile-common.sh"
if [[ ! -f "$RUNTIME_PROFILE_HELPER" ]]; then
  echo "Missing runtime profile helper: ${RUNTIME_PROFILE_HELPER}" >&2
  exit 1
fi
# shellcheck source=bin/_runtime-profile-common.sh
source "$RUNTIME_PROFILE_HELPER"

validate_runtime_profile_name "$profile_name"

if [[ -z "$runtime_dir" ]]; then
  runtime_dir="$(resolve_runtime_dir_from_profile "$ROOT_DIR" "$profile_name")"
fi

catalog_args=(list-runtime-backups --runtime-dir "$runtime_dir")
if [[ "$json_output" -eq 1 ]]; then
  catalog_args+=(--json)
fi

if binary_path="$(resolve_binary_path "$ROOT_DIR" "$prefer_release")"; then
  exec "$binary_path" "${catalog_args[@]}"
fi

if command -v cargo >/dev/null 2>&1; then
  exec cargo run -p local-minimal-node --offline -- "${catalog_args[@]}"
fi

echo "local-minimal-node binary not found under target/debug or target/release, and cargo is unavailable." >&2
exit 1
