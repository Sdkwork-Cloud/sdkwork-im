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

test('admin-api package remains the dedicated /api/admin boundary while control-plane SDK stays separate', () => {
  const apiEntry = path.join(
    appRoot,
    'packages',
    'sdkwork-craw-chat-admin-admin-api',
    'src',
    'index.ts',
  );
  const rootPackageJson = path.join(appRoot, 'package.json');
  const appReadme = path.join(appRoot, 'README.md');
  const adminSdkReadme = path.resolve(
    appRoot,
    '..',
    '..',
    'sdks',
    'sdkwork-craw-chat-sdk-admin',
    'README.md',
  );

  assert.equal(existsSync(apiEntry), true);
  assert.equal(existsSync(rootPackageJson), true);
  assert.equal(existsSync(appReadme), true);
  assert.equal(existsSync(adminSdkReadme), true);

  const apiSource = readFileSync(apiEntry, 'utf8');
  const packageJsonSource = readFileSync(rootPackageJson, 'utf8');
  const appReadmeSource = readFileSync(appReadme, 'utf8');
  const adminSdkReadmeSource = readFileSync(adminSdkReadme, 'utf8');

  assert.match(apiSource, /@sdkwork\/craw-chat-sdk-management/);
  assert.match(apiSource, /@sdkwork\/craw-chat-management-backend-sdk/);
  assert.doesNotMatch(apiSource, /['"`]\/(?:auth|users|tenants|projects|api-keys|gateway|extensions)\//);
  assert.match(packageJsonSource, /sdkwork-craw-chat-admin-admin-api/);
  assert.match(appReadmeSource, /compatible management backend that serves `\/api\/admin\/\*`/);
  assert.match(appReadmeSource, /it is not a drop-in replacement for this admin app's `\/api\/admin\/\*` contract/);
  assert.match(adminSdkReadmeSource, /control-plane read surfaces/);
  assert.match(adminSdkReadmeSource, /It is not intended for:/);
  assert.match(adminSdkReadmeSource, /app-facing chat or conversation facades/);
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

test('management sdk authority snapshot formalizes the current /api/admin boundary', () => {
  const managementSdkRoot = path.resolve(
    appRoot,
    '..',
    '..',
    'sdks',
    'sdkwork-craw-chat-sdk-management',
  );
  const assemblyPath = path.join(managementSdkRoot, '.sdkwork-assembly.json');
  const authorityPath = path.join(
    managementSdkRoot,
    'openapi',
    'craw-chat-management.openapi.json',
  );
  const derivedPath = path.join(
    managementSdkRoot,
    'openapi',
    'craw-chat-management.sdkgen.json',
  );

  assert.equal(existsSync(managementSdkRoot), true);
  assert.equal(existsSync(assemblyPath), true);
  assert.equal(existsSync(authorityPath), true);
  assert.equal(existsSync(derivedPath), true);

  const assembly = JSON.parse(readFileSync(assemblyPath, 'utf8'));
  const authority = JSON.parse(readFileSync(authorityPath, 'utf8'));
  const derived = JSON.parse(readFileSync(derivedPath, 'utf8'));
  const pathKeys = Object.keys(authority.paths ?? {});
  const surfaceGroups = (assembly.discoverySurface?.surfaceGroups ?? []).map(
    (entry) => entry.operationGroup,
  );

  assert.equal(assembly.workspace, 'sdkwork-craw-chat-sdk-management');
  assert.equal(authority.openapi, '3.1.0');
  assert.equal(derived.openapi, '3.1.0');
  assert.equal(assembly.discoverySurface?.sdkTarget, 'crawChatManagementSdk');
  assert.ok(pathKeys.length >= 20, 'management authority should cover the current admin route inventory');
  assert.ok(pathKeys.includes('/api/admin/auth/login'));
  assert.ok(pathKeys.includes('/api/admin/auth/me'));
  assert.ok(pathKeys.includes('/api/admin/users/operators'));
  assert.ok(pathKeys.includes('/api/admin/tenants'));
  assert.ok(pathKeys.includes('/api/admin/projects'));
  assert.ok(pathKeys.includes('/api/admin/api-keys'));
  assert.ok(pathKeys.includes('/api/admin/gateway/rate-limit-policies'));
  assert.ok(pathKeys.includes('/api/admin/extensions/runtime-reloads'));
  assert.ok(surfaceGroups.includes('auth'));
  assert.ok(surfaceGroups.includes('users'));
  assert.ok(surfaceGroups.includes('tenants'));
  assert.ok(surfaceGroups.includes('access'));
  assert.ok(surfaceGroups.includes('catalog'));
  assert.ok(surfaceGroups.includes('operations'));
});

test('management sdk workspace reserves the standard generated and composed TypeScript package layout', () => {
  const managementSdkRoot = path.resolve(
    appRoot,
    '..',
    '..',
    'sdks',
    'sdkwork-craw-chat-sdk-management',
  );
  const generatedPackagePath = path.join(
    managementSdkRoot,
    'sdkwork-craw-chat-sdk-management-typescript',
    'generated',
    'server-openapi',
    'package.json',
  );
  const composedPackagePath = path.join(
    managementSdkRoot,
    'sdkwork-craw-chat-sdk-management-typescript',
    'composed',
    'package.json',
  );

  assert.equal(existsSync(generatedPackagePath), true);
  assert.equal(existsSync(composedPackagePath), true);

  const generatedPackage = JSON.parse(readFileSync(generatedPackagePath, 'utf8'));
  const composedPackage = JSON.parse(readFileSync(composedPackagePath, 'utf8'));

  assert.equal(generatedPackage.name, '@sdkwork/craw-chat-management-backend-sdk');
  assert.equal(composedPackage.name, '@sdkwork/craw-chat-sdk-management');
  assert.match(
    String(composedPackage.dependencies?.['@sdkwork/craw-chat-management-backend-sdk'] || ''),
    /\.\.\/generated\/server-openapi/,
  );
});

test('admin-api package depends on the management sdk boundary instead of hand-rolled transport-only HTTP', () => {
  const adminApiPackagePath = path.join(
    appRoot,
    'packages',
    'sdkwork-craw-chat-admin-admin-api',
    'package.json',
  );

  assert.equal(existsSync(adminApiPackagePath), true);

  const adminApiPackage = JSON.parse(readFileSync(adminApiPackagePath, 'utf8'));

  assert.equal(
    adminApiPackage.dependencies?.['@sdkwork/craw-chat-sdk-management'],
    'file:../../../sdks/sdkwork-craw-chat-sdk-management/sdkwork-craw-chat-sdk-management-typescript/composed',
  );
  assert.equal(
    adminApiPackage.dependencies?.['@sdkwork/craw-chat-management-backend-sdk'],
    'file:../../../sdks/sdkwork-craw-chat-sdk-management/sdkwork-craw-chat-sdk-management-typescript/generated/server-openapi',
  );
});
