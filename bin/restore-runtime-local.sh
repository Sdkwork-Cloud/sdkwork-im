#!/usr/bin/env bash
set -euo pipefail

show_help() {
  cat <<'EOF'
Usage: bash bin/restore-runtime-local.sh --backup-dir <path> [--profile <local-minimal|local-default>] [--runtime-dir <path>] [--json] [--release]
       bash bin/restore-runtime-local.sh --backup-dir <path> [--profile <local-minimal|local-default>] [--runtime-dir <path>] [--expected-preview-fingerprint <value>] [--json] [--release]

Restore managed local runtime-dir state files for the selected local-minimal/local-default profile from an explicit backup snapshot through the local-minimal-node restore entrypoint.
EOF
}

# Resolves SDKWORK_IM_RUNTIME_DIR from the selected profile config before preferring target/debug/local-minimal-node or target/release/local-minimal-node.
profile_name="local-minimal"
runtime_dir=""
backup_dir=""
expected_preview_fingerprint=""
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
    --backup-dir)
      if [[ $# -lt 2 ]]; then
        echo "--backup-dir requires a value" >&2
        exit 1
      fi
      backup_dir="$2"
      shift 2
      ;;
    --json)
      json_output=1
      shift
      ;;
    --expected-preview-fingerprint)
      if [[ $# -lt 2 ]]; then
        echo "--expected-preview-fingerprint requires a value" >&2
        exit 1
      fi
      expected_preview_fingerprint="$2"
      shift 2
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

if [[ -z "$backup_dir" ]]; then
  echo "--backup-dir is required" >&2
  show_help >&2
  exit 1
fi

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

restore_args=(restore-runtime-dir --runtime-dir "$runtime_dir" --backup-dir "$backup_dir")
if [[ -n "$expected_preview_fingerprint" ]]; then
  restore_args+=(--expected-preview-fingerprint "$expected_preview_fingerprint")
fi
if [[ "$json_output" -eq 1 ]]; then
  restore_args+=(--json)
fi

if binary_path="$(resolve_binary_path "$ROOT_DIR" "$prefer_release")"; then
  exec "$binary_path" "${restore_args[@]}"
fi

if command -v cargo >/dev/null 2>&1; then
  exec cargo run -p local-minimal-node --offline -- "${restore_args[@]}"
fi

echo "local-minimal-node binary not found under target/debug or target/release, and cargo is unavailable." >&2
exit 1
