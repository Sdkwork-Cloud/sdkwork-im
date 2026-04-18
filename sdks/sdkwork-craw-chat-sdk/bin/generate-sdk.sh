#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
WORKSPACE_DIR="$(cd "${SCRIPT_DIR}/.." && pwd)"
APP_ROOT="$(cd "${WORKSPACE_DIR}/../.." && pwd)"
GENERATOR_ROOT="$(node "${SCRIPT_DIR}/sdk-paths.mjs" --workspace "${WORKSPACE_DIR}")"
BASE_SPEC="${WORKSPACE_DIR}/openapi/craw-chat-app.openapi.yaml"
SDKGEN_SPEC="${WORKSPACE_DIR}/openapi/craw-chat-app.sdkgen.yaml"
FLUTTER_SDKGEN_SPEC="${WORKSPACE_DIR}/openapi/craw-chat-app.flutter.sdkgen.yaml"
RESOLVE_VERSION_SCRIPT="${GENERATOR_ROOT}/bin/resolve-sdk-version.js"
SDK_GENERATOR_SCRIPT="${GENERATOR_ROOT}/bin/sdkgen.js"
REFRESH_LIVE_OPENAPI_SOURCE_SCRIPT="${SCRIPT_DIR}/refresh-live-openapi-source.mjs"
FLUTTER_WORKSPACE_VERIFY_SCRIPT="${SCRIPT_DIR}/verify-flutter-workspace.mjs"
TYPESCRIPT_WORKSPACE_VERIFY_SCRIPT="${SCRIPT_DIR}/verify-typescript-workspace.mjs"
RUST_WORKSPACE_VERIFY_SCRIPT="${SCRIPT_DIR}/verify-rust-workspace.mjs"
JAVA_WORKSPACE_VERIFY_SCRIPT="${SCRIPT_DIR}/verify-java-workspace.mjs"
CSHARP_WORKSPACE_VERIFY_SCRIPT="${SCRIPT_DIR}/verify-csharp-workspace.mjs"
SWIFT_WORKSPACE_VERIFY_SCRIPT="${SCRIPT_DIR}/verify-swift-workspace.mjs"
KOTLIN_WORKSPACE_VERIFY_SCRIPT="${SCRIPT_DIR}/verify-kotlin-workspace.mjs"
GO_WORKSPACE_VERIFY_SCRIPT="${SCRIPT_DIR}/verify-go-workspace.mjs"
PYTHON_WORKSPACE_VERIFY_SCRIPT="${SCRIPT_DIR}/verify-python-workspace.mjs"
TYPESCRIPT_GENERATED_BUILD_DETERMINISM_VERIFY_SCRIPT="${SCRIPT_DIR}/verify-typescript-generated-build-determinism.mjs"
NORMALIZE_GENERATED_AUTH_SURFACE_SCRIPT="${SCRIPT_DIR}/normalize-generated-auth-surface.mjs"
PREPARE_GENERATED_OUTPUT_SCRIPT="${SCRIPT_DIR}/prepare-generated-output.mjs"
NORMALIZE_TYPESCRIPT_GENERATED_PACKAGE_MANIFEST_SCRIPT="${SCRIPT_DIR}/normalize-typescript-generated-package-manifest.mjs"
SDK_NAME="sdkwork-craw-chat-sdk"
SDK_TYPE="backend"
BASE_URL="http://127.0.0.1:18090"
API_PREFIX="/api/v1"
SCHEMA_URL=""
KEEP_SERVICE_RUNNING=0
REQUESTED_VERSION=""
LANGUAGES=("typescript" "flutter" "rust" "java" "csharp" "swift" "kotlin" "go" "python")
LANGUAGES_EXPLICIT=false
STARTED_LOCAL_SERVICE=0
OFFICIAL_LANGUAGES=("typescript" "flutter" "rust" "java" "csharp" "swift" "kotlin" "go" "python")

fail() {
  echo "[sdkwork-craw-chat-sdk] $1" >&2
  exit 1
}

