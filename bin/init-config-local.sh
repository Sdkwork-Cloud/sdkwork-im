#!/usr/bin/env bash
set -euo pipefail

show_help() {
  cat <<'EOF'
Usage: bash bin/init-config-local.sh [--bind-addr <host:port>] [--force]

Create or update the local-minimal runtime config file.
EOF
}

bind_addr="127.0.0.1:18090"
force_mode=0

while [[ $# -gt 0 ]]; do
  case "$1" in
    --bind-addr)
      if [[ $# -lt 2 ]]; then
        echo "--bind-addr requires a value" >&2
        exit 1
      fi
      bind_addr="$2"
      shift 2
      ;;
    --force)
      force_mode=1
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
RUNTIME_DIR="${ROOT_DIR}/.runtime/local-minimal"
CONFIG_DIR="${RUNTIME_DIR}/config"
LOGS_DIR="${RUNTIME_DIR}/logs"
PIDS_DIR="${RUNTIME_DIR}/pids"
STATE_DIR="${RUNTIME_DIR}/state"
CONFIG_FILE="${CONFIG_DIR}/local-minimal.env"

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

generate_public_bearer_secret() {
  if command -v openssl >/dev/null 2>&1; then
    openssl rand -base64 48 | tr -d '\r\n' | tr '+/' '-_' | tr -d '='
    return
  fi

  if [[ -r /dev/urandom ]]; then
    od -An -tx1 -N32 /dev/urandom | tr -d ' \n'
    return
  fi

  echo "Unable to generate CRAW_CHAT_PUBLIC_BEARER_HS256_SECRET" >&2
  exit 1
}

mkdir -p "$CONFIG_DIR" "$LOGS_DIR" "$PIDS_DIR" "$STATE_DIR"

if [[ -f "$CONFIG_FILE" && "$force_mode" -ne 1 ]]; then
  echo "Config already exists: ${CONFIG_FILE}"
  exit 0
fi

public_bearer_secret="$(read_config_value "CRAW_CHAT_PUBLIC_BEARER_HS256_SECRET" || true)"
if [[ -z "$public_bearer_secret" ]]; then
  public_bearer_secret="$(generate_public_bearer_secret)"
fi

cat >"$CONFIG_FILE" <<EOF
# local-minimal runtime config
CRAW_CHAT_BIND_ADDR=${bind_addr}
CRAW_CHAT_RUNTIME_DIR=${RUNTIME_DIR}
CRAW_CHAT_PUBLIC_BEARER_HS256_SECRET=${public_bearer_secret}
EOF

echo "Config written: ${CONFIG_FILE}"
