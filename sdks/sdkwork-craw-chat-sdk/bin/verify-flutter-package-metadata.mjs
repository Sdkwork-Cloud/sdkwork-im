#!/usr/bin/env node
import { readFileSync } from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const scriptDir = path.dirname(fileURLToPath(import.meta.url));
const workspaceRoot = path.resolve(scriptDir, '..');
const flutterRoot = path.join(workspaceRoot, 'sdkwork-craw-chat-sdk-flutter');

function readJson(filePath) {
  return JSON.parse(readFileSync(filePath, 'utf8'));
}

function readYamlScalar(filePath, key) {
  const source = readFileSync(filePath, 'utf8');
  const match = source.match(new RegExp(`^${key}:\\s*(.+)$`, 'm'));
  return match ? match[1].trim() : '';
}

const generatedPubspecName = readYamlScalar(
  path.join(flutterRoot, 'generated', 'server-openapi', 'pubspec.yaml'),
  'name',
);
const generatedMetadata = readJson(
  path.join(flutterRoot, 'generated', 'server-openapi', 'sdkwork-sdk.json'),
);
const composedPubspec = readFileSync(
  path.join(flutterRoot, 'composed', 'pubspec.yaml'),
  'utf8',
);
const overridePubspec = readFileSync(
  path.join(flutterRoot, 'composed', 'pubspec_overrides.yaml'),
  'utf8',
);
const generatedOverridePubspec = readFileSync(
  path.join(flutterRoot, 'generated', 'server-openapi', 'pubspec_overrides.yaml'),
  'utf8',
);

const failures = [];

function escapeRegExp(value) {
  return value.replace(/[.*+?^${}()|[\]\\]/g, '\\$&');
}

function readOverridePath(source, packageName) {
  const match = source.match(
    new RegExp(`^\\s{2}${escapeRegExp(packageName)}:\\s*\\r?\\n\\s{4}path:\\s*(.+)$`, 'm'),
  );
  return match ? match[1].trim().replace(/^['"]|['"]$/g, '') : '';
}

if (!generatedPubspecName) {
  failures.push('Flutter generated pubspec.yaml must declare a package name.');
}

if (!generatedMetadata.packageName) {
  failures.push('Flutter generated sdkwork-sdk.json must declare packageName.');
}

if (generatedPubspecName && generatedMetadata.packageName && generatedPubspecName !== generatedMetadata.packageName) {
  failures.push(
    `Flutter generated package metadata mismatch: pubspec name is "${generatedPubspecName}" but sdkwork-sdk.json packageName is "${generatedMetadata.packageName}".`,
  );
}

if (generatedPubspecName && !new RegExp(`\\n\\s{2}${generatedPubspecName}:\\s`).test(composedPubspec)) {
  failures.push(
    `Flutter composed pubspec.yaml must depend on the generated package name "${generatedPubspecName}".`,
  );
}

if (generatedPubspecName && !new RegExp(`\\n\\s{2}${generatedPubspecName}:\\s`).test(overridePubspec)) {
  failures.push(
    `Flutter pubspec_overrides.yaml must override the generated package name "${generatedPubspecName}".`,
  );
}

const generatedCommonFlutterOverride = readOverridePath(generatedOverridePubspec, 'sdkwork_common_flutter');
if (!generatedCommonFlutterOverride) {
  failures.push('Flutter generated pubspec_overrides.yaml must override sdkwork_common_flutter.');
}

const composedCommonFlutterOverride = readOverridePath(overridePubspec, 'sdkwork_common_flutter');
if (!composedCommonFlutterOverride) {
  failures.push('Flutter composed pubspec_overrides.yaml must override sdkwork_common_flutter.');
}

if (generatedCommonFlutterOverride && composedCommonFlutterOverride) {
  const generatedCommonFlutterAbsolute = path.resolve(
    flutterRoot,
    'generated',
    'server-openapi',
    generatedCommonFlutterOverride,
  );
  const composedCommonFlutterAbsolute = path.resolve(
    flutterRoot,
    'composed',
    composedCommonFlutterOverride,
  );
  if (generatedCommonFlutterAbsolute !== composedCommonFlutterAbsolute) {
    failures.push(
      `Flutter composed pubspec_overrides.yaml must point sdkwork_common_flutter to ${path.relative(path.join(flutterRoot, 'composed'), generatedCommonFlutterAbsolute).replaceAll('\\', '/')}.`,
    );
  }
}

if (failures.length > 0) {
  console.error('[sdkwork-craw-chat-sdk] Flutter package metadata verification failed:');
  for (const failure of failures) {
    console.error(`- ${failure}`);
  }
  process.exit(1);
}

console.log('[sdkwork-craw-chat-sdk] Flutter package metadata verification passed.');
