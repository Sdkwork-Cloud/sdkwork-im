#!/usr/bin/env bash
set -euo pipefail

show_help() {
  cat <<'EOF'
Usage: bash bin/install-local.sh [--release] [--bind-addr <host:port>]

Build local-minimal-node offline, initialize config, and prepare .runtime directories.
EOF
}

release_mode=0
bind_addr="127.0.0.1:18090"
bind_addr_provided=0

while [[ $# -gt 0 ]]; do
  case "$1" in
    --release)
      release_mode=1
      shift
      ;;
    --bind-addr)
      if [[ $# -lt 2 ]]; then
        echo "--bind-addr requires a value" >&2
        exit 1
      fi
      bind_addr="$2"
      bind_addr_provided=1
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
cd "$ROOT_DIR"

if ! command -v cargo >/dev/null 2>&1; then
  echo "cargo is unavailable. Install the Rust toolchain and ensure cargo is on PATH." >&2
  exit 1
fi

RUNTIME_ROOT="${ROOT_DIR}/.runtime"
SERVICE_ROOT="${RUNTIME_ROOT}/local-minimal"
CONFIG_DIR="${SERVICE_ROOT}/config"
LOGS_DIR="${SERVICE_ROOT}/logs"
PIDS_DIR="${SERVICE_ROOT}/pids"

mkdir -p "$CONFIG_DIR" "$LOGS_DIR" "$PIDS_DIR"

init_args=(--bind-addr "$bind_addr")
if [[ "$bind_addr_provided" -eq 1 ]]; then
  init_args+=(--force)
fi

bash "${ROOT_DIR}/bin/init-config-local.sh" "${init_args[@]}"

if [[ "$release_mode" -eq 1 ]]; then
  echo "Building local-minimal-node in release mode..."
  cargo build --release -p local-minimal-node --offline
else
  echo "Building local-minimal-node in debug mode..."
  cargo build -p local-minimal-node --offline
fi

echo "Runtime directories prepared under ${SERVICE_ROOT}"
