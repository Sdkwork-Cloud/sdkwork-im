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
if [[ ! -x "${binary_path}" ]]; then
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