is_loopback_base_url() {
  local base_url="$1"
  local host=""

  host="$(node -e "const value=new URL(process.argv[1]); process.stdout.write(value.hostname);" "$base_url")"
  [[ "$host" == "127.0.0.1" || "$host" == "localhost" || "$host" == "0.0.0.0" || "$host" == "::1" || "$host" == "::" ]]
}

health_url_from_base_url() {
  local base_url="$1"
  node -e "const value=new URL(process.argv[1]); console.log(new URL('/healthz', value).toString());" "$base_url"
}

schema_url_from_base_url() {
  local base_url="$1"
  local explicit_schema_url="${2:-}"

  if [[ -n "$explicit_schema_url" ]]; then
    printf '%s\n' "$explicit_schema_url"
    return 0
  fi

  node -e "const value=new URL(process.argv[1]); console.log(new URL('/openapi/craw-chat-app.openapi.yaml', value).toString());" "$base_url"
}

bind_addr_from_base_url() {
  local base_url="$1"
  node -e "const value=new URL(process.argv[1]); const port=value.port || (value.protocol === 'https:' ? '443' : '80'); console.log(\`\${value.hostname}:\${port}\`);" "$base_url"
}

probe_health() {
  local health_url="$1"

  if command -v curl >/dev/null 2>&1; then
    curl --fail --silent --show-error "$health_url" >/dev/null 2>&1
    return
  fi

  if command -v wget >/dev/null 2>&1; then
    wget -q -O /dev/null "$health_url" >/dev/null 2>&1
    return
  fi

  fail "Neither curl nor wget is available for SDK generation health verification."
}

cleanup() {
  local final_exit_code="$?"

  if [[ "$STARTED_LOCAL_SERVICE" -eq 1 && "$KEEP_SERVICE_RUNNING" -ne 1 ]]; then
    if ! bash "${APP_ROOT}/bin/stop-local.sh" --profile local-minimal >/dev/null; then
      if [[ "$final_exit_code" -ne 0 ]]; then
        echo "[sdkwork-craw-chat-sdk] warning: stop-local cleanup failed after an earlier generation failure." >&2
      else
        echo "[sdkwork-craw-chat-sdk] stop-local cleanup failed." >&2
        exit 1
      fi
    fi
  fi

  exit "$final_exit_code"
}

trap cleanup EXIT

while [[ $# -gt 0 ]]; do
  case "$1" in
    --language)
      if [[ $# -lt 2 ]]; then
        fail "Missing value for --language"
      fi
      if [[ "${LANGUAGES_EXPLICIT}" == "false" ]]; then
        LANGUAGES=()
        LANGUAGES_EXPLICIT=true
      fi
      LANGUAGES+=("$2")
      shift 2
      ;;
    --requested-version)
      REQUESTED_VERSION="${2:-}"
      shift 2
      ;;
    --base-url)
      BASE_URL="${2:-}"
      shift 2
      ;;
    --api-prefix)
      API_PREFIX="${2:-}"
      shift 2
      ;;
    --schema-url)
      SCHEMA_URL="${2:-}"
      shift 2
      ;;
    --keep-service-running)
      KEEP_SERVICE_RUNNING=1
      shift
      ;;
    *)
      fail "Unknown argument: $1"
      ;;
  esac
done

HEALTH_URL="$(health_url_from_base_url "$BASE_URL")"
RESOLVED_SCHEMA_URL="$(schema_url_from_base_url "$BASE_URL" "$SCHEMA_URL")"

if ! probe_health "$HEALTH_URL"; then
  if ! is_loopback_base_url "$BASE_URL"; then
    fail "SDK generation requires a healthy service at ${HEALTH_URL}. Automatic start is only supported for local base URLs."
  fi

  bash "${APP_ROOT}/bin/start-local.sh" --profile local-minimal --bind-addr "$(bind_addr_from_base_url "$BASE_URL")"
  STARTED_LOCAL_SERVICE=1
fi

node "${REFRESH_LIVE_OPENAPI_SOURCE_SCRIPT}" \
  --schema-url "${RESOLVED_SCHEMA_URL}" \
  --output "${BASE_SPEC}" >/dev/null

