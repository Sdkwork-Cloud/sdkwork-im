#!/usr/bin/env node

import assert from 'node:assert/strict';
import { execFileSync } from 'node:child_process';
import { existsSync, mkdirSync, readFileSync, rmSync, writeFileSync } from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';

const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..', '..');

function repoPath(...segments) {
  return path.join(repoRoot, ...segments);
}

function readText(...segments) {
  return readFileSync(repoPath(...segments), 'utf8');
}

function assertFile(relativePath) {
  assert.equal(
    existsSync(repoPath(...relativePath.split('/'))),
    true,
    `${relativePath} should exist`,
  );
}

async function importRepoModule(relativePath) {
  return await import(pathToFileURL(repoPath(...relativePath.split('/'))).href);
}

const rootPackageJson = JSON.parse(readText('package.json'));

for (const [scriptName, expectedCommand] of Object.entries({
  'release:plan': 'node scripts/release/plan-craw-chat-install-packages.mjs',
  'release:build:production': 'node scripts/release/build-craw-chat-production.mjs',
  'release:stage': 'node scripts/release/stage-craw-chat-release-package.mjs',
  'release:package': 'node scripts/release/build-craw-chat-install-package.mjs',
  'release:package:check': 'node scripts/release/build-craw-chat-install-package.mjs --check --dry-run --all',
  'release:validate': 'node scripts/release/validate-craw-chat-install-artifacts.mjs',
  'release:desktop': 'node scripts/release/build-craw-chat-production.mjs --target desktop',
})) {
  assert.equal(rootPackageJson.scripts?.[scriptName], expectedCommand, `package.json script ${scriptName}`);
}

for (const relativePath of [
  'scripts/release/craw-chat-release-version.mjs',
  'scripts/release/plan-craw-chat-install-packages.mjs',
  'scripts/release/build-craw-chat-production.mjs',
  'scripts/release/stage-craw-chat-release-package.mjs',
  'scripts/release/build-craw-chat-install-package.mjs',
  'scripts/release/collect-craw-chat-desktop-bundles.mjs',
  'scripts/release/desktop-targets.mjs',
  'scripts/release/validate-craw-chat-install-artifacts.mjs',
]) {
  assertFile(relativePath);
}

const planModule = await importRepoModule('scripts/release/plan-craw-chat-install-packages.mjs');
for (const exportName of [
  'createCrawChatInstallPackagePlan',
  'validateCrawChatInstallPackagePlan',
  'renderCrawChatInstallPackagePlan',
  'SUPPORTED_PLATFORMS',
  'SUPPORTED_ARCHITECTURES',
  'SUPPORTED_DEPLOYMENT_MODES',
]) {
  assert.equal(typeof planModule[exportName], exportName.startsWith('SUPPORTED_') ? 'object' : 'function', `${exportName} export`);
}

const releasePlan = planModule.createCrawChatInstallPackagePlan({ version: '1.2.3' });
const planIssues = planModule.validateCrawChatInstallPackagePlan(releasePlan);
assert.deepEqual(planIssues, [], `release package plan issues: ${planIssues.join('; ')}`);
const renderedReleasePlan = planModule.renderCrawChatInstallPackagePlan(releasePlan).join('\n');
for (const expectedText of [
  'paths=install=/opt/sdkwork/chat config=/etc/sdkwork/chat data=/var/lib/sdkwork/chat log=/var/log/sdkwork/chat run=/run/sdkwork/chat',
  'paths=install=/usr/lib/sdkwork/chat config=/Library/Application Support/sdkwork/chat data=/Library/Application Support/sdkwork/chat/Data log=/Library/Logs/sdkwork/chat run=/Library/Application Support/sdkwork/chat/Run',
  'paths=install=%ProgramFiles%/sdkwork/chat config=%ProgramData%/sdkwork/chat data=%ProgramData%/sdkwork/chat/Data log=%ProgramData%/sdkwork/chat/Logs run=%ProgramData%/sdkwork/chat/Run',
]) {
  assert.match(renderedReleasePlan, new RegExp(expectedText.replace(/[.*+?^${}()|[\]\\]/gu, '\\$&'), 'u'));
}
assert.equal(releasePlan.appCode, 'chat');
assert.equal(releasePlan.runtimeName, 'chat');
assert.equal(releasePlan.product, 'chat');
assert.equal(releasePlan.packageName, 'sdkwork-chat');
assert.equal(releasePlan.artifactPolicy?.noSecretsInPackage, true);
assert.equal(releasePlan.artifactPolicy?.envLocalGeneratedOnHost, true);
assert.equal(releasePlan.artifactPolicy?.generatedFromProductionBuild, true);
assert.deepEqual(releasePlan.deploymentModes, ['server-archive', 'desktop']);

