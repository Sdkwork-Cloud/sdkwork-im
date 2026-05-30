#!/usr/bin/env bash
set -euo pipefail

show_help() {
  cat <<'EOF'
Usage: bash bin/init-config-local.sh [--profile <local-minimal|local-default>] [--bind-addr <host:port>] [--force]

Create or update the selected local runtime config file.
EOF
}

profile_name="local-minimal"
bind_addr="127.0.0.1:18090"
force_mode=0

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

resolve_primary_config_file_from_profile() {
  local root_dir="$1"
  local selected_profile="$2"

  while IFS= read -r config_file; do
    printf '%s\n' "$config_file"
    return 0
  done < <(runtime_profile_config_files "$root_dir" "$selected_profile")

  printf '%s\n' "${root_dir}/.runtime/local-minimal/config/local-minimal.env"
}

generate_random_secret() {
  if command -v openssl >/dev/null 2>&1; then
    openssl rand -base64 48 | tr -d '\r\n' | tr '+/' '-_' | tr -d '='
    return
  fi

  if [[ -r /dev/urandom ]]; then
    od -An -tx1 -N32 /dev/urandom | tr -d ' \n'
    return
  fi

  echo "Unable to generate random secret" >&2
  exit 1
}

CONFIG_FILE="$(resolve_primary_config_file_from_profile "$ROOT_DIR" "$profile_name")"
CONFIG_DIR="$(dirname "$CONFIG_FILE")"
RUNTIME_DIR="$(resolve_runtime_dir_from_profile "$ROOT_DIR" "$profile_name")"
LOGS_DIR="${RUNTIME_DIR}/logs"
PIDS_DIR="${RUNTIME_DIR}/pids"
STATE_DIR="${RUNTIME_DIR}/state"

mkdir -p "$CONFIG_DIR" "$RUNTIME_DIR" "$LOGS_DIR" "$PIDS_DIR" "$STATE_DIR"

if [[ -f "$CONFIG_FILE" && "$force_mode" -ne 1 ]]; then
  echo "Config already exists: ${CONFIG_FILE}"
  exit 0
fi

friend_request_cursor_secret="$(read_config_value_from_file "$CONFIG_FILE" "CRAW_CHAT_FRIEND_REQUEST_CURSOR_HS256_SECRET" || true)"
if [[ -z "$friend_request_cursor_secret" ]]; then
  friend_request_cursor_secret="$(generate_random_secret)"
fi

cat >"$CONFIG_FILE" <<EOF
# ${profile_name} runtime config
CRAW_CHAT_BIND_ADDR=${bind_addr}
CRAW_CHAT_RUNTIME_DIR=${RUNTIME_DIR}
CRAW_CHAT_FRIEND_REQUEST_CURSOR_HS256_SECRET=${friend_request_cursor_secret}
EOF

echo "Config written: ${CONFIG_FILE}"
