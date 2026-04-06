#!/usr/bin/env bash
set -euo pipefail

base_url=""
tenant_id="t_demo"
conversation_id=""
user_id=""
session_id=""
device_id=""
label=""
message_prefix=""
release_flag=""

usage() {
  cat <<'EOF'
Usage: bin/chat-window.sh --conversation-id <id> --user-id <id> [--base-url <url>] [--tenant-id <id>] [--session-id <id>] [--device-id <id>] [--label <name>] [--message-prefix <prefix>] [--release]
Open one interactive chat terminal backed by bin/chat-cli.sh chat-session.
EOF
}

while [[ $# -gt 0 ]]; do
  case "$1" in
    --base-url)
      base_url="$2"
      shift 2
      ;;
    --tenant-id)
      tenant_id="$2"
      shift 2
      ;;
    --conversation-id)
      conversation_id="$2"
      shift 2
      ;;
    --user-id)
      user_id="$2"
      shift 2
      ;;
    --session-id)
      session_id="$2"
      shift 2
      ;;
    --device-id)
      device_id="$2"
      shift 2
      ;;
    --label)
      label="$2"
      shift 2
      ;;
    --message-prefix)
      message_prefix="$2"
      shift 2
      ;;
    --release|-Release)
      release_flag="--release"
      shift
      ;;
    --help|-h)
      usage
      exit 0
      ;;
    *)
      echo "Unknown argument: $1" >&2
      usage >&2
      exit 1
      ;;
  esac
done

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

read_config_value() {
  local key="$1"
  local config_file="$script_dir/../.runtime/local-minimal/config/local-minimal.env"
  [[ -f "$config_file" ]] || return 1
  grep -E "^${key}=" "$config_file" | head -n 1 | cut -d '=' -f 2-
}

resolve_base_url() {
  if [[ -n "$base_url" ]]; then
    return 0
  fi

  local bind_address
  bind_address="$(read_config_value CRAW_CHAT_BIND_ADDR || true)"
  if [[ -z "$bind_address" ]]; then
    base_url="http://127.0.0.1:18090"
    return 0
  fi

  local port="${bind_address##*:}"
  local host="${bind_address%:*}"
  if [[ -z "$host" || "$host" == "0.0.0.0" || "$host" == "::" || "$host" == "[::]" ]]; then
    host="127.0.0.1"
  fi
  base_url="http://${host}:${port}"
}

if [[ -z "$conversation_id" || -z "$user_id" ]]; then
  usage >&2
  exit 1
fi

if [[ -z "$label" ]]; then
  label="$user_id"
fi
if [[ -z "$session_id" ]]; then
  session_id="s_${user_id}"
fi
if [[ -z "$device_id" ]]; then
  device_id="d_${user_id}"
fi
if [[ -z "$message_prefix" ]]; then
  message_prefix="[$label] "
fi

resolve_base_url

echo "Opening chat session: conversation=$conversation_id user=$user_id label=$label baseUrl=$base_url"
echo "Type /quit to exit."

args=()
if [[ -n "$release_flag" ]]; then
  args+=("$release_flag")
fi
args+=(
  --base-url "$base_url"
  --tenant-id "$tenant_id"
  --user-id "$user_id"
  --session-id "$session_id"
  --device-id "$device_id"
  chat-session
  --conversation-id "$conversation_id"
  --label "$label"
)

if [[ -n "$message_prefix" ]]; then
  args+=(--message-prefix "$message_prefix")
fi

"$script_dir/chat-cli.sh" "${args[@]}"