const serverPackagesByPlatform = new Map(
  releasePlan.packages
    .filter((item) => item.deploymentMode === 'server-archive' && item.architecture === 'x64')
    .map((item) => [item.platform, item]),
);
assert.equal(
  serverPackagesByPlatform.get('linux')?.databasePolicy?.configFile?.path,
  '/etc/sdkwork/chat/chat.toml',
  'linux server config file should use the Ubuntu production config root',
);
assert.deepEqual(
  serverPackagesByPlatform.get('linux')?.runtimePaths,
  {
    installRoot: '/opt/sdkwork/chat',
    configDir: '/etc/sdkwork/chat',
    dataDir: '/var/lib/sdkwork/chat',
    logDir: '/var/log/sdkwork/chat',
    runDir: '/run/sdkwork/chat',
  },
  'linux server package should expose the complete Ubuntu production path matrix',
);
assert.equal(
  serverPackagesByPlatform.get('linux')?.databasePolicy?.dataDirectory?.path,
  '/var/lib/sdkwork/chat',
  'linux server data directory should use the SDKWork production data root',
);
assert.equal(
  serverPackagesByPlatform.get('linux')?.databasePolicy?.passwordFile?.path,
  '/etc/sdkwork/chat/database.secret',
  'linux PostgreSQL password file should stay under the production config root',
);
assert.equal(
  serverPackagesByPlatform.get('windows')?.databasePolicy?.configFile?.path,
  '%ProgramData%/sdkwork/chat/chat.toml',
  'windows server config file should use ProgramData SDKWork Chat config root',
);
assert.deepEqual(
  serverPackagesByPlatform.get('windows')?.runtimePaths,
  {
    installRoot: '%ProgramFiles%/sdkwork/chat',
    configDir: '%ProgramData%/sdkwork/chat',
    dataDir: '%ProgramData%/sdkwork/chat/Data',
    logDir: '%ProgramData%/sdkwork/chat/Logs',
    runDir: '%ProgramData%/sdkwork/chat/Run',
  },
  'windows server package should expose the complete ProgramData path matrix',
);
assert.equal(
  serverPackagesByPlatform.get('windows')?.databasePolicy?.dataDirectory?.path,
  '%ProgramData%/sdkwork/chat/Data',
  'windows server data directory should use ProgramData SDKWork Chat data root',
);
assert.equal(
  serverPackagesByPlatform.get('windows')?.databasePolicy?.passwordFile?.path,
  '%ProgramData%/sdkwork/chat/database.secret',
  'windows PostgreSQL password file should stay under ProgramData SDKWork Chat config root',
);
assert.equal(
  serverPackagesByPlatform.get('macos')?.databasePolicy?.configFile?.path,
  '/Library/Application Support/sdkwork/chat/chat.toml',
  'macOS server config file should use the SDKWork Chat application support config root',
);
assert.deepEqual(
  serverPackagesByPlatform.get('macos')?.runtimePaths,
  {
    installRoot: '/usr/lib/sdkwork/chat',
    configDir: '/Library/Application Support/sdkwork/chat',
    dataDir: '/Library/Application Support/sdkwork/chat/Data',
    logDir: '/Library/Logs/sdkwork/chat',
    runDir: '/Library/Application Support/sdkwork/chat/Run',
  },
  'macOS server package should expose the complete application support path matrix',
);
assert.equal(
  serverPackagesByPlatform.get('macos')?.databasePolicy?.dataDirectory?.path,
  '/Library/Application Support/sdkwork/chat/Data',
  'macOS server data directory should use the SDKWork Chat application support data root',
);
assert.equal(
  serverPackagesByPlatform.get('macos')?.databasePolicy?.passwordFile?.path,
  '/Library/Application Support/sdkwork/chat/database.secret',
  'macOS PostgreSQL password file should stay under the SDKWork Chat config root',
);

