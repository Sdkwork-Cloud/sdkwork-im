#!/usr/bin/env node
import { existsSync, readFileSync, writeFileSync } from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

function fail(message) {
  console.error(`[sdkwork-craw-chat-sdk] ${message}`);
  process.exit(1);
}

function escapeRegExp(value) {
  return value.replace(/[.*+?^${}()|[\]\\]/g, '\\$&');
}

function readOverridePath(source, packageName) {
  const match = source.match(
    new RegExp(`^\\s{2}${escapeRegExp(packageName)}:\\s*\\r?\\n\\s{4}path:\\s*(.+)$`, 'm'),
  );
  return match ? match[1].trim().replace(/^['"]|['"]$/g, '') : '';
}

const scriptDir = path.dirname(fileURLToPath(import.meta.url));
const workspaceRoot = path.resolve(scriptDir, '..');
const flutterRoot = path.join(workspaceRoot, 'sdkwork-craw-chat-sdk-flutter');
const generatedDir = path.join(flutterRoot, 'generated', 'server-openapi');
const composedDir = path.join(flutterRoot, 'composed');
const generatedOverridePath = path.join(generatedDir, 'pubspec_overrides.yaml');
const composedOverridePath = path.join(composedDir, 'pubspec_overrides.yaml');

if (!existsSync(generatedOverridePath)) {
  fail('Flutter generated pubspec_overrides.yaml is missing; generate the Flutter workspace before syncing overrides.');
}

const generatedOverrideSource = readFileSync(generatedOverridePath, 'utf8');
const generatedCommonFlutterOverride = readOverridePath(generatedOverrideSource, 'sdkwork_common_flutter');
if (!generatedCommonFlutterOverride) {
  fail('Flutter generated pubspec_overrides.yaml must override sdkwork_common_flutter.');
}

const generatedCommonFlutterAbsolute = path.resolve(generatedDir, generatedCommonFlutterOverride);
const composedCommonFlutterRelative = path
  .relative(composedDir, generatedCommonFlutterAbsolute)
  .replaceAll('\\', '/');

const nextSource = `dependency_overrides:
  backend_sdk:
    path: ../generated/server-openapi
  sdkwork_common_flutter:
    path: ${composedCommonFlutterRelative}
`;

const currentSource = existsSync(composedOverridePath)
  ? readFileSync(composedOverridePath, 'utf8')
  : '';

if (currentSource !== nextSource) {
  writeFileSync(composedOverridePath, nextSource, 'utf8');
}

console.log('[sdkwork-craw-chat-sdk] Flutter composed pubspec_overrides.yaml synchronized.');
