import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const repoRoot = path.resolve(__dirname, '..', '..');

function read(relativePath) {
  return fs.readFileSync(path.join(repoRoot, relativePath), 'utf8');
}

function readJson(relativePath) {
  return JSON.parse(read(relativePath));
}

const localSpec = read('specs/im-app-api-sdk-integration.spec.md');
const specsReadme = read('specs/README.md');
const componentSpec = readJson('specs/component.spec.json');
const appPackageJson = readJson('apps/sdkwork-chat-pc/package.json');
const appSdkClientSource = read('apps/sdkwork-chat-pc/packages/sdkwork-clawchat-pc-core/src/sdk/appSdkClient.ts');
const appAuthRuntimeSource = read('apps/sdkwork-chat-pc/packages/sdkwork-clawchat-pc-core/src/sdk/appAuthRuntime.ts');
const imSdkClientSource = read('apps/sdkwork-chat-pc/packages/sdkwork-clawchat-pc-core/src/sdk/imSdkClient.ts');
const appAuthServiceSource = read('apps/sdkwork-chat-pc/packages/sdkwork-clawchat-pc-core/src/sdk/appAuthService.ts');
const viteConfigSource = read('apps/sdkwork-chat-pc/vite.config.ts');
const localApiSource = read('apps/sdkwork-chat-pc/local-api.ts');
const devCommandSource = read('scripts/dev/run-sdkwork-chat-pc-dev.mjs');
const localAppApiSource = read('scripts/dev/start-craw-chat-local-app-api.mjs');
const sharedDatabaseSource = read('scripts/dev/craw-chat-shared-database.mjs');
const releaseSources = readJson('config/shared-sdk-release-sources.json');
const workspaceSource = read('apps/sdkwork-chat-pc/pnpm-workspace.yaml');

assert.match(
  specsReadme,
  /im-app-api-sdk-integration\.spec\.md/u,
  'Craw Chat specs README must register the local IM app API and SDK integration standard.',
);
assert.ok(
  componentSpec.localExtensionSpecs?.some((entry) => entry.file === 'im-app-api-sdk-integration.spec.md'),
  'component.spec.json must register the local IM app API and SDK integration standard.',
);

for (const requiredText of [
  'IM API',
  'im-open-api',
  '/im/v3/api',
  'IM App API',
  '/app/v3/api',
  'IM Backend API',
  '/backend/v3/api',
  'sdkwork-im-app-sdk',
  'SdkworkImAppClient',
  'sdkwork-im-backend-sdk',
  'SdkworkImBackendClient',
  '@sdkwork/im-sdk',
  'openPlatform.qrAuth.sessions.create',
  'openPlatform.qrAuth.sessions.retrieve',
  'scans.create',
  'passwords.create',
  'sqlite',
  'PostgreSQL',
  'SDKWORK_SHARED_SDK_MODE=git',
]) {
  assert.match(localSpec, new RegExp(requiredText.replace(/[.*+?^${}()|[\]\\]/g, '\\$&'), 'u'));
}

assert.match(
  localSpec,
  /must not import `@sdkwork\/app-sdk`, `@sdkwork\/backend-sdk`, `spring-ai-plus-app-api`, or `spring-ai-plus-backend-api`/iu,
  'local standard must explicitly forbid generic app/backend SDK imports for Craw Chat.',
);
assert.match(
  localSpec,
  /After login, chat and RTC code must be able to call IM SDK methods without a second login/iu,
  'local standard must require login-to-IM session continuity.',
);
assert.match(
  localSpec,
  /must not pass unsupported runtime-config fields such as `qrLoginType`/iu,
  'local standard must document that current canonical appbase auth config only accepts qrLoginEnabled.',
);

assert.ok(appPackageJson.dependencies['@sdkwork-internal/im-app-api-generated']);
assert.ok(appPackageJson.dependencies['@sdkwork-internal/im-backend-api-generated']);
assert.ok(!appPackageJson.dependencies['@sdkwork/app-sdk']);
assert.ok(!appPackageJson.dependencies['@sdkwork/backend-sdk']);

assert.match(appSdkClientSource, /SdkworkImAppClient/u);
assert.match(appSdkClientSource, /@sdkwork-internal\/im-app-api-generated/u);
assert.doesNotMatch(
  appSdkClientSource,
  /@sdkwork\/(?:app|backend)-sdk|spring-ai-plus-(?:app|backend)-api/u,
);