const expectedPackageIds = [
  'linux-x64-server-archive',
  'linux-arm64-server-archive',
  'macos-x64-server-archive',
  'macos-arm64-server-archive',
  'windows-x64-server-archive',
  'windows-arm64-server-archive',
  'linux-x64-desktop',
  'linux-arm64-desktop',
  'macos-x64-desktop',
  'macos-arm64-desktop',
  'windows-x64-desktop',
  'windows-arm64-desktop',
];
assert.deepEqual(releasePlan.packages.map((item) => item.id), expectedPackageIds);

const subsetPlanJson = execFileSync(
  process.execPath,
  [
    'scripts/release/plan-craw-chat-install-packages.mjs',
    '--platform',
    'windows',
    '--architecture',
    'x64',
    '--deployment-mode',
    'desktop',
    '--version',
    '1.2.3',
    '--json',
  ],
  { cwd: repoRoot, encoding: 'utf8' },
);
const subsetPlanPayload = JSON.parse(subsetPlanJson);
assert.equal(subsetPlanPayload.ok, true, 'filtered release plan should validate');
assert.deepEqual(
  subsetPlanPayload.plan.packages.map((item) => item.id),
  ['windows-x64-desktop'],
  'filtered release plan should include only the requested package id',
);

for (const packageItem of releasePlan.packages) {
  assert.equal(packageItem.security?.noSecretsInPackage, true, `${packageItem.id} no secrets policy`);
  assert.equal(packageItem.version, '1.2.3', `${packageItem.id} version`);
  if (packageItem.deploymentMode === 'server-archive') {
    assert.equal(packageItem.runtimeProfile, 'server', `${packageItem.id} runtime profile`);
    assert.equal(packageItem.databasePolicy?.defaultEngine, 'postgresql', `${packageItem.id} database engine`);
    assert.equal(packageItem.databasePolicy?.requiresExternalDatabase, true, `${packageItem.id} external database`);
    assert.equal(packageItem.databasePolicy?.defaultDatabase, 'sdkwork_chat_prod', `${packageItem.id} production database`);
    assert.equal(packageItem.databasePolicy?.defaultUsername, 'sdkwork_chat_prod', `${packageItem.id} production database user`);
    for (const expectedArtifact of [
      'server-binary',
      'server-lifecycle-scripts',
      'server-config-template',
      'server-env-template',
      'postgresql-config-template',
      'pc-web-dist',
      'service-templates',
      'install-guide',
      'install-manifest',
    ]) {
      assert.equal(
        packageItem.artifacts.some((artifact) => artifact.kind === expectedArtifact && artifact.required === true),
        true,
        `${packageItem.id} should include ${expectedArtifact}`,
      );
    }
    assert.match(packageItem.archiveName, /^craw-chat-server-.+\.(zip|tar\.gz)$/u);
  } else {
    assert.equal(packageItem.runtimeProfile, 'desktop', `${packageItem.id} runtime profile`);
    assert.equal(packageItem.databasePolicy?.defaultEngine, 'sqlite', `${packageItem.id} database engine`);
    assert.equal(packageItem.databasePolicy?.requiresExternalDatabase, false, `${packageItem.id} external database`);
    for (const expectedArtifact of ['desktop-installers', 'desktop-manifest']) {
      assert.equal(
        packageItem.artifacts.some((artifact) => artifact.kind === expectedArtifact && artifact.required === true),
        true,
        `${packageItem.id} should include ${expectedArtifact}`,
      );
    }
    assert.match(packageItem.archiveName, /^craw-chat-desktop-.+\.zip$/u);
  }
}

const packageBuilder = await importRepoModule('scripts/release/build-craw-chat-install-package.mjs');
const desktopBundleCollector = await importRepoModule('scripts/release/collect-craw-chat-desktop-bundles.mjs');
const productionDryRunJson = execFileSync(
  process.execPath,
  ['scripts/release/build-craw-chat-production.mjs', '--target', 'desktop', '--dry-run', '--json'],
  {
    cwd: repoRoot,
    encoding: 'utf8',
    env: {
      ...process.env,
      CRAW_CHAT_TEST_SECRET: 'do-not-leak-release-secret',
    },
  },
);
assert.doesNotMatch(productionDryRunJson, /do-not-leak-release-secret/u, 'production dry-run JSON must not leak env values');
assert.doesNotMatch(productionDryRunJson, /"env"\s*:/u, 'production dry-run JSON must not expose raw env objects');
const productionDryRunPayload = JSON.parse(productionDryRunJson);
assert.deepEqual(
  productionDryRunPayload.plan.steps.map((step) => step.label),
  [
    'build sdkwork-chat-pc web assets',
    'build desktop installer x86_64-pc-windows-msvc',
  ],
  'desktop-only production build should prepare web assets before Tauri packaging',
);

