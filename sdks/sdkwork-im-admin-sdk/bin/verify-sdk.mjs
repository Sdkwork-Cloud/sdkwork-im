import { existsSync, readFileSync } from 'node:fs';
import path from 'node:path';
import { assembleSdk } from './assemble-sdk.mjs';
import { verifySdkAutomation } from './verify-sdk-automation.mjs';
import { runFlutterWorkspaceVerification, verifyFlutterWorkspace } from './verify-flutter-workspace.mjs';
import {
  runTypeScriptWorkspaceVerification,
  verifyTypeScriptWorkspace,
} from './verify-typescript-workspace.mjs';

const workspaceRoot = path.resolve(import.meta.dirname, '..');
const authorityPath = path.join(workspaceRoot, 'openapi', 'im-admin.openapi.json');
const derivedPath = path.join(workspaceRoot, 'openapi', 'im-admin.sdkgen.json');
const assemblyPath = path.join(workspaceRoot, '.sdkwork-assembly.json');
const typeScriptReadmePath = path.join(
  workspaceRoot,
  'sdkwork-im-admin-sdk-typescript',
  'README.md',
);
const flutterReadmePath = path.join(
  workspaceRoot,
  'sdkwork-im-admin-sdk-flutter',
  'README.md',
);

function readJson(targetPath) {
  return JSON.parse(readFileSync(targetPath, 'utf8'));
}

function assert(condition, message) {
  if (!condition) {
    throw new Error(message);
  }
}

assembleSdk({ workspaceRoot });

const automationFailures = verifySdkAutomation({ workspaceRoot });
assert(automationFailures.length === 0, automationFailures.join('\n'));

assert(existsSync(authorityPath), `Missing authority contract: ${authorityPath}`);
assert(existsSync(derivedPath), `Missing derived sdkgen contract: ${derivedPath}`);
assert(existsSync(assemblyPath), `Missing assembly snapshot: ${assemblyPath}`);
assert(existsSync(typeScriptReadmePath), `Missing TypeScript workspace placeholder: ${typeScriptReadmePath}`);
assert(existsSync(flutterReadmePath), `Missing Flutter workspace placeholder: ${flutterReadmePath}`);

const authority = readJson(authorityPath);
const derived = readJson(derivedPath);
const assembly = readJson(assemblyPath);
const pathKeys = Object.keys(authority.paths ?? {});
const groups = new Set(
  (assembly.discoverySurface?.surfaceGroups ?? []).map((entry) => entry.operationGroup),
);

assert(authority.openapi === '3.1.0', 'Authority contract must stay on OpenAPI 3.1.0');
assert(derived.openapi === '3.1.0', 'Derived contract must stay on OpenAPI 3.1.0');
assert(
  assembly.workspace === 'sdkwork-im-admin-sdk',
  'Assembly workspace name drifted',
);
assert(
  assembly.discoverySurface?.sdkTarget === 'imAdminSdk',
  'Assembly sdkTarget drifted',
);
assert(pathKeys.length >= 20, 'IM admin authority should cover at least the current route baseline');

for (const requiredPath of [
  '/api/admin/auth/login',
  '/api/admin/auth/me',
  '/api/admin/users/operators',
  '/api/admin/tenants',
  '/api/admin/api-keys',
  '/api/admin/gateway/rate-limit-policies',
  '/api/admin/extensions/runtime-reloads',
]) {
  assert(pathKeys.includes(requiredPath), `Missing required IM admin route: ${requiredPath}`);
}

for (const requiredGroup of [
  'auth',
  'users',
  'marketing',
  'tenants',
  'access',
  'routing',
  'catalog',
  'usage',
  'billing',
  'operations',
  'storage',
]) {
  assert(groups.has(requiredGroup), `Missing required IM admin surface group: ${requiredGroup}`);
}

console.log(
  `Verified IM admin SDK authority with ${pathKeys.length} paths and ${groups.size} surface groups.`,
);

const typeScriptFailures = verifyTypeScriptWorkspace({ workspaceRoot });
assert(typeScriptFailures.length === 0, typeScriptFailures.join('\n'));
runTypeScriptWorkspaceVerification({ workspaceRoot });

const flutterFailures = verifyFlutterWorkspace({ workspaceRoot });
assert(flutterFailures.length === 0, flutterFailures.join('\n'));
runFlutterWorkspaceVerification({ workspaceRoot });
