#!/usr/bin/env bash
set -euo pipefail

show_help() {
  cat <<'EOF'
Usage: bash bin/start-local.sh [--release] [--foreground] [--bind-addr <host:port>]

Build and start local-minimal-node with config, pid/log management, and health wait.
EOF
}

release_mode=0
foreground=0
bind_addr=""
bind_addr_provided=0

while [[ $# -gt 0 ]]; do
  case "$1" in
    --release)
      release_mode=1
      shift
      ;;
    --foreground)
      foreground=1
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

CONFIG_FILE="${ROOT_DIR}/.runtime/local-minimal/config/local-minimal.env"
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

stop_managed_process_and_remove_pid_file() {
  local pid_file="$1"
  local pid="${2:-}"

  if [[ -z "$pid" ]]; then
    pid="$(get_running_pid_from_pid_file "$pid_file")"
  fi

  if [[ -n "$pid" ]] && pid_matches_expected_process "$pid"; then
    kill "$pid" >/dev/null 2>&1 || true

    for _ in $(seq 1 5); do
      if ! pid_matches_expected_process "$pid"; then
        break
      fi
      sleep 1
    done

    if pid_matches_expected_process "$pid"; then
      kill -9 "$pid" >/dev/null 2>&1 || true

      for _ in $(seq 1 5); do
        if ! pid_matches_expected_process "$pid"; then
          break
        fi
        sleep 1
      done
    fi

    if pid_matches_expected_process "$pid"; then
      return 1
    fi
  fi

  rm -f "$pid_file"
  return 0
}

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

has_health_probe_tool() {
  command -v curl >/dev/null 2>&1 || command -v wget >/dev/null 2>&1
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

resolve_bind_addr() {
  local cli_bind_addr="$1"
  if [[ -n "$cli_bind_addr" ]]; then
    printf '%s\n' "$cli_bind_addr"
    return
  fi

  local config_bind_addr
  config_bind_addr="$(read_config_value "CRAW_CHAT_BIND_ADDR" || true)"
  if [[ -n "$config_bind_addr" ]]; then
    printf '%s\n' "$config_bind_addr"
    return
  fi

  printf '%s\n' "127.0.0.1:18090"
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

install_args=()
profile_dir="debug"
if [[ "$release_mode" -eq 1 ]]; then
  install_args+=(--release)
  profile_dir="release"
fi

if [[ "$bind_addr_provided" -eq 1 ]]; then
  install_args+=(--bind-addr "$bind_addr")
fi

bash "${ROOT_DIR}/bin/install-local.sh" "${install_args[@]}"

EXE_PATH="${ROOT_DIR}/target/${profile_dir}/local-minimal-node"
if [[ ! -x "$EXE_PATH" ]]; then
  echo "Binary not found: $EXE_PATH" >&2
  exit 1
fi

RUNTIME_DIR="${ROOT_DIR}/.runtime/local-minimal"
LOGS_DIR="${RUNTIME_DIR}/logs"
PIDS_DIR="${RUNTIME_DIR}/pids"
PID_FILE="${PIDS_DIR}/local-minimal-node.pid"
STDOUT_LOG="${LOGS_DIR}/local-minimal-node.out.log"
STDERR_LOG="${LOGS_DIR}/local-minimal-node.err.log"

existing_pid="$(get_running_pid_from_pid_file "$PID_FILE")"
if [[ -n "$existing_pid" ]]; then
  echo "local-minimal-node is already running with PID ${existing_pid}." >&2
  exit 1
fi

resolved_bind_addr="$(resolve_bind_addr "$bind_addr")"
resolved_runtime_dir="$(read_config_value "CRAW_CHAT_RUNTIME_DIR" || true)"
if [[ -z "$resolved_runtime_dir" ]]; then
  resolved_runtime_dir="$RUNTIME_DIR"
fi
resolved_public_bearer_secret="$(read_config_value "CRAW_CHAT_PUBLIC_BEARER_HS256_SECRET" || true)"
if [[ -z "$resolved_public_bearer_secret" ]]; then
  resolved_public_bearer_secret="${CRAW_CHAT_PUBLIC_BEARER_HS256_SECRET:-}"
fi
if [[ -z "$resolved_public_bearer_secret" ]]; then
  echo "CRAW_CHAT_PUBLIC_BEARER_HS256_SECRET must be configured before starting local-minimal-node." >&2
  exit 1
fi

if [[ "$foreground" -eq 1 ]]; then
  echo "Starting local-minimal-node in foreground on http://${resolved_bind_addr}"
  exec env CRAW_CHAT_BIND_ADDR="$resolved_bind_addr" CRAW_CHAT_RUNTIME_DIR="$resolved_runtime_dir" CRAW_CHAT_PUBLIC_BEARER_HS256_SECRET="$resolved_public_bearer_secret" "$EXE_PATH"
fi

if ! has_health_probe_tool; then
  echo "Neither curl nor wget is available for health verification." >&2
  exit 1
fi

echo "Starting local-minimal-node in background on http://${resolved_bind_addr}"
nohup env CRAW_CHAT_BIND_ADDR="$resolved_bind_addr" CRAW_CHAT_RUNTIME_DIR="$resolved_runtime_dir" CRAW_CHAT_PUBLIC_BEARER_HS256_SECRET="$resolved_public_bearer_secret" "$EXE_PATH" >>"$STDOUT_LOG" 2>>"$STDERR_LOG" &
pid=$!
echo "$pid" >"$PID_FILE"

health_url="$(health_url_from_bind_addr "$resolved_bind_addr")"
ready=0
for _ in $(seq 1 30); do
  sleep 1
  if ! pid_matches_expected_process "$pid"; then
    stop_managed_process_and_remove_pid_file "$PID_FILE" "$pid" || true
    echo "local-minimal-node exited before becoming ready. Check logs: ${STDERR_LOG}" >&2
    exit 1
  fi

  if probe_health "$health_url"; then
    ready=1
    break
  fi
done

if [[ "$ready" -ne 1 ]]; then
  if ! stop_managed_process_and_remove_pid_file "$PID_FILE" "$pid"; then
    echo "local-minimal-node remained running after startup rollback. Check logs: ${STDERR_LOG}" >&2
    exit 1
  fi
  echo "local-minimal-node did not become healthy within 30 seconds: ${health_url}" >&2
  exit 1
fi

echo "PID: ${pid}"
echo "stdout log: ${STDOUT_LOG}"
echo "stderr log: ${STDERR_LOG}"
echo "pid file: ${PID_FILE}"
echo "health: ${health_url}"