const dryRunBuildPlan = packageBuilder.createCrawChatInstallPackageBuildPlan({
  packageId: 'windows-x64-server-archive',
  version: '1.2.3',
  requireStagedFiles: false,
});
assert.equal(
  packageBuilder.validateCrawChatInstallPackageBuildPlan(dryRunBuildPlan).length,
  0,
  'dry-run package build plan should be valid without staged files',
);
for (const entry of dryRunBuildPlan.entries) {
  assert.doesNotMatch(entry.archivePath, /(^|\/)\.env($|\.|\/)|secret|secrets\/|node_modules|\.runtime/u);
  assert.doesNotMatch(entry.archivePath, /\.\.|^[A-Za-z]:|^\/|\\/u);
}

const validatorModule = await importRepoModule('scripts/release/validate-craw-chat-install-artifacts.mjs');
for (const exportName of [
  'parseValidateArgs',
  'readTarEntries',
  'readZipEntries',
  'validateCrawChatInstallArtifact',
  'validateTarGzArtifact',
  'validateZipArtifact',
]) {
  assert.equal(typeof validatorModule[exportName], 'function', `${exportName} export`);
}

const validatorTempRoot = mkTempDir('craw-chat-release-validator-');
try {
  const serverStage = path.join(validatorTempRoot, 'stage', 'windows-x64-server-archive');
  const desktopStage = path.join(validatorTempRoot, 'stage', 'windows-x64-desktop');
  const outputDir = path.join(validatorTempRoot, 'out');
  writeFixture(serverStage, 'bin/craw-chat-server.exe', 'server');
  writeFixture(serverStage, 'config/chat.toml.example', '[server]\nbind_address = "127.0.0.1:18080"\n');
  writeFixture(serverStage, 'config/server.env.example', 'SDKWORK_CHAT_SERVER_BIND=127.0.0.1:18080\n');
  writeFixture(serverStage, 'config/postgresql.yaml.example', 'engine: postgresql');
  writeFixture(serverStage, 'INSTALL.md', '# install');
  writeFixture(serverStage, 'install-manifest.json', '{"product":"chat"}');
  writeFixture(serverStage, 'web/sdkwork-chat-pc/dist/index.html', '<!doctype html>');
  writeFixture(serverStage, 'service/windows/CrawChatServer.xml', '<service />');
  const serverBuildPlan = packageBuilder.createCrawChatInstallPackageBuildPlan({
    outputDir,
    packageId: 'windows-x64-server-archive',
    root: repoRoot,
    stagingRoot: serverStage,
    version: '1.2.3',
  });
  const serverArchive = await packageBuilder.buildCrawChatInstallPackageArchive(serverBuildPlan);
  const serverValidation = validatorModule.validateCrawChatInstallArtifact({
    artifactPath: serverArchive.archivePath,
    packageId: 'windows-x64-server-archive',
    root: repoRoot,
    version: '1.2.3',
  });
  assert.equal(serverValidation.ok, true, `server archive validation issues: ${serverValidation.issues.join('; ')}`);

  writeFixture(desktopStage, 'desktop/Craw Chat_1.2.3_x64-setup.exe', 'desktop');
  writeFixture(desktopStage, 'desktop-manifest.json', JSON.stringify({
    product: 'chat',
    version: '1.2.3',
    files: [{ path: 'desktop/Craw Chat_1.2.3_x64-setup.exe' }],
  }));
  const desktopBuildPlan = packageBuilder.createCrawChatInstallPackageBuildPlan({
    outputDir,
    packageId: 'windows-x64-desktop',
    root: repoRoot,
    stagingRoot: desktopStage,
    version: '1.2.3',
  });
  const desktopArchive = await packageBuilder.buildCrawChatInstallPackageArchive(desktopBuildPlan);
  const desktopValidation = validatorModule.validateCrawChatInstallArtifact({
    artifactPath: desktopArchive.archivePath,
    packageId: 'windows-x64-desktop',
    root: repoRoot,
    version: '1.2.3',
  });
  assert.equal(desktopValidation.ok, true, `desktop archive validation issues: ${desktopValidation.issues.join('; ')}`);
} finally {
  rmSync(validatorTempRoot, { recursive: true, force: true });
}

