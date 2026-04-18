#!/usr/bin/env node
import { existsSync, readFileSync } from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

function fail(message) {
  console.error(`[sdkwork-craw-chat-sdk] ${message}`);
  process.exit(1);
}

function read(relativePath) {
  return readFileSync(path.join(workspaceRoot, relativePath), 'utf8');
}

function readTomlSection(source, sectionName) {
  const header = `[${sectionName}]`;
  const startIndex = source.indexOf(header);
  if (startIndex < 0) {
    return '';
  }

  const afterHeader = source.slice(startIndex + header.length);
  const nextHeaderMatch = afterHeader.match(/\r?\n\[[^\r\n]+\]/);
  if (!nextHeaderMatch || typeof nextHeaderMatch.index !== 'number') {
    return afterHeader;
  }

  return afterHeader.slice(0, nextHeaderMatch.index);
}

function readTomlString(sectionSource, key) {
  const pattern = new RegExp(`^\\s*${key}\\s*=\\s*"([^"]*)"\\s*$`, 'm');
  const match = sectionSource.match(pattern);
  return match ? match[1] : '';
}

const scriptDir = path.dirname(fileURLToPath(import.meta.url));
const workspaceRoot = path.resolve(scriptDir, '..');
const failures = [];

const requiredPaths = [
  'sdkwork-craw-chat-sdk-rust/generated/server-openapi/Cargo.toml',
  'sdkwork-craw-chat-sdk-rust/generated/server-openapi/README.md',
  'sdkwork-craw-chat-sdk-rust/generated/server-openapi/sdkwork-sdk.json',
  'sdkwork-craw-chat-sdk-rust/composed/Cargo.toml',
];

for (const relativePath of requiredPaths) {
  if (!existsSync(path.join(workspaceRoot, relativePath))) {
    failures.push(`Missing required Rust workspace file: ${relativePath}`);
  }
}

const generatedCargoSource = read('sdkwork-craw-chat-sdk-rust/generated/server-openapi/Cargo.toml');
const composedCargoSource = read('sdkwork-craw-chat-sdk-rust/composed/Cargo.toml');
const generatedReadmeSource = read('sdkwork-craw-chat-sdk-rust/generated/server-openapi/README.md');
const workspaceReadmeSource = read('sdkwork-craw-chat-sdk-rust/README.md');
const generatedSdkConfig = JSON.parse(
  read('sdkwork-craw-chat-sdk-rust/generated/server-openapi/sdkwork-sdk.json'),
);

const generatedPackageSection = readTomlSection(generatedCargoSource, 'package');
const generatedLibSection = readTomlSection(generatedCargoSource, 'lib');
const composedPackageSection = readTomlSection(composedCargoSource, 'package');
const composedLibSection = readTomlSection(composedCargoSource, 'lib');

if (readTomlString(generatedPackageSection, 'name') !== 'sdkwork-craw-chat-backend-sdk') {
  failures.push('Rust generated Cargo manifest must publish package sdkwork-craw-chat-backend-sdk.');
}
if (readTomlString(generatedLibSection, 'name') !== 'sdkwork_craw_chat_backend_sdk') {
  failures.push('Rust generated Cargo manifest must expose lib crate sdkwork_craw_chat_backend_sdk.');
}
if (readTomlString(generatedLibSection, 'path') !== 'src/lib.rs') {
  failures.push('Rust generated Cargo manifest must keep lib path src/lib.rs.');
}
if (!/^\[workspace\]\s*$/m.test(generatedCargoSource)) {
  failures.push('Rust generated Cargo manifest must declare an empty [workspace] table.');
}
if (readTomlString(composedPackageSection, 'name') !== 'craw-chat-sdk') {
  failures.push('Rust composed Cargo manifest must publish package craw-chat-sdk.');
}
if (readTomlString(composedLibSection, 'name') !== 'craw_chat_sdk') {
  failures.push('Rust composed Cargo manifest must expose lib crate craw_chat_sdk.');
}
if (readTomlString(composedLibSection, 'path') !== 'src/lib.rs') {
  failures.push('Rust composed Cargo manifest must keep lib path src/lib.rs.');
}
if (!/^\[workspace\]\s*$/m.test(composedCargoSource)) {
  failures.push('Rust composed Cargo manifest must declare an empty [workspace] table.');
}
if (generatedSdkConfig.language !== 'rust') {
  failures.push('Rust generated sdkwork-sdk.json must declare language = rust.');
}
if (generatedSdkConfig.packageName !== 'sdkwork-craw-chat-backend-sdk') {
  failures.push('Rust generated sdkwork-sdk.json must declare packageName = sdkwork-craw-chat-backend-sdk.');
}
if (!/set_auth_token/.test(generatedReadmeSource)) {
  failures.push('Rust generated README must document set_auth_token.');
}
if (!/sdk-verify/.test(workspaceReadmeSource)) {
  failures.push('Rust workspace README must reference sdk-verify.');
}
if (!/verify-rust-workspace\.mjs/.test(workspaceReadmeSource)) {
  failures.push('Rust workspace README must document verify-rust-workspace.mjs.');
}
if (!/CARGO_TARGET_DIR/.test(workspaceReadmeSource)) {
  failures.push('Rust workspace README must document the short CARGO_TARGET_DIR verification boundary.');
}

if (failures.length > 0) {
  console.error('[sdkwork-craw-chat-sdk] Rust package metadata verification failed:');
  for (const failure of failures) {
    console.error(`- ${failure}`);
  }
  process.exit(1);
}

console.log('[sdkwork-craw-chat-sdk] Rust package metadata verification passed.');
