import assert from 'node:assert/strict';
import {
  existsSync,
  mkdirSync,
  mkdtempSync,
  readdirSync,
  readFileSync,
  utimesSync,
  writeFileSync,
} from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import test from 'node:test';
import { pathToFileURL } from 'node:url';

import * as tauriCliRunner from '../../../scripts/run-tauri-cli.mjs';
import * as viteRuntimeLib from '../../../scripts/dev/vite-runtime-lib.mjs';

const appRoot = path.resolve(import.meta.dirname, '..');
const workspaceRoot = path.resolve(appRoot, '..', '..');

function read(relativePath) {
  return readFileSync(path.join(appRoot, relativePath), 'utf8');
}

function writePackageManifest(packageRoot, packageName = 'test-package') {
  mkdirSync(packageRoot, { recursive: true });
  writeFileSync(
    path.join(packageRoot, 'package.json'),
    JSON.stringify({ name: packageName, version: '0.0.0-test' }),
  );
}

function addNodeModulesPackage(appPackageRoot, packageName) {
  const packageRoot = path.join(appPackageRoot, 'node_modules', ...packageName.split('/'));
  writePackageManifest(packageRoot, packageName);
}

function listVerifiedWorktreeAdminRoots(currentWorkspaceRoot) {
  const worktreesRoot = path.basename(path.dirname(currentWorkspaceRoot)) === '.worktrees'
    ? path.resolve(currentWorkspaceRoot, '..')
    : path.join(currentWorkspaceRoot, '.worktrees');

  if (!existsSync(worktreesRoot)) {
    return [];
  }

  return readdirSync(worktreesRoot, { withFileTypes: true })
    .filter((entry) => entry.isDirectory() && !entry.name.startsWith('.'))
    .map((entry) => path.join(worktreesRoot, entry.name, 'apps', 'craw-chat-admin'))
    .filter((candidateRoot) => (
      candidateRoot !== appRoot
      && existsSync(path.join(candidateRoot, 'package.json'))
      && existsSync(path.join(candidateRoot, 'node_modules'))
    ));
}

const requiredPackages = [
  'sdkwork-craw-chat-admin-types',
  'sdkwork-craw-chat-admin-core',
  'sdkwork-craw-chat-admin-shell',
  'sdkwork-craw-chat-admin-auth',
  'sdkwork-craw-chat-admin-overview',
  'sdkwork-craw-chat-admin-tenants',
  'sdkwork-craw-chat-admin-users',
  'sdkwork-craw-chat-admin-conversations',
  'sdkwork-craw-chat-admin-messages',
  'sdkwork-craw-chat-admin-groups',
  'sdkwork-craw-chat-admin-moderation',
  'sdkwork-craw-chat-admin-automation',
  'sdkwork-craw-chat-admin-announcements',
  'sdkwork-craw-chat-admin-realtime',
  'sdkwork-craw-chat-admin-system',
  'sdkwork-craw-chat-admin-settings',
];

test('standalone craw-chat-admin app root exists', () => {
  assert.equal(existsSync(path.join(appRoot, 'package.json')), true);
  assert.equal(existsSync(path.join(appRoot, 'pnpm-workspace.yaml')), true);
  assert.equal(existsSync(path.join(appRoot, 'turbo.json')), true);
  assert.equal(existsSync(path.join(appRoot, 'src', 'App.tsx')), true);
  assert.equal(existsSync(path.join(appRoot, 'src', 'main.tsx')), true);
  assert.equal(existsSync(path.join(appRoot, 'src-tauri', 'Cargo.toml')), true);
  assert.equal(existsSync(path.join(appRoot, 'src-tauri', 'src', 'main.rs')), true);
});

test('app root exposes standalone browser and tauri scripts', () => {
  const packageJsonSource = read('package.json');
  const packageJson = JSON.parse(packageJsonSource);

  assert.equal(typeof packageJson.scripts?.dev, 'string');
  assert.equal(typeof packageJson.scripts?.build, 'string');
  assert.equal(typeof packageJson.scripts?.typecheck, 'string');
  assert.equal(typeof packageJson.scripts?.preview, 'string');
  assert.equal(typeof packageJson.scripts?.['tauri:dev'], 'string');
  assert.equal(typeof packageJson.scripts?.['tauri:build'], 'string');
  assert.match(packageJsonSource, /run-vite-cli\.mjs --configLoader native --host 0\.0\.0\.0/);
  assert.match(packageJsonSource, /run-vite-cli\.mjs build --configLoader native/);
  assert.match(packageJsonSource, /run-tsc-cli\.mjs --noEmit/);
  assert.match(packageJsonSource, /run-vite-cli\.mjs preview --configLoader native --host 0\.0\.0\.0 --port 4173 --strictPort/);
  assert.match(packageJsonSource, /run-tauri-cli\.mjs dev/);
  assert.match(packageJsonSource, /run-tauri-cli\.mjs build/);
  assert.match(packageJsonSource, /craw-chat-admin/);
});

test('workspace-level cli runner scripts mirror router-admin bootstrap conventions', () => {
  const runViteCli = path.join(workspaceRoot, 'scripts', 'dev', 'run-vite-cli.mjs');
  const runTscCli = path.join(workspaceRoot, 'scripts', 'dev', 'run-tsc-cli.mjs');
  const viteRuntimeLib = path.join(workspaceRoot, 'scripts', 'dev', 'vite-runtime-lib.mjs');
  const vitePreload = path.join(
    workspaceRoot,
    'scripts',
    'dev',
    'vite-windows-realpath-preload.mjs',
  );
  const runTauriCli = path.join(workspaceRoot, 'scripts', 'run-tauri-cli.mjs');
  const desktopTargets = path.join(
    workspaceRoot,
    'scripts',
    'release',
    'desktop-targets.mjs',
  );

  assert.equal(existsSync(runViteCli), true);
  assert.equal(existsSync(runTscCli), true);
  assert.equal(existsSync(viteRuntimeLib), true);
  assert.equal(existsSync(vitePreload), true);
  assert.equal(existsSync(runTauriCli), true);
  assert.equal(existsSync(desktopTargets), true);
  assert.match(readFileSync(runViteCli, 'utf8'), /resolveReadablePackageEntry/);
  assert.match(readFileSync(runTscCli, 'utf8'), /resolveReadableTypeScriptCliPath/);
  assert.match(readFileSync(runTauriCli, 'utf8'), /resolveReadableTauriCliPath/);
  assert.match(readFileSync(runTauriCli, 'utf8'), /Visual Studio 17 2022/);
  assert.match(readFileSync(runTauriCli, 'utf8'), /CARGO_BUILD_JOBS/);
  assert.match(readFileSync(runTauriCli, 'utf8'), /CMAKE_BUILD_PARALLEL_LEVEL/);
});

test('tauri cli runner defaults windows native build concurrency to one job', () => {
  const resolvedEnv = tauriCliRunner.withStableWindowsNativeBuildConcurrency(
    {},
    'win32',
  );

  assert.equal(resolvedEnv.CARGO_BUILD_JOBS, '1');
  assert.equal(resolvedEnv.NUM_JOBS, '1');
  assert.equal(resolvedEnv.CMAKE_BUILD_PARALLEL_LEVEL, '1');
});

test('tauri cli runner preserves explicit windows native build concurrency overrides', () => {
  const resolvedEnv = tauriCliRunner.withStableWindowsNativeBuildConcurrency(
    {
      CARGO_BUILD_JOBS: '4',
      NUM_JOBS: '6',
      CMAKE_BUILD_PARALLEL_LEVEL: '8',
    },
    'win32',
  );

  assert.equal(resolvedEnv.CARGO_BUILD_JOBS, '4');
  assert.equal(resolvedEnv.NUM_JOBS, '6');
  assert.equal(resolvedEnv.CMAKE_BUILD_PARALLEL_LEVEL, '8');
});

test('tauri cli runner extracts wix bundle artifacts from tauri output', () => {
  const sampleOutput = [
    'Running candle for "C:\\temp\\sdkwork-tauri-target\\craw-chat-admin\\release\\wix\\x64\\main.wxs"',
    'Running light to produce C:\\temp\\sdkwork-tauri-target\\craw-chat-admin\\release\\bundle\\msi\\Craw Chat Admin_0.1.0_x64_en-US.msi',
  ].join('\n');

  const artifacts = tauriCliRunner.extractWindowsWixBundleArtifacts(sampleOutput);

  assert.deepEqual(artifacts, {
    wixDir: 'C:\\temp\\sdkwork-tauri-target\\craw-chat-admin\\release\\wix\\x64',
    wixSourcePath: 'C:\\temp\\sdkwork-tauri-target\\craw-chat-admin\\release\\wix\\x64\\main.wxs',
    wixObjPath: 'C:\\temp\\sdkwork-tauri-target\\craw-chat-admin\\release\\wix\\x64\\main.wixobj',
    wixPdbPath: 'C:\\temp\\sdkwork-tauri-target\\craw-chat-admin\\release\\wix\\x64\\output.wixpdb',
    localePath: 'C:\\temp\\sdkwork-tauri-target\\craw-chat-admin\\release\\wix\\x64\\locale.wxl',
    msiPath: 'C:\\temp\\sdkwork-tauri-target\\craw-chat-admin\\release\\bundle\\msi\\Craw Chat Admin_0.1.0_x64_en-US.msi',
  });
});

