#!/usr/bin/env bash
set -euo pipefail

show_help() {
  cat <<'EOF'
Usage: bash bin/install-server.sh [--instance <name>] [--install-root <path>] [--config-dir <path>] [--data-dir <path>] [--log-dir <path>] [--run-dir <path>] [--non-interactive] [--force]

Create the craw-chat-server install/config/data/log/run directory skeleton and stage canonical payload examples.
EOF
}

instance_name="default"
install_root="/opt/craw-chat"
config_dir="/etc/craw-chat/default"
data_dir="/var/lib/craw-chat/default"
log_dir="/var/log/craw-chat/default"
run_dir="/var/run/craw-chat/default"
non_interactive=0
force_copy=0

while [[ $# -gt 0 ]]; do
  case "$1" in
    --instance)
      instance_name="$2"
      config_dir="/etc/craw-chat/${instance_name}"
      data_dir="/var/lib/craw-chat/${instance_name}"
      log_dir="/var/log/craw-chat/${instance_name}"
      run_dir="/var/run/craw-chat/${instance_name}"
      shift 2
      ;;
    --install-root)
      install_root="$2"
      shift 2
      ;;
    --config-dir)
      config_dir="$2"
      shift 2
      ;;
    --data-dir)
      data_dir="$2"
      shift 2
      ;;
    --log-dir)
      log_dir="$2"
      shift 2
      ;;
    --run-dir)
      run_dir="$2"
      shift 2
      ;;
    --non-interactive)
      non_interactive=1
      shift
      ;;
    --force)
      force_copy=1
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
template_root="${ROOT_DIR}/deployments/templates"
storage_dir="${config_dir}/storage"
secrets_dir="${config_dir}/secrets"
install_json="${config_dir}/install.json"

mkdir -p "$install_root" "$config_dir" "$data_dir" "$log_dir" "$run_dir" "$storage_dir" "$secrets_dir"

copy_if_needed() {
  local source_path="$1"
  local dest_path="$2"
  if [[ ! -f "$dest_path" || "$force_copy" -eq 1 ]]; then
    cp "$source_path" "$dest_path"
  fi
}

copy_if_needed "${template_root}/server.yaml.example" "${config_dir}/server.yaml.example"
copy_if_needed "${template_root}/server.env.example" "${config_dir}/server.env.example"
copy_if_needed "${template_root}/postgresql.yaml.example" "${storage_dir}/postgresql.yaml.example"

cat >"$install_json" <<EOF
{
  "product": "craw-chat-server",
  "instance": "${instance_name}",
  "installRoot": "${install_root}",
  "configDir": "${config_dir}",
  "dataDir": "${data_dir}",
  "logDir": "${log_dir}",
  "runDir": "${run_dir}",
  "nonInteractive": ${non_interactive}
}
EOF

echo "Prepared craw-chat-server directories for instance '${instance_name}'."
echo "ConfigDir: ${config_dir}"
echo "DataDir: ${data_dir}"
echo "LogDir: ${log_dir}"
echo "RunDir: ${run_dir}"