PREPARED_INPUT="$(
  node "${SCRIPT_DIR}/prepare-openapi-source.mjs" \
    --base "${BASE_SPEC}" \
    --derived "${SDKGEN_SPEC}" \
    --prefer-derived
)"

PREPARED_FLUTTER_INPUT="$(
  node "${SCRIPT_DIR}/prepare-openapi-source.mjs" \
    --base "${BASE_SPEC}" \
    --derived "${FLUTTER_SDKGEN_SPEC}" \
    --prefer-derived \
    --target-language flutter
)"

AUTHORITY_VERSION="$(
  node -e 'const fs=require("node:fs"); const raw=fs.readFileSync(process.argv[1],"utf8"); const match=raw.match(/^\s{2}version:\s*["'"'"']?([^"'"'"'\n]+)["'"'"']?/m); if(!match){process.exit(1)} process.stdout.write(match[1]);' \
    "${BASE_SPEC}"
)"

if [[ -z "${REQUESTED_VERSION}" ]]; then
  REQUESTED_VERSION="${AUTHORITY_VERSION}"
fi

RESOLVED_SDK_VERSION="$(
  node "${RESOLVE_VERSION_SCRIPT}" \
    --sdk-root "${WORKSPACE_DIR}" \
    --sdk-name "${SDK_NAME}" \
    --sdk-type "${SDK_TYPE}" \
    --requested-version "${REQUESTED_VERSION}" \
    --package-name "@sdkwork/craw-chat-backend-sdk" \
    --no-sync-published-version
)"

if [[ -z "${RESOLVED_SDK_VERSION}" ]]; then
  fail "Failed to resolve SDK version"
fi