test('tauri cli runner flags windows light failures for sval retry', () => {
  const sampleOutput = [
    'Running candle for "C:\\temp\\sdkwork-tauri-target\\craw-chat-admin\\release\\wix\\x64\\main.wxs"',
    'Running light to produce C:\\temp\\sdkwork-tauri-target\\craw-chat-admin\\release\\bundle\\msi\\Craw Chat Admin_0.1.0_x64_en-US.msi',
    'failed to bundle project `failed to run C:\\Users\\admin\\AppData\\Local\\tauri\\WixTools314\\light.exe`',
  ].join('\n');

  assert.equal(
    tauriCliRunner.shouldRetryWindowsMsiBundleWithSval({
      commandName: 'build',
      output: sampleOutput,
      platform: 'win32',
    }),
    true,
  );
  assert.equal(
    tauriCliRunner.shouldRetryWindowsMsiBundleWithSval({
      commandName: 'dev',
      output: sampleOutput,
      platform: 'win32',
    }),
    false,
  );
});

test('tauri cli runner keeps foreground stdio inherited to avoid nested windows spawn eperm failures', () => {
  const runTauriCli = readFileSync(
    path.join(workspaceRoot, 'scripts', 'run-tauri-cli.mjs'),
    'utf8',
  );

  assert.match(runTauriCli, /stdio:\s*'inherit'/);
  assert.doesNotMatch(runTauriCli, /\['inherit', 'pipe', 'pipe'\]/);
  assert.doesNotMatch(runTauriCli, /\['pipe', 'pipe', 'pipe'\]/);
});

test('tauri cli runner can infer wix bundle artifacts from the cargo target dir without captured output', () => {
  const sandboxRoot = mkdtempSync(path.join(os.tmpdir(), 'craw-chat-admin-wix-artifacts-'));
  const targetDir = path.join(sandboxRoot, 'sdkwork-tauri-target', 'craw-chat-admin');
  const wixDir = path.join(targetDir, 'release', 'wix', 'x64');
  const buildStartedAt = Date.now() - 1_000;

  mkdirSync(wixDir, { recursive: true });
  mkdirSync(path.join(targetDir, 'release', 'bundle', 'msi'), { recursive: true });
  writeFileSync(
    path.join(wixDir, 'main.wxs'),
    [
      '<Wix xmlns="http://schemas.microsoft.com/wix/2006/wi">',
      '  <Product Name="Craw Chat Admin" Version="0.1.0">',
      '  </Product>',
      '</Wix>',
    ].join('\n'),
  );
  writeFileSync(path.join(wixDir, 'main.wixobj'), 'wix-object');
  writeFileSync(
    path.join(wixDir, 'locale.wxl'),
    '<WixLocalization Culture="en-us" xmlns="http://schemas.microsoft.com/wix/2006/localization" />',
  );
  writeFileSync(path.join(wixDir, 'output.wixpdb'), 'wix-pdb');
  utimesSync(path.join(wixDir, 'main.wxs'), new Date(), new Date());
  utimesSync(path.join(wixDir, 'main.wixobj'), new Date(), new Date());

  const artifacts = tauriCliRunner.resolveWindowsWixBundleArtifactsFromTargetDir({
    targetDir,
    platform: 'win32',
    buildStartedAt,
  });

  assert.deepEqual(artifacts, {
    targetDir,
    wixDir,
    wixSourcePath: path.join(wixDir, 'main.wxs'),
    wixObjPath: path.join(wixDir, 'main.wixobj'),
    wixPdbPath: path.join(wixDir, 'output.wixpdb'),
    localePath: path.join(wixDir, 'locale.wxl'),
    msiPath: path.join(
      targetDir,
      'release',
      'bundle',
      'msi',
      'Craw Chat Admin_0.1.0_x64_en-US.msi',
    ),
    architecture: 'x64',
  });
});

test('tauri cli runner ignores stale wix bundle artifacts from earlier builds', () => {
  const sandboxRoot = mkdtempSync(path.join(os.tmpdir(), 'craw-chat-admin-wix-stale-'));
  const targetDir = path.join(sandboxRoot, 'sdkwork-tauri-target', 'craw-chat-admin');
  const wixDir = path.join(targetDir, 'release', 'wix', 'x64');

  mkdirSync(wixDir, { recursive: true });
  writeFileSync(
    path.join(wixDir, 'main.wxs'),
    [
      '<Wix xmlns="http://schemas.microsoft.com/wix/2006/wi">',
      '  <Product Name="Craw Chat Admin" Version="0.1.0">',
      '  </Product>',
      '</Wix>',
    ].join('\n'),
  );
  writeFileSync(path.join(wixDir, 'main.wixobj'), 'wix-object');
  writeFileSync(
    path.join(wixDir, 'locale.wxl'),
    '<WixLocalization Culture="en-us" xmlns="http://schemas.microsoft.com/wix/2006/localization" />',
  );
  writeFileSync(path.join(wixDir, 'output.wixpdb'), 'wix-pdb');

  const staleTimestamp = new Date(Date.now() - 120_000);
  utimesSync(path.join(wixDir, 'main.wxs'), staleTimestamp, staleTimestamp);
  utimesSync(path.join(wixDir, 'main.wixobj'), staleTimestamp, staleTimestamp);

  const artifacts = tauriCliRunner.resolveWindowsWixBundleArtifactsFromTargetDir({
    targetDir,
    platform: 'win32',
    buildStartedAt: Date.now() - 15_000,
  });

  assert.equal(artifacts, null);
});

test('workspace donor roots include at least one available sibling or worktree dependency donor', () => {
  const donorRoots = viteRuntimeLib.resolveWorkspaceDonorRoots(appRoot);
  const expectedDonorCandidates = [
    ...listVerifiedWorktreeAdminRoots(workspaceRoot),
    path.resolve(workspaceRoot, '..', 'claw-studio'),
  ].filter((candidateRoot) => (
    existsSync(path.join(candidateRoot, 'package.json'))
    && existsSync(path.join(candidateRoot, 'node_modules'))
  ));

  assert.notEqual(expectedDonorCandidates.length, 0, 'test requires at least one available donor root');
  assert.equal(
    expectedDonorCandidates.some((candidateRoot) => donorRoots.includes(candidateRoot)),
    true,
    'official app should discover at least one available sibling or worktree donor root',
  );
});

test('workspace runtime library can materialize a local node_modules donor link', () => {
  const sandboxRoot = mkdtempSync(path.join(os.tmpdir(), 'craw-chat-admin-runtime-lib-'));
  const isolatedAppRoot = path.join(sandboxRoot, 'official-app');
  const donorAppRoot = path.join(sandboxRoot, 'donor-app');

  mkdirSync(isolatedAppRoot, { recursive: true });
  mkdirSync(donorAppRoot, { recursive: true });
  writeFileSync(path.join(isolatedAppRoot, 'package.json'), '{"name":"official-app"}');
  writeFileSync(path.join(donorAppRoot, 'package.json'), '{"name":"donor-app"}');
  mkdirSync(path.join(donorAppRoot, 'node_modules', 'react'), { recursive: true });
  writeFileSync(
    path.join(donorAppRoot, 'node_modules', 'react', 'package.json'),
    '{"name":"react","version":"0.0.0-test"}',
  );

  const localNodeModulesPath = viteRuntimeLib.ensureLocalNodeModules({
    appRoot: isolatedAppRoot,
    donorRoots: [donorAppRoot],
  });

  assert.equal(localNodeModulesPath, path.join(isolatedAppRoot, 'node_modules'));
  assert.equal(existsSync(localNodeModulesPath), true);
  assert.equal(
    existsSync(path.join(isolatedAppRoot, 'node_modules', 'react', 'package.json')),
    true,
  );
});

test('workspace runtime library replaces incomplete local node_modules when required packages are missing', () => {
  const sandboxRoot = mkdtempSync(path.join(os.tmpdir(), 'craw-chat-admin-runtime-lib-stale-'));
  const isolatedAppRoot = path.join(sandboxRoot, 'official-app');
  const donorAppRoot = path.join(sandboxRoot, 'donor-app');

  mkdirSync(path.join(isolatedAppRoot, 'node_modules', '.pnpm'), { recursive: true });
  mkdirSync(path.join(isolatedAppRoot, 'node_modules', '.bin'), { recursive: true });
  mkdirSync(donorAppRoot, { recursive: true });
  writeFileSync(path.join(isolatedAppRoot, 'package.json'), '{"name":"official-app"}');
  writeFileSync(path.join(donorAppRoot, 'package.json'), '{"name":"donor-app"}');
  mkdirSync(path.join(donorAppRoot, 'node_modules', 'react'), { recursive: true });
  writeFileSync(
    path.join(donorAppRoot, 'node_modules', 'react', 'package.json'),
    '{"name":"react","version":"0.0.0-test"}',
  );

  const localNodeModulesPath = viteRuntimeLib.ensureLocalNodeModules({
    appRoot: isolatedAppRoot,
    donorRoots: [donorAppRoot],
    requiredPackages: ['react'],
  });

  assert.equal(localNodeModulesPath, path.join(isolatedAppRoot, 'node_modules'));
  assert.equal(
    existsSync(path.join(isolatedAppRoot, 'node_modules', 'react', 'package.json')),
    true,
  );
});

