#!/usr/bin/env bash

validate_runtime_profile_name() {
  local profile_name="$1"

  case "$profile_name" in
    local-minimal|local-default)
      ;;
    *)
      echo "Unsupported runtime operations profile: ${profile_name}" >&2
      return 1
      ;;
  esac
}

runtime_profile_config_files() {
  local root_dir="$1"
  local profile_name="$2"

  case "$profile_name" in
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
  local profile_name="$2"
  local config_file
  local config_runtime_dir

  while IFS= read -r config_file; do
    config_runtime_dir="$(read_config_value_from_file "$config_file" "SDKWORK_IM_RUNTIME_DIR" || true)"
    if [[ -n "$config_runtime_dir" ]]; then
      printf '%s\n' "$config_runtime_dir"
      return 0
    fi
  done < <(runtime_profile_config_files "$root_dir" "$profile_name")

  # local-default still reuses the current local-minimal runtime contract until it owns a dedicated topology.
  printf '%s\n' "${root_dir}/.runtime/local-minimal"
}

resolve_binary_path() {
  local root_dir="$1"
  local prefer_release="$2"
  local release_path="${root_dir}/target/release/local-minimal-node"
  local debug_path="${root_dir}/target/debug/local-minimal-node"
  local candidate

  if [[ "$prefer_release" -eq 1 ]]; then
    for candidate in "$release_path" "$debug_path"; do
      if [[ -x "$candidate" ]]; then
        printf '%s\n' "$candidate"
        return 0
      fi
    done
  else
    for candidate in "$debug_path" "$release_path"; do
      if [[ -x "$candidate" ]]; then
        printf '%s\n' "$candidate"
        return 0
      fi
    done
  fi

  return 1
}
