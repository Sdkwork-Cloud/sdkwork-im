import assert from 'node:assert/strict';
import { existsSync, readdirSync, readFileSync, statSync } from 'node:fs';
import path from 'node:path';
import test from 'node:test';

const appRoot = path.resolve(import.meta.dirname, '..');
const packagesRoot = path.join(appRoot, 'packages');
const workspaceRoot = path.resolve(appRoot, '..', '..');

function walkFiles(rootPath) {
  const entries = readdirSync(rootPath);
  const files = [];

  for (const entry of entries) {
    if (entry === 'node_modules') {
      continue;
    }

    const nextPath = path.join(rootPath, entry);
    const stat = statSync(nextPath);

    if (stat.isDirectory()) {
      files.push(...walkFiles(nextPath));
      continue;
    }

    files.push(nextPath);
  }

  return files;
}

test('formal admin SDK package is the only supported admin client boundary', () => {
  const typedAdminSdkPackage = path.join(
    workspaceRoot,
    'sdks',
    'sdkwork-craw-chat-sdk-admin',
    'sdkwork-craw-chat-sdk-admin-typescript',
    'composed',
    'package.json',
  );
  const rootPackageJson = path.join(appRoot, 'package.json');
  const legacyAdminApiManifest = path.join(
    appRoot,
    'packages',
    'sdkwork-craw-chat-admin-admin-api',
    'package.json',
  );

  assert.equal(
    existsSync(legacyAdminApiManifest),
    false,
  );
  assert.equal(existsSync(typedAdminSdkPackage), true);
  assert.equal(existsSync(rootPackageJson), true);

  const typedAdminSdkSource = readFileSync(typedAdminSdkPackage, 'utf8');
  const packageJsonSource = readFileSync(rootPackageJson, 'utf8');

  assert.match(typedAdminSdkSource, /@sdkwork\/craw-chat-admin-sdk/);
  assert.match(packageJsonSource, /@sdkwork\/craw-chat-admin-sdk/);
  assert.doesNotMatch(packageJsonSource, /sdkwork-craw-chat-admin-admin-api/);
});

test('business packages do not hardcode raw admin control-plane HTTP calls', () => {
  assert.equal(existsSync(packagesRoot), true);

  const files = walkFiles(packagesRoot).filter((filePath) => filePath.endsWith('.ts') || filePath.endsWith('.tsx'));

  for (const filePath of files) {
    const source = readFileSync(filePath, 'utf8');

    assert.doesNotMatch(source, /\/admin\/im\/v3\//, `raw admin URL in ${filePath}`);
    assert.doesNotMatch(source, /\bfetch\(/, `raw fetch in ${filePath}`);
    assert.doesNotMatch(source, /axios\./, `raw axios in ${filePath}`);
  }
});
