#!/usr/bin/env bash
set -euo pipefail

release_mode=0
cli_args=()

while [[ $# -gt 0 ]]; do
  case "$1" in
    --)
      shift
      while [[ $# -gt 0 ]]; do
        cli_args+=("$1")
        shift
      done
      ;;
    --release)
      release_mode=1
      shift
      ;;
    *)
      cli_args+=("$1")
      shift
      ;;
  esac
done

script_source="${BASH_SOURCE[0]}"
script_dir="${script_source%/*}"
if [[ "$script_dir" == "$script_source" ]]; then
  script_dir="."
fi
ROOT_DIR="$(cd -- "$script_dir/.." && pwd)"
cd "$ROOT_DIR"

profile_dir="debug"
if [[ "$release_mode" -eq 1 ]]; then
  profile_dir="release"
fi

binary_path="${ROOT_DIR}/target/${profile_dir}/craw-chat-cli"
chat_cli_build_inputs=(
  "${ROOT_DIR}/Cargo.lock"
  "${ROOT_DIR}/tools/chat-cli/Cargo.toml"
)
chat_cli_source_roots=(
  "${ROOT_DIR}/tools/chat-cli/src"
)

chat_cli_binary_needs_build() {
  local input_path=""
  if [[ ! -x "${binary_path}" ]]; then
    return 0
  fi

  for input_path in "${chat_cli_build_inputs[@]}"; do
    if [[ -e "${input_path}" && "${input_path}" -nt "${binary_path}" ]]; then
      return 0
    fi
  done

  for input_path in "${chat_cli_source_roots[@]}"; do
    [[ -d "${input_path}" ]] || continue

    if ! command -v find >/dev/null 2>&1; then
      return 0
    fi

    while IFS= read -r -d '' source_path; do
      if [[ "${source_path}" -nt "${binary_path}" ]]; then
        return 0
      fi
    done < <(find "${input_path}" -type f -print0)
  done

  return 1
}

if chat_cli_binary_needs_build; then
  cargo_args=(build -p craw-chat-cli)
  if [[ "$release_mode" -eq 1 ]]; then
    cargo_args+=(--release)
  fi
  cargo "${cargo_args[@]}"
fi

if [[ ! -x "${binary_path}" ]]; then
  echo "craw-chat-cli binary was not found after build: ${binary_path}" >&2
  exit 1
fi

exec "${binary_path}" "${cli_args[@]}"
