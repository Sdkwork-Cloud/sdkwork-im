#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
WORKSPACE_DIR="$(cd "${SCRIPT_DIR}/.." && pwd)"
GENERATOR_ROOT="$(node "${SCRIPT_DIR}/sdk-paths.mjs" --workspace "${WORKSPACE_DIR}")"
BASE_SPEC="${WORKSPACE_DIR}/openapi/craw-chat-app.openapi.yaml"
SDKGEN_SPEC="${WORKSPACE_DIR}/openapi/craw-chat-app.sdkgen.yaml"
FLUTTER_SDKGEN_SPEC="${WORKSPACE_DIR}/openapi/craw-chat-app.flutter.sdkgen.yaml"
RESOLVE_VERSION_SCRIPT="${GENERATOR_ROOT}/bin/resolve-sdk-version.js"
SDK_GENERATOR_SCRIPT="${GENERATOR_ROOT}/bin/sdkgen.js"
FLUTTER_WORKSPACE_VERIFY_SCRIPT="${SCRIPT_DIR}/verify-flutter-workspace.mjs"
TYPESCRIPT_WORKSPACE_VERIFY_SCRIPT="${SCRIPT_DIR}/verify-typescript-workspace.mjs"
TYPESCRIPT_GENERATED_BUILD_DETERMINISM_VERIFY_SCRIPT="${SCRIPT_DIR}/verify-typescript-generated-build-determinism.mjs"
NORMALIZE_GENERATED_AUTH_SURFACE_SCRIPT="${SCRIPT_DIR}/normalize-generated-auth-surface.mjs"
SDK_NAME="sdkwork-craw-chat-sdk"
SDK_TYPE="backend"
BASE_URL="http://127.0.0.1:18090"
API_PREFIX="/api/v1"
REQUESTED_VERSION=""
LANGUAGES=("typescript" "flutter")
LANGUAGES_EXPLICIT=false

fail() {
  echo "[sdkwork-craw-chat-sdk] $1" >&2
  exit 1
}

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
    *)
      fail "Unknown argument: $1"
      ;;
  esac
done

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

ASSEMBLE_ARGS=("${SCRIPT_DIR}/assemble-sdk.mjs")

for LANGUAGE in "${LANGUAGES[@]}"; do
  NORMALIZED_LANGUAGE="$(echo "${LANGUAGE}" | tr '[:upper:]' '[:lower:]')"
  case "${NORMALIZED_LANGUAGE}" in
    typescript)
      OUTPUT_DIR="${WORKSPACE_DIR}/sdkwork-craw-chat-sdk-typescript/generated/server-openapi"
      PACKAGE_NAME="@sdkwork/craw-chat-backend-sdk"
      INPUT_SPEC="${PREPARED_INPUT}"
      ;;
    flutter)
      OUTPUT_DIR="${WORKSPACE_DIR}/sdkwork-craw-chat-sdk-flutter/generated/server-openapi"
      PACKAGE_NAME="backend_sdk"
      INPUT_SPEC="${PREPARED_FLUTTER_INPUT}"
      ;;
    *)
      fail "Unsupported language: ${LANGUAGE}"
      ;;
  esac

  mkdir -p "${OUTPUT_DIR}"
  node "${SDK_GENERATOR_SCRIPT}" generate \
    --input "${INPUT_SPEC}" \
    --output "${OUTPUT_DIR}" \
    --name "${SDK_NAME}" \
    --type "${SDK_TYPE}" \
    --language "${NORMALIZED_LANGUAGE}" \
    --base-url "${BASE_URL}" \
    --api-prefix "${API_PREFIX}" \
    --fixed-sdk-version "${RESOLVED_SDK_VERSION}" \
    --sdk-root "${WORKSPACE_DIR}" \
    --sdk-name "${SDK_NAME}" \
    --package-name "${PACKAGE_NAME}"

  node "${NORMALIZE_GENERATED_AUTH_SURFACE_SCRIPT}" --language "${NORMALIZED_LANGUAGE}"

  if [[ "${NORMALIZED_LANGUAGE}" == "flutter" ]]; then
    node "${FLUTTER_WORKSPACE_VERIFY_SCRIPT}"
  fi

  if [[ "${NORMALIZED_LANGUAGE}" == "typescript" ]]; then
    node "${TYPESCRIPT_WORKSPACE_VERIFY_SCRIPT}"
    node "${TYPESCRIPT_GENERATED_BUILD_DETERMINISM_VERIFY_SCRIPT}"
  fi

  ASSEMBLE_ARGS+=(--language "${NORMALIZED_LANGUAGE}")
done

node "${ASSEMBLE_ARGS[@]}"
