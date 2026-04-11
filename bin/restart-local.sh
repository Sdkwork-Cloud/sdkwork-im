#!/usr/bin/env bash
set -euo pipefail

show_help() {
  cat <<'EOF'
Usage: bash bin/restart-local.sh [--profile <local-minimal|local-default>] [--release] [--foreground] [--bind-addr <host:port>]

Restart local-minimal-node using the stop/start lifecycle scripts.
EOF
}

profile_name="local-minimal"
start_args=()

while [[ $# -gt 0 ]]; do
  case "$1" in
    --profile)
      if [[ $# -lt 2 ]]; then
        echo "--profile requires a value" >&2
        exit 1
      fi
      profile_name="$2"
      start_args+=("$1" "$2")
      shift 2
      ;;
    --release|--foreground)
      start_args+=("$1")
      shift
      ;;
    --bind-addr)
      if [[ $# -lt 2 ]]; then
        echo "--bind-addr requires a value" >&2
        exit 1
      fi
      start_args+=("$1" "$2")
      shift 2
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

bash "${ROOT_DIR}/bin/stop-local.sh" --profile "$profile_name"
bash "${ROOT_DIR}/bin/start-local.sh" "${start_args[@]}"
