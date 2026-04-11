#!/usr/bin/env bash
set -euo pipefail

show_help() {
  cat <<'EOF'
Usage: bash bin/repair-runtime-local.sh [--profile <local-minimal|local-default>] [--runtime-dir <path>] [--json] [--release]

Repair missing managed local runtime-dir state files for the selected local-minimal/local-default profile, then replay social journal truth through control-plane-api when state/social-commit-journal.json is present.
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

resolve_control_plane_binary_path() {
  local root_dir="$1"
  local prefer_release="$2"
  local release_path="${root_dir}/target/release/control-plane-api"
  local debug_path="${root_dir}/target/debug/control-plane-api"
  local candidate

  if [[ "$prefer_release" -eq 1 ]]; then
    for candidate in "$release_path" "$debug_path"; do
      if [[ -x "$candidate" ]]; then
        printf '%s\n' "$candidate"
        return 0
      fi
    done
  else
    for candidate in "$debug_path" "$release_path"; do
      if [[ -x "$candidate" ]]; then
        printf '%s\n' "$candidate"
        return 0
      fi
    done
  fi

  return 1
}

repair_args=(repair-runtime-dir --runtime-dir "$runtime_dir")
if [[ "$json_output" -eq 1 ]]; then
  repair_args+=(--json)
fi

if binary_path="$(resolve_binary_path "$ROOT_DIR" "$prefer_release")"; then
  "$binary_path" "${repair_args[@]}"
elif command -v cargo >/dev/null 2>&1; then
  cargo run -p local-minimal-node --offline -- "${repair_args[@]}"
else
  echo "local-minimal-node binary not found under target/debug or target/release, and cargo is unavailable." >&2
  exit 1
fi

social_journal_path="${runtime_dir}/state/social-commit-journal.json"
if [[ ! -f "$social_journal_path" ]]; then
  exit 0
fi

social_repair_args=(repair-social-runtime-dir --runtime-dir "$runtime_dir")
if [[ "$json_output" -eq 1 ]]; then
  social_repair_args+=(--json)
fi

if social_binary_path="$(resolve_control_plane_binary_path "$ROOT_DIR" "$prefer_release")"; then
  if [[ "$json_output" -eq 1 ]]; then
    "$social_binary_path" "${social_repair_args[@]}" >/dev/null
  else
    "$social_binary_path" "${social_repair_args[@]}"
  fi
  exit 0
fi

if command -v cargo >/dev/null 2>&1; then
  if [[ "$json_output" -eq 1 ]]; then
    cargo run -p control-plane-api --offline -- "${social_repair_args[@]}" >/dev/null
  else
    cargo run -p control-plane-api --offline -- "${social_repair_args[@]}"
  fi
  exit 0
fi

echo "social commit journal exists at ${social_journal_path}, but control-plane-api binary was not found under target/debug or target/release and cargo is unavailable." >&2
exit 1
