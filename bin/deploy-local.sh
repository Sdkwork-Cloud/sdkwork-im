#!/usr/bin/env bash
set -euo pipefail

show_help() {
  cat <<'EOF'
Usage: bash bin/deploy-local.sh [--profile <local-minimal|local-default>] [--skip-smoke] [--smoke-base-url <url>]

Start the selected Docker deployment profile with docker compose.
EOF
}

skip_smoke=0
profile_name="local-minimal"
smoke_base_url=""

while [[ $# -gt 0 ]]; do
  case "$1" in
    --profile)
      if [[ $# -lt 2 ]]; then
        echo "--profile requires a value" >&2
        exit 1
      fi
      profile_name="$2"
      shift 2
      ;;
    --skip-smoke)
      skip_smoke=1
      shift
      ;;
    --smoke-base-url)
      if [[ $# -lt 2 ]]; then
        echo "--smoke-base-url requires a value" >&2
        exit 1
      fi
      smoke_base_url="$2"
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

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

SMOKE_SCRIPT="tools/smoke/local_stack_smoke.sh"
case "$profile_name" in
  local-minimal|local-default)
    ;;
  *)
    echo "Unsupported deployment profile: ${profile_name}" >&2
    exit 1
    ;;
esac

COMPOSE_FILE="deployments/docker-compose/${profile_name}.yml"

print_compose_diagnostics() {
  echo "Collecting docker compose diagnostics for ${profile_name} profile..." >&2
  echo "Running docker compose -f \"$COMPOSE_FILE\" ps" >&2
  docker compose -f "$COMPOSE_FILE" ps || true
  echo "Running docker compose -f \"$COMPOSE_FILE\" logs --tail 200" >&2
  docker compose -f "$COMPOSE_FILE" logs --tail 200 || true
}

if ! command -v docker >/dev/null 2>&1; then
  echo "docker is unavailable. Install Docker Engine/Desktop and ensure docker is on PATH." >&2
  exit 1
fi

if ! docker info >/dev/null 2>&1; then
  echo "Docker daemon is unavailable. Start Docker and retry." >&2
  exit 1
fi

if ! docker compose version >/dev/null 2>&1; then
  echo "docker compose is unavailable. Install the Docker Compose plugin and retry." >&2
  exit 1
fi

if [[ ! -f "$COMPOSE_FILE" ]]; then
  echo "Missing compose profile: ${COMPOSE_FILE}" >&2
  exit 1
fi

echo "Building and starting ${profile_name} deployment profile with docker compose..."
if ! docker compose -f "$COMPOSE_FILE" up -d --build; then
  print_compose_diagnostics
  echo "Docker compose failed for ${profile_name} profile." >&2
  exit 1
fi

if [[ "$skip_smoke" -eq 0 ]]; then
  if [[ ! -f "$SMOKE_SCRIPT" ]]; then
    echo "Missing smoke script: ${SMOKE_SCRIPT}" >&2
    exit 1
  fi

  smoke_args=()
  if [[ -n "$smoke_base_url" ]]; then
    smoke_args+=(--base-url "$smoke_base_url")
  fi

  if ! bash "$SMOKE_SCRIPT" "${smoke_args[@]}"; then
    print_compose_diagnostics
    echo "Smoke verification failed for ${profile_name} profile." >&2
    exit 1
  fi
  exit 0
fi

echo "${profile_name} profile started without smoke verification."
