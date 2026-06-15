#!/usr/bin/env bash
set -euo pipefail

show_help() {
  cat <<'EOF'
Usage: bash bin/install-local.sh [--profile <local-minimal|local-default>] [--release] [--bind-addr <host:port>]

Build local-minimal-node offline, initialize config, and prepare .runtime directories.
EOF
}

profile_name="local-minimal"
release_mode=0
bind_addr="127.0.0.1:18090"
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

cd "$ROOT_DIR"

if ! command -v cargo >/dev/null 2>&1; then
  echo "cargo is unavailable. Install the Rust toolchain and ensure cargo is on PATH." >&2
  exit 1
fi

init_args=(--profile "$profile_name" --bind-addr "$bind_addr")
if [[ "$bind_addr_provided" -eq 1 ]]; then
  init_args+=(--force)
fi

bash "${ROOT_DIR}/bin/init-config-local.sh" "${init_args[@]}"

SERVICE_ROOT="$(resolve_runtime_dir_from_profile "$ROOT_DIR" "$profile_name")"
LOGS_DIR="${SERVICE_ROOT}/logs"
PIDS_DIR="${SERVICE_ROOT}/pids"

mkdir -p "$SERVICE_ROOT" "$LOGS_DIR" "$PIDS_DIR"

if [[ "$release_mode" -eq 1 ]]; then
  echo "Building local-minimal-node in release mode..."
  cargo build --release -p local-minimal-node --offline
else
  echo "Building local-minimal-node in debug mode..."
  cargo build -p local-minimal-node --offline
fi

echo "Runtime directories prepared under ${SERVICE_ROOT}"
