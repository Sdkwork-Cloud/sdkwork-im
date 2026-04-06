#!/usr/bin/env bash
set -euo pipefail

show_help() {
  cat <<'EOF'
Usage: bash bin/status-local.sh

Show local-minimal-node pid, config, stdout/stderr logs, and health status.
For deeper state validation, run bash bin/inspect-runtime-local.sh
For backup-first missing-file repair, run bash bin/repair-runtime-local.sh
For backup snapshot listing, run bash bin/list-runtime-backups-local.sh
For restore dry-run preview, run bash bin/preview-runtime-restore-local.sh --backup-dir <path>
For explicit backup restore, run bash bin/restore-runtime-local.sh --backup-dir <path> --expected-preview-fingerprint <previewFingerprint>
EOF
}

if [[ "${1:-}" == "-h" || "${1:-}" == "--help" ]]; then
  show_help
  exit 0
fi

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
CONFIG_FILE="${ROOT_DIR}/.runtime/local-minimal/config/local-minimal.env"
PID_FILE="${ROOT_DIR}/.runtime/local-minimal/pids/local-minimal-node.pid"
STDOUT_LOG="${ROOT_DIR}/.runtime/local-minimal/logs/local-minimal-node.out.log"
STDERR_LOG="${ROOT_DIR}/.runtime/local-minimal/logs/local-minimal-node.err.log"
EXPECTED_PROCESS_NAME="local-minimal-node"
bind_addr="127.0.0.1:18090"

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

health_url_from_bind_addr() {
  local resolved_bind_addr="$1"
  local host="${resolved_bind_addr%:*}"
  local port="${resolved_bind_addr##*:}"

  if [[ "$host" == "$resolved_bind_addr" || -z "$host" || "$host" == "0.0.0.0" || "$host" == "::" || "$host" == "[::]" ]]; then
    host="127.0.0.1"
  fi

  printf 'http://%s:%s/healthz\n' "$host" "$port"
}

probe_health() {
  local health_url="$1"

  if command -v curl >/dev/null 2>&1; then
    curl --fail --silent --show-error "$health_url" >/dev/null 2>&1
    return
  fi

  if command -v wget >/dev/null 2>&1; then
    wget -q -O /dev/null "$health_url" >/dev/null 2>&1
    return
  fi

  return 127
}

if [[ -f "$CONFIG_FILE" ]]; then
  while IFS='=' read -r key value; do
    key="${key%$'\r'}"
    value="${value%$'\r'}"
    [[ -z "$key" || "$key" == \#* ]] && continue
    if [[ "$key" == "CRAW_CHAT_BIND_ADDR" ]]; then
      bind_addr="$value"
      break
    fi
  done <"$CONFIG_FILE"
fi

health_url="$(health_url_from_bind_addr "$bind_addr")"

echo "config: ${CONFIG_FILE}"
echo "bind: ${bind_addr}"
echo "health: ${health_url}"
echo "stdout log: ${STDOUT_LOG}"
echo "stderr log: ${STDERR_LOG}"
echo "runtime inspection: bash bin/inspect-runtime-local.sh"
echo "runtime repair: bash bin/repair-runtime-local.sh"
echo "runtime backups: bash bin/list-runtime-backups-local.sh"
echo "runtime restore preview: bash bin/preview-runtime-restore-local.sh --backup-dir <path>"
echo "runtime restore: bash bin/restore-runtime-local.sh --backup-dir <path> --expected-preview-fingerprint <previewFingerprint>"

if [[ ! -f "$PID_FILE" ]]; then
  echo "status: stopped"
  echo "health status: stopped"
  exit 0
fi

pid="$(get_running_pid_from_pid_file "$PID_FILE")"
if [[ -z "$pid" ]]; then
  echo "status: stopped"
  echo "health status: stopped"
  exit 0
fi

echo "status: running"
echo "pid: ${pid}"

if probe_health "$health_url"; then
  echo "health status: ok"
elif [[ $? -eq 127 ]]; then
  echo "health status: probe-unavailable"
else
  echo "health status: unreachable"
fi
