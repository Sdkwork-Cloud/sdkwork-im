#!/usr/bin/env bash
set -euo pipefail

show_help() {
  cat <<'EOF'
Usage: bash bin/init-config-server.sh [--instance <name>] [--config-dir <path>] [--data-dir <path>] [--log-dir <path>] [--run-dir <path>] [--bind-address <host:port>] [--base-url <url>] [--api-base-url <url>] [--websocket-base-url <url>] [--non-interactive] [--force]

Render craw-chat-server configuration files for the selected instance and preserve file-based PostgreSQL settings.
EOF
}

instance_name="default"
config_dir="/etc/craw-chat/default"
data_dir="/var/lib/craw-chat/default"
log_dir="/var/log/craw-chat/default"
run_dir="/var/run/craw-chat/default"
bind_address="0.0.0.0:18080"
base_url="http://127.0.0.1:18080"
api_base_url="http://127.0.0.1:18080"
websocket_base_url="ws://127.0.0.1:18080/api/v1/realtime/ws"
non_interactive=0
force_write=0

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
    --bind-address)
      bind_address="$2"
      shift 2
      ;;
    --base-url)
      base_url="$2"
      shift 2
      ;;
    --api-base-url)
      api_base_url="$2"
      shift 2
      ;;
    --websocket-base-url)
      websocket_base_url="$2"
      shift 2
      ;;
    --non-interactive)
      non_interactive=1
      shift
      ;;
    --force)
      force_write=1
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

storage_dir="${config_dir}/storage"
secrets_dir="${config_dir}/secrets"
server_yaml="${config_dir}/server.yaml"
server_env="${config_dir}/server.env"
postgresql_yaml="${storage_dir}/postgresql.yaml"
password_file="${secrets_dir}/postgresql.password"

mkdir -p "$config_dir" "$data_dir" "$log_dir" "$run_dir" "$storage_dir" "$secrets_dir"

write_if_needed() {
  local path="$1"
  local content="$2"
  if [[ ! -f "$path" || "$force_write" -eq 1 ]]; then
    printf '%s' "$content" >"$path"
  fi
}

write_if_needed "$server_yaml" "instance:
  name: \"${instance_name}\"

network:
  bindAddress: \"${bind_address}\"

publicEndpoints:
  baseUrl: \"${base_url}\"
  apiBaseUrl: \"${api_base_url}\"
  websocketBaseUrl: \"${websocket_base_url}\"
  docsBaseUrl: \"${base_url}/docs\"

runtime:
  configDir: \"${config_dir}\"
  dataDir: \"${data_dir}\"
  logDir: \"${log_dir}\"
  runDir: \"${run_dir}\"

storage:
  postgresqlConfig: \"${postgresql_yaml}\"
"

write_if_needed "$server_env" "CRAW_CHAT_SERVER_INSTANCE=${instance_name}
CRAW_CHAT_SERVER_CONFIG_DIR=${config_dir}
CRAW_CHAT_SERVER_DATA_DIR=${data_dir}
CRAW_CHAT_SERVER_LOG_DIR=${log_dir}
CRAW_CHAT_SERVER_RUN_DIR=${run_dir}
CRAW_CHAT_SERVER_BIND_ADDRESS=${bind_address}
CRAW_CHAT_SERVER_BASE_URL=${base_url}
CRAW_CHAT_SERVER_API_BASE_URL=${api_base_url}
CRAW_CHAT_SERVER_WEBSOCKET_BASE_URL=${websocket_base_url}
"

write_if_needed "$postgresql_yaml" "provider: postgresql

connection:
  host: 127.0.0.1
  port: 5432
  database: craw_chat
  username: craw_chat_app
  passwordFile: \"${password_file}\"
  sslmode: prefer
  applicationName: craw-chat-server
  connectTimeoutSeconds: 10

schema:
  name: craw_chat
  provisioningMode: none
  migrationMode: apply
  expectedVersion: latest

pool:
  minConnections: 5
  maxConnections: 30
  idleTimeoutSeconds: 300
  maxLifetimeSeconds: 1800

# init-storage-server modes:
# - verify-only
# - bootstrap-schema
# - create-db-and-schema
"

if [[ ! -f "$password_file" || "$force_write" -eq 1 ]]; then
  printf '%s\n' "replace-me" >"$password_file"
fi

echo "Rendered craw-chat-server configuration for instance '${instance_name}'."
echo "server.yaml: ${server_yaml}"
echo "server.env: ${server_env}"
echo "storage/postgresql.yaml: ${postgresql_yaml}"
