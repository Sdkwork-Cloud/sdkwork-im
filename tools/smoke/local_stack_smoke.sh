#!/usr/bin/env bash
set -euo pipefail

show_help() {
  cat <<'EOF'
Usage: bash tools/smoke/local_stack_smoke.sh [--base-url <url>] [--public-bearer-secret <secret>] [--bearer-token <token>]

Run a minimal local-stack smoke check against the local-minimal deployment profile.
EOF
}

DEFAULT_BASE_URL="http://127.0.0.1:18090"
DEFAULT_HEALTH_URL="http://127.0.0.1:18090/healthz"
DEFAULT_DOCKER_PUBLIC_BEARER_SECRET="local-minimal-public-dev-secret"
base_url="$DEFAULT_BASE_URL"
public_bearer_secret="${CRAW_CHAT_PUBLIC_BEARER_HS256_SECRET:-}"
bearer_token="${CRAW_CHAT_SMOKE_BEARER_TOKEN:-}"
ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
LOCAL_CONFIG_FILE="${ROOT_DIR}/.runtime/local-minimal/config/local-minimal.env"

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
    --public-bearer-secret)
      if [[ $# -lt 2 ]]; then
        echo "--public-bearer-secret requires a value" >&2
        exit 1
      fi
      public_bearer_secret="$2"
      shift 2
      ;;
    --bearer-token)
      if [[ $# -lt 2 ]]; then
        echo "--bearer-token requires a value" >&2
        exit 1
      fi
      bearer_token="$2"
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

CONTENT_TYPE_HEADER="Content-Type: application/json"

read_config_value() {
  local key="$1"

  if [[ ! -f "$LOCAL_CONFIG_FILE" ]]; then
    return 1
  fi

  grep -E "^${key}=" "$LOCAL_CONFIG_FILE" \
    | tail -n 1 \
    | cut -d= -f2- \
    | tr -d '\r'
}

have_openssl() {
  command -v openssl >/dev/null 2>&1
}

base64url_encode() {
  openssl base64 -A | tr '+/' '-_' | tr -d '='
}

normalize_bearer_header() {
  local value="$1"

  if [[ "$value" == Bearer\ * || "$value" == bearer\ * ]]; then
    printf '%s' "$value"
    return
  fi

  printf 'Bearer %s' "$value"
}

resolve_public_bearer_secret() {
  local resolved="$public_bearer_secret"

  if [[ -z "$resolved" ]]; then
    resolved="$(read_config_value "CRAW_CHAT_PUBLIC_BEARER_HS256_SECRET" || true)"
  fi

  if [[ -z "$resolved" ]]; then
    resolved="$DEFAULT_DOCKER_PUBLIC_BEARER_SECRET"
  fi

  printf '%s' "$resolved"
}

generate_hs256_bearer() {
  local secret="$1"
  local header_segment payload_segment signing_input signature_segment

  if ! have_openssl; then
    echo "openssl is required to generate HS256 smoke bearer tokens. Install openssl or pass --bearer-token." >&2
    exit 1
  fi

  header_segment="$(printf '%s' '{"alg":"HS256","typ":"JWT"}' | base64url_encode)"
  payload_segment="$(printf '%s' '{"tenant_id":"t_demo","sub":"u_demo","actor_kind":"user","sid":"s_demo"}' | base64url_encode)"
  signing_input="${header_segment}.${payload_segment}"
  signature_segment="$(
    printf '%s' "$signing_input" \
      | openssl dgst -binary -sha256 -hmac "$secret" \
      | base64url_encode
  )"

  printf 'Bearer %s.%s' "$signing_input" "$signature_segment"
}

resolve_authorization_header() {
  if [[ -n "$bearer_token" ]]; then
    normalize_bearer_header "$bearer_token"
    return
  fi

  local resolved_secret
  resolved_secret="$(resolve_public_bearer_secret)"
  generate_hs256_bearer "$resolved_secret"
}

AUTHORIZATION_HEADER="Authorization: $(resolve_authorization_header)"

have_curl() {
  command -v curl >/dev/null 2>&1
}

have_wget() {
  command -v wget >/dev/null 2>&1
}

http_get() {
  local url="$1"

  if have_curl; then
    curl --fail --silent --show-error \
      -H "$AUTHORIZATION_HEADER" \
      "$url"
    return
  fi

  if have_wget; then
    wget -q -O - \
      --header="$AUTHORIZATION_HEADER" \
      "$url"
    return
  fi

  echo "Neither curl nor wget is available for smoke verification." >&2
  exit 1
}

http_post() {
  local url="$1"
  local body="$2"

  if have_curl; then
    curl --fail --silent --show-error \
      -X POST \
      -H "$AUTHORIZATION_HEADER" \
      -H "$CONTENT_TYPE_HEADER" \
      -d "$body" \
      "$url"
    return
  fi

  if have_wget; then
    wget -q -O - \
      --method=POST \
      --header="$AUTHORIZATION_HEADER" \
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

conversation_id="c_smoke_$(date +%s)_$$"

create_body="$(cat <<EOF
{"conversationId":"${conversation_id}","conversationType":"group"}
EOF
)"
http_post "${base_url}/api/v1/conversations" "$create_body" >/dev/null

message_body="$(cat <<'EOF'
{"clientMsgId":"smoke_client","summary":"smoke","text":"smoke"}
EOF
)"
http_post "${base_url}/api/v1/conversations/${conversation_id}/messages" "$message_body" >/dev/null

summary_response="$(http_get "${base_url}/api/v1/conversations/${conversation_id}")"
summary_compact="$(printf '%s' "$summary_response" | normalize_json)"
if [[ "$summary_compact" != *'"lastSummary":"smoke"'* ]]; then
  echo "Unexpected conversation summary payload" >&2
  exit 1
fi

echo "local stack smoke check passed."
