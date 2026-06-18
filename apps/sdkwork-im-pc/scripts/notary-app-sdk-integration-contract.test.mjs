#!/usr/bin/env node

import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';

const appRoot = path.resolve(import.meta.dirname, '..');
const repoRoot = path.resolve(appRoot, '..', '..');

function readText(...segments) {
  return fs.readFileSync(path.join(appRoot, ...segments), 'utf8');
}

function readJson(...segments) {
  return JSON.parse(readText(...segments));
}

function readRepoText(...segments) {
  return fs.readFileSync(path.join(repoRoot, ...segments), 'utf8');
}

function readRepoJson(...segments) {
  return JSON.parse(readRepoText(...segments));
}

function functionBody(source, functionName) {
  const match = new RegExp(`function\\s+${functionName}\\s*\\(`, 'u').exec(source);
  assert.ok(match, `Expected ${functionName} in source.`);

  const openBraceIndex = source.indexOf('{', match.index);
  assert.notEqual(openBraceIndex, -1, `Expected ${functionName} body.`);

  let depth = 0;
  for (let index = openBraceIndex; index < source.length; index += 1) {
    const character = source[index];
    if (character === '{') {
      depth += 1;
    } else if (character === '}') {
      depth -= 1;
      if (depth === 0) {
        return source.slice(openBraceIndex, index + 1);
      }
    }
  }

  throw new Error(`Could not find closing brace for ${functionName}.`);
}

const packageJson = readJson('package.json');
const tsconfig = readJson('tsconfig.json');
const pnpmWorkspaceSource = readText('pnpm-workspace.yaml');
const viteConfigSource = readText('vite.config.ts');
const releaseSources = readRepoJson('config', 'shared-sdk-release-sources.json');
const sharedSdkGitSource = readRepoText('scripts', 'dev', 'prepare-shared-sdk-git-sources.mjs');
const devRunnerSource = readRepoText('scripts', 'lib', 'im-pc-dev.mjs');
const unifiedServerSource = readRepoText('scripts', 'im-server-dev.mjs');
const gatewayConfigSource = readRepoText('crates', 'sdkwork-im-gateway-config', 'src', 'lib.rs');
const gatewaySource = readRepoText('services', 'sdkwork-im-gateway', 'src', 'lib.rs');
const workflow = readRepoJson('sdkwork.workflow.json');
const packageWorkflowSource = readRepoText('.github', 'workflows', 'package.yml');
const appAuthRuntimeSource = readText(
  'packages',
  'sdkwork-im-pc-core',
  'src',
  'sdk',
  'appAuthRuntime.ts',
);
const notaryClientSource = readText(
  'packages',
  'sdkwork-im-pc-core',
  'src',
  'sdk',
  'notaryAppSdkClient.ts',
);
const notaryServiceSource = readText(
  'packages',
  'sdkwork-im-pc-notary',
  'src',
  'services',
  'NotaryService.ts',
);
const partyDrawerSource = readText(
  'packages',
  'sdkwork-im-pc-notary',
  'src',
  'PartyDrawer.tsx',
);
const notaryPackageSources = fs
  .readdirSync(path.join(appRoot, 'packages', 'sdkwork-im-pc-notary', 'src'), { recursive: true })
  .filter((entry) => typeof entry === 'string' && /\.(?:ts|tsx)$/u.test(entry))
  .map((entry) => readText('packages', 'sdkwork-im-pc-notary', 'src', entry))
  .join('\n');

assert.equal(
  packageJson.dependencies?.['@sdkwork/notary-app-sdk'],
  'workspace:*',
  'Chat PC must consume sdkwork-notary through the workspace app SDK package.',
);

assert.equal(
  packageJson.pnpm?.overrides?.['@sdkwork/notary-app-sdk'],
  'workspace:*',
  'Chat PC pnpm overrides must keep sdkwork-notary app SDK on workspace:*.',
);

assert.equal(
  packageJson.scripts?.['test:notary-app-sdk-integration'],
  'node scripts/notary-app-sdk-integration-contract.test.mjs',
  'Chat PC must expose a dedicated notary app SDK integration contract script.',
);

assert.match(
  pnpmWorkspaceSource,
  /sdkwork-notary[\\/]sdks[\\/]sdkwork-notary-app-sdk[\\/]sdkwork-notary-app-sdk-typescript/u,
  'pnpm-workspace.yaml must include the sdkwork-notary app SDK workspace root.',
);

assert.match(
  releaseSources.sources?.['sdkwork-notary']?.repoUrl ?? '',
  /^(?:https:\/\/github\.com\/|git@github\.com:)Sdkwork-Cloud\/sdkwork-notary\.git$/u,
  'Shared SDK release config must materialize sdkwork-notary from the canonical git repository.',
);

