#!/usr/bin/env bash
set -euo pipefail

show_help() {
  cat <<'EOF'
Usage: bash bin/start-local.sh [--profile <local-minimal|local-default>] [--release] [--foreground] [--bind-addr <host:port>]

Build and start local-minimal-node with config, pid/log management, and health wait.
EOF
}

profile_name="local-minimal"
release_mode=0
foreground=0
bind_addr=""
bind_addr_provided=0

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

RUNTIME_PROFILE_HELPER="${ROOT_DIR}/bin/_runtime-profile-common.sh"
if [[ -f "$RUNTIME_PROFILE_HELPER" ]]; then
  # shellcheck source=bin/_runtime-profile-common.sh
  source "$RUNTIME_PROFILE_HELPER"
else
  validate_runtime_profile_name() {
    local selected_profile="$1"

    case "$selected_profile" in
      local-minimal|local-default)
        ;;
      *)
        echo "Unsupported runtime operations profile: ${selected_profile}" >&2
        return 1
        ;;
    esac
  }

  runtime_profile_config_files() {
    local root_dir="$1"
    local selected_profile="$2"

    case "$selected_profile" in
      local-default)
        printf '%s\n' \
          "${root_dir}/.runtime/local-default/config/local-default.env" \
          "${root_dir}/.runtime/local-minimal/config/local-minimal.env"
        ;;
      *)
        printf '%s\n' "${root_dir}/.runtime/local-minimal/config/local-minimal.env"
        ;;
    esac
  }

  read_config_value_from_file() {
    local config_file="$1"
    local key="$2"
    [[ -f "$config_file" ]] || return 1

    while IFS='=' read -r current_key current_value; do
      current_key="${current_key%$'\r'}"
      current_value="${current_value%$'\r'}"
      [[ -z "$current_key" || "$current_key" == \#* ]] && continue
      if [[ "$current_key" == "$key" ]]; then
        printf '%s\n' "$current_value"
        return 0
      fi
    done <"$config_file"

    return 1
  }

  resolve_runtime_dir_from_profile() {
    local root_dir="$1"
    local selected_profile="$2"
    local config_file=""
    local config_runtime_dir=""

    while IFS= read -r config_file; do
      config_runtime_dir="$(read_config_value_from_file "$config_file" "CRAW_CHAT_RUNTIME_DIR" || true)"
      if [[ -n "$config_runtime_dir" ]]; then
        printf '%s\n' "$config_runtime_dir"
        return 0
      fi
    done < <(runtime_profile_config_files "$root_dir" "$selected_profile")

    printf '%s\n' "${root_dir}/.runtime/local-minimal"
  }
fi

validate_runtime_profile_name "$profile_name"

resolve_profile_config_file() {
  local root_dir="$1"
  local selected_profile="$2"
  local config_file=""
  local first_candidate=""

  while IFS= read -r config_file; do
    if [[ -z "$first_candidate" ]]; then
      first_candidate="$config_file"
    fi
    if [[ -f "$config_file" ]]; then
      printf '%s\n' "$config_file"
      return 0
    fi
  done < <(runtime_profile_config_files "$root_dir" "$selected_profile")

  printf '%s\n' "${first_candidate:-${root_dir}/.runtime/local-minimal/config/local-minimal.env}"
}

resolve_config_value_from_profile() {
  local root_dir="$1"
  local selected_profile="$2"
  local key="$3"
  local config_file=""
  local value=""

  while IFS= read -r config_file; do
    value="$(read_config_value_from_file "$config_file" "$key" || true)"
    if [[ -n "$value" ]]; then
      printf '%s\n' "$value"
      return 0
    fi
  done < <(runtime_profile_config_files "$root_dir" "$selected_profile")

  return 1
}

pid_matches_expected_process() {
  local pid="$1"
  local process_name=""
  local process_path=""

  [[ -n "$pid" ]] || return 1

  if ! kill -0 "$pid" >/dev/null 2>&1; then
    return 1
  fi

  process_name="$(ps -p "$pid" -o args= 2>/dev/null | tr -d '\r' || true)"
  process_name="${process_name#"${process_name%%[![:space:]]*}"}"
  process_path="${process_name%% *}"
  process_name="${process_path##*/}"

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

  local config_bind_addr=""
  config_bind_addr="$(resolve_config_value_from_profile "$ROOT_DIR" "$profile_name" "CRAW_CHAT_BIND_ADDR" || true)"
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

install_args=(--profile "$profile_name")
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

CONFIG_FILE="$(resolve_profile_config_file "$ROOT_DIR" "$profile_name")"
RUNTIME_DIR="$(resolve_runtime_dir_from_profile "$ROOT_DIR" "$profile_name")"
LOGS_DIR="${RUNTIME_DIR}/logs"
PIDS_DIR="${RUNTIME_DIR}/pids"
PID_FILE="${PIDS_DIR}/local-minimal-node.pid"
STDOUT_LOG="${LOGS_DIR}/local-minimal-node.out.log"
STDERR_LOG="${LOGS_DIR}/local-minimal-node.err.log"
EXPECTED_PROCESS_NAME="local-minimal-node"

existing_pid="$(get_running_pid_from_pid_file "$PID_FILE")"
if [[ -n "$existing_pid" ]]; then
  echo "local-minimal-node is already running with PID ${existing_pid}." >&2
  exit 1
fi

resolved_bind_addr="$(resolve_bind_addr "$bind_addr")"
resolved_runtime_dir="$RUNTIME_DIR"
resolved_public_bearer_secret="$(resolve_config_value_from_profile "$ROOT_DIR" "$profile_name" "CRAW_CHAT_PUBLIC_BEARER_HS256_SECRET" || true)"
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
echo "config: ${CONFIG_FILE}"