assert.match(appAuthServiceSource, /createIamAppSdkAdapter/u);
assert.match(appAuthServiceSource, /openPlatform\.qrAuth\.sessions\.create/u);
assert.match(appAuthServiceSource, /openPlatform\.qrAuth\.sessions\.retrieve/u);
assert.match(appAuthServiceSource, /openPlatform\.qrAuth\.sessions\.scans\.create/u);
assert.match(appAuthServiceSource, /openPlatform\.qrAuth\.sessions\.passwords\.create/u);
assert.doesNotMatch(appAuthServiceSource, /\bfetch\s*\(/u);

assert.match(appAuthRuntimeSource, /loginMethods:\s*\[\s*['"]password['"]\s*\]/u);
assert.match(appAuthRuntimeSource, /qrLoginEnabled:\s*true/u);
assert.doesNotMatch(
  appAuthRuntimeSource,
  /qrLoginType/u,
  'Craw Chat runtime config must not pass qrLoginType until canonical @sdkwork/auth-pc-react exposes it.',
);

for (const requiredImSessionFragment of [
  'tokenProvider: tokenManager',
  'accessToken: resolveAppSdkAccessToken',
  'headerProvider: () => buildImSdkContextHeaders',
  'X-Sdkwork-Session-Id',
  'ImWebSocketAuthOptions.automatic',
]) {
  assert.match(
    imSdkClientSource,
    new RegExp(requiredImSessionFragment.replace(/[.*+?^${}()|[\]\\]/g, '\\$&'), 'u'),
    `IM SDK client must preserve IAM session continuity via ${requiredImSessionFragment}`,
  );
}
assert.doesNotMatch(
  imSdkClientSource,
  /tenantId:\s*resolveAppSdkTenantId|organizationId:\s*resolveAppSdkOrganizationId/u,
  'IM SDK client must not pass current tenantId/organizationId as static options; JWT-backed Context belongs in the request headerProvider.',
);
assert.doesNotMatch(imSdkClientSource, /\bfetch\s*\(/u);

assert.match(viteConfigSource, /sdkworkChatLocalApiPlugin/u);
assert.match(viteConfigSource, /handleSdkworkChatLocalApiRequest/u);
for (const localShellEndpoint of ['/api/config/modules', '/api/agent/doc', '/api/agent/icon']) {
  assert.match(localSpec, new RegExp(localShellEndpoint.replace(/[.*+?^${}()|[\]\\]/g, '\\$&'), 'u'));
  assert.match(localApiSource, new RegExp(localShellEndpoint.replace(/[.*+?^${}()|[\]\\]/g, '\\$&'), 'u'));
}
assert.match(
  localSpec,
  /must be classified under `im-app-api`, added to `\/app\/v3\/api` OpenAPI, regenerated through `sdkwork-im-app-sdk`/u,
  'local shell endpoints must have an explicit im-app-api OpenAPI promotion rule.',
);
assert.match(
  localSpec,
  /Local PC development exposes one public backend entrypoint, `http:\/\/127\.0\.0\.1:18079`/u,
  'local standard must require the PC renderer to use the unified gateway instead of the internal appbase port.',
);

assert.match(localAppApiSource, /Rust unified server/u);
assert.match(localAppApiSource, /resolveCrawChatSharedDatabaseConfig/u);
assert.match(sharedDatabaseSource, /\.sdkwork/u);
assert.match(sharedDatabaseSource, /APP_CODE\s*=\s*['"]chat['"]/u);
assert.match(sharedDatabaseSource, /\$\{APP_CODE\}\.sqlite/u);
assert.match(sharedDatabaseSource, /SDKWORK_CHAT_DATABASE_ENGINE/u);
assert.match(sharedDatabaseSource, /SDKWORK_CHAT_DATABASE_SSL_MODE/u);
assert.match(sharedDatabaseSource, /postgres(?:ql)?:/u);
assert.match(devCommandSource, /resolveCrawChatSharedDatabaseConfig/u);
assert.match(devCommandSource, /CRAW_CHAT_WEB_GATEWAY_RUNTIME_MODE:\s*['"]embedded['"]/u);
assert.doesNotMatch(
  `${devCommandSource}\n${localAppApiSource}`,
  /\bmvn(?:\.cmd)?\b|spring-ai-plus-server-app|spring-boot:run|CRAW_CHAT_APPBASE_APP_API_UPSTREAM|SDKWORK_APPBASE_APP_API_BIND_ADDR|SDKWORK_APPBASE_BROWSER_ORIGINS/u,
  'local PC development must use the Rust unified server for app-api instead of Java/appbase upstream startup.',
);

for (const sourceName of [
  'sdkwork-im-app-sdk',
  'sdkwork-im-backend-sdk',
  'sdkwork-im-sdk',
  'sdkwork-appbase',
  'sdkwork-core',
  'sdkwork-ui',
  'sdkwork-claw-router',
  'sdkwork-birdcoder',
]) {
  assert.ok(releaseSources.sources?.[sourceName], `release shared SDK config must include ${sourceName}`);
}

assert.doesNotMatch(
  workspaceSource,
  /\.\.\/\.\.\/\.\.\/\.\.\/apps\/sdkwork-(?:appbase|core|ui)\//u,
  'chat-pc workspace must not register sibling appbase/core/ui packages as install targets.',
);

console.log('sdkwork-chat-pc IM app API standard contract passed');
