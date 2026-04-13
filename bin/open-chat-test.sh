#!/usr/bin/env bash
set -euo pipefail

base_url=""
tenant_id="t_demo"
conversation_id=""
owner_user_id="u_owner"
owner_login=""
owner_password=""
guest_user_id="u_guest"
guest_login=""
guest_password=""
owner_label="owner"
guest_label="guest"
release_flag=""
skip_start="false"
scripted_validation="false"
validation_message=""
json_output="false"
owner_auth_user_id=""
owner_bearer_token=""
guest_auth_user_id=""
guest_bearer_token=""
owner_cli_auth_args=()
guest_cli_auth_args=()

usage() {
  printf '%s\n' \
    "Usage: bin/open-chat-test.sh [--conversation-id <id>] [--base-url <url>] [--tenant-id <id>] [--owner-user-id <id>] [--owner-login <id>] [--owner-password <secret>] [--guest-user-id <id>] [--guest-login <id>] [--guest-password <secret>] [--owner-label <label>] [--guest-label <label>] [--release] [--skip-start] [--scripted-validation] [--validation-message <text>] [--json]" \
    "Create a local test conversation, authenticate owner and guest through real login, then either open two terminal windows or run scripted watch/timeline validation."
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
    --owner-login)
      owner_login="$2"
      shift 2
      ;;
    --owner-password)
      owner_password="$2"
      shift 2
      ;;
    --guest-user-id)
      guest_user_id="$2"
      shift 2
      ;;
    --guest-login)
      guest_login="$2"
      shift 2
      ;;
    --guest-password)
      guest_password="$2"
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
    --scripted-validation)
      scripted_validation="true"
      shift
      ;;
    --validation-message)
      validation_message="$2"
      shift 2
      ;;
    --json)
      json_output="true"
      shift
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

script_source="${BASH_SOURCE[0]}"
script_dir="${script_source%/*}"
if [[ "$script_dir" == "$script_source" ]]; then
  script_dir="."
fi
script_dir="$(cd -- "$script_dir" && pwd)"

