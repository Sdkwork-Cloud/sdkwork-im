#!/usr/bin/env bash
set -euo pipefail

show_help() {
  cat <<'EOF'
Usage: bash bin/preview-runtime-restore-local.sh --backup-dir <path> [--runtime-dir <path>] [--json] [--release]

Preview managed local-minimal runtime-dir restore actions from an explicit backup snapshot through the local-minimal-node preview entrypoint.
EOF
}

runtime_dir=""
backup_dir=""
json_output=0
prefer_release=0

while [[ $# -gt 0 ]]; do
  case "$1" in
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
CONFIG_FILE="${ROOT_DIR}/.runtime/local-minimal/config/local-minimal.env"

read_config_value() {
  local key="$1"
  [[ -f "$CONFIG_FILE" ]] || return 1

  while IFS='=' read -r current_key current_value; do
    current_key="${current_key%$'\r'}"
    current_value="${current_value%$'\r'}"
    [[ -z "$current_key" || "$current_key" == \#* ]] && continue
    if [[ "$current_key" == "$key" ]]; then
      printf '%s\n' "$current_value"
      return 0
    fi
  done <"$CONFIG_FILE"

  return 1
}

resolve_binary_path() {
  local release_path="${ROOT_DIR}/target/release/local-minimal-node"
  local debug_path="${ROOT_DIR}/target/debug/local-minimal-node"

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

if [[ -z "$runtime_dir" ]]; then
  runtime_dir="$(read_config_value "CRAW_CHAT_RUNTIME_DIR" || true)"
fi
if [[ -z "$runtime_dir" ]]; then
  runtime_dir="${ROOT_DIR}/.runtime/local-minimal"
fi

preview_args=(preview-runtime-restore --runtime-dir "$runtime_dir" --backup-dir "$backup_dir")
if [[ "$json_output" -eq 1 ]]; then
  preview_args+=(--json)
fi

if binary_path="$(resolve_binary_path)"; then
  exec "$binary_path" "${preview_args[@]}"
fi

if command -v cargo >/dev/null 2>&1; then
  exec cargo run -p local-minimal-node --offline -- "${preview_args[@]}"
fi

echo "local-minimal-node binary not found under target/debug or target/release, and cargo is unavailable." >&2
exit 1
