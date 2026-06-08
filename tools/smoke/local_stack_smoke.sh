#!/usr/bin/env bash
set -euo pipefail

show_help() {
  cat <<'EOF'
Usage: bash tools/smoke/local_stack_smoke.sh [--base-url <url>]

Run a minimal local-stack smoke check against the local-minimal deployment profile.
EOF
}

DEFAULT_BASE_URL="http://127.0.0.1:18090"
DEFAULT_HEALTH_URL="http://127.0.0.1:18090/healthz"
base_url="$DEFAULT_BASE_URL"

while [[ $# -gt 0 ]]; do
  case "$1" in
    --base-url)
      if [[ $# -lt 2 ]]; then
        echo "--base-url requires a value" >&2
        exit 1
      fi
      base_url="$2"
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

APP_CONTEXT_HEADERS=(
  "x-sdkwork-tenant-id: t_demo"
  "x-sdkwork-user-id: u_demo"
  "x-sdkwork-actor-id: u_demo"
  "x-sdkwork-actor-kind: user"
  "x-sdkwork-session-id: s_demo"
  "x-sdkwork-device-id: d_demo"
  "x-sdkwork-permission-scope: chat.write"
)
CONTENT_TYPE_HEADER="Content-Type: application/json"
SIGNED_APP_CONTEXT_HEADER_NAMES=(
  "x-sdkwork-app-id"
  "x-sdkwork-tenant-id"
  "x-sdkwork-organization-id"
  "x-sdkwork-user-id"
  "x-sdkwork-session-id"
  "x-sdkwork-environment"
  "x-sdkwork-deployment-mode"
  "x-sdkwork-auth-level"
  "x-sdkwork-data-scope"
  "x-sdkwork-permission-scope"
  "x-sdkwork-actor-id"
  "x-sdkwork-actor-kind"
  "x-sdkwork-device-id"
)

have_curl() {
  command -v curl >/dev/null 2>&1
}

have_wget() {
  command -v wget >/dev/null 2>&1
}

truthy() {
  local normalized=""
  normalized="$(printf '%s' "$1" | tr '[:upper:]' '[:lower:]')"
  case "$normalized" in
    1|true|yes|on)
      return 0
      ;;
    *)
      return 1
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

resolve_local_config_value() {
  local key="$1"
  local config_file=""
  for config_file in \
    ".runtime/local-minimal/config/local-minimal.env" \
    ".runtime/local-default/config/local-default.env"; do
    read_config_value_from_file "$config_file" "$key" && return 0
  done

  return 1
}

resolve_default_compose_signature_secret() {
  local compose_file="deployments/docker-compose/local-minimal.yml"
  [[ "$base_url" == "$DEFAULT_BASE_URL" && -f "$compose_file" ]] || return 1

  sed -n 's/^[[:space:]]*CRAW_CHAT_APP_CONTEXT_SIGNATURE_SECRET:[[:space:]]*//p' "$compose_file" \
    | head -n 1 \
    | tr -d '"' \
    | tr -d "'"
}

app_context_header_value() {
  local header_name="$1"
  local header=""
  local current_name=""
  local current_value=""

  for header in "${APP_CONTEXT_HEADERS[@]}"; do
    current_name="${header%%:*}"
    current_value="${header#*:}"
    current_name="$(printf '%s' "$current_name" | tr '[:upper:]' '[:lower:]')"
    if [[ "$current_name" == "$header_name" ]]; then
      printf '%s\n' "${current_value#"${current_value%%[![:space:]]*}"}"
      return 0
    fi
  done

  printf '\n'
}

canonicalize_app_context_headers() {
  local first=1
  local header_name=""

  for header_name in "${SIGNED_APP_CONTEXT_HEADER_NAMES[@]}"; do
    if [[ "$first" -eq 0 ]]; then
      printf '\n'
    fi
    first=0
    printf '%s:%s' "$header_name" "$(app_context_header_value "$header_name")"
  done
}

sign_app_context_headers() {
  local secret="$1"

  if ! command -v openssl >/dev/null 2>&1; then
    echo "openssl is required to sign SDKWork AppContext smoke headers." >&2
    exit 1
  fi

  canonicalize_app_context_headers \
    | openssl dgst -sha256 -hmac "$secret" -binary \
    | openssl base64 -A \
    | tr '+/' '-_' \
    | tr -d '='
}

configure_app_context_signature() {
  local require_signature="${CRAW_CHAT_APP_CONTEXT_REQUIRE_SIGNATURE:-}"
  local signature_secret="${CRAW_CHAT_APP_CONTEXT_SIGNATURE_SECRET:-}"

  if [[ -z "$require_signature" ]]; then
    require_signature="$(resolve_local_config_value "CRAW_CHAT_APP_CONTEXT_REQUIRE_SIGNATURE" || true)"
  fi
  if [[ -z "$signature_secret" ]]; then
    signature_secret="$(resolve_local_config_value "CRAW_CHAT_APP_CONTEXT_SIGNATURE_SECRET" || true)"
  fi
  if [[ -z "$signature_secret" ]]; then
    signature_secret="$(resolve_default_compose_signature_secret || true)"
  fi

  if [[ -n "$signature_secret" ]]; then
    APP_CONTEXT_HEADERS+=("x-sdkwork-context-signature: $(sign_app_context_headers "$signature_secret")")
    return
  fi

  if truthy "$require_signature"; then
    echo "CRAW_CHAT_APP_CONTEXT_SIGNATURE_SECRET is required when CRAW_CHAT_APP_CONTEXT_REQUIRE_SIGNATURE=true." >&2
    exit 1
  fi
}

curl_app_context_args() {
  local args=()
  local header
  for header in "${APP_CONTEXT_HEADERS[@]}"; do
    args+=("-H" "$header")
  done
  printf '%s\n' "${args[@]}"
}

wget_app_context_args() {
  local args=()
  local header
  for header in "${APP_CONTEXT_HEADERS[@]}"; do
    args+=("--header=$header")
  done
  printf '%s\n' "${args[@]}"
}

http_get() {
  local url="$1"

  if have_curl; then
    mapfile -t app_context_args < <(curl_app_context_args)
    curl --fail --silent --show-error "${app_context_args[@]}" "$url"
    return
  fi

  if have_wget; then
    mapfile -t app_context_args < <(wget_app_context_args)
    wget -q -O - "${app_context_args[@]}" "$url"
    return
  fi

  echo "Neither curl nor wget is available for smoke verification." >&2
  exit 1
}

http_post() {
  local url="$1"
  local body="$2"

  if have_curl; then
    mapfile -t app_context_args < <(curl_app_context_args)
    curl --fail --silent --show-error \
      -X POST \
      "${app_context_args[@]}" \
      -H "$CONTENT_TYPE_HEADER" \
      -d "$body" \
      "$url"
    return
  fi

  if have_wget; then
    mapfile -t app_context_args < <(wget_app_context_args)
    wget -q -O - \
      --method=POST \
      "${app_context_args[@]}" \
      --header="$CONTENT_TYPE_HEADER" \
      --body-data="$body" \
      "$url"
    return
  fi

  echo "Neither curl nor wget is available for smoke verification." >&2
  exit 1
}

wait_healthy() {
  local url="$1"
  local health_url="${url}/healthz"

  if [[ "$url" == "$DEFAULT_BASE_URL" ]]; then
    health_url="$DEFAULT_HEALTH_URL"
  fi

  for _ in $(seq 1 20); do
    if have_curl; then
      if curl --fail --silent --show-error "$health_url" >/dev/null 2>&1; then
        return
      fi
    elif have_wget; then
      if wget -q -O /dev/null "$health_url" >/dev/null 2>&1; then
        return
      fi
    else
      echo "Neither curl nor wget is available for smoke verification." >&2
      exit 1
    fi

    sleep 2
  done

  echo "Timed out waiting for ${base_url}/healthz" >&2
  exit 1
}

normalize_json() {
  tr -d '\r\n\t '
}

wait_healthy "$base_url"
configure_app_context_signature

conversation_id="c_smoke_$(date +%s)_$$"

create_body="$(cat <<EOF
{"conversationId":"${conversation_id}","conversationType":"group"}
EOF
)"
http_post "${base_url}/im/v3/api/chat/conversations" "$create_body" >/dev/null

message_body="$(cat <<'EOF'
{"clientMsgId":"smoke_client","summary":"smoke","text":"smoke"}
EOF
)"
http_post "${base_url}/im/v3/api/chat/conversations/${conversation_id}/messages" "$message_body" >/dev/null

summary_response="$(http_get "${base_url}/im/v3/api/chat/conversations/${conversation_id}")"
summary_compact="$(printf '%s' "$summary_response" | normalize_json)"
if [[ "$summary_compact" != *'"lastSummary":"smoke"'* ]]; then
  echo "Unexpected conversation summary payload" >&2
  exit 1
fi

echo "local stack smoke check passed."
