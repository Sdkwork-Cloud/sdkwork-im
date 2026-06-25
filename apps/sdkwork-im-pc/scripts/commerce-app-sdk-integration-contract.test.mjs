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

const packageJson = readJson('package.json');
const releaseSources = readRepoJson('config', 'shared-sdk-release-sources.json');
const workflow = readRepoJson('sdkwork.workflow.json');
const gatewayConfigSource = readRepoText('crates', 'sdkwork-im-cloud-gateway-config', 'src', 'lib.rs');
const gatewaySource = readRepoText('services', 'sdkwork-im-cloud-gateway', 'src', 'lib.rs');
const sharedSdkGitSource = readRepoText('scripts', 'dev', 'prepare-shared-sdk-git-sources.mjs');
const releaseBuildSource = readRepoText('scripts', 'release', 'run-sdkwork-im-pc-release-build.mjs');
const devRunnerSource = readRepoText('scripts', 'lib', 'im-pc-dev.mjs');
const componentSpec = readRepoJson('specs', 'component.spec.json');

assert.equal(
  packageJson.scripts?.['test:commerce-app-sdk-integration'],
  'node scripts/commerce-app-sdk-integration-contract.test.mjs',
  'Chat PC must expose a dedicated commerce app SDK integration contract script.',
);

assert.match(
  releaseSources.sources?.['sdkwork-commerce']?.repoUrl ?? '',
  /^(?:https:\/\/github\.com\/|git@github\.com:)Sdkwork-Cloud\/sdkwork-commerce\.git$/u,
  'Shared SDK release config must materialize sdkwork-commerce from the canonical git repository.',
);

assert.ok(
  typeof releaseSources.sources?.['sdkwork-commerce']?.ref === 'string'
    && releaseSources.sources['sdkwork-commerce'].ref.trim().length > 0,
  'Shared SDK release config must pin a non-empty sdkwork-commerce git ref.',
);

assert.equal(
  releaseSources.sources?.['sdkwork-commerce']?.ref,
  workflow.dependencies?.find((dependency) => dependency.id === 'sdkwork-commerce')?.ref,
  'Shared SDK release config must use the same pinned sdkwork-commerce ref as sdkwork.workflow.json.',
);

assert.match(
  sharedSdkGitSource,
  /id:\s*['"]sdkwork-commerce['"][\s\S]*sdkwork-commerce-app-sdk[\\/]sdkwork-commerce-app-sdk-typescript[\\/]package\.json/u,
  'Shared SDK git materializer must know how to prepare the sdkwork-commerce app SDK source.',
);

assert.match(
  sharedSdkGitSource,
  /SDKWORK_SHARED_COMMERCE_REPO_URL[\s\S]*SDKWORK_SHARED_COMMERCE_GIT_REF/u,
  'Shared SDK git materializer must expose sdkwork-commerce repo/ref override environment variables.',
);

assert.match(
  releaseBuildSource,
  /SDKWORK_SHARED_COMMERCE_GIT_REF[\s\S]*SDKWORK_COMMERCE_REF/u,
  'Release build plan must bridge SDKWORK_COMMERCE_REF into the shared SDK materializer ref for the commerce app SDK.',
);

assert.match(
  devRunnerSource,
  /SDKWORK_IM_PLATFORM_API_GATEWAY_HTTP_URL[\s\S]*explicitCommerceAppApiUpstream[\s\S]*SDKWORK_IM_COMMERCE_APP_API_UPSTREAM/u,
  'PC dev runner must default sdkwork-commerce traffic through the shared gateway root while preserving explicit Commerce split upstream overrides.',
);

assert.match(
  gatewayConfigSource,
  /sdkwork-commerce-app-api[\s\S]*SDKWORK_IM_COMMERCE_APP_API_UPSTREAM[\s\S]*SDKWORK_COMMERCE_APP_API_UPSTREAM[\s\S]*SDKWORK_COMMERCE_APP_API_BASE_URL/u,
  'Gateway config must expose deterministic sdkwork-commerce app-api upstream environment keys.',
);

assert.match(
  gatewaySource,
  /COMMERCE_APP_API_SEGMENTS[\s\S]*"catalog"[\s\S]*"orders"/u,
  'Web gateway must declare sdkwork-commerce catalog and orders app-api route segments.',
);

assert.match(
  gatewaySource,
  /"sdkwork-commerce-app-api"[\s\S]*SdkworkCommerceAppSdk/u,
  'Web gateway must route sdkwork-commerce app-api paths to the Commerce app SDK upstream.',
);

assert.ok(
  workflow.dependencies?.some((dependency) => (
    dependency.id === 'sdkwork-commerce'
      && dependency.repository === 'Sdkwork-Cloud/sdkwork-commerce'
      && dependency.refInput === 'SDKWORK_COMMERCE_REF'
      && dependency.tokenSecret === 'SDKWORK_RELEASE_TOKEN'
  )),
  'sdkwork.workflow.json must declare sdkwork-commerce as a release dependency.',
);

const dependencySurface = componentSpec.contracts?.dependencyApiSurfaces?.find(
  (surface) => surface.apiAuthority === 'sdkwork-commerce-app-api',
);
assert.ok(dependencySurface, 'component.spec.json must declare sdkwork-commerce-app-api dependency surface.');
assert.equal(
  dependencySurface.sdkFamily,
  'sdkwork-commerce-app-sdk',
  'component.spec.json must bind sdkwork-commerce-app-api to sdkwork-commerce-app-sdk.',
);
assert.equal(
  dependencySurface.targetRuntimeIntegration?.gatewayApplication,
  'sdkwork-api-cloud-gateway',
  'sdkwork-commerce app API must route through the shared sdkwork-api-cloud-gateway root.',
);

const splitOverrideEnvKeys =
  componentSpec.integration?.foundationApiGateway?.splitOverrideEnvKeys?.['sdkwork-commerce-app-api'];
assert.deepEqual(
  splitOverrideEnvKeys,
  [
    'SDKWORK_IM_COMMERCE_APP_API_UPSTREAM',
    'SDKWORK_COMMERCE_APP_API_UPSTREAM',
    'SDKWORK_COMMERCE_APP_API_BASE_URL',
  ],
  'component.spec.json must document commerce split upstream override env keys.',
);

console.log('commerce app SDK integration contract checks passed');
