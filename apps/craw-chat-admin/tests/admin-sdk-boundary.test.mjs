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

test('control-plane SDK package is the supported admin app client boundary', () => {
  const typedAdminSdkPackagePath = path.join(
    workspaceRoot,
    'sdks',
    'sdkwork-control-plane-sdk',
    'sdkwork-control-plane-sdk-typescript',
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
    'sdkwork-control-plane-sdk',
    'README.md',
  );
  const adminTypeScriptReadmePath = path.resolve(
    appRoot,
    '..',
    '..',
    'sdks',
    'sdkwork-control-plane-sdk',
    'sdkwork-control-plane-sdk-typescript',
    'README.md',
  );

  assert.equal(existsSync(typedAdminSdkPackagePath), true);
  assert.equal(existsSync(rootPackageJsonPath), true);
  assert.equal(existsSync(appReadmePath), true);
  assert.equal(existsSync(adminSdkReadmePath), true);
  assert.equal(existsSync(adminTypeScriptReadmePath), true);
  assert.equal(
    existsSync(path.join(appRoot, 'packages', 'sdkwork-control-plane-admin-api', 'package.json')),
    false,
  );

  const typedAdminSdkPackage = JSON.parse(readFileSync(typedAdminSdkPackagePath, 'utf8'));
  const packageJsonSource = readFileSync(rootPackageJsonPath, 'utf8');
  const appReadmeSource = readFileSync(appReadmePath, 'utf8');
  const adminSdkReadmeSource = readFileSync(adminSdkReadmePath, 'utf8');
  const adminTypeScriptReadmeSource = readFileSync(adminTypeScriptReadmePath, 'utf8');

  assert.equal(typedAdminSdkPackage.name, '@sdkwork/control-plane-sdk');
  assert.match(packageJsonSource, /@sdkwork\/control-plane-sdk/);
  assert.doesNotMatch(packageJsonSource, /@sdkwork\/im-admin-sdk/);
  assert.doesNotMatch(packageJsonSource, /sdkwork-control-plane-admin-api/);
  assert.match(appReadmeSource, /compatible admin backend that serves `\/api\/admin\/\*`/);
  assert.match(appReadmeSource, /sdks\/sdkwork-control-plane-sdk/);
  assert.match(appReadmeSource, /sdks\/sdkwork-im-admin-sdk/);
  assert.match(appReadmeSource, /@sdkwork\/control-plane-sdk/);
  assert.match(adminSdkReadmeSource, /sdkwork-control-plane-sdk/);
  assert.match(adminSdkReadmeSource, /@sdkwork\/control-plane-sdk/);
  assert.match(adminTypeScriptReadmeSource, /ControlPlaneSdkClient/);
  assert.match(adminTypeScriptReadmeSource, /@sdkwork\/control-plane-backend-sdk/);
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

test('im admin sdk authority snapshot formalizes the current /api/admin boundary', () => {
  const imAdminSdkRoot = path.resolve(
    appRoot,
    '..',
    '..',
    'sdks',
    'sdkwork-im-admin-sdk',
  );
  const assemblyPath = path.join(imAdminSdkRoot, '.sdkwork-assembly.json');
  const authorityPath = path.join(
    imAdminSdkRoot,
    'openapi',
    'im-admin.openapi.json',
  );
  const derivedPath = path.join(
    imAdminSdkRoot,
    'openapi',
    'im-admin.sdkgen.json',
  );

  assert.equal(existsSync(imAdminSdkRoot), true);
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

  assert.equal(assembly.workspace, 'sdkwork-im-admin-sdk');
  assert.equal(authority.openapi, '3.1.0');
  assert.equal(derived.openapi, '3.1.0');
  assert.equal(assembly.discoverySurface?.sdkTarget, 'imAdminSdk');
  assert.ok(pathKeys.length >= 20, 'IM admin authority should cover the current admin route inventory');
  assert.ok(pathKeys.includes('/api/admin/auth/login'));
  assert.ok(pathKeys.includes('/api/admin/auth/me'));
  assert.ok(pathKeys.includes('/api/admin/users/operators'));
  assert.ok(pathKeys.includes('/api/admin/tenants'));
  assert.ok(pathKeys.includes('/api/admin/projects'));
  assert.ok(pathKeys.includes('/api/admin/api-keys'));
  assert.ok(pathKeys.includes('/api/admin/gateway/rate-limit-policies'));
  assert.ok(pathKeys.includes('/api/admin/extensions/runtime-reloads'));
  assert.ok(pathKeys.includes('/api/admin/storage/providers'));
  assert.ok(surfaceGroups.includes('auth'));
  assert.ok(surfaceGroups.includes('users'));
  assert.ok(surfaceGroups.includes('marketing'));
  assert.ok(surfaceGroups.includes('tenants'));
  assert.ok(surfaceGroups.includes('access'));
  assert.ok(surfaceGroups.includes('routing'));
  assert.ok(surfaceGroups.includes('catalog'));
  assert.ok(surfaceGroups.includes('usage'));
  assert.ok(surfaceGroups.includes('billing'));
  assert.ok(surfaceGroups.includes('operations'));
  assert.ok(surfaceGroups.includes('storage'));
});

test('im admin sdk workspace reserves the standard generated and composed TypeScript package layout', () => {
  const imAdminSdkRoot = path.resolve(
    appRoot,
    '..',
    '..',
    'sdks',
    'sdkwork-im-admin-sdk',
  );
  const generatedPackagePath = path.join(
    imAdminSdkRoot,
    'sdkwork-im-admin-sdk-typescript',
    'generated',
    'server-openapi',
    'package.json',
  );
  const composedPackagePath = path.join(
    imAdminSdkRoot,
    'sdkwork-im-admin-sdk-typescript',
    'composed',
    'package.json',
  );

  assert.equal(existsSync(generatedPackagePath), true);
  assert.equal(existsSync(composedPackagePath), true);

  const generatedPackage = JSON.parse(readFileSync(generatedPackagePath, 'utf8'));
  const composedPackage = JSON.parse(readFileSync(composedPackagePath, 'utf8'));

  assert.equal(generatedPackage.name, '@sdkwork/im-admin-backend-sdk');
  assert.equal(composedPackage.name, '@sdkwork/im-admin-sdk');
  assert.match(
    String(composedPackage.dependencies?.['@sdkwork/im-admin-backend-sdk'] || ''),
    /\.\.\/generated\/server-openapi/,
  );
});

test('im admin sdk remains the checked-in authority inventory for the /api/admin surface', () => {
  const appReadmePath = path.join(appRoot, 'README.md');
  const sdkReadmePath = path.resolve(appRoot, '..', '..', 'sdks', 'sdkwork-im-admin-sdk', 'README.md');

  assert.equal(existsSync(appReadmePath), true);
  assert.equal(existsSync(sdkReadmePath), true);

  const appReadmeSource = readFileSync(appReadmePath, 'utf8');
  const sdkReadmeSource = readFileSync(sdkReadmePath, 'utf8');

  assert.match(appReadmeSource, /sdks\/sdkwork-im-admin-sdk/);
  assert.match(sdkReadmeSource, /sdkwork-im-admin-sdk/);
  assert.match(sdkReadmeSource, /\/api\/admin\/\*/);
});
