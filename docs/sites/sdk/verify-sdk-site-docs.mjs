#!/usr/bin/env node
import { existsSync, readFileSync } from 'node:fs';
import path from 'node:path';

function read(rootDir, relativePath) {
  return readFileSync(path.join(rootDir, relativePath), 'utf8');
}

function expectIncludes(failures, source, expected, message) {
  if (!source.includes(expected)) {
    failures.push(message);
  }
}

function expectExcludes(failures, source, unexpected, message) {
  if (source.includes(unexpected)) {
    failures.push(message);
  }
}

export function verifySdkSiteDocs(options = {}) {
  const rootDir = options.rootDir || path.resolve(import.meta.dirname, '..', '..', '..');
  const failures = [];
  const legacyManagementName = 'sdkwork-craw-chat-sdk-management';
  const legacyManagementRoute = '/sdk/management-sdk';

  const docsPackage = JSON.parse(read(rootDir, 'docs/sites/package.json'));
  const vitepressConfigSource = read(rootDir, 'docs/sites/.vitepress/config.mjs');
  const siteHomeSource = read(rootDir, 'docs/sites/index.md');
  const sdkIndexSource = read(rootDir, 'docs/sites/sdk/index.md');
  const appSource = read(rootDir, 'docs/sites/sdk/app-sdk.md');
  const adminSource = read(rootDir, 'docs/sites/sdk/control-plane-sdk.md');
  const imAdminSource = read(rootDir, 'docs/sites/sdk/im-admin-sdk.md');
  const languageSupportSource = read(rootDir, 'docs/sites/sdk/language-support.md');
  const gettingStartedIndexSource = read(rootDir, 'docs/sites/getting-started/index.md');
  const capabilitiesSource = read(rootDir, 'docs/sites/features/capabilities.md');
  const cliReferenceSource = read(rootDir, 'docs/sites/reference/cli-and-scripts.md');
  const apiReferenceIndexSource = read(rootDir, 'docs/sites/api-reference/index.md');
  const platformApiSource = read(rootDir, 'docs/sites/api-reference/platform-api.md');
  const releaseCatalog = JSON.parse(
    read(rootDir, 'artifacts/releases/wave-d-2026-04-08/sdk-release-catalog.json'),
  );

  for (const [scriptName, taskName] of [
    ['docs:generate', 'generate'],
    ['docs:dev', 'dev'],
    ['docs:build', 'build'],
    ['docs:preview', 'preview'],
    ['docs:verify', 'verify'],
  ]) {
    const scriptValue = String(docsPackage.scripts?.[scriptName] || '');
    if (!scriptValue.includes(`./scripts/run-docs-task.mjs ${taskName}`)) {
      failures.push(`docs/sites/package.json must route ${scriptName} through scripts/run-docs-task.mjs ${taskName}.`);
    }
    if (!scriptValue.includes('npm_node_execpath')) {
      failures.push(`docs/sites/package.json must keep the npm_node_execpath fallback for ${scriptName}.`);
    }
  }

  expectIncludes(
    failures,
    vitepressConfigSource,
    '{ text: "IM Admin SDK", link: "/sdk/im-admin-sdk" }',
    'docs/sites/.vitepress/config.mjs must include the IM Admin SDK page in the SDK sidebar.',
  );
  expectExcludes(
    failures,
    vitepressConfigSource,
    legacyManagementRoute,
    'docs/sites/.vitepress/config.mjs must not link to the removed management SDK page.',
  );

  if (existsSync(path.join(rootDir, 'docs/sites/sdk/management-sdk.md'))) {
    failures.push('docs/sites/sdk/management-sdk.md must be removed after the IM admin rename.');
  }

  for (const [label, source] of [
    ['docs/sites/index.md', siteHomeSource],
    ['docs/sites/sdk/index.md', sdkIndexSource],
    ['docs/sites/sdk/control-plane-sdk.md', adminSource],
    ['docs/sites/sdk/language-support.md', languageSupportSource],
    ['docs/sites/getting-started/index.md', gettingStartedIndexSource],
    ['docs/sites/features/capabilities.md', capabilitiesSource],
    ['docs/sites/reference/cli-and-scripts.md', cliReferenceSource],
    ['docs/sites/api-reference/index.md', apiReferenceIndexSource],
    ['docs/sites/api-reference/platform-api.md', platformApiSource],
  ]) {
    expectExcludes(
      failures,
      source,
      legacyManagementName,
      `${label} must not mention the removed sdkwork-craw-chat-sdk-management workspace.`,
    );
    expectExcludes(
      failures,
      source,
      legacyManagementRoute,
      `${label} must not link to the removed /sdk/management-sdk route.`,
    );
  }

  expectIncludes(
    failures,
    sdkIndexSource,
    '`sdkwork-im-admin-sdk`',
    'docs/sites/sdk/index.md must describe the sdkwork-im-admin-sdk family.',
  );
  expectIncludes(
    failures,
    sdkIndexSource,
    '[IM Admin SDK](/sdk/im-admin-sdk)',
    'docs/sites/sdk/index.md must route /api/admin/* readers to the IM Admin SDK page.',
  );

  expectIncludes(
    failures,
    adminSource,
    '[IM Admin SDK](/sdk/im-admin-sdk)',
    'docs/sites/sdk/control-plane-sdk.md must link to the IM Admin SDK page for /api/admin/* flows.',
  );

  for (const requiredEntry of [
    '`sdkwork-im-admin-sdk`',
    '`@sdkwork/im-admin-backend-sdk`',
    '`@sdkwork/im-admin-sdk`',
    '`im_admin_backend_sdk`',
    '`im_admin_sdk`',
    '`ImAdminSdkClient`',
    '`generated/server-openapi/src/*`',
    '`generated/server-openapi/lib/src`',
    'node ./sdks/sdkwork-im-admin-sdk/bin/verify-sdk.mjs',
  ]) {
    expectIncludes(
      failures,
      imAdminSource,
      requiredEntry,
      `docs/sites/sdk/im-admin-sdk.md must mention ${requiredEntry}.`,
    );
  }

  for (const removedEntry of [
    legacyManagementName,
    '@sdkwork/craw-chat-management-backend-sdk',
    '@sdkwork/craw-chat-sdk-management',
    'craw_chat_management_backend_sdk',
    'im_sdk_management',
    'ImSdkManagementClient',
    'createImSdkManagementClient',
    'CrawChatManagementClient',
  ]) {
    expectExcludes(
      failures,
      imAdminSource,
      removedEntry,
      `docs/sites/sdk/im-admin-sdk.md must not mention removed legacy entry ${removedEntry}.`,
    );
  }

  expectIncludes(
    failures,
    appSource,
    '`@sdkwork/im-sdk`',
    'docs/sites/sdk/app-sdk.md must continue documenting @sdkwork/im-sdk as the app package root.',
  );

  for (const requiredEntry of [
    '`sdks/sdkwork-im-admin-sdk`',
    '`sdks/sdkwork-im-admin-sdk/sdkwork-im-admin-sdk-typescript`',
    '`sdks/sdkwork-im-admin-sdk/sdkwork-im-admin-sdk-flutter`',
    '`@sdkwork/im-admin-backend-sdk`',
    '`@sdkwork/im-admin-sdk`',
    '`im_admin_backend_sdk`',
    '`im_admin_sdk`',
    '`im-admin-typescript`',
    '`im-admin-flutter`',
  ]) {
    expectIncludes(
      failures,
      languageSupportSource,
      requiredEntry,
      `docs/sites/sdk/language-support.md must mention ${requiredEntry}.`,
    );
  }

  expectIncludes(
    failures,
    siteHomeSource,
    'IM Admin SDK families',
    'docs/sites/index.md must summarize the IM Admin SDK family on the landing page.',
  );
  expectIncludes(
    failures,
    gettingStartedIndexSource,
    '`sdkwork-im-admin-sdk` maps to the deployed `/api/admin/*` surface.',
    'docs/sites/getting-started/index.md must summarize sdkwork-im-admin-sdk entry targeting.',
  );
  expectIncludes(
    failures,
    capabilitiesSource,
    '`sdks/sdkwork-im-admin-sdk/`',
    'docs/sites/features/capabilities.md must describe the sdkwork-im-admin-sdk workspace.',
  );
  expectIncludes(
    failures,
    capabilitiesSource,
    '@sdkwork/control-plane-sdk',
    'docs/sites/features/capabilities.md must document @sdkwork/control-plane-sdk as the admin-console consumer surface.',
  );
  expectIncludes(
    failures,
    sdkIndexSource,
    '@sdkwork/control-plane-sdk',
    'docs/sites/sdk/index.md must document @sdkwork/control-plane-sdk as the admin-console consumer boundary.',
  );
  expectIncludes(
    failures,
    cliReferenceSource,
    '`sdks/sdkwork-im-admin-sdk`',
    'docs/sites/reference/cli-and-scripts.md must document the sdkwork-im-admin-sdk workspace.',
  );
  expectIncludes(
    failures,
    cliReferenceSource,
    'node .\\sdks\\sdkwork-im-admin-sdk\\bin\\verify-sdk.mjs',
    'docs/sites/reference/cli-and-scripts.md must document the IM admin verify-sdk command.',
  );
  expectIncludes(
    failures,
    apiReferenceIndexSource,
    '`sdkwork-im-admin-sdk` maps to the unified gateway\'s `/api/admin/*` operator surface.',
    'docs/sites/api-reference/index.md must map /api/admin/* to sdkwork-im-admin-sdk.',
  );
  expectIncludes(
    failures,
    platformApiSource,
    '`sdkwork-im-admin-sdk`',
    'docs/sites/api-reference/platform-api.md must reference sdkwork-im-admin-sdk for operator /api/admin/* surfaces.',
  );

  const releaseArtifacts = Array.isArray(releaseCatalog.sdkArtifacts) ? releaseCatalog.sdkArtifacts : [];
  const imAdminArtifacts = releaseArtifacts.filter((artifact) => artifact.audience === 'im-admin');
  if (imAdminArtifacts.length !== 2) {
    failures.push('artifacts/releases/wave-d-2026-04-08/sdk-release-catalog.json must contain exactly two im-admin artifacts.');
  }
  for (const [expectedId, expectedPackage, expectedReadmePath] of [
    [
      'im-admin-typescript',
      'sdkwork-im-admin-sdk-typescript',
      'sdks/sdkwork-im-admin-sdk/sdkwork-im-admin-sdk-typescript/README.md',
    ],
    [
      'im-admin-flutter',
      'sdkwork-im-admin-sdk-flutter',
      'sdks/sdkwork-im-admin-sdk/sdkwork-im-admin-sdk-flutter/README.md',
    ],
  ]) {
    const artifact = imAdminArtifacts.find((entry) => entry.id === expectedId);
    if (!artifact) {
      failures.push(`SDK release catalog is missing ${expectedId}.`);
      continue;
    }
    if (artifact.package !== expectedPackage) {
      failures.push(`SDK release catalog ${expectedId} must use package ${expectedPackage}.`);
    }
    if (artifact.readmePath !== expectedReadmePath) {
      failures.push(`SDK release catalog ${expectedId} must use readmePath ${expectedReadmePath}.`);
    }
    if (artifact.generationStatus !== 'generated') {
      failures.push(`SDK release catalog ${expectedId} must remain in generationStatus=generated.`);
    }
    if (artifact.releaseStatus !== 'not_published') {
      failures.push(`SDK release catalog ${expectedId} must remain in releaseStatus=not_published.`);
    }
  }

  return failures;
}

const isCli = process.argv[1] && path.resolve(process.argv[1]) === import.meta.filename;

if (isCli) {
  const failures = verifySdkSiteDocs();
  if (failures.length > 0) {
    console.error('[docs/sites/sdk] SDK site docs verification failed:');
    for (const failure of failures) {
      console.error(`- ${failure}`);
    }
    process.exit(1);
  }

  console.log('[docs/sites/sdk] SDK site docs verification passed.');
}