test('workspace runtime library prefers donor roots that satisfy all required packages', () => {
  const sandboxRoot = mkdtempSync(path.join(os.tmpdir(), 'craw-chat-admin-runtime-lib-complete-'));
  const isolatedAppRoot = path.join(sandboxRoot, 'official-app');
  const incompleteDonorAppRoot = path.join(sandboxRoot, 'incomplete-donor-app');
  const completeDonorAppRoot = path.join(sandboxRoot, 'complete-donor-app');
  const requiredPackages = ['react', 'react-router-dom', '@sdkwork/ui-pc-react'];

  mkdirSync(isolatedAppRoot, { recursive: true });
  mkdirSync(incompleteDonorAppRoot, { recursive: true });
  mkdirSync(completeDonorAppRoot, { recursive: true });
  writeFileSync(path.join(isolatedAppRoot, 'package.json'), '{"name":"official-app"}');
  writeFileSync(path.join(incompleteDonorAppRoot, 'package.json'), '{"name":"incomplete-donor-app"}');
  writeFileSync(path.join(completeDonorAppRoot, 'package.json'), '{"name":"complete-donor-app"}');

  addNodeModulesPackage(incompleteDonorAppRoot, 'react');
  for (const packageName of requiredPackages) {
    addNodeModulesPackage(completeDonorAppRoot, packageName);
  }

  const localNodeModulesPath = viteRuntimeLib.ensureLocalNodeModules({
    appRoot: isolatedAppRoot,
    donorRoots: [incompleteDonorAppRoot, completeDonorAppRoot],
    requiredPackages,
  });

  assert.equal(localNodeModulesPath, path.join(isolatedAppRoot, 'node_modules'));
  assert.equal(
    existsSync(path.join(localNodeModulesPath, 'react', 'package.json')),
    true,
  );
  assert.equal(
    existsSync(path.join(localNodeModulesPath, 'react-router-dom', 'package.json')),
    true,
  );
  assert.equal(
    existsSync(path.join(localNodeModulesPath, '@sdkwork', 'ui-pc-react', 'package.json')),
    true,
  );
});

test('required packages exist under packages/', () => {
  for (const packageName of requiredPackages) {
    assert.equal(
      existsSync(path.join(appRoot, 'packages', packageName, 'package.json')),
      true,
      `missing ${packageName}`,
    );
  }
});

test('root app stays thin and mounts the shell package', () => {
  const app = read('src/App.tsx');
  const main = read('src/main.tsx');

  assert.match(app, /sdkwork-craw-chat-admin-shell/);
  assert.match(app, /AppRoot/);
  assert.match(main, /bootstrapShellRuntime/);
  assert.match(main, /@sdkwork\/ui-pc-react\/styles\.css/);
  assert.doesNotMatch(app, /ConversationsPage|MessagesPage|GroupsPage|ModerationPage/);
});

test('core route manifest formalizes IM product modules', () => {
  const routeManifest = read('packages/sdkwork-craw-chat-admin-core/src/routeManifest.ts');
  const coreIndex = read('packages/sdkwork-craw-chat-admin-core/src/index.tsx');

  assert.match(routeManifest, /sdkwork-craw-chat-admin-overview/);
  assert.match(routeManifest, /sdkwork-craw-chat-admin-conversations/);
  assert.match(routeManifest, /sdkwork-craw-chat-admin-messages/);
  assert.match(routeManifest, /sdkwork-craw-chat-admin-moderation/);
  assert.match(routeManifest, /sdkwork-craw-chat-admin-system/);
  assert.match(routeManifest, /requiredPermissions:/);
  assert.match(routeManifest, /capabilityTags:/);
  assert.match(routeManifest, /strategy: 'lazy'/);
  assert.match(coreIndex, /adminRouteManifest/);
  assert.doesNotMatch(
    routeManifest,
    /sdkwork-router-admin|sdkwork-craw-chat-admin-(apirouter|traffic|catalog|coupons|commercial|pricing)/,
  );
});

test('shell owns router and auth isolation', () => {
  const routes = read('packages/sdkwork-craw-chat-admin-shell/src/application/router/AppRoutes.tsx');
  const layout = read('packages/sdkwork-craw-chat-admin-shell/src/application/layouts/MainLayout.tsx');
  const shellHostStyles = read('packages/sdkwork-craw-chat-admin-shell/src/styles/shell-host.css');

  assert.match(routes, /AdminLoginPage/);
  assert.match(routes, /ROUTE_PATHS\.LOGIN/);
  assert.match(routes, /ROUTE_PATHS\.REGISTER/);
  assert.match(routes, /ROUTE_PATHS\.FORGOT_PASSWORD/);
  assert.match(layout, /Sidebar/);
  assert.match(layout, /AppHeader/);
  assert.match(layout, /data-sdk-shell="craw-chat-admin-desktop"/);
  assert.match(shellHostStyles, /\[data-sdk-shell='craw-chat-admin-desktop'\]/);
  assert.doesNotMatch(layout, /router-admin-desktop/);
  assert.doesNotMatch(shellHostStyles, /router-admin-desktop/);
  assert.doesNotMatch(layout, /Craw Chat Admin|API Router|Catalog/);
});

test('auth login page only prefills sandbox credentials from explicit dev environment variables', () => {
  const authPage = read('packages/sdkwork-craw-chat-admin-auth/src/index.tsx');

  assert.match(authPage, /VITE_ADMIN_SANDBOX_EMAIL/);
  assert.match(authPage, /VITE_ADMIN_SANDBOX_PASSWORD/);
  assert.doesNotMatch(authPage, /admin@sdkwork\.local|ChangeMe123!/);
});

test('vite config serves the admin shell from /admin/', () => {
  const viteConfig = read('vite.config.ts');

  assert.match(viteConfig, /base:\s*'\/admin\/'/);
  assert.match(viteConfig, /port:\s*5173/);
  assert.match(viteConfig, /strictPort:\s*true/);
});

test('tauri config uses craw-chat desktop asset orchestration instead of router-admin residue', () => {
  const tauriConfig = JSON.parse(read('src-tauri/tauri.conf.json'));
  const desktopAssetBuildScript = path.join(
    workspaceRoot,
    'scripts',
    'build-craw-chat-desktop-assets.mjs',
  );

  assert.equal(tauriConfig.identifier, 'com.sdkwork.craw-chat.admin');
  assert.equal(
    tauriConfig.build?.beforeBuildCommand,
    'node ../../scripts/build-craw-chat-desktop-assets.mjs',
  );
  assert.equal(existsSync(desktopAssetBuildScript), true);
  assert.equal(
    tauriConfig.bundle?.resources?.['../dist-portal/'],
    'embedded-sites/portal/',
  );
  assert.equal(
    Object.prototype.hasOwnProperty.call(tauriConfig.bundle?.resources ?? {}, '../../craw-chat-portal/dist-desktop/'),
    false,
  );
  assert.equal(
    Object.prototype.hasOwnProperty.call(tauriConfig.bundle?.resources ?? {}, '../../craw-chat-portal/dist/'),
    false,
  );
  assert.equal(
    Object.prototype.hasOwnProperty.call(tauriConfig.bundle?.resources ?? {}, '../../sdkwork-router-portal/dist/'),
    false,
  );
});

test('tauri desktop host keeps runtime compatibility crates inside the craw-chat workspace', () => {
  const cargoToml = read('src-tauri/Cargo.toml');
  const tauriMain = read('src-tauri/src/main.rs');
  const localConfigCrate = path.join(workspaceRoot, 'crates', 'sdkwork-api-config');
  const localRuntimeCrate = path.join(workspaceRoot, 'crates', 'sdkwork-api-product-runtime');

  assert.match(
    cargoToml,
    /sdkwork-api-config = \{ path = "\.\.\/\.\.\/\.\.\/crates\/sdkwork-api-config" \}/,
  );
  assert.match(
    cargoToml,
    /sdkwork-api-product-runtime = \{ path = "\.\.\/\.\.\/\.\.\/crates\/sdkwork-api-product-runtime" \}/,
  );
  assert.equal(existsSync(path.join(localConfigCrate, 'Cargo.toml')), true);
  assert.equal(existsSync(path.join(localConfigCrate, 'src', 'lib.rs')), true);
  assert.equal(existsSync(path.join(localRuntimeCrate, 'Cargo.toml')), true);
  assert.equal(existsSync(path.join(localRuntimeCrate, 'src', 'lib.rs')), true);
  assert.match(tauriMain, /choose_site_dir_for_runtime/);
  assert.match(tauriMain, /cfg!\(debug_assertions\)/);
  assert.match(tauriMain, /bundle\.resources/);
  assert.doesNotMatch(cargoToml, /sdkwork-api-router/);
});

