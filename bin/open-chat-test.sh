#!/usr/bin/env bash
set -euo pipefail

base_url=""
tenant_id="t_demo"
conversation_id=""
owner_user_id="u_owner"
guest_user_id="u_guest"
owner_label="owner"
guest_label="guest"
release_flag=""
skip_start="false"

usage() {
  cat <<'EOF'
Usage: bin/open-chat-test.sh [--conversation-id <id>] [--base-url <url>] [--tenant-id <id>] [--owner-user-id <id>] [--guest-user-id <id>] [--owner-label <label>] [--guest-label <label>] [--release] [--skip-start]
Create a local test conversation and open two terminal windows for manual chat validation.
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
    --owner-user-id)
      owner_user_id="$2"
      shift 2
      ;;
    --guest-user-id)
      guest_user_id="$2"
      shift 2
      ;;
    --owner-label)
      owner_label="$2"
      shift 2
      ;;
    --guest-label)
      guest_label="$2"
      shift 2
      ;;
    --release|-Release)
      release_flag="--release"
      shift
      ;;
    --skip-start)
      skip_start="true"
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

healthcheck() {
  curl --silent --fail --max-time 2 "$base_url/healthz" >/dev/null 2>&1
}

invoke_chat_cli() {
  local args=()
  if [[ -n "$release_flag" ]]; then
    args+=("$release_flag")
  fi
  args+=("$@")
  "$script_dir/chat-cli.sh" "${args[@]}"
}

open_terminal() {
  local title="$1"
  local command="$2"

  if [[ "$OSTYPE" == darwin* ]]; then
    osascript \
      -e 'tell application "Terminal" to activate' \
      -e "tell application \"Terminal\" to do script $(printf '%q' "$command")" >/dev/null
    return 0
  fi

  if command -v x-terminal-emulator >/dev/null 2>&1; then
    x-terminal-emulator -T "$title" -e bash -lc "$command" >/dev/null 2>&1 &
    return 0
  fi
  if command -v gnome-terminal >/dev/null 2>&1; then
    gnome-terminal --title="$title" -- bash -lc "$command" >/dev/null 2>&1 &
    return 0
  fi
  if command -v konsole >/dev/null 2>&1; then
    konsole --new-tab -p tabtitle="$title" -e bash -lc "$command" >/dev/null 2>&1 &
    return 0
  fi
  if command -v xfce4-terminal >/dev/null 2>&1; then
    xfce4-terminal --title="$title" --command="bash -lc '$command'" >/dev/null 2>&1 &
    return 0
  fi
  if command -v xterm >/dev/null 2>&1; then
    xterm -T "$title" -e bash -lc "$command" >/dev/null 2>&1 &
    return 0
  fi

  echo "No supported terminal emulator found." >&2
  return 1
}

if [[ -z "$conversation_id" ]]; then
  conversation_id="c_demo_$(date +%Y%m%d%H%M%S)"
fi

resolve_base_url

owner_session_id="s_${owner_user_id}"
owner_device_id="d_${owner_user_id}"
guest_session_id="s_${guest_user_id}"
guest_device_id="d_${guest_user_id}"

if [[ "$skip_start" != "true" ]] && ! healthcheck; then
  echo "Local service is not healthy. Starting local-minimal-node..."
  start_args=()
  if [[ -n "$release_flag" ]]; then
    start_args+=("$release_flag")
  fi
  "$script_dir/start-local.sh" "${start_args[@]}"
fi

if ! healthcheck; then
  echo "Chat service is not healthy at $base_url" >&2
  exit 1
fi

invoke_chat_cli \
  --base-url "$base_url" \
  --tenant-id "$tenant_id" \
  --user-id "$owner_user_id" \
  --session-id "$owner_session_id" \
  --device-id "$owner_device_id" \
  create-conversation \
  --conversation-id "$conversation_id" \
  --conversation-type group

invoke_chat_cli \
  --base-url "$base_url" \
  --tenant-id "$tenant_id" \
  --user-id "$owner_user_id" \
  --session-id "$owner_session_id" \
  --device-id "$owner_device_id" \
  add-member \
  --conversation-id "$conversation_id" \
  --principal-id "$guest_user_id" \
  --principal-kind user \
  --role member

owner_command="$(printf '%q ' "$script_dir/chat-window.sh" ${release_flag:+$release_flag} --base-url "$base_url" --tenant-id "$tenant_id" --conversation-id "$conversation_id" --user-id "$owner_user_id" --session-id "$owner_session_id" --device-id "$owner_device_id" --label "$owner_label" --message-prefix "[$owner_label] ")"
guest_command="$(printf '%q ' "$script_dir/chat-window.sh" ${release_flag:+$release_flag} --base-url "$base_url" --tenant-id "$tenant_id" --conversation-id "$conversation_id" --user-id "$guest_user_id" --session-id "$guest_session_id" --device-id "$guest_device_id" --label "$guest_label" --message-prefix "[$guest_label] ")"

open_terminal "craw-chat [$owner_label]" "$owner_command"
open_terminal "craw-chat [$guest_label]" "$guest_command"

echo "Opened two chat windows."
echo "conversationId: $conversation_id"
echo "owner: $owner_user_id"
echo "guest: $guest_user_id"