for LANGUAGE in "${LANGUAGES[@]}"; do
  NORMALIZED_LANGUAGE="$(echo "${LANGUAGE}" | tr '[:upper:]' '[:lower:]')"
  if [[ ! " ${OFFICIAL_LANGUAGES[*]} " =~ (^|[[:space:]])"${NORMALIZED_LANGUAGE}"($|[[:space:]]) ]]; then
    fail "Unsupported language: ${LANGUAGE}"
  fi
  VERIFY_SCRIPT=""
  NAMESPACE=""
  case "${NORMALIZED_LANGUAGE}" in
    typescript)
      OUTPUT_DIR="${WORKSPACE_DIR}/sdkwork-craw-chat-sdk-typescript/generated/server-openapi"
      PACKAGE_NAME="@sdkwork/craw-chat-backend-sdk"
      INPUT_SPEC="${PREPARED_INPUT}"
      VERIFY_SCRIPT="${TYPESCRIPT_WORKSPACE_VERIFY_SCRIPT}"
      ;;
    flutter)
      OUTPUT_DIR="${WORKSPACE_DIR}/sdkwork-craw-chat-sdk-flutter/generated/server-openapi"
      PACKAGE_NAME="backend_sdk"
      INPUT_SPEC="${PREPARED_FLUTTER_INPUT}"
      VERIFY_SCRIPT="${FLUTTER_WORKSPACE_VERIFY_SCRIPT}"
      ;;
    rust)
      OUTPUT_DIR="${WORKSPACE_DIR}/sdkwork-craw-chat-sdk-rust/generated/server-openapi"
      PACKAGE_NAME="sdkwork-craw-chat-backend-sdk"
      INPUT_SPEC="${PREPARED_INPUT}"
      VERIFY_SCRIPT="${RUST_WORKSPACE_VERIFY_SCRIPT}"
      ;;
    java)
      OUTPUT_DIR="${WORKSPACE_DIR}/sdkwork-craw-chat-sdk-java/generated/server-openapi"
      PACKAGE_NAME="com.sdkwork:craw-chat-backend-sdk"
      INPUT_SPEC="${PREPARED_INPUT}"
      VERIFY_SCRIPT="${JAVA_WORKSPACE_VERIFY_SCRIPT}"
      ;;
    csharp)
      OUTPUT_DIR="${WORKSPACE_DIR}/sdkwork-craw-chat-sdk-csharp/generated/server-openapi"
      PACKAGE_NAME="Sdkwork.CrawChat.BackendSdk"
      NAMESPACE="Sdkwork.CrawChat.BackendSdk"
      INPUT_SPEC="${PREPARED_INPUT}"
      VERIFY_SCRIPT="${CSHARP_WORKSPACE_VERIFY_SCRIPT}"
      ;;
    swift)
      OUTPUT_DIR="${WORKSPACE_DIR}/sdkwork-craw-chat-sdk-swift/generated/server-openapi"
      PACKAGE_NAME="CrawChatBackendSdk"
      INPUT_SPEC="${PREPARED_INPUT}"
      VERIFY_SCRIPT="${SWIFT_WORKSPACE_VERIFY_SCRIPT}"
      ;;
    kotlin)
      OUTPUT_DIR="${WORKSPACE_DIR}/sdkwork-craw-chat-sdk-kotlin/generated/server-openapi"
      PACKAGE_NAME="com.sdkwork:craw-chat-backend-sdk"
      INPUT_SPEC="${PREPARED_INPUT}"
      VERIFY_SCRIPT="${KOTLIN_WORKSPACE_VERIFY_SCRIPT}"
      ;;
    go)
      OUTPUT_DIR="${WORKSPACE_DIR}/sdkwork-craw-chat-sdk-go/generated/server-openapi"
      PACKAGE_NAME="github.com/sdkwork/craw-chat-backend-sdk"
      INPUT_SPEC="${PREPARED_INPUT}"
      VERIFY_SCRIPT="${GO_WORKSPACE_VERIFY_SCRIPT}"
      ;;
    python)
      OUTPUT_DIR="${WORKSPACE_DIR}/sdkwork-craw-chat-sdk-python/generated/server-openapi"
      PACKAGE_NAME="sdkwork-craw-chat-backend-sdk"
      INPUT_SPEC="${PREPARED_INPUT}"
      VERIFY_SCRIPT="${PYTHON_WORKSPACE_VERIFY_SCRIPT}"
      ;;
    *)
      fail "Generation mapping not implemented yet for official language: ${LANGUAGE}"
      ;;
  esac

  node "${PREPARE_GENERATED_OUTPUT_SCRIPT}" --language "${NORMALIZED_LANGUAGE}"
  mkdir -p "${OUTPUT_DIR}"
  GENERATOR_ARGS=(
    "${SDK_GENERATOR_SCRIPT}"
    generate
    --input "${INPUT_SPEC}"
    --output "${OUTPUT_DIR}"
    --name "${SDK_NAME}"
    --type "${SDK_TYPE}"
    --language "${NORMALIZED_LANGUAGE}"
    --base-url "${BASE_URL}"
    --api-prefix "${API_PREFIX}"
    --fixed-sdk-version "${RESOLVED_SDK_VERSION}"
    --sdk-root "${WORKSPACE_DIR}"
    --sdk-name "${SDK_NAME}"
    --package-name "${PACKAGE_NAME}"
  )
  if [[ -n "${NAMESPACE}" ]]; then
    GENERATOR_ARGS+=(--namespace "${NAMESPACE}")
  fi
  node "${GENERATOR_ARGS[@]}"

  node "${NORMALIZE_GENERATED_AUTH_SURFACE_SCRIPT}" --language "${NORMALIZED_LANGUAGE}"

  if [[ "${NORMALIZED_LANGUAGE}" == "typescript" ]]; then
    node "${NORMALIZE_TYPESCRIPT_GENERATED_PACKAGE_MANIFEST_SCRIPT}"
  fi

  if [[ -n "${VERIFY_SCRIPT}" ]]; then
    node "${VERIFY_SCRIPT}"
  fi

  if [[ "${NORMALIZED_LANGUAGE}" == "typescript" ]]; then
    node "${TYPESCRIPT_GENERATED_BUILD_DETERMINISM_VERIFY_SCRIPT}"
  fi
done

node "${SCRIPT_DIR}/assemble-sdk.mjs"
