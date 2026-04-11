#!/usr/bin/env bash
set -euo pipefail

show_help() {
  cat <<'EOF'
Usage: bash bin/status-local.sh [--profile <local-minimal|local-default>] [--runtime-dir <path>]

Show local-minimal-node pid, config, stdout/stderr logs, health status, and the next runtime-dir inspection/repair/list/archive/prune/preview/restore steps.
For deeper state validation, run bash bin/inspect-runtime-local.sh
For backup-first missing-file repair, run bash bin/repair-runtime-local.sh
For backup snapshot listing, run bash bin/list-runtime-backups-local.sh
For backup snapshot archiving, run bash bin/archive-runtime-backup-local.sh --backup-dir <path>
For archived snapshot pruning, run bash bin/prune-runtime-archives-local.sh
For restore dry-run preview, run bash bin/preview-runtime-restore-local.sh --backup-dir <path>
For explicit backup restore, run bash bin/restore-runtime-local.sh --backup-dir <path> --expected-preview-fingerprint <previewFingerprint>
EOF
}

profile_name="local-minimal"
runtime_dir=""

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
    --runtime-dir)
      if [[ $# -lt 2 ]]; then
        echo "--runtime-dir requires a value" >&2
        exit 1
      fi
      runtime_dir="$2"
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
if [[ ! -f "$RUNTIME_PROFILE_HELPER" ]]; then
  echo "Missing runtime profile helper: ${RUNTIME_PROFILE_HELPER}" >&2
  exit 1
fi
# shellcheck source=bin/_runtime-profile-common.sh
source "$RUNTIME_PROFILE_HELPER"

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

resolve_bind_addr_from_profile() {
  local root_dir="$1"
  local selected_profile="$2"
  local config_file=""
  local config_bind_addr=""

  while IFS= read -r config_file; do
    config_bind_addr="$(read_config_value_from_file "$config_file" "CRAW_CHAT_BIND_ADDR" || true)"
    if [[ -n "$config_bind_addr" ]]; then
      printf '%s\n' "$config_bind_addr"
      return 0
    fi
  done < <(runtime_profile_config_files "$root_dir" "$selected_profile")

  printf '127.0.0.1:18090\n'
}

if [[ -z "$runtime_dir" ]]; then
  runtime_dir="$(resolve_runtime_dir_from_profile "$ROOT_DIR" "$profile_name")"
fi

CONFIG_FILE="$(resolve_profile_config_file "$ROOT_DIR" "$profile_name")"
PID_FILE="${runtime_dir}/pids/local-minimal-node.pid"
STDOUT_LOG="${runtime_dir}/logs/local-minimal-node.out.log"
STDERR_LOG="${runtime_dir}/logs/local-minimal-node.err.log"
EXPECTED_PROCESS_NAME="local-minimal-node"
bind_addr="$(resolve_bind_addr_from_profile "$ROOT_DIR" "$profile_name")"
profile_suffix=""
if [[ "$profile_name" != "local-minimal" ]]; then
  profile_suffix=" --profile ${profile_name}"
fi

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

health_url="$(health_url_from_bind_addr "$bind_addr")"

echo "profile: ${profile_name}"
echo "config: ${CONFIG_FILE}"
echo "bind: ${bind_addr}"
echo "health: ${health_url}"
echo "stdout log: ${STDOUT_LOG}"
echo "stderr log: ${STDERR_LOG}"
echo "runtime inspection: bash bin/inspect-runtime-local.sh${profile_suffix}"
echo "runtime repair: bash bin/repair-runtime-local.sh${profile_suffix}"
echo "runtime backups: bash bin/list-runtime-backups-local.sh${profile_suffix}"
echo "runtime archive: bash bin/archive-runtime-backup-local.sh --backup-dir <path>${profile_suffix}"
echo "runtime archive prune: bash bin/prune-runtime-archives-local.sh${profile_suffix}"
echo "runtime restore preview: bash bin/preview-runtime-restore-local.sh --backup-dir <path>${profile_suffix}"
echo "runtime restore: bash bin/restore-runtime-local.sh --backup-dir <path> --expected-preview-fingerprint <previewFingerprint>${profile_suffix}"

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
