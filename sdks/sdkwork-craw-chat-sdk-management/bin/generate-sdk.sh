#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
WORKSPACE_ROOT="$(cd "${SCRIPT_DIR}/.." && pwd)"

node "${WORKSPACE_ROOT}/bin/materialize-management-authority.mjs"
node "${WORKSPACE_ROOT}/bin/materialize-management-typescript-workspace.mjs"
node "${WORKSPACE_ROOT}/bin/materialize-management-flutter-workspace.mjs"
node "${WORKSPACE_ROOT}/bin/assemble-sdk.mjs"
node "${WORKSPACE_ROOT}/bin/verify-sdk.mjs"
node "${WORKSPACE_ROOT}/bin/verify-typescript-workspace.mjs"
