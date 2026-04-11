import assert from 'node:assert/strict';
import { existsSync, readdirSync, readFileSync, statSync } from 'node:fs';
import path from 'node:path';
import test from 'node:test';

const appRoot = path.resolve(import.meta.dirname, '..');
const packagesRoot = path.join(appRoot, 'packages');

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

test('admin-api package exists and references the IM admin SDK boundary', () => {
  const apiEntry = path.join(
    appRoot,
    'packages',
    'sdkwork-craw-chat-admin-admin-api',
    'src',
    'index.ts',
  );
  const rootPackageJson = path.join(appRoot, 'package.json');

  assert.equal(existsSync(apiEntry), true);
  assert.equal(existsSync(rootPackageJson), true);

  const apiSource = readFileSync(apiEntry, 'utf8');
  const packageJsonSource = readFileSync(rootPackageJson, 'utf8');

  assert.match(apiSource, /sdkwork-craw-chat-sdk-admin|im-admin-backend-sdk|openchat/);
  assert.match(packageJsonSource, /sdkwork-craw-chat-admin-admin-api/);
});

test('business packages do not hardcode raw admin control-plane HTTP calls', () => {
  assert.equal(existsSync(packagesRoot), true);

  const files = walkFiles(packagesRoot).filter((filePath) => filePath.endsWith('.ts') || filePath.endsWith('.tsx'));

  for (const filePath of files) {
    if (filePath.includes('sdkwork-craw-chat-admin-admin-api')) {
      continue;
    }

    const source = readFileSync(filePath, 'utf8');

    assert.doesNotMatch(source, /\/admin\/im\/v3\//, `raw admin URL in ${filePath}`);
    assert.doesNotMatch(source, /\bfetch\(/, `raw fetch in ${filePath}`);
    assert.doesNotMatch(source, /axios\./, `raw axios in ${filePath}`);
  }
});
