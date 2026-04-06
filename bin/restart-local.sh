#!/usr/bin/env bash
set -euo pipefail

show_help() {
  cat <<'EOF'
Usage: bash bin/restart-local.sh [--release] [--foreground] [--bind-addr <host:port>]

Restart local-minimal-node using the stop/start lifecycle scripts.
EOF
}

if [[ "${1:-}" == "-h" || "${1:-}" == "--help" ]]; then
  show_help
  exit 0
fi

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

bash "${ROOT_DIR}/bin/stop-local.sh"
bash "${ROOT_DIR}/bin/start-local.sh" "$@"
