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

AUTHORIZATION_HEADER="Authorization: Bearer eyJhbGciOiJub25lIiwidHlwIjoiSldUIn0.eyJ0ZW5hbnRfaWQiOiJ0X2RlbW8iLCJzdWIiOiJ1X2RlbW8iLCJzaWQiOiJzX2RlbW8ifQ."
CONTENT_TYPE_HEADER="Content-Type: application/json"

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