const desktopCollectorTempRoot = mkTempDir('craw-chat-release-desktop-bundles-');
try {
  writeFixture(desktopCollectorTempRoot, 'nsis/Craw Chat_1.2.3_x64-setup.exe', 'x64 exe');
  writeFixture(desktopCollectorTempRoot, 'nsis/Craw Chat_1.2.3_arm64-setup.exe', 'arm64 exe');
  writeFixture(desktopCollectorTempRoot, 'msi/Craw Chat_1.2.3_x64_en-US.msi', 'x64 msi');
  writeFixture(desktopCollectorTempRoot, 'msi/Craw Chat_1.2.3_arm64_en-US.msi', 'arm64 msi');
  const x64DesktopBundles = desktopBundleCollector.collectCrawChatDesktopBundles({
    arch: 'x64',
    bundleRoot: desktopCollectorTempRoot,
    platform: 'windows',
    root: repoRoot,
    version: '1.2.3',
  });
  assert.deepEqual(
    x64DesktopBundles.files.map((file) => file.path).sort(),
    [
      'msi/Craw Chat_1.2.3_x64_en-US.msi',
      'nsis/Craw Chat_1.2.3_x64-setup.exe',
    ],
    'desktop collector should exclude installer artifacts for the opposite architecture',
  );
  assert.deepEqual(
    desktopBundleCollector.validateCrawChatDesktopBundleManifest(x64DesktopBundles),
    [],
    'desktop collector manifest with matching architecture files should validate',
  );
} finally {
  rmSync(desktopCollectorTempRoot, { recursive: true, force: true });
}

const stagingModule = await importRepoModule('scripts/release/stage-craw-chat-release-package.mjs');
const dryRunStagingPlan = stagingModule.createCrawChatReleaseStagingPlan({
  packageId: 'windows-x64-server-archive',
  version: '1.2.3',
});
assert.equal(
  stagingModule.validateCrawChatReleaseStagingPlan(dryRunStagingPlan, { requireSources: false }).length,
  0,
  'dry-run staging plan should be valid without source artifacts',
);
const stagingArchivePaths = new Set(dryRunStagingPlan.actions.map((action) => action.archivePath).filter(Boolean));
for (const expectedPath of [
  'config/chat.toml.example',
  'config/postgresql.yaml.example',
  'service/linux/craw-chat-server.service',
  'service/macos/com.sdkwork.crawchat.server.plist',
  'service/windows/CrawChatServer.xml',
  'web/sdkwork-chat-pc/dist',
]) {
  assert.equal(
    stagingArchivePaths.has(expectedPath),
    true,
    `staging plan should expose archive path ${expectedPath}`,
  );
}

const linuxStagingPlan = stagingModule.createCrawChatReleaseStagingPlan({
  packageId: 'linux-x64-server-archive',
  version: '1.2.3',
});
const linuxGeneratedEnvAction = linuxStagingPlan.actions.find((action) => action.label === 'server env template');
assert.equal(typeof linuxGeneratedEnvAction?.contentFactory, 'function', 'linux staging plan should generate server env template');
const linuxGeneratedEnv = linuxGeneratedEnvAction.contentFactory();
for (const expectedText of [
  'SDKWORK_CHAT_CONFIG_FILE=/etc/sdkwork/chat/chat.toml',
  'SDKWORK_CHAT_DATA_DIR=/var/lib/sdkwork/chat',
  'SDKWORK_CHAT_LOG_DIR=/var/log/sdkwork/chat',
  'SDKWORK_CHAT_RUN_DIR=/run/sdkwork/chat',
  'SDKWORK_CHAT_ID_NODE_ID=1',
  'SDKWORK_CHAT_SERVER_BASE_URL=https://chat.example.com/sdkwork/chat',
  'SDKWORK_CHAT_SERVER_API_BASE_URL=https://chat.example.com/sdkwork/chat',
  'SDKWORK_CHAT_SERVER_WEBSOCKET_BASE_URL=wss://chat.example.com/sdkwork/chat',
  'CRAW_CHAT_ADMIN_SITE_DIR=/opt/sdkwork/chat/web/sdkwork-chat-pc/dist',
]) {
  assert.match(linuxGeneratedEnv, new RegExp(expectedText.replace(/[.*+?^${}()|[\]\\]/gu, '\\$&'), 'u'));
}
assert.doesNotMatch(linuxGeneratedEnv, /\/etc\/craw-chat\/default|\/opt\/craw-chat/u);