assert.ok(
  typeof releaseSources.sources?.['sdkwork-notary']?.ref === 'string'
    && releaseSources.sources['sdkwork-notary'].ref.trim().length > 0,
  'Shared SDK release config must pin a non-empty sdkwork-notary git ref.',
);

assert.equal(
  releaseSources.sources?.['sdkwork-notary']?.ref,
  workflow.dependencies?.find((dependency) => dependency.id === 'sdkwork-notary')?.ref,
  'Shared SDK release config must use the same pinned sdkwork-notary ref as sdkwork.workflow.json.',
);

assert.match(
  releaseSources.sources?.['sdkwork-drive']?.repoUrl ?? '',
  /^(?:https:\/\/github\.com\/|git@github\.com:)Sdkwork-Cloud\/sdkwork-drive\.git$/u,
  'Shared SDK release config must materialize sdkwork-drive because notary file operations compose Drive app SDK.',
);

assert.equal(
  releaseSources.sources?.['sdkwork-drive']?.ref,
  workflow.dependencies?.find((dependency) => dependency.id === 'sdkwork-drive')?.ref,
  'Shared SDK release config must use the same pinned sdkwork-drive ref as sdkwork.workflow.json.',
);

assert.match(
  sharedSdkGitSource,
  /id:\s*['"]sdkwork-notary['"][\s\S]*sdkwork-notary-app-sdk[\\/]sdkwork-notary-app-sdk-typescript[\\/]package\.json/u,
  'Shared SDK git materializer must know how to prepare the sdkwork-notary app SDK source.',
);

assert.match(
  sharedSdkGitSource,
  /id:\s*['"]sdkwork-drive['"][\s\S]*sdkwork-drive-app-sdk[\\/]sdkwork-drive-app-sdk-typescript[\\/]package\.json/u,
  'Shared SDK git materializer must know how to prepare the sdkwork-drive app SDK source used by notary file operations.',
);

assert.match(
  sharedSdkGitSource,
  /SDKWORK_SHARED_NOTARY_REPO_URL[\s\S]*SDKWORK_SHARED_NOTARY_GIT_REF/u,
  'Shared SDK git materializer must expose sdkwork-notary repo/ref override environment variables.',
);

assert.match(
  devRunnerSource,
  /SDKWORK_IM_PLATFORM_API_GATEWAY_HTTP_URL[\s\S]*explicitNotaryAppApiUpstream[\s\S]*SDKWORK_IM_NOTARY_APP_API_UPSTREAM/u,
  'PC dev runner must default sdkwork-notary traffic through the shared gateway root while preserving explicit Notary split upstream overrides.',
);

assert.match(
  unifiedServerSource,
  /SDKWORK_IM_PLATFORM_API_GATEWAY_HTTP_URL[\s\S]*resolveSdkworkApiGatewayBaseUrl/u,
  'Unified server launcher must configure the shared sdkwork-api-gateway root for local notary SDK traffic.',
);
assert.doesNotMatch(
  unifiedServerSource,
  /SDKWORK_IM_NOTARY_APP_API_UPSTREAM\s*=/u,
  'Unified server launcher must not default local notary SDK traffic to a separate per-module upstream.',
);

assert.match(
  gatewayConfigSource,
  /sdkwork-notary-app-api[\s\S]*SDKWORK_IM_NOTARY_APP_API_UPSTREAM[\s\S]*SDKWORK_NOTARY_APP_API_UPSTREAM[\s\S]*SDKWORK_NOTARY_APP_API_BASE_URL/u,
  'Gateway config must expose deterministic sdkwork-notary app-api upstream environment keys.',
);

assert.match(
  gatewaySource,
  /sdkwork-notary-app-api[\s\S]*\/app\/v3\/api\/notary\/\{\*path\}[\s\S]*SdkworkNotaryAppSdk/u,
  'Web gateway must route sdkwork-notary app-api paths to the Notary app SDK upstream.',
);

assert.ok(
  workflow.dependencies?.some((dependency) => (
    dependency.id === 'sdkwork-notary'
      && dependency.repository === 'Sdkwork-Cloud/sdkwork-notary'
      && dependency.refInput === 'SDKWORK_NOTARY_REF'
      && dependency.tokenSecret === 'SDKWORK_RELEASE_TOKEN'
  )),
  'sdkwork.workflow.json must declare sdkwork-notary as a release dependency.',
);

assert.ok(
  workflow.dependencies?.some((dependency) => (
    dependency.id === 'sdkwork-drive'
      && dependency.repository === 'Sdkwork-Cloud/sdkwork-drive'
      && dependency.refInput === 'SDKWORK_DRIVE_REF'
      && dependency.tokenSecret === 'SDKWORK_RELEASE_TOKEN'
  )),
  'sdkwork.workflow.json must declare sdkwork-drive as the notary file capability release dependency.',
);

assert.match(
  packageWorkflowSource,
  /sdkwork_notary_ref[\s\S]*SDKWORK_NOTARY_REF/u,
  '.github/workflows/package.yml must expose sdkwork-notary release ref input and forward SDKWORK_NOTARY_REF.',
);

assert.match(
  packageWorkflowSource,
  /sdkwork_drive_ref[\s\S]*SDKWORK_DRIVE_REF/u,
  '.github/workflows/package.yml must expose sdkwork-drive release ref input and forward SDKWORK_DRIVE_REF.',
);

assert.deepEqual(
  tsconfig.compilerOptions?.paths?.['@sdkwork/notary-app-sdk'],
  ['../../../sdkwork-notary/sdks/sdkwork-notary-app-sdk/sdkwork-notary-app-sdk-typescript/src/index.ts'],
  'tsconfig must resolve @sdkwork/notary-app-sdk to the canonical generated app SDK entry.',
);

assert.match(
  viteConfigSource,
  /generatedNotaryAppSdkEntry[\s\S]*find:\s*['"]@sdkwork\/notary-app-sdk['"][\s\S]*replacement:\s*generatedNotaryAppSdkEntry/u,
  'Vite must resolve @sdkwork/notary-app-sdk from the canonical sdkwork-notary app SDK source.',
);

assert.match(
  viteConfigSource,
  /optimizeDeps:\s*\{[\s\S]*exclude:\s*\[[\s\S]*['"]@sdkwork\/notary-app-sdk['"]/u,
  'Vite must exclude @sdkwork/notary-app-sdk from dependency pre-bundling so source-linked SDK edits stay live.',
);

assert.match(
  notaryClientSource,
  /createNotaryAppClient/u,
  'Core notary client must use the sdkwork-notary generated app SDK factory.',
);

assert.match(
  notaryClientSource,
  /tokenManager:\s*getSdkworkChatGlobalTokenManager\(\)/u,
  'Core notary client must share the Sdkwork IM global token manager.',
);

assert.doesNotMatch(
  notaryClientSource,
  /fetch\(|axios|Authorization|Access-Token/u,
  'Core notary client must not assemble raw HTTP or auth headers.',
);

assert.match(
  appAuthRuntimeSource,
  /getNotaryAppSdkClient/u,
  'Auth runtime must import the notary app SDK client.',
);

assert.match(
  appAuthRuntimeSource,
  /resetNotaryAppSdkClient/u,
  'Auth runtime must import the notary app SDK reset hook.',
);

assert.match(
  functionBody(appAuthRuntimeSource, 'resetSdkworkChatAuthenticatedSdkClients'),
  /resetNotaryAppSdkClient\(\)/u,
  'Session reset must reset the notary app SDK client with the other authenticated SDK clients.',
);

assert.match(
  functionBody(appAuthRuntimeSource, 'getAuthenticatedSdkClients'),
  /getNotaryAppSdkClient\(\)/u,
  'Auth runtime sdkClients inventory must include the notary app SDK client.',
);

assert.match(
  notaryServiceSource,
  /createNotaryApi/u,
  'Notary service must use the sdkwork-notary composed app SDK facade.',
);

assert.doesNotMatch(
  notaryServiceSource,
  /fetch\(|axios|Authorization|Access-Token|MockNotaryService|mockTasks|picsum\.photos/u,
  'Notary service must not bypass SDKs, assemble auth headers, or keep mock data.',
);

assert.doesNotMatch(
  notaryPackageSources,
  /\b(?:Mock|mock|fake|stub|demo)\b/u,
  'Notary production package must not keep mock/fake/stub/demo markers in authored sources.',
);

assert.doesNotMatch(
  notaryPackageSources,
  /Math\.random\s*\(/u,
  'Notary production package must use deterministic local temporary ids instead of Math.random().',
);

assert.doesNotMatch(
  partyDrawerSource,
  /setCompareResult\(98\.5\)/u,
  'Notary party identity compare must not publish a hard-coded local success score.',
);

assert.doesNotMatch(
  partyDrawerSource,
  /setTimeout\([\s\S]*?setCompareResult/u,
  'Notary party identity compare must not simulate asynchronous SDK-backed verification locally.',
);

assert.match(
  partyDrawerSource,
  /toast\(['"]身份材料将在提交后由公证服务完成核验。['"],\s*['"]info['"]\)/u,
  'Notary party identity compare must fail closed and tell users the SDK-backed notary service verifies identity materials after submission.',
);

console.log('sdkwork im notary app SDK integration contract passed.');
