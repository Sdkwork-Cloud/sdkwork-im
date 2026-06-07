import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const appRoot = path.resolve(__dirname, '..');
const repoRoot = path.resolve(appRoot, '..', '..');

function read(relativePath) {
  return readFileSync(path.join(repoRoot, relativePath), 'utf8');
}

const packageJsonSource = read('apps/sdkwork-chat-pc/package.json');
const tsconfigSource = read('apps/sdkwork-chat-pc/tsconfig.json');
const viteConfigSource = read('apps/sdkwork-chat-pc/vite.config.ts');
const coreIndexSource = read('apps/sdkwork-chat-pc/packages/sdkwork-clawchat-pc-core/src/index.ts');
const appbaseBackendSdkClientSource = read(
  'apps/sdkwork-chat-pc/packages/sdkwork-clawchat-pc-core/src/sdk/appbaseBackendSdkClient.ts',
);
const tenantServiceSource = read(
  'apps/sdkwork-chat-pc/packages/sdkwork-clawchat-admin-tenants/src/services/TenantService.ts',
);
const globalUserServiceSource = read(
  'apps/sdkwork-chat-pc/packages/sdkwork-clawchat-admin-tenants/src/services/GlobalUserService.ts',
);
const consoleUserServiceSource = read(
  'apps/sdkwork-chat-pc/packages/sdkwork-clawchat-console-users/src/services/UserService.ts',
);
const consoleRoleServiceSource = read(
  'apps/sdkwork-chat-pc/packages/sdkwork-clawchat-console-roles/src/services/RoleService.ts',
);

for (const [label, source] of [
  ['package.json', packageJsonSource],
  ['tsconfig.json', tsconfigSource],
  ['vite.config.ts', viteConfigSource],
]) {
  assert.match(
    source,
    /@sdkwork\/appbase-backend-sdk/u,
    `${label} must register the appbase backend dependency SDK for IAM admin capabilities.`,
  );
}

assert.match(
  appbaseBackendSdkClientSource,
  /from ['"]@sdkwork\/appbase-backend-sdk['"]/u,
  'PC core appbase backend wrapper must import the generated appbase backend SDK package.',
);
assert.match(
  appbaseBackendSdkClientSource,
  /SdkworkBackendClient/u,
  'PC core appbase backend wrapper must expose the generated SdkworkBackendClient.',
);
assert.match(
  appbaseBackendSdkClientSource,
  /getSdkworkChatGlobalTokenManager/u,
  'PC appbase backend wrapper must share the runtime global TokenManager.',
);
assert.match(
  appbaseBackendSdkClientSource,
  /createSdkworkChatRequestContextInterceptors/u,
  'PC appbase backend wrapper must attach dynamic SDKWork AppContext request interceptors.',
);
assert.doesNotMatch(
  appbaseBackendSdkClientSource,
  /\bfetch\s*\(|\b(Authorization|Access-Token|X-API-Key)\b/u,
  'PC appbase backend wrapper must not use raw HTTP or assemble auth headers manually.',
);
assert.match(
  coreIndexSource,
  /export \* from '\.\/sdk\/appbaseBackendSdkClient'/u,
  'PC core package must export the appbase backend SDK wrapper.',
);

for (const [label, source] of [
  ['tenant service', tenantServiceSource],
  ['global user service', globalUserServiceSource],
  ['console user service', consoleUserServiceSource],
  ['console role service', consoleRoleServiceSource],
]) {
  assert.match(
    source,
    /getAppbaseBackendSdkClientWithSession/u,
    `${label} must consume IAM admin data through the appbase backend SDK wrapper.`,
  );
  assert.doesNotMatch(
    source,
    /mock|setTimeout|new Promise\s*\(|\bfetch\s*\(|\b(Authorization|Access-Token|X-API-Key)\b/u,
    `${label} must not keep mock data, fake delay, raw HTTP, or manual auth header logic.`,
  );
}

assert.match(tenantServiceSource, /\.iam\.tenants\.list\s*\(/u);
assert.match(globalUserServiceSource, /\.iam\.users\.list\s*\(/u);
assert.match(globalUserServiceSource, /\.iam\.users\.update\s*\(/u);
assert.match(globalUserServiceSource, /\.iam\.users\.delete\s*\(/u);
assert.match(consoleUserServiceSource, /\.iam\.users\.list\s*\(/u);
assert.match(consoleUserServiceSource, /\.iam\.users\.delete\s*\(/u);
assert.match(consoleRoleServiceSource, /\.iam\.roles\.list\s*\(/u);
assert.match(consoleRoleServiceSource, /\.iam\.roles\.update\s*\(/u);

console.log('sdkwork chat pc appbase backend SDK integration contract passed.');
