#!/usr/bin/env bash
set -euo pipefail

show_help() {
  cat <<'EOF'
Usage: bash bin/stop-local.sh

Stop the local-minimal-node background process and remove the pid file.
EOF
}

if [[ "${1:-}" == "-h" || "${1:-}" == "--help" ]]; then
  show_help
  exit 0
fi

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
PID_FILE="${ROOT_DIR}/.runtime/local-minimal/pids/local-minimal-node.pid"
EXPECTED_PROCESS_NAME="local-minimal-node"

pid_matches_expected_process() {
  local pid="$1"
  local process_name=""

  [[ -n "$pid" ]] || return 1

  if ! kill -0 "$pid" >/dev/null 2>&1; then
    return 1
  fi

  process_name="$(ps -p "$pid" -o comm= 2>/dev/null | tr -d '\r' | tr -d '[:space:]' || true)"
  process_name="${process_name##*/}"

  [[ -n "$process_name" && "$process_name" == "$EXPECTED_PROCESS_NAME" ]]
}

get_running_pid_from_pid_file() {
  local pid_file="$1"
  local pid=""

  if [[ ! -f "$pid_file" ]]; then
    return 0
  fi

  pid="$(tr -d '[:space:]' < "$pid_file" 2>/dev/null || true)"
  if [[ -z "$pid" ]]; then
    rm -f "$pid_file"
    return 0
  fi

  if ! pid_matches_expected_process "$pid"; then
    rm -f "$pid_file"
    return 0
  fi

  printf '%s\n' "$pid"
}

if [[ ! -f "$PID_FILE" ]]; then
  echo "local-minimal-node is not running."
  exit 0
fi

pid="$(get_running_pid_from_pid_file "$PID_FILE")"
if [[ -z "$pid" ]]; then
  echo "local-minimal-node is not running."
  exit 0
fi

echo "Stopping local-minimal-node PID ${pid}"
kill "$pid"

for _ in $(seq 1 30); do
  if ! pid_matches_expected_process "$pid"; then
    rm -f "$PID_FILE"
    echo "local-minimal-node stopped."
    exit 0
  fi
  sleep 1
done

echo "local-minimal-node PID ${pid} did not exit within 30 seconds." >&2
exit 1
