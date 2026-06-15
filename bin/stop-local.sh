#!/usr/bin/env bash
set -euo pipefail

show_help() {
  cat <<'EOF'
Usage: bash bin/stop-local.sh [--profile <local-minimal|local-default>]

Stop the local-minimal-node background process and remove the pid file.
EOF
}

profile_name="local-minimal"

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
      config_runtime_dir="$(read_config_value_from_file "$config_file" "SDKWORK_IM_RUNTIME_DIR" || true)"
      if [[ -n "$config_runtime_dir" ]]; then
        printf '%s\n' "$config_runtime_dir"
        return 0
      fi
    done < <(runtime_profile_config_files "$root_dir" "$selected_profile")

    printf '%s\n' "${root_dir}/.runtime/local-minimal"
  }
fi

validate_runtime_profile_name "$profile_name"

PID_FILE="$(resolve_runtime_dir_from_profile "$ROOT_DIR" "$profile_name")/pids/local-minimal-node.pid"
EXPECTED_PROCESS_NAME="local-minimal-node"

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