read_config_value() {
  local key="$1"
  local config_file="$script_dir/../.runtime/local-minimal/config/local-minimal.env"
  [[ -f "$config_file" ]] || return 1
  while IFS= read -r line || [[ -n "$line" ]]; do
    line="${line%$'\r'}"
    [[ -z "$line" || "$line" == \#* ]] && continue
    local current_key="${line%%=*}"
    local current_value="${line#*=}"
    if [[ "$current_key" == "$key" ]]; then
      printf '%s\n' "$current_value"
      return 0
    fi
  done < "$config_file"
  return 1
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
  "${BASH:-bash}" "$script_dir/chat-cli.sh" "${args[@]}"
}

capture_chat_cli() {
  local args=()
  if [[ -n "$release_flag" ]]; then
    args+=("$release_flag")
  fi
  args+=("$@")

  local output
  if ! output="$("${BASH:-bash}" "$script_dir/chat-cli.sh" "${args[@]}" 2>&1)"; then
    printf '%s\n' "$output" >&2
    return 1
  fi

  printf '%s' "$output"
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

json_escape() {
  local value="${1//\\/\\\\}"
  value="${value//\"/\\\"}"
  value="${value//$'\n'/\\n}"
  value="${value//$'\r'/\\r}"
  printf '%s' "$value"
}

print_json_string_array() {
  local first="true"
  printf '['
  for value in "$@"; do
    if [[ "$first" == "true" ]]; then
      first="false"
    else
      printf ', '
    fi
    printf '"%s"' "$(json_escape "$value")"
  done
  printf ']'
}

create_temp_file() {
  if command -v mktemp >/dev/null 2>&1; then
    mktemp
    return 0
  fi

  local temp_root="${TMPDIR:-${TEMP:-${TMP:-$script_dir}}}"
  mkdir -p "$temp_root" >/dev/null 2>&1 || true
  local nonce=0
  while [[ $nonce -lt 1000 ]]; do
    local candidate="${temp_root}/craw-chat-open-chat-test-${$}-${nonce}.tmp"
    if [[ ! -e "$candidate" ]]; then
      : > "$candidate"
      printf '%s\n' "$candidate"
      return 0
    fi
    nonce=$((nonce + 1))
  done
  echo "unable to allocate temporary file path" >&2
  return 1
}

pause_seconds() {
  local seconds="$1"
  if command -v sleep >/dev/null 2>&1; then
    sleep "$seconds"
    return 0
  fi
  read -r -t "$seconds" _ || true
}

compact_timestamp() {
  local ts
  if printf -v ts '%(%Y%m%d%H%M%S)T' -1 2>/dev/null; then
    printf '%s' "$ts"
    return 0
  fi
  if command -v date >/dev/null 2>&1; then
    date +%Y%m%d%H%M%S
    return 0
  fi
  printf '%s' "$SECONDS"
}

file_contains_text() {
  local file_path="$1"
  local needle="$2"
  while IFS= read -r line || [[ -n "$line" ]]; do
    if [[ "$line" == *"$needle"* ]]; then
      return 0
    fi
  done < "$file_path"
  return 1
}

print_file_to_stderr() {
  local file_path="$1"
  [[ -f "$file_path" ]] || return 0
  while IFS= read -r line || [[ -n "$line" ]]; do
    printf '%s\n' "$line" >&2
  done < "$file_path"
}

cleanup_temp_files() {
  if command -v rm >/dev/null 2>&1; then
    rm -f "$@"
    return 0
  fi
  for file_path in "$@"; do
    if [[ -n "$file_path" && -e "$file_path" ]]; then
      : > "$file_path" || true
    fi
  done
}

normalize_json_text() {
  local raw_text="$1"
  local found_json="false"
  local line
  local normalized=""

  while IFS= read -r line || [[ -n "$line" ]]; do
    if [[ "$found_json" != "true" ]]; then
      if [[ "$line" == *"{"* ]]; then
        found_json="true"
      else
        continue
      fi
    fi
    normalized+="$line"
  done <<< "$raw_text"

  normalized="${normalized//$'\r'/}"
  normalized="${normalized//$'\n'/}"
  normalized="${normalized//$'\t'/}"
  normalized="${normalized// /}"
  printf '%s' "$normalized"
}

extract_json_string() {
  local json_text="$1"
  local key="$2"
  local compact
  compact="$(normalize_json_text "$json_text")"
  local marker="\"${key}\":\""
  if [[ "$compact" != *"$marker"* ]]; then
    return 0
  fi

  local remainder="${compact#*$marker}"
  printf '%s' "${remainder%%\"*}"
}

extract_login_user_id() {
  local json_text="$1"
  local compact
  compact="$(normalize_json_text "$json_text")"
  local user_marker='"user":{'
  if [[ "$compact" != *"$user_marker"* ]]; then
    return 0
  fi

  local user_section="${compact#*$user_marker}"
  local id_marker='"id":"'
  if [[ "$user_section" != *"$id_marker"* ]]; then
    return 0
  fi

  local remainder="${user_section#*$id_marker}"
  printf '%s' "${remainder%%\"*}"
}

resolve_seeded_im_password() {
  case "$1" in
    u_owner)
      printf '%s\n' "Owner#2026"
      ;;
    u_guest)
      printf '%s\n' "Guest#2026"
      ;;
    u_demo)
      printf '%s\n' "Demo#2026"
      ;;
    *)
      return 1
      ;;
  esac
}

resolve_im_login() {
  local requested_user_id="$1"
  local requested_login="$2"
  if [[ -n "$requested_login" ]]; then
    printf '%s\n' "$requested_login"
    return 0
  fi

  printf '%s\n' "$requested_user_id"
}

resolve_im_password() {
  local login="$1"
  local requested_password="$2"
  if [[ -n "$requested_password" ]]; then
    printf '%s\n' "$requested_password"
    return 0
  fi

  if resolve_seeded_im_password "$login"; then
    return 0
  fi

  printf '%s\n' "No password was provided for login '$login'. Supply --owner-password/--guest-password for non-seeded accounts." >&2
  return 1
}