test('desktop asset build script mirrors router-admin pnpm launch safety while targeting craw-chat apps', () => {
  const desktopAssetBuildScript = readFileSync(
    path.join(workspaceRoot, 'scripts', 'build-craw-chat-desktop-assets.mjs'),
    'utf8',
  );
  const pnpmLaunchLib = path.join(workspaceRoot, 'scripts', 'dev', 'pnpm-launch-lib.mjs');

  assert.equal(existsSync(pnpmLaunchLib), true);
  assert.match(desktopAssetBuildScript, /pnpmProcessSpec/);
  assert.match(desktopAssetBuildScript, /craw-chat-admin/);
  assert.match(desktopAssetBuildScript, /craw-chat-portal/);
  assert.match(desktopAssetBuildScript, /dist-portal/);
  assert.match(desktopAssetBuildScript, /assertPortalDistReleaseSafe/);
  assert.doesNotMatch(desktopAssetBuildScript, /spawn\(step\.command, step\.args, \{[\s\S]*shell:\s*false/);
});

test('desktop asset build script syncs the portal release dist into dist-portal and removes stale files', async () => {
  const desktopAssetBuildModule = await import(
    pathToFileURL(path.join(workspaceRoot, 'scripts', 'build-craw-chat-desktop-assets.mjs')).href
  );

  const sandboxRoot = mkdtempSync(path.join(os.tmpdir(), 'craw-chat-desktop-assets-'));
  const adminDistRoot = path.join(sandboxRoot, 'apps', 'craw-chat-admin', 'dist');
  const portalDistRoot = path.join(sandboxRoot, 'apps', 'craw-chat-portal', 'dist');
  const adminDistPortalRoot = path.join(sandboxRoot, 'apps', 'craw-chat-admin', 'dist-portal');

  mkdirSync(adminDistRoot, { recursive: true });
  mkdirSync(path.join(portalDistRoot, '__vendor__', 'sdkwork-craw-chat-sdk'), { recursive: true });
  mkdirSync(path.join(portalDistRoot, '__vendor__', 'sdkwork-craw-chat-backend-sdk'), { recursive: true });
  mkdirSync(path.join(portalDistRoot, 'src'), { recursive: true });
  mkdirSync(path.join(portalDistRoot, 'packages', 'craw-chat-portal-core'), { recursive: true });
  mkdirSync(adminDistPortalRoot, { recursive: true });

  writeFileSync(path.join(adminDistRoot, 'index.html'), '<!doctype html><title>admin-dist</title>');
  writeFileSync(path.join(portalDistRoot, 'index.html'), '<!doctype html><title>portal-dist</title>');
  writeFileSync(
    path.join(portalDistRoot, '__vendor__', 'sdkwork-craw-chat-sdk', 'index.js'),
    'export const source = "portal-dist-vendor";',
  );
  writeFileSync(
    path.join(portalDistRoot, '__vendor__', 'sdkwork-craw-chat-backend-sdk', 'index.js'),
    'export const source = "portal-dist-backend-vendor";',
  );
  writeFileSync(
    path.join(portalDistRoot, 'src', 'main.js'),
    'export const source = "portal-dist-main";',
  );
  writeFileSync(
    path.join(portalDistRoot, 'packages', 'craw-chat-portal-core', 'index.js'),
    'export const source = "portal-dist-package";',
  );
  writeFileSync(path.join(adminDistPortalRoot, 'stale.txt'), 'stale-desktop-artifact');

  await desktopAssetBuildModule.syncPortalDesktopAssets({
    workspaceRoot: sandboxRoot,
  });

  assert.equal(existsSync(path.join(adminDistPortalRoot, 'stale.txt')), false);
  assert.equal(
    readFileSync(path.join(adminDistPortalRoot, 'index.html'), 'utf8'),
    '<!doctype html><title>portal-dist</title>',
  );
  assert.equal(
    readFileSync(
      path.join(adminDistPortalRoot, '__vendor__', 'sdkwork-craw-chat-sdk', 'index.js'),
      'utf8',
    ),
    'export const source = "portal-dist-vendor";',
  );
  assert.equal(
    readFileSync(
      path.join(adminDistPortalRoot, '__vendor__', 'sdkwork-craw-chat-backend-sdk', 'index.js'),
      'utf8',
    ),
    'export const source = "portal-dist-backend-vendor";',
  );
  assert.equal(
    readFileSync(path.join(adminDistPortalRoot, 'src', 'main.js'), 'utf8'),
    'export const source = "portal-dist-main";',
  );
  assert.equal(
    readFileSync(
      path.join(adminDistPortalRoot, 'packages', 'craw-chat-portal-core', 'index.js'),
      'utf8',
    ),
    'export const source = "portal-dist-package";',
  );

  await assert.doesNotReject(() => desktopAssetBuildModule.assertDesktopEmbeddedSitesReady({
    workspaceRoot: sandboxRoot,
  }));
});

test('desktop asset build script rejects portal bundles that are missing vendored sdk assets', async () => {
  const desktopAssetBuildModule = await import(
    pathToFileURL(path.join(workspaceRoot, 'scripts', 'build-craw-chat-desktop-assets.mjs')).href
  );

  const sandboxRoot = mkdtempSync(path.join(os.tmpdir(), 'craw-chat-desktop-assets-invalid-'));
  const adminDistRoot = path.join(sandboxRoot, 'apps', 'craw-chat-admin', 'dist');
  const portalDistRoot = path.join(sandboxRoot, 'apps', 'craw-chat-portal', 'dist');

  mkdirSync(adminDistRoot, { recursive: true });
  mkdirSync(portalDistRoot, { recursive: true });
  writeFileSync(path.join(adminDistRoot, 'index.html'), '<!doctype html><title>admin-dist</title>');
  writeFileSync(path.join(portalDistRoot, 'index.html'), '<!doctype html><title>portal-dist</title>');

  await assert.rejects(
    () => desktopAssetBuildModule.assertDesktopEmbeddedSitesReady({
      workspaceRoot: sandboxRoot,
    }),
    /portal site build required asset is missing: .*sdkwork-craw-chat-sdk[\\/]index\.js/i,
  );
});

test('desktop asset build script rejects embedded portal bundles that contain retired demo credentials', async () => {
  const desktopAssetBuildModule = await import(
    pathToFileURL(path.join(workspaceRoot, 'scripts', 'build-craw-chat-desktop-assets.mjs')).href
  );

  const sandboxRoot = mkdtempSync(path.join(os.tmpdir(), 'craw-chat-desktop-assets-leaky-'));
  const adminDistRoot = path.join(sandboxRoot, 'apps', 'craw-chat-admin', 'dist');
  const portalDistRoot = path.join(sandboxRoot, 'apps', 'craw-chat-portal', 'dist');

  mkdirSync(adminDistRoot, { recursive: true });
  mkdirSync(path.join(portalDistRoot, '__vendor__', 'sdkwork-craw-chat-sdk'), { recursive: true });
  mkdirSync(path.join(portalDistRoot, '__vendor__', 'sdkwork-craw-chat-backend-sdk'), { recursive: true });
  mkdirSync(path.join(portalDistRoot, 'packages', 'craw-chat-portal-auth', 'src'), {
    recursive: true,
  });

  writeFileSync(path.join(adminDistRoot, 'index.html'), '<!doctype html><title>admin-dist</title>');
  writeFileSync(path.join(portalDistRoot, 'index.html'), '<!doctype html><title>portal-dist</title>');
  writeFileSync(
    path.join(portalDistRoot, '__vendor__', 'sdkwork-craw-chat-sdk', 'index.js'),
    'export const source = "portal-dist-vendor";',
  );
  writeFileSync(
    path.join(portalDistRoot, '__vendor__', 'sdkwork-craw-chat-backend-sdk', 'index.js'),
    'export const source = "portal-dist-backend-vendor";',
  );
  writeFileSync(
    path.join(portalDistRoot, 'packages', 'craw-chat-portal-auth', 'src', 'index.js'),
    'export const leakedPassword = "Portal#2026";',
  );

  await desktopAssetBuildModule.syncPortalDesktopAssets({
    workspaceRoot: sandboxRoot,
  });

  await assert.rejects(
    () => desktopAssetBuildModule.assertDesktopEmbeddedSitesReady({
      workspaceRoot: sandboxRoot,
    }),
    /Portal#2026/,
  );
});

test('vite cli runner requires the shared ui package before reusing donor node_modules', () => {
  const runViteCli = readFileSync(
    path.join(workspaceRoot, 'scripts', 'dev', 'run-vite-cli.mjs'),
    'utf8',
  );

  assert.match(runViteCli, /'@sdkwork\/ui-pc-react'/);
});

test('workspace cli runners prebuild sdkwork ui dist before build and typecheck', () => {
  const runViteCli = readFileSync(
    path.join(workspaceRoot, 'scripts', 'dev', 'run-vite-cli.mjs'),
    'utf8',
  );
  const runTscCli = readFileSync(
    path.join(workspaceRoot, 'scripts', 'dev', 'run-tsc-cli.mjs'),
    'utf8',
  );

  assert.match(runViteCli, /ensureSdkworkUiDist/);
  assert.match(runTscCli, /ensureSdkworkUiDist/);
});

test('vite config resolves @sdkwork/ui-pc-react through dist entries after prebuild', () => {
  const viteConfig = read('vite.config.ts');

  assert.match(viteConfig, /sdkwork-ui-pc-react'\)/);
  assert.match(viteConfig, /path\.join\(packageRoot, 'dist', entryPath\)/);
  assert.match(viteConfig, /sdkwork-ui\.css/);
  assert.match(viteConfig, /theme\.js/);
  assert.match(viteConfig, /components-ui\.js/);
  assert.match(viteConfig, /ui-feedback\.js/);
  assert.match(viteConfig, /patterns-app-shell\.js/);
  assert.match(viteConfig, /patterns-desktop-shell\.js/);
  assert.match(viteConfig, /index\.js/);
  assert.doesNotMatch(viteConfig, /path\.join\(packageRoot, 'src', \.\.\.sourceSegments\)/);
});

test('tsconfig mirrors dist-backed ui type shims for root and grouped ui entries', () => {
  const tsconfig = read('tsconfig.json');
  const uiShim = read('src/types/sdkwork-ui-pc-react-shim.d.ts');

  assert.equal(existsSync(path.join(appRoot, 'src', 'types', 'sdkwork-ui-pc-react-shim.d.ts')), true);
  assert.match(
    tsconfig,
    /"@sdkwork\/ui-pc-react":\s*\["src\/types\/sdkwork-ui-pc-react-shim\.d\.ts"\]/,
  );
  assert.match(
    tsconfig,
    /"@sdkwork\/ui-pc-react\/theme":\s*\[\s*"[^"]*sdkwork-ui-pc-react\/dist\/theme\/index\.d\.ts"\s*\]/,
  );
  assert.match(
    tsconfig,
    /"@sdkwork\/ui-pc-react\/\*":\s*\[\s*"[^"]*sdkwork-ui-pc-react\/dist\/\*"\s*\]/,
  );
  assert.match(
    tsconfig,
    /"@sdkwork\/ui-pc-react\/styles\.css":\s*\[\s*"[^"]*sdkwork-ui-pc-react\/dist\/sdkwork-ui\.css"\s*\]/,
  );
  assert.match(uiShim, /export \* from '[^']*sdkwork-ui-pc-react\/dist\/index';/);
});

test('ui shim and tsconfig path targets resolve to real sdkwork-ui dist assets', () => {
  const shimPath = path.join(appRoot, 'src', 'types', 'sdkwork-ui-pc-react-shim.d.ts');
  const shimSource = readFileSync(shimPath, 'utf8');
  const shimMatch = shimSource.match(/export \* from '([^']+)'/);
  const shimTargetPath = path.resolve(path.dirname(shimPath), shimMatch[1]);
  assert.notEqual(shimMatch, null, 'shim should re-export the sdkwork-ui dist index');
  assert.equal(
    existsSync(shimTargetPath) || existsSync(`${shimTargetPath}.js`) || existsSync(`${shimTargetPath}.d.ts`),
    true,
    'shim target should resolve to a real sdkwork-ui dist index file',
  );

  const tsconfig = JSON.parse(read('tsconfig.json'));
  const pathMappings = [
    ['@sdkwork/ui-pc-react/theme', 'theme index'],
    ['@sdkwork/ui-pc-react/*', 'wildcard dist root'],
    ['@sdkwork/ui-pc-react/styles.css', 'stylesheet asset'],
  ];

  for (const [key, description] of pathMappings) {
    const target = tsconfig.compilerOptions?.paths?.[key]?.[0];
    assert.equal(typeof target, 'string', `${key} should declare a ${description} path mapping`);
    const normalizedTarget = target.replace(/\/\*$/, '').replace(/\*$/, '');
    assert.equal(
      existsSync(path.resolve(appRoot, normalizedTarget)),
      true,
      `${key} should resolve to a real ${description} path`,
    );
  }
});

test('theme establishes a dedicated admin visual system', () => {
  const theme = read('src/theme.css');

  assert.match(theme, /--admin-font-sans:/);
  assert.match(theme, /--admin-font-display:/);
  assert.match(theme, /body::before/);
  assert.doesNotMatch(theme, /font-family:\s*ui-sans-serif,\s*system-ui/);
});

test('core i18n surface stays focused on craw-chat admin runtime concerns', () => {
  const i18n = read('packages/sdkwork-craw-chat-admin-core/src/i18n.tsx');

  assert.match(i18n, /export const ADMIN_LOCALE_OPTIONS/);
  assert.match(i18n, /export function AdminI18nProvider/);
  assert.match(i18n, /export function useAdminI18n/);
  assert.doesNotMatch(i18n, /ADMIN_ZH_APIROUTER_SURFACE_TRANSLATIONS/);
  assert.doesNotMatch(i18n, /ADMIN_ZH_COMMERCIAL_ACCOUNT_TRANSLATIONS/);
  assert.doesNotMatch(i18n, /ADMIN_ZH_PRICING_TRANSLATIONS/);
});

test('core workbench consumes the formal admin SDK boundary and avoids router-admin commerce preload and catalog language', () => {
  const packageJsonSource = read('package.json');
  const workbench = read('packages/sdkwork-craw-chat-admin-core/src/workbench.tsx');
  const workbenchActions = read('packages/sdkwork-craw-chat-admin-core/src/workbenchActions.ts');
  const workbenchSnapshot = read('packages/sdkwork-craw-chat-admin-core/src/workbenchSnapshot.ts');

  assert.match(packageJsonSource, /@sdkwork\/craw-chat-admin-sdk/);
  assert.doesNotMatch(packageJsonSource, /sdkwork-craw-chat-admin-admin-api/);
  assert.match(workbench, /@sdkwork\/craw-chat-admin-sdk/);
  assert.match(workbenchActions, /@sdkwork\/craw-chat-admin-sdk/);
  assert.match(workbenchSnapshot, /@sdkwork\/craw-chat-admin-sdk/);
  assert.doesNotMatch(
    workbench,
    /listCoupons|listRecentCommerceOrders|listCommercePaymentEvents|listMarketingCoupon|listCommercialAccount|listCommercialPricing/,
  );
  assert.doesNotMatch(
    workbenchActions,
    /handleSaveCoupon|handleToggleCoupon|handleDeleteCoupon|handleUpdateMarketingCouponTemplateStatus|handleUpdateMarketingCampaignStatus|handleUpdateMarketingCampaignBudgetStatus|handleUpdateMarketingCouponCodeStatus|handleCreateCommercialPricingPlan|handleCreateCommercialPricingRate|handleUpdateCommercialPricingPlan|handleCloneCommercialPricingPlan|handlePublishCommercialPricingPlan|handleScheduleCommercialPricingPlan|handleRetireCommercialPricingPlan|handleSynchronizeCommercialPricingLifecycle|handleUpdateCommercialPricingRate/,
  );
  assert.doesNotMatch(
    workbenchSnapshot,
    /coupon-repository|No model catalog entries|routing catalog|Create or upsert models in Catalog|credentials in Catalog/,
  );
});

test('workspace snapshot stays focused on IM runtime data and excludes dormant commerce payloads', () => {
  const typesSource = read('packages/sdkwork-craw-chat-admin-types/src/index.ts');
  const workbench = read('packages/sdkwork-craw-chat-admin-core/src/workbench.tsx');
  const workbenchSnapshot = read('packages/sdkwork-craw-chat-admin-core/src/workbenchSnapshot.ts');

  assert.match(typesSource, /marketingCampaigns:/);
  assert.doesNotMatch(
    typesSource,
    /export (interface|type) (CouponRecord|MarketingBenefitKind|MarketingStackingPolicy|MarketingSubjectScope|CouponTemplateStatus|CouponDistributionKind|CampaignBudgetStatus|CouponCodeStatus|CouponReservationStatus|CouponRedemptionStatus|CouponRollbackType|CouponRollbackStatus|CouponBenefitSpec|CouponRestrictionSpec|CouponTemplateRecord|CampaignBudgetRecord|CouponCodeRecord|CouponReservationRecord|CouponRedemptionRecord|CouponRollbackRecord|CommerceOrderStatus|CommerceSettlementStatus|CommercePaymentEventType|CommercePaymentEventProcessingStatus|CommerceOrderRecord|PaymentMethodRecord|PaymentMethodCredentialBindingRecord|CommercePaymentEventRecord|CommerceOrderAuditRecord|CommercialAccountType|CommercialAccountStatus|CommercialAccountBenefitType|CommercialAccountBenefitSourceType|CommercialAccountBenefitLotStatus|CommercialAccountHoldStatus|CommercialRequestSettlementStatus|CommercialAccountLedgerEntryType|CommercialAccountRecord|CommercialAccountLotBalanceSnapshot|CommercialAccountBalanceSnapshot|CommercialAccountSummary|CommercialAccountBenefitLotRecord|CommercialAccountHoldRecord|CommercialRequestSettlementRecord|CommercialAccountLedgerEntryRecord|CommercialAccountLedgerAllocationRecord|CommercialAccountLedgerHistoryEntry|CommercialPricingPlanRecord|CommercialPricingChargeUnit|CommercialPricingMethod|CommercialPricingRateRecord|CommercialPricingPlanCreateInput|CommercialPricingRateCreateInput|CommercialPricingLifecycleSynchronizationReport)/,
  );
  assert.doesNotMatch(typesSource, /export \* from '\.\/commercePayments';/);
  assert.doesNotMatch(typesSource, /coupons: CouponRecord\[]/);
  assert.doesNotMatch(typesSource, /couponTemplates: CouponTemplateRecord\[]/);
  assert.doesNotMatch(typesSource, /campaignBudgets: CampaignBudgetRecord\[]/);
  assert.doesNotMatch(typesSource, /couponCodes: CouponCodeRecord\[]/);
  assert.doesNotMatch(typesSource, /couponReservations: CouponReservationRecord\[]/);
  assert.doesNotMatch(typesSource, /couponRedemptions: CouponRedemptionRecord\[]/);
  assert.doesNotMatch(typesSource, /couponRollbacks: CouponRollbackRecord\[]/);
  assert.doesNotMatch(typesSource, /commerceOrders: CommerceOrderRecord\[]/);
  assert.doesNotMatch(typesSource, /commercePaymentEvents: CommercePaymentEventRecord\[]/);
  assert.doesNotMatch(typesSource, /commercialAccounts: CommercialAccountSummary\[]/);
  assert.doesNotMatch(typesSource, /commercialAccountHolds: CommercialAccountHoldRecord\[]/);
  assert.doesNotMatch(typesSource, /commercialAccountLedger: CommercialAccountLedgerHistoryEntry\[]/);
  assert.doesNotMatch(
    typesSource,
    /commercialRequestSettlements: CommercialRequestSettlementRecord\[]/,
  );
  assert.doesNotMatch(typesSource, /commercialPricingPlans: CommercialPricingPlanRecord\[]/);
  assert.doesNotMatch(typesSource, /commercialPricingRates: CommercialPricingRateRecord\[]/);
  assert.doesNotMatch(workbench, /commerceOrders|commercePaymentEvents|couponTemplates|campaignBudgets|couponCodes|couponReservations|couponRedemptions|couponRollbacks|commercialAccounts|commercialAccountHolds|commercialAccountLedger|commercialRequestSettlements|commercialPricingPlans|commercialPricingRates/);
  assert.doesNotMatch(workbenchSnapshot, /coupons: \[]|couponTemplates: \[]|campaignBudgets: \[]|couponCodes: \[]|couponReservations: \[]|couponRedemptions: \[]|couponRollbacks: \[]|commerceOrders: \[]|commercePaymentEvents: \[]|commercialAccounts: \[]|commercialAccountHolds: \[]|commercialAccountLedger: \[]|commercialRequestSettlements: \[]|commercialPricingPlans: \[]|commercialPricingRates: \[]/);
});

test('app header avoids import meta asset resolution to stay worktree-safe', () => {
  const header = read('packages/sdkwork-craw-chat-admin-shell/src/components/AppHeader.tsx');

  assert.doesNotMatch(header, /import\.meta\.url/);
  assert.doesNotMatch(header, /new URL\(/);
  assert.match(header, /dataSlot="app-header-search"/);
  assert.match(header, /Ctrl K/);
  assert.match(header, /setCommandPaletteOpen|openCommandPalette/);
  assert.match(header, /Open command center|Open command search/);
});

test('shell command center is implemented as an in-place command palette', () => {
  const layout = read('packages/sdkwork-craw-chat-admin-shell/src/application/layouts/MainLayout.tsx');
  const header = read('packages/sdkwork-craw-chat-admin-shell/src/components/AppHeader.tsx');
  const commandPalette = read(
    'packages/sdkwork-craw-chat-admin-shell/src/components/CommandPalette.tsx',
  );
  const shellIndex = read('packages/sdkwork-craw-chat-admin-shell/src/index.ts');
  const store = read('packages/sdkwork-craw-chat-admin-core/src/store.ts');

  assert.match(layout, /CommandPalette/);
  assert.match(header, /setCommandPaletteOpen|openCommandPalette/);
  assert.match(commandPalette, /SearchCommandPalette/);
  assert.match(commandPalette, /adminRouteManifest/);
  assert.match(commandPalette, /prefetchSidebarRoute/);
  assert.match(commandPalette, /refreshWorkspace/);
  assert.match(commandPalette, /handleLogout/);
  assert.match(commandPalette, /Command center/);
  assert.match(shellIndex, /CommandPalette/);
  assert.match(store, /isCommandPaletteOpen/);
  assert.match(store, /commandSearchValue/);
});

test('shell exposes a persistent operations pulse drawer for cross-route continuity', () => {
  const layout = read('packages/sdkwork-craw-chat-admin-shell/src/application/layouts/MainLayout.tsx');
  const header = read('packages/sdkwork-craw-chat-admin-shell/src/components/AppHeader.tsx');
  const operationsPulse = read(
    'packages/sdkwork-craw-chat-admin-shell/src/components/OperationsPulseDrawer.tsx',
  );
  const shellIndex = read('packages/sdkwork-craw-chat-admin-shell/src/index.ts');
  const store = read('packages/sdkwork-craw-chat-admin-core/src/store.ts');

  assert.match(layout, /OperationsPulseDrawer/);
  assert.match(header, /dataSlot="app-header-pulse"/);
  assert.match(header, /openOperationsPulse|setOperationsPulseOpen/);
  assert.match(operationsPulse, /Operations pulse/);
  assert.match(operationsPulse, /Incident watch/);
  assert.match(operationsPulse, /Shift handoff/);
  assert.match(operationsPulse, /First response SLA/);
  assert.match(operationsPulse, /Reconnect watch/);
  assert.match(operationsPulse, /Retry queue/);
  assert.match(shellIndex, /OperationsPulseDrawer/);
  assert.match(store, /isOperationsPulseOpen/);
  assert.match(store, /openOperationsPulse/);
  assert.match(store, /closeOperationsPulse/);
});

test('shell exposes a persistent route context strip for active module governance', () => {
  const layout = read('packages/sdkwork-craw-chat-admin-shell/src/application/layouts/MainLayout.tsx');
  const routeContext = read(
    'packages/sdkwork-craw-chat-admin-shell/src/components/RouteContextStrip.tsx',
  );
  const shellIndex = read('packages/sdkwork-craw-chat-admin-shell/src/index.ts');

  assert.match(layout, /RouteContextStrip/);
  assert.match(routeContext, /adminRouteManifest/);
  assert.match(routeContext, /adminRouteKeyFromPathname/);
  assert.match(routeContext, /Continuity cue/);
  assert.match(routeContext, /Required permissions/);
  assert.match(routeContext, /Capability tags/);
  assert.match(routeContext, /Open command center/);
  assert.match(routeContext, /Open operations pulse|Open pulse/);
  assert.match(routeContext, /Open settings center|Operations directory/);
  assert.match(shellIndex, /RouteContextStrip/);
});

test('legacy router-admin subapps are removed from IM module packages', () => {
  const removedPaths = [
    'packages/sdkwork-craw-chat-admin-overview/src/view-model.ts',
    'packages/sdkwork-craw-chat-admin-core/src/commercialPricing.ts',
    'packages/sdkwork-craw-chat-admin-core/src/i18nTranslations.ts',
    'packages/sdkwork-craw-chat-admin-core/src/i18nTranslationsCommercial.ts',
    'packages/sdkwork-craw-chat-admin-core/src/i18nTranslationsCore.ts',
    'packages/sdkwork-craw-chat-admin-core/src/i18nTranslationsRecovery.ts',
    'packages/sdkwork-craw-chat-admin-core/src/i18nTranslationsRouting.ts',
    'packages/sdkwork-craw-chat-admin-admin-api/package.json',
    'packages/sdkwork-craw-chat-admin-admin-api/src/index.ts',
    'packages/sdkwork-craw-chat-admin-admin-api/src/storage.ts',
    'packages/sdkwork-craw-chat-admin-admin-api/src/transport.ts',
    'packages/sdkwork-craw-chat-admin-admin-api/src/commerce.ts',
    'packages/sdkwork-craw-chat-admin-users/src/page/OperatorUserDialog.tsx',
    'packages/sdkwork-craw-chat-admin-users/src/page/PortalUserDialog.tsx',
    'packages/sdkwork-craw-chat-admin-users/src/page/shared.tsx',
    'packages/sdkwork-craw-chat-admin-users/src/page/UsersDetailDrawer.tsx',
    'packages/sdkwork-craw-chat-admin-users/src/page/UsersDetailPanel.tsx',
    'packages/sdkwork-craw-chat-admin-users/src/page/UsersRegistrySection.tsx',
    'packages/sdkwork-craw-chat-admin-groups/src/page/CatalogChannelDialog.tsx',
    'packages/sdkwork-craw-chat-admin-groups/src/page/CatalogChannelModelDialog.tsx',
    'packages/sdkwork-craw-chat-admin-groups/src/page/CatalogCredentialDialog.tsx',
    'packages/sdkwork-craw-chat-admin-groups/src/page/CatalogDetailDrawer.tsx',
    'packages/sdkwork-craw-chat-admin-groups/src/page/CatalogDetailPanel.tsx',
    'packages/sdkwork-craw-chat-admin-groups/src/page/CatalogDialogs.tsx',
    'packages/sdkwork-craw-chat-admin-groups/src/page/CatalogModelPriceDialog.tsx',
    'packages/sdkwork-craw-chat-admin-groups/src/page/CatalogProviderDialog.tsx',
    'packages/sdkwork-craw-chat-admin-groups/src/page/CatalogRegistrySection.tsx',
    'packages/sdkwork-craw-chat-admin-groups/src/page/shared.tsx',
    'packages/sdkwork-craw-chat-admin-groups/src/page/useCatalogWorkspaceState.tsx',
    'packages/sdkwork-craw-chat-admin-moderation/src/page/CouponDialog.tsx',
    'packages/sdkwork-craw-chat-admin-moderation/src/page/CouponsDetailDrawer.tsx',
    'packages/sdkwork-craw-chat-admin-moderation/src/page/CouponsDetailPanel.tsx',
    'packages/sdkwork-craw-chat-admin-moderation/src/page/CouponsRegistrySection.tsx',
    'packages/sdkwork-craw-chat-admin-moderation/src/page/shared.tsx',
    'packages/sdkwork-craw-chat-admin-conversations/src/index.ts',
    'packages/sdkwork-craw-chat-admin-conversations/src/pages/billingEventAnalytics.ts',
    'packages/sdkwork-craw-chat-admin-conversations/src/pages/GatewayAccessPage.tsx',
    'packages/sdkwork-craw-chat-admin-conversations/src/pages/GatewayModelMappingsPage.tsx',
    'packages/sdkwork-craw-chat-admin-conversations/src/pages/GatewayRateLimitsPage.tsx',
    'packages/sdkwork-craw-chat-admin-conversations/src/pages/GatewayRoutesPage.tsx',
    'packages/sdkwork-craw-chat-admin-conversations/src/pages/GatewayUsagePage.tsx',
    'packages/sdkwork-craw-chat-admin-conversations/src/pages/shared.tsx',
    'packages/sdkwork-craw-chat-admin-conversations/src/pages/useGatewayAccessWorkspaceState.tsx',
    'packages/sdkwork-craw-chat-admin-conversations/src/pages/access/GatewayAccessDetailDrawer.tsx',
    'packages/sdkwork-craw-chat-admin-conversations/src/pages/access/GatewayAccessDetailPanel.tsx',
    'packages/sdkwork-craw-chat-admin-conversations/src/pages/access/GatewayAccessForms.tsx',
    'packages/sdkwork-craw-chat-admin-conversations/src/pages/access/GatewayAccessRegistrySection.tsx',
    'packages/sdkwork-craw-chat-admin-conversations/src/pages/access/GatewayApiKeyCreateDialog.tsx',
    'packages/sdkwork-craw-chat-admin-conversations/src/pages/access/GatewayApiKeyEditDialog.tsx',
    'packages/sdkwork-craw-chat-admin-conversations/src/pages/access/GatewayApiKeyGroupsDialog.tsx',
    'packages/sdkwork-craw-chat-admin-conversations/src/pages/access/GatewayApiKeyRouteDialog.tsx',
    'packages/sdkwork-craw-chat-admin-conversations/src/pages/access/GatewayApiKeyUsageDialog.tsx',
    'packages/sdkwork-craw-chat-admin-conversations/src/pages/access/shared.ts',
    'packages/sdkwork-craw-chat-admin-conversations/src/pages/mappings/GatewayModelMappingEditorDialog.tsx',
    'packages/sdkwork-craw-chat-admin-conversations/src/pages/mappings/GatewayModelMappingsDetailDrawer.tsx',
    'packages/sdkwork-craw-chat-admin-conversations/src/pages/mappings/GatewayModelMappingsDetailPanel.tsx',
    'packages/sdkwork-craw-chat-admin-conversations/src/pages/mappings/GatewayModelMappingsRegistrySection.tsx',
    'packages/sdkwork-craw-chat-admin-conversations/src/pages/mappings/shared.ts',
    'packages/sdkwork-craw-chat-admin-conversations/src/pages/rate-limits/GatewayRateLimitPolicyDialog.tsx',
    'packages/sdkwork-craw-chat-admin-conversations/src/pages/rate-limits/GatewayRateLimitsDetailDrawer.tsx',
    'packages/sdkwork-craw-chat-admin-conversations/src/pages/rate-limits/GatewayRateLimitsRegistrySection.tsx',
    'packages/sdkwork-craw-chat-admin-conversations/src/pages/rate-limits/shared.ts',
    'packages/sdkwork-craw-chat-admin-conversations/src/pages/routes/GatewayProviderDialog.tsx',
    'packages/sdkwork-craw-chat-admin-conversations/src/pages/routes/GatewayRoutesDetailDrawer.tsx',
    'packages/sdkwork-craw-chat-admin-conversations/src/pages/routes/GatewayRoutesDetailPanel.tsx',
    'packages/sdkwork-craw-chat-admin-conversations/src/pages/routes/GatewayRoutesRegistrySection.tsx',
    'packages/sdkwork-craw-chat-admin-conversations/src/pages/routes/GatewayRoutingProfilesDialog.tsx',
    'packages/sdkwork-craw-chat-admin-conversations/src/pages/routes/GatewayRoutingSnapshotsDialog.tsx',
    'packages/sdkwork-craw-chat-admin-conversations/src/pages/routes/routingSnapshotAnalytics.ts',
    'packages/sdkwork-craw-chat-admin-conversations/src/pages/routes/shared.ts',
    'packages/sdkwork-craw-chat-admin-conversations/src/pages/usage/GatewayUsageDetailDrawer.tsx',
    'packages/sdkwork-craw-chat-admin-conversations/src/pages/usage/GatewayUsageDetailPanel.tsx',
    'packages/sdkwork-craw-chat-admin-conversations/src/pages/usage/GatewayUsageRegistrySection.tsx',
    'packages/sdkwork-craw-chat-admin-conversations/src/pages/usage/shared.ts',
    'packages/sdkwork-craw-chat-admin-conversations/src/services/gatewayApiKeyAccessService.ts',
    'packages/sdkwork-craw-chat-admin-conversations/src/services/gatewayOverlayStore.ts',
    'packages/sdkwork-craw-chat-admin-conversations/src/services/gatewayViewService.ts',
    'packages/sdkwork-craw-chat-admin-automation/src/commercialOrderAuditDrawer.tsx',
    'packages/sdkwork-craw-chat-admin-automation/src/commercialOverviewSections.tsx',
    'packages/sdkwork-craw-chat-admin-automation/src/formatters.ts',
    'packages/sdkwork-craw-chat-admin-automation/src/ledgerTimeline.ts',
    'packages/sdkwork-craw-chat-admin-automation/src/orderAuditLookup.ts',
    'packages/sdkwork-craw-chat-admin-automation/src/orderPaymentAudit.ts',
    'packages/sdkwork-craw-chat-admin-automation/src/paymentCredentialBindingsDialog.tsx',
    'packages/sdkwork-craw-chat-admin-automation/src/paymentMethodDialog.tsx',
    'packages/sdkwork-craw-chat-admin-automation/src/paymentMethodManagerSection.tsx',
    'packages/sdkwork-craw-chat-admin-automation/src/paymentOrderOperationsSection.tsx',
    'packages/sdkwork-craw-chat-admin-automation/src/paymentReconciliationSection.tsx',
    'packages/sdkwork-craw-chat-admin-automation/src/paymentRefundDialog.tsx',
    'packages/sdkwork-craw-chat-admin-automation/src/paymentShared.tsx',
    'packages/sdkwork-craw-chat-admin-automation/src/paymentWebhookInboxSection.tsx',
  ];

  for (const relativePath of removedPaths) {
    assert.equal(
      existsSync(path.join(appRoot, relativePath)),
      false,
      `${relativePath} should be removed from the IM admin workspace`,
    );
  }
});

test('tenants module restores router-admin page decomposition for IM governance workflows', () => {
  const tenantPageFiles = [
    'packages/sdkwork-craw-chat-admin-tenants/src/page/ApiKeyDialog.tsx',
    'packages/sdkwork-craw-chat-admin-tenants/src/page/PlaintextApiKeyDialog.tsx',
    'packages/sdkwork-craw-chat-admin-tenants/src/page/ProjectDialog.tsx',
    'packages/sdkwork-craw-chat-admin-tenants/src/page/shared.tsx',
    'packages/sdkwork-craw-chat-admin-tenants/src/page/TenantDialog.tsx',
    'packages/sdkwork-craw-chat-admin-tenants/src/page/TenantsDetailDrawer.tsx',
    'packages/sdkwork-craw-chat-admin-tenants/src/page/TenantsDetailPanel.tsx',
    'packages/sdkwork-craw-chat-admin-tenants/src/page/TenantsRegistrySection.tsx',
  ];

  for (const relativePath of tenantPageFiles) {
    assert.equal(
      existsSync(path.join(appRoot, relativePath)),
      true,
      `${relativePath} should exist for the tenant registry/detail workflow`,
    );
  }

  const tenantsIndex = read('packages/sdkwork-craw-chat-admin-tenants/src/index.tsx');
  const registrySection = read(
    'packages/sdkwork-craw-chat-admin-tenants/src/page/TenantsRegistrySection.tsx',
  );
  const detailDrawer = read(
    'packages/sdkwork-craw-chat-admin-tenants/src/page/TenantsDetailDrawer.tsx',
  );
  const projectDialog = read('packages/sdkwork-craw-chat-admin-tenants/src/page/ProjectDialog.tsx');
  const apiKeyDialog = read('packages/sdkwork-craw-chat-admin-tenants/src/page/ApiKeyDialog.tsx');
  const plaintextApiKeyDialog = read(
    'packages/sdkwork-craw-chat-admin-tenants/src/page/PlaintextApiKeyDialog.tsx',
  );

  assert.match(tenantsIndex, /TenantsRegistrySection/);
  assert.match(tenantsIndex, /TenantsDetailDrawer/);
  assert.match(tenantsIndex, /TenantDialog/);
  assert.match(tenantsIndex, /ProjectDialog/);
  assert.match(tenantsIndex, /ApiKeyDialog/);
  assert.match(tenantsIndex, /PlaintextApiKeyDialog/);
  assert.match(tenantsIndex, /handleSaveTenant/);
  assert.match(tenantsIndex, /handleSaveProject/);
  assert.match(tenantsIndex, /handleCreateApiKey/);
  assert.match(registrySection, /DataTable/);
  assert.match(registrySection, /Issue key/);
  assert.match(registrySection, /New workspace/);
  assert.match(detailDrawer, /Key issuance ready|Key issuance guardrail/);
  assert.match(detailDrawer, /Issue key/);
  assert.match(projectDialog, /Workspace profile|New workspace|Save workspace/);
  assert.match(apiKeyDialog, /Issue key|Access key profile|Environment/);
  assert.match(plaintextApiKeyDialog, /Plaintext key|Copy key|Operator handoff/);
});

test('tenants page decomposition stays on the router-admin root ui entrypoint', () => {
  const tenantSources = [
    'packages/sdkwork-craw-chat-admin-tenants/src/index.tsx',
    'packages/sdkwork-craw-chat-admin-tenants/src/page/ApiKeyDialog.tsx',
    'packages/sdkwork-craw-chat-admin-tenants/src/page/PlaintextApiKeyDialog.tsx',
    'packages/sdkwork-craw-chat-admin-tenants/src/page/ProjectDialog.tsx',
    'packages/sdkwork-craw-chat-admin-tenants/src/page/TenantDialog.tsx',
    'packages/sdkwork-craw-chat-admin-tenants/src/page/TenantsDetailDrawer.tsx',
    'packages/sdkwork-craw-chat-admin-tenants/src/page/TenantsDetailPanel.tsx',
    'packages/sdkwork-craw-chat-admin-tenants/src/page/TenantsRegistrySection.tsx',
    'packages/sdkwork-craw-chat-admin-tenants/src/page/shared.tsx',
  ];

  for (const relativePath of tenantSources) {
    const source = read(relativePath);

    assert.doesNotMatch(
      source,
      /@sdkwork\/ui-pc-react\/components\/ui(?:\/|')/,
      `${relativePath} should rely on the root ui entrypoint instead of grouped ui runtime imports`,
    );
    assert.match(
      source,
      /@sdkwork\/ui-pc-react'/,
      `${relativePath} should import from the root @sdkwork/ui-pc-react entrypoint`,
    );
  }
});

test('desktop runtime config does not ship a fake admin backend fallback', () => {
  const runtimeConfigSource = readFileSync(
    path.join(workspaceRoot, 'crates', 'sdkwork-api-config', 'src', 'lib.rs'),
    'utf8',
  );

  assert.match(runtimeConfigSource, /SDKWORK_ADMIN_PROXY_TARGET|SDKWORK_ADMIN_BIND/);
  assert.match(runtimeConfigSource, /CRAW_CHAT_PORTAL_API_BASE_URL|CRAW_CHAT_BIND_ADDR/);
  assert.doesNotMatch(runtimeConfigSource, /127\.0\.0\.1:9981/);
});

test('vite admin proxy requires an explicit compatible backend target', () => {
  const viteConfig = read('vite.config.ts');

  assert.match(viteConfig, /SDKWORK_ADMIN_PROXY_TARGET/);
  assert.match(viteConfig, /Admin backend proxy target is not configured/);
  assert.match(viteConfig, /statusCode:\s*503|statusCode = 503|writeHead\(503/);
  assert.doesNotMatch(viteConfig, /127\.0\.0\.1:9981/);
});

test('README documents the IM admin runtime contract without router-admin residue', () => {
  const readme = read('README.md');

  assert.match(readme, /Craw Chat Admin/);
  assert.match(readme, /SDKWORK_ADMIN_PROXY_TARGET/);
  assert.match(readme, /CRAW_CHAT_PORTAL_API_BASE_URL/);
  assert.match(readme, /SDKWORK_ADMIN_SANDBOX/);
  assert.match(readme, /\/api\/admin/);
  assert.match(readme, /\/api\/v1\/control/);
  assert.match(readme, /18081/);
  assert.doesNotMatch(readme, /Coupons|API Router|Catalog|sdkwork-router-portal|router-product-service/);
  assert.doesNotMatch(readme, /127\.0\.0\.1:8081|127\.0\.0\.1:9981/);
});

test('admin workspace ships an explicit opt-in sandbox backend implementation', () => {
  const sandboxModulePath = path.join(appRoot, 'dev', 'admin-sandbox.mjs');
  const sandboxSeedPath = path.join(appRoot, 'dev', 'admin-sandbox-seed.json');

  assert.equal(existsSync(sandboxModulePath), true);
  assert.equal(existsSync(sandboxSeedPath), true);

  const sandboxModule = readFileSync(sandboxModulePath, 'utf8');
  const sandboxSeed = readFileSync(sandboxSeedPath, 'utf8');

  assert.match(sandboxModule, /createAdminSandboxState/);
  assert.match(sandboxModule, /handleAdminSandboxRequest/);
  assert.match(sandboxModule, /SDKWORK_ADMIN_SANDBOX/);
  assert.match(sandboxModule, /SDKWORK_ADMIN_SANDBOX_PASSWORD/);
  assert.match(sandboxModule, /randomBytes/);
  assert.doesNotMatch(sandboxSeed, /"sandboxPassword"\s*:/);
});

test('vite config supports explicit sandbox mode before falling back to 503 guidance', () => {
  const viteConfig = read('vite.config.ts');

  assert.match(viteConfig, /SDKWORK_ADMIN_SANDBOX/);
  assert.match(viteConfig, /createAdminSandboxState|handleAdminSandboxRequest|createAdminSandboxMiddleware/);
  assert.match(viteConfig, /adminProxyTarget/);
  assert.match(viteConfig, /503/);
});

test('shared web security headers define the required browser hardening baseline', async () => {
  const securityHeadersModule = await import(
    pathToFileURL(path.join(workspaceRoot, 'scripts', 'dev', 'web-security-headers.mjs')).href
  );

  assert.equal(
    securityHeadersModule.WEB_SECURITY_HEADERS['X-Content-Type-Options'],
    'nosniff',
  );
  assert.equal(
    securityHeadersModule.WEB_SECURITY_HEADERS['X-Frame-Options'],
    'DENY',
  );
  assert.equal(
    securityHeadersModule.WEB_SECURITY_HEADERS['Referrer-Policy'],
    'strict-origin-when-cross-origin',
  );
  assert.match(
    securityHeadersModule.WEB_SECURITY_HEADERS['Content-Security-Policy'],
    /default-src 'self'/,
  );
  assert.match(
    securityHeadersModule.WEB_SECURITY_HEADERS['Permissions-Policy'],
    /camera=\(\)/,
  );
});

test('release safety blocks production builds when the admin sandbox is enabled', async () => {
  const releaseSafetyModule = await import(
    pathToFileURL(path.join(appRoot, 'dev', 'release-safety.mjs')).href
  );

  assert.throws(
    () => {
      releaseSafetyModule.assertAdminReleaseSafety({
        command: 'build',
        env: { SDKWORK_ADMIN_SANDBOX: '1' },
      });
    },
    /SDKWORK_ADMIN_SANDBOX/,
  );

  assert.doesNotThrow(() => {
    releaseSafetyModule.assertAdminReleaseSafety({
      command: 'serve',
      env: { SDKWORK_ADMIN_SANDBOX: '1' },
    });
  });

  assert.doesNotThrow(() => {
    releaseSafetyModule.assertAdminReleaseSafety({
      command: 'build',
      env: { SDKWORK_ADMIN_SANDBOX: '0' },
    });
  });
});

test('release safety rejects forbidden sandbox and demo credentials from admin bundles', async () => {
  const releaseSafetyModule = await import(
    pathToFileURL(path.join(appRoot, 'dev', 'release-safety.mjs')).href
  );

  assert.throws(
    () => {
      releaseSafetyModule.assertAdminBundleContentSafe({
        'assets/index.js': {
          type: 'chunk',
          code: 'const password = "ChangeMe123"; const email = "admin@sdkwork.local";',
        },
      });
    },
    /ChangeMe123|admin@sdkwork\.local/,
  );

  assert.doesNotThrow(() => {
    releaseSafetyModule.assertAdminBundleContentSafe({
      'assets/index.js': {
        type: 'chunk',
        code: 'const mode = "production-release";',
      },
      'assets/index.css': {
        type: 'asset',
        source: 'body{color:#111;}',
      },
    });
  });
});

test('vite config wires shared security headers and release safety into the admin runtime', () => {
  const viteConfig = read('vite.config.ts');

  assert.match(viteConfig, /web-security-headers/);
  assert.match(viteConfig, /headers:\s*createWebSecurityHeaders\(\)/);
  assert.match(viteConfig, /assertAdminReleaseSafety|createAdminReleaseSafetyPlugin/);
});