const serverYamlTemplate = readText('deployments', 'templates', 'chat.toml.example');
for (const expectedText of [
  'config_file = "/etc/sdkwork/chat/chat.toml"',
  'base_url = "https://chat.example.com/sdkwork/chat"',
  'api_base_url = "https://chat.example.com/sdkwork/chat"',
  'websocket_base_url = "wss://realtime.example.com/sdkwork/chat"',
  'docs_base_url = "https://chat.example.com/sdkwork/chat/docs"',
  'data_directory = "/var/lib/sdkwork/chat"',
  'log_directory = "/var/log/sdkwork/chat"',
  'runtime_directory = "/run/sdkwork/chat"',
  'password_file = "/etc/sdkwork/chat/database.secret"',
]) {
  assert.match(serverYamlTemplate, new RegExp(expectedText.replace(/[.*+?^${}()|[\]\\]/gu, '\\$&'), 'u'));
}
assert.doesNotMatch(serverYamlTemplate, /\/etc\/craw-chat\/default|\/var\/run\/craw-chat\/default/u);

const serverEnvTemplate = readText('deployments', 'templates', 'server.env.example');
for (const expectedText of [
  'SDKWORK_CHAT_CONFIG_FILE=/etc/sdkwork/chat/chat.toml',
  'SDKWORK_CHAT_DATA_DIR=/var/lib/sdkwork/chat',
  'SDKWORK_CHAT_LOG_DIR=/var/log/sdkwork/chat',
  'SDKWORK_CHAT_RUN_DIR=/run/sdkwork/chat',
  'SDKWORK_CHAT_ID_NODE_ID=1',
]) {
  assert.match(serverEnvTemplate, new RegExp(expectedText.replace(/[.*+?^${}()|[\]\\]/gu, '\\$&'), 'u'));
}
assert.doesNotMatch(serverEnvTemplate, /\/etc\/craw-chat\/default|\/var\/run\/craw-chat\/default/u);

const postgresqlTemplate = readText('deployments', 'templates', 'postgresql.yaml.example');
assert.match(postgresqlTemplate, /passwordFile: \/etc\/sdkwork\/chat\/database\.secret/u);
assert.doesNotMatch(postgresqlTemplate, /\/etc\/craw-chat\/default/u);

const systemdTemplate = readText('deployments', 'systemd', 'craw-chat-server.service');
for (const expectedText of [
  'WorkingDirectory=/opt/sdkwork/chat',
  'EnvironmentFile=/etc/sdkwork/chat/server.env',
  'ExecStart=/opt/sdkwork/chat/bin/craw-chat-server --config /etc/sdkwork/chat/chat.toml',
]) {
  assert.match(systemdTemplate, new RegExp(expectedText.replace(/[.*+?^${}()|[\]\\]/gu, '\\$&'), 'u'));
}
assert.doesNotMatch(systemdTemplate, /\/etc\/craw-chat|\/opt\/craw-chat/u);

for (const relativePath of [
  'bin/dev.ps1',
  'bin/dev.sh',
  'bin/build.ps1',
  'bin/build.sh',
  'bin/package.ps1',
  'bin/package.sh',
  'bin/start-prod.ps1',
  'bin/start-prod.sh',
]) {
  assertFile(relativePath);
  const scriptText = readText(...relativePath.split('/'));
  assert.doesNotMatch(scriptText, /Invoke-Expression|\biex\b|\beval\b/u, `${relativePath} should not use dynamic shell execution`);
}

const installServerPs1 = readText('bin', 'install-server.ps1');
assert.match(
  installServerPs1,
  /Resolve-ServerTemplatePath/u,
  'bin/install-server.ps1 should resolve templates from packaged config paths',
);
assert.match(
  installServerPs1,
  /config[\\/]chat\.toml\.example/u,
  'bin/install-server.ps1 should support server archive config/chat.toml.example',
);
assert.match(
  installServerPs1,
  /config[\\/]postgresql\.yaml\.example/u,
  'bin/install-server.ps1 should support server archive config/postgresql.yaml.example',
);
const installServerSh = readText('bin', 'install-server.sh');
assert.match(
  installServerSh,
  /resolve_template_path/u,
  'bin/install-server.sh should resolve templates from packaged config paths',
);
assert.match(
  installServerSh,
  /config\/chat\.toml\.example/u,
  'bin/install-server.sh should support server archive config/chat.toml.example',
);
assert.match(
  installServerSh,
  /config\/postgresql\.yaml\.example/u,
  'bin/install-server.sh should support server archive config/postgresql.yaml.example',
);