login_im_user() {
  local requested_user_id="$1"
  local login="$2"
  local password="$3"
  local session_id="$4"
  local device_id="$5"
  local login_json
  login_json="$(
    capture_chat_cli \
      --base-url "$base_url" \
      --tenant-id "$tenant_id" \
      --user-id "$requested_user_id" \
      --session-id "$session_id" \
      --device-id "$device_id" \
      login \
      --login "$login" \
      --password "$password" \
      --client-kind im_user
  )"

  local access_token
  access_token="$(extract_json_string "$login_json" "accessToken")"
  local refresh_token
  refresh_token="$(extract_json_string "$login_json" "refreshToken")"
  local resolved_user_id
  resolved_user_id="$(extract_login_user_id "$login_json")"

  if [[ -z "$access_token" ]]; then
    printf '%s\n' "login response did not include accessToken for '$login'" >&2
    return 1
  fi

  if [[ -z "$resolved_user_id" ]]; then
    resolved_user_id="$requested_user_id"
  fi

  printf '%s\t%s\t%s\n' "$resolved_user_id" "$access_token" "$refresh_token"
}

run_scripted_validation() {
  local resolved_validation_message="$validation_message"
  if [[ -z "$resolved_validation_message" ]]; then
    resolved_validation_message="step12 scripted validation $conversation_id"
  fi

  local watch_stdout
  local watch_stderr
  watch_stdout="$(create_temp_file)"
  watch_stderr="$(create_temp_file)"

  local watch_args=()
  if [[ -n "$release_flag" ]]; then
    watch_args+=("$release_flag")
  fi
  watch_args+=("${guest_cli_auth_args[@]}")
  watch_args+=(
    watch
    --conversation-id "$conversation_id"
    --event-type message.posted
    --exit-after-events 1
    --idle-timeout-seconds 5
  )

  "${BASH:-bash}" "$script_dir/chat-cli.sh" "${watch_args[@]}" >"$watch_stdout" 2>"$watch_stderr" &
  local watch_pid=$!
  pause_seconds 1

  invoke_chat_cli \
    "${owner_cli_auth_args[@]}" \
    send-message \
    --conversation-id "$conversation_id" \
    --summary "$resolved_validation_message" \
    --text "$resolved_validation_message" \
    --client-msg-id "open_chat_test_scripted_$(compact_timestamp)" \
    >/dev/null

  local watch_exit=0
  if ! wait "$watch_pid"; then
    watch_exit=$?
  fi

  if [[ $watch_exit -ne 0 ]]; then
    echo "scripted validation watch failed" >&2
    print_file_to_stderr "$watch_stderr"
    print_file_to_stderr "$watch_stdout"
    cleanup_temp_files "$watch_stdout" "$watch_stderr"
    exit "$watch_exit"
  fi

  local watch_frame_types=()
  while IFS= read -r line || [[ -n "$line" ]]; do
    if [[ "$line" =~ \"type\"[[:space:]]*:[[:space:]]*\"([^\"]+)\" ]]; then
      watch_frame_types+=("${BASH_REMATCH[1]}")
    fi
  done < "$watch_stdout"

  if [[ ${#watch_frame_types[@]} -eq 0 ]]; then
    echo "scripted validation watch did not produce any frames" >&2
    print_file_to_stderr "$watch_stderr"
    print_file_to_stderr "$watch_stdout"
    cleanup_temp_files "$watch_stdout" "$watch_stderr"
    exit 1
  fi

  local timeline_json
  timeline_json="$(
    invoke_chat_cli \
      "${guest_cli_auth_args[@]}" \
      timeline \
      --conversation-id "$conversation_id"
  )"

  local watch_delivered="false"
  if file_contains_text "$watch_stdout" "$resolved_validation_message"; then
    watch_delivered="true"
  fi

  local timeline_contains="false"
  if [[ "$timeline_json" == *"$resolved_validation_message"* ]]; then
    timeline_contains="true"
  fi

  if [[ "$json_output" == "true" ]]; then
    printf '{\n'
    printf '  "mode": "scripted",\n'
    printf '  "conversationId": "%s",\n' "$(json_escape "$conversation_id")"
    printf '  "ownerUserId": "%s",\n' "$(json_escape "$owner_auth_user_id")"
    printf '  "guestUserId": "%s",\n' "$(json_escape "$guest_auth_user_id")"
    printf '  "validationMessage": "%s",\n' "$(json_escape "$resolved_validation_message")"
    printf '  "watchFrameTypes": '
    print_json_string_array "${watch_frame_types[@]}"
    printf ',\n'
    printf '  "watchDelivered": %s,\n' "$watch_delivered"
    printf '  "timelineContainsValidationMessage": %s\n' "$timeline_contains"
    printf '}\n'
  else
    echo "Scripted validation completed."
    echo "conversationId: $conversation_id"
    echo "validationMessage: $resolved_validation_message"
    echo "watchFrameTypes: ${watch_frame_types[*]}"
    echo "watchDelivered: $watch_delivered"
    echo "timelineContainsValidationMessage: $timeline_contains"
  fi

  cleanup_temp_files "$watch_stdout" "$watch_stderr"
}

if [[ -z "$conversation_id" ]]; then
  conversation_id="c_demo_$(compact_timestamp)"
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
  "${BASH:-bash}" "$script_dir/start-local.sh" "${start_args[@]}"
fi

if ! healthcheck; then
  echo "Chat service is not healthy at $base_url" >&2
  exit 1
fi

resolved_owner_login="$(resolve_im_login "$owner_user_id" "$owner_login")"
resolved_owner_password="$(resolve_im_password "$resolved_owner_login" "$owner_password")"
resolved_guest_login="$(resolve_im_login "$guest_user_id" "$guest_login")"
resolved_guest_password="$(resolve_im_password "$resolved_guest_login" "$guest_password")"

owner_login_result="$(
  login_im_user \
    "$owner_user_id" \
    "$resolved_owner_login" \
    "$resolved_owner_password" \
    "$owner_session_id" \
    "$owner_device_id"
)"
IFS=$'\t' read -r owner_auth_user_id owner_bearer_token _ <<< "$owner_login_result"

guest_login_result="$(
  login_im_user \
    "$guest_user_id" \
    "$resolved_guest_login" \
    "$resolved_guest_password" \
    "$guest_session_id" \
    "$guest_device_id"
)"
IFS=$'\t' read -r guest_auth_user_id guest_bearer_token _ <<< "$guest_login_result"

owner_cli_auth_args=(
  --base-url "$base_url"
  --tenant-id "$tenant_id"
  --user-id "$owner_auth_user_id"
  --session-id "$owner_session_id"
  --device-id "$owner_device_id"
  --bearer-token "$owner_bearer_token"
)
guest_cli_auth_args=(
  --base-url "$base_url"
  --tenant-id "$tenant_id"
  --user-id "$guest_auth_user_id"
  --session-id "$guest_session_id"
  --device-id "$guest_device_id"
  --bearer-token "$guest_bearer_token"
)

invoke_chat_cli \
  "${owner_cli_auth_args[@]}" \
  create-conversation \
  --conversation-id "$conversation_id" \
  --conversation-type group \
  >/dev/null

invoke_chat_cli \
  "${owner_cli_auth_args[@]}" \
  add-member \
  --conversation-id "$conversation_id" \
  --principal-id "$guest_auth_user_id" \
  --principal-kind user \
  --role member \
  >/dev/null

if [[ "$scripted_validation" == "true" ]]; then
  run_scripted_validation
  exit 0
fi

owner_command_args=("$script_dir/chat-window.sh")
if [[ -n "$release_flag" ]]; then
  owner_command_args+=("$release_flag")
fi
owner_command_args+=(
  --base-url "$base_url"
  --tenant-id "$tenant_id"
  --conversation-id "$conversation_id"
  --user-id "$owner_auth_user_id"
  --session-id "$owner_session_id"
  --device-id "$owner_device_id"
  --bearer-token "$owner_bearer_token"
  --label "$owner_label"
  --message-prefix "[$owner_label] "
)
printf -v owner_command '%q ' "${owner_command_args[@]}"

guest_command_args=("$script_dir/chat-window.sh")
if [[ -n "$release_flag" ]]; then
  guest_command_args+=("$release_flag")
fi
guest_command_args+=(
  --base-url "$base_url"
  --tenant-id "$tenant_id"
  --conversation-id "$conversation_id"
  --user-id "$guest_auth_user_id"
  --session-id "$guest_session_id"
  --device-id "$guest_device_id"
  --bearer-token "$guest_bearer_token"
  --label "$guest_label"
  --message-prefix "[$guest_label] "
)
printf -v guest_command '%q ' "${guest_command_args[@]}"

open_terminal "craw-chat [$owner_label]" "$owner_command"
open_terminal "craw-chat [$guest_label]" "$guest_command"

echo "Opened two chat windows."
echo "conversationId: $conversation_id"
echo "owner: $owner_auth_user_id"
echo "guest: $guest_auth_user_id"
