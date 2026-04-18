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
  const typedAdminSdkPackagePath = path.join(
    workspaceRoot,
    'sdks',
    'sdkwork-craw-chat-sdk-admin',
    'sdkwork-craw-chat-sdk-admin-typescript',
    'composed',
    'package.json',
  );
  const rootPackageJsonPath = path.join(appRoot, 'package.json');
  const appReadmePath = path.join(appRoot, 'README.md');
  const adminSdkReadmePath = path.resolve(
    appRoot,
    '..',
    '..',
    'sdks',
    'sdkwork-craw-chat-sdk-admin',
    'README.md',
  );
  const adminTypeScriptReadmePath = path.resolve(
    appRoot,
    '..',
    '..',
    'sdks',
    'sdkwork-craw-chat-sdk-admin',
    'sdkwork-craw-chat-sdk-admin-typescript',
    'README.md',
  );

  assert.equal(existsSync(typedAdminSdkPackagePath), true);
  assert.equal(existsSync(rootPackageJsonPath), true);
  assert.equal(existsSync(appReadmePath), true);
  assert.equal(existsSync(adminSdkReadmePath), true);
  assert.equal(existsSync(adminTypeScriptReadmePath), true);
  assert.equal(
    existsSync(path.join(appRoot, 'packages', 'sdkwork-craw-chat-admin-admin-api', 'package.json')),
    false,
  );

  const typedAdminSdkPackage = JSON.parse(readFileSync(typedAdminSdkPackagePath, 'utf8'));
  const packageJsonSource = readFileSync(rootPackageJsonPath, 'utf8');
  const appReadmeSource = readFileSync(appReadmePath, 'utf8');
  const adminSdkReadmeSource = readFileSync(adminSdkReadmePath, 'utf8');
  const adminTypeScriptReadmeSource = readFileSync(adminTypeScriptReadmePath, 'utf8');

  assert.equal(typedAdminSdkPackage.name, '@sdkwork/craw-chat-admin-sdk');
  assert.match(packageJsonSource, /@sdkwork\/craw-chat-admin-sdk/);
  assert.doesNotMatch(packageJsonSource, /sdkwork-craw-chat-admin-admin-api/);
  assert.match(appReadmeSource, /compatible management backend that serves `\/api\/admin\/\*`/);
  assert.match(appReadmeSource, /sdks\/sdkwork-craw-chat-sdk-management/);
  assert.match(appReadmeSource, /@sdkwork\/craw-chat-admin-sdk/);
  assert.match(adminSdkReadmeSource, /browser-only operator helpers currently required by `apps\/craw-chat-admin` for `\/api\/admin\/\*`/);
  assert.match(adminSdkReadmeSource, /second handwritten admin API package/);
  assert.match(adminTypeScriptReadmeSource, /manual-owned browser helpers/);
  assert.match(adminTypeScriptReadmeSource, /target browser-facing `\/api\/admin\/\*` routes/);
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

test('management sdk remains the checked-in authority inventory for the /api/admin surface', () => {
  const appReadmePath = path.join(appRoot, 'README.md');
  const sdkIndexPath = path.resolve(appRoot, '..', '..', 'docs', 'sites', 'sdk', 'management-sdk.md');

  assert.equal(existsSync(appReadmePath), true);
  assert.equal(existsSync(sdkIndexPath), true);

  const appReadmeSource = readFileSync(appReadmePath, 'utf8');
  const managementSdkDocSource = readFileSync(sdkIndexPath, 'utf8');

  assert.match(appReadmeSource, /sdks\/sdkwork-craw-chat-sdk-management/);
  assert.match(managementSdkDocSource, /sdkwork-craw-chat-sdk-management/);
  assert.match(managementSdkDocSource, /\/api\/admin\/\*/);
});