assertFile('.github/workflows/release-package.yml');
const workflowText = readText('.github', 'workflows', 'release-package.yml');
assert.doesNotMatch(
  workflowText,
  /inputs\.build_desktop\s*\|\|/u,
  'workflow must not coerce an explicit build_desktop=false input back to true',
);
assert.match(
  workflowText,
  /tag:\s*\r?\n\s+description: GitHub Release tag[^\r\n]*\r?\n\s+required: true\r?\n\s+default: v0\.1\.0/u,
  'workflow_dispatch tag input should be required and default to a release tag',
);
assert.match(
  workflowText,
  /package_version:\s*\r?\n\s+description: Package version[^\r\n]*\r?\n\s+required: true\r?\n\s+default: 0\.1\.0/u,
  'workflow_dispatch package_version input should be required and default to a package version',
);
assert.match(
  workflowText,
  /Build all production artifacts[\s\S]*?if: needs\.plan\.outputs\.build_server == 'true' && needs\.plan\.outputs\.build_desktop == 'true'[\s\S]*?pnpm release:build:production -- --target all --target-triple \$\{\{ matrix\.desktop_target \}\}/u,
  'workflow should build server and desktop production artifacts in one pass when both modes are selected',
);
assert.match(
  workflowText,
  /Build server production artifacts[\s\S]*?if: needs\.plan\.outputs\.build_server == 'true' && needs\.plan\.outputs\.build_desktop != 'true'[\s\S]*?pnpm release:build:production -- --target server/u,
  'workflow server-only build step should not run when desktop build is also selected',
);
assert.match(
  workflowText,
  /Build desktop installer artifacts[\s\S]*?if: needs\.plan\.outputs\.build_desktop == 'true' && needs\.plan\.outputs\.build_server != 'true'[\s\S]*?pnpm release:build:production -- --target desktop --target-triple \$\{\{ matrix\.desktop_target \}\}/u,
  'workflow desktop-only build step should not run when server build is also selected',
);
assert.match(
  workflowText,
  /Scope package manifest[\s\S]*?release-packages-manifest-\$\{\{ matrix\.platform \}\}-\$\{\{ matrix\.architecture \}\}\.json[\s\S]*?Remove-Item -LiteralPath \$aggregateManifest -Force/u,
  'workflow should scope the aggregate manifest per platform runner before uploading artifacts',
);
assert.match(
  workflowText,
  /find dist\/release-upload -mindepth 2 -type f -name 'SHA256SUMS' -delete/u,
  'workflow should remove per-runner SHA256SUMS files before creating the aggregate release checksum file',
);
assert.doesNotMatch(
  workflowText,
  /!\s+-name 'release-packages-manifest\.json'/u,
  'workflow should not exclude release package manifests from SHA256SUMS or GitHub Release assets',
);
for (const expectedText of [
  'workflow_dispatch:',
  'platform:',
  'architecture:',
  'deployment_mode:',
  'Plan package matrix',
  'fromJson(needs.plan.outputs.matrix)',
  'tags:',
  'v*',
  'contents: write',
  'ubuntu-latest',
  'windows-latest',
  'macos-latest',
  'pnpm release:plan -- --check',
  'pnpm release:build:production',
  'pnpm release:stage',
  'pnpm release:package',
  'pnpm release:validate',
  'gh release',
  'actions/upload-artifact',
  'actions/download-artifact',
  'merge-multiple: false',
  'sha256sum "$file"',
]) {
  assert.match(workflowText, new RegExp(expectedText.replace(/[.*+?^${}()|[\]\\]/gu, '\\$&'), 'u'), `workflow should include ${expectedText}`);
}

console.log('[craw-chat-release-package] contract passed');

function mkTempDir(prefix) {
  return mkdtempSafe(path.join(os.tmpdir(), prefix));
}

function mkdtempSafe(prefix) {
  const tempRoot = `${prefix}${process.pid}-${Date.now()}`;
  mkdirSync(tempRoot, { recursive: true });
  return tempRoot;
}

function writeFixture(root, relativePath, content) {
  const absolutePath = path.join(root, ...relativePath.split('/'));
  mkdirSync(path.dirname(absolutePath), { recursive: true });
  writeFileSync(absolutePath, content);
}
