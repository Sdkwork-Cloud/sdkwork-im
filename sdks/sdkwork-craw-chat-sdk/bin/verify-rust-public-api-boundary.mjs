#!/usr/bin/env node
import { readdirSync, readFileSync, statSync } from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

function collectRustFiles(rootDir) {
  const result = [];
  for (const entry of readdirSync(rootDir)) {
    const entryPath = path.join(rootDir, entry);
    const stats = statSync(entryPath);
    if (stats.isDirectory()) {
      result.push(...collectRustFiles(entryPath));
      continue;
    }
    if (entryPath.endsWith('.rs')) {
      result.push(entryPath);
    }
  }
  return result;
}

const scriptDir = path.dirname(fileURLToPath(import.meta.url));
const workspaceRoot = path.resolve(scriptDir, '..');
const composedSrcDir = path.join(
  workspaceRoot,
  'sdkwork-craw-chat-sdk-rust',
  'composed',
  'src',
);
const rustFiles = collectRustFiles(composedSrcDir);
const failures = [];

for (const filePath of rustFiles) {
  const relativePath = path.relative(workspaceRoot, filePath).replaceAll('\\', '/');
  const source = readFileSync(filePath, 'utf8');

  if (/sdkwork_craw_chat_backend_sdk::api::/.test(source)) {
    failures.push(`${relativePath} must not import generated api subpaths directly.`);
  }
  if (/sdkwork_craw_chat_backend_sdk::models::/.test(source)) {
    failures.push(`${relativePath} must not import generated models subpaths directly.`);
  }
  if (/sdkwork_craw_chat_backend_sdk::http::/.test(source)) {
    failures.push(`${relativePath} must not import generated http subpaths directly.`);
  }
  if (
    /sdkwork_craw_chat_backend_sdk::src::/.test(source) ||
    /generated\/server-openapi\/src/.test(source)
  ) {
    failures.push(`${relativePath} must consume the generated crate only through its root public exports.`);
  }
}

const clientSource = readFileSync(path.join(composedSrcDir, 'client.rs'), 'utf8');
const contextSource = readFileSync(path.join(composedSrcDir, 'context.rs'), 'utf8');

if (!/sdkwork_craw_chat_backend_sdk::\{[^}]*SdkworkBackendClient/.test(clientSource)) {
  failures.push('composed/src/client.rs must depend on the generated crate root export SdkworkBackendClient.');
}
if (!/sdkwork_craw_chat_backend_sdk::\{[^}]*SdkworkBackendClient/.test(contextSource)) {
  failures.push('composed/src/context.rs must depend on the generated crate root export SdkworkBackendClient.');
}

if (failures.length > 0) {
  console.error('[sdkwork-craw-chat-sdk] Rust public API boundary verification failed:');
  for (const failure of failures) {
    console.error(`- ${failure}`);
  }
  process.exit(1);
}

console.log('[sdkwork-craw-chat-sdk] Rust public API boundary verification passed.');
