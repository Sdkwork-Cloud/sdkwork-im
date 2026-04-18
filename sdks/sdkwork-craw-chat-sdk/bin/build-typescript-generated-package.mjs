#!/usr/bin/env node
import {
  existsSync,
  realpathSync,
  mkdirSync,
  readFileSync,
  readdirSync,
  renameSync,
  rmSync,
  writeFileSync,
} from 'node:fs';
import path from 'node:path';
import { spawnSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';

const scriptDir = path.dirname(fileURLToPath(import.meta.url));
const workspaceRoot = path.resolve(scriptDir, '..');
const generatedRoot = path.join(
  workspaceRoot,
  'sdkwork-craw-chat-sdk-typescript',
  'generated',
  'server-openapi',
);
const packageLockPath = path.join(generatedRoot, 'package-lock.json');
const cacheDir = path.join(generatedRoot, '.npm-cache');
const distRoot = path.join(generatedRoot, 'dist');
const browserRoot = path.join(generatedRoot, 'browser');
const nodeModulesRoot = path.join(generatedRoot, 'node_modules');
const locksRoot = path.join(generatedRoot, '.sdkwork', 'locks');
const buildLockDir = path.join(locksRoot, 'stable-typescript-generated-build.lock');
const lockInfoPath = path.join(buildLockDir, 'owner.json');
const buildRunId = `run-${process.pid}-${Date.now()}-${Math.random().toString(16).slice(2, 10)}`;
const tmpWorkspaceRoot = path.join(generatedRoot, '.sdkwork', 'tmp', 'stable-typescript-build');
const tmpRoot = path.join(tmpWorkspaceRoot, buildRunId);
const esmTmpRoot = path.join(tmpRoot, 'esm');
const staleOutputsRoot = path.join(generatedRoot, '.sdkwork', 'tmp', 'stable-typescript-build-stale');
const tscBin = path.join(generatedRoot, 'node_modules', 'typescript', 'bin', 'tsc');
const rollupBin = path.join(generatedRoot, 'node_modules', 'rollup', 'dist', 'bin', 'rollup');
const lockTimeoutMs = 5 * 60 * 1000;
const lockPollMs = 200;
const cleanupRetryCount = process.platform === 'win32' ? 8 : 2;
const cleanupRetryDelayMs = process.platform === 'win32' ? 150 : 25;
const npmCliCandidates = [
  process.env.npm_execpath,
  path.join(path.dirname(process.execPath), 'node_modules', 'npm', 'bin', 'npm-cli.js'),
];
let buildLockHeld = false;

function fail(message) {
  console.error(`[sdkwork-craw-chat-sdk] ${message}`);
  process.exit(1);
}

function sleep(ms) {
  return new Promise((resolve) => {
    setTimeout(resolve, ms);
  });
}

function run(command, args, options = {}) {
  const result = spawnSync(command, args, {
    cwd: generatedRoot,
    stdio: 'inherit',
    shell: false,
    env: {
      ...process.env,
      NPM_CONFIG_CACHE: cacheDir,
      ...(options.env || {}),
    },
  });

  if (result.error) {
    fail(`${options.step || command} failed to start: ${result.error.message}`);
  }
  if ((result.status ?? 1) !== 0) {
    fail(`${options.step || command} failed with exit code ${result.status}`);
  }
}

function runNpm(args, options = {}) {
  for (const npmCliPath of npmCliCandidates) {
    if (npmCliPath && existsSync(npmCliPath)) {
      run(process.execPath, [npmCliPath, ...args], options);
      return;
    }
  }

  fail('Unable to locate npm CLI for TypeScript generated package builds.');
}

function walkFiles(rootDirectory) {
  const files = [];
  const queue = [rootDirectory];

  while (queue.length > 0) {
    const currentDirectory = queue.shift();
    for (const entry of readdirSync(currentDirectory, { withFileTypes: true })) {
      const absolutePath = path.join(currentDirectory, entry.name);
      if (entry.isDirectory()) {
        queue.push(absolutePath);
        continue;
      }
      if (entry.isFile()) {
        files.push(absolutePath);
      }
    }
  }

  return files;
}

function withPosixSeparators(input) {
  return input.replaceAll('\\', '/');
}

function resolveJsSpecifier(specifier, filePath) {
  if (!specifier.startsWith('.')) {
    return specifier;
  }
  if (/\.(?:[cm]?js|json)$/i.test(specifier)) {
    return specifier;
  }

  const absoluteBase = path.resolve(path.dirname(filePath), specifier);
  if (existsSync(`${absoluteBase}.js`)) {
    return `${specifier}.js`;
  }
  if (existsSync(path.join(absoluteBase, 'index.js'))) {
    return `${withPosixSeparators(specifier.replace(/\/+$/, ''))}/index.js`;
  }

  return specifier;
}

function rewriteEsmSpecifiers(rootDirectory) {
  for (const absolutePath of walkFiles(rootDirectory)) {
    if (!absolutePath.endsWith('.js')) {
      continue;
    }

    const source = readFileSync(absolutePath, 'utf8');
    const rewritten = source
      .replace(/(\bfrom\s*['"])(\.{1,2}\/[^'"]+)(['"])/g, (_match, prefix, specifier, suffix) => {
        return `${prefix}${resolveJsSpecifier(specifier, absolutePath)}${suffix}`;
      })
      .replace(/(\bimport\s*['"])(\.{1,2}\/[^'"]+)(['"])/g, (_match, prefix, specifier, suffix) => {
        return `${prefix}${resolveJsSpecifier(specifier, absolutePath)}${suffix}`;
      });

    if (rewritten !== source) {
      writeFileSync(absolutePath, rewritten, 'utf8');
    }
  }
}

function copyDirectory(sourceRoot, targetRoot) {
  mkdirSync(targetRoot, { recursive: true });

  for (const absolutePath of walkFiles(sourceRoot)) {
    const relativePath = path.relative(sourceRoot, absolutePath);
    const targetPath = path.join(targetRoot, relativePath);
    mkdirSync(path.dirname(targetPath), { recursive: true });
    writeFileSync(targetPath, readFileSync(absolutePath));
  }
}

function isPathInside(parentPath, candidatePath) {
  const normalizedParent = withPosixSeparators(path.resolve(parentPath)).replace(/\/+$/, '');
  const normalizedCandidate = withPosixSeparators(path.resolve(candidatePath)).replace(/\/+$/, '');
  return normalizedCandidate === normalizedParent || normalizedCandidate.startsWith(`${normalizedParent}/`);
}

function resolveExistingRealPath(targetPath) {
  try {
    if (!existsSync(targetPath)) {
      return null;
    }
    return realpathSync(targetPath);
  } catch {
    return null;
  }
}

function hasUnsafeToolingInstall() {
  if (!existsSync(nodeModulesRoot)) {
    return false;
  }

  for (const toolDirectory of ['typescript', 'rollup', 'vite', 'vite-plugin-dts']) {
    const toolPath = path.join(nodeModulesRoot, toolDirectory);
    const resolvedToolPath = resolveExistingRealPath(toolPath);

    if (!resolvedToolPath) {
      return true;
    }
    if (!isPathInside(nodeModulesRoot, resolvedToolPath)) {
      return true;
    }
  }

  return false;
}

function resetToolingInstall() {
  if (!existsSync(nodeModulesRoot)) {
    return;
  }

  const resolvedNodeModulesRoot = path.resolve(nodeModulesRoot);
  if (!isPathInside(generatedRoot, resolvedNodeModulesRoot)) {
    fail(`Refusing to remove node_modules outside generated workspace: ${resolvedNodeModulesRoot}`);
  }

  rmSync(resolvedNodeModulesRoot, { recursive: true, force: true });
}

function ensurePathInside(parentPath, candidatePath, description) {
  if (!isPathInside(parentPath, candidatePath)) {
    fail(`${description} escapes the allowed root: ${candidatePath}`);
  }
}

function listChildDirectories(rootDirectory) {
  if (!existsSync(rootDirectory)) {
    return [];
  }

  return readdirSync(rootDirectory, { withFileTypes: true })
    .filter((entry) => entry.isDirectory())
    .map((entry) => path.join(rootDirectory, entry.name));
}

function isRetriableCleanupError(error) {
  return Boolean(error && ['EPERM', 'EBUSY', 'ENOTEMPTY'].includes(error.code));
}

async function removePathWithRetries(targetPath, description, tolerateFailure = false) {
  if (!existsSync(targetPath)) {
    return true;
  }

  let lastError = null;
  for (let attempt = 0; attempt <= cleanupRetryCount; attempt += 1) {
    try {
      rmSync(targetPath, { recursive: true, force: true });
      return true;
    } catch (error) {
      lastError = error;
      if (!isRetriableCleanupError(error) || attempt === cleanupRetryCount) {
        break;
      }

      await sleep(cleanupRetryDelayMs * (attempt + 1));
    }
  }

  if (tolerateFailure) {
    return false;
  }

  fail(`${description} could not be removed: ${lastError?.message || 'unknown cleanup error'}`);
}

function stageOutputForCleanup(sourcePath, label) {
  if (!existsSync(sourcePath)) {
    return null;
  }

  const resolvedSourcePath = path.resolve(sourcePath);
  ensurePathInside(generatedRoot, resolvedSourcePath, `TypeScript generated ${label} output`);

  mkdirSync(staleOutputsRoot, { recursive: true });
  const stagedOutputPath = path.join(staleOutputsRoot, `${buildRunId}-${label}`);
  ensurePathInside(staleOutputsRoot, stagedOutputPath, `TypeScript generated staged ${label} output`);
  renameSync(resolvedSourcePath, stagedOutputPath);
  return stagedOutputPath;
}

async function disposeStagedOutput(sourcePath, label) {
  const stagedOutputPath = stageOutputForCleanup(sourcePath, label);
  if (!stagedOutputPath) {
    return;
  }

  const removed = await removePathWithRetries(
    stagedOutputPath,
    `TypeScript generated staged ${label} output`,
    true,
  );
  if (!removed) {
    console.warn(
      `[sdkwork-craw-chat-sdk] Retaining staged TypeScript generated output for later cleanup: ${stagedOutputPath}`,
    );
  }
}

async function tryPruneRetainedBuildRoots() {
  for (const retainedBuildRoot of listChildDirectories(tmpWorkspaceRoot)) {
    ensurePathInside(tmpWorkspaceRoot, retainedBuildRoot, 'Retained TypeScript generated build root');
    if (path.resolve(retainedBuildRoot) === path.resolve(tmpRoot)) {
      continue;
    }
    await removePathWithRetries(
      retainedBuildRoot,
      `Retained TypeScript generated build root ${retainedBuildRoot}`,
      true,
    );
  }
}

async function tryPruneRetainedStaleOutputs() {
  for (const retainedStaleOutputRoot of listChildDirectories(staleOutputsRoot)) {
    ensurePathInside(staleOutputsRoot, retainedStaleOutputRoot, 'Retained TypeScript generated stale output root');
    await removePathWithRetries(
      retainedStaleOutputRoot,
      `Retained TypeScript generated stale output root ${retainedStaleOutputRoot}`,
      true,
    );
  }
}

function ensureToolingInstalled() {
  if (hasUnsafeToolingInstall()) {
    resetToolingInstall();
  }

  runNpm(existsSync(packageLockPath) ? ['ci', '--ignore-scripts'] : ['install', '--ignore-scripts'], {
    step: existsSync(packageLockPath)
      ? 'typescript-generated:npm-ci-ignore-scripts'
      : 'typescript-generated:npm-install-ignore-scripts',
  });

  if (!existsSync(tscBin)) {
    fail(`TypeScript compiler not found after install: ${tscBin}`);
  }
  if (!existsSync(rollupBin)) {
    fail(`Rollup CLI not found after install: ${rollupBin}`);
  }
}

function describeLockOwner() {
  if (!existsSync(lockInfoPath)) {
    return 'unknown owner';
  }

  try {
    const lockInfo = JSON.parse(readFileSync(lockInfoPath, 'utf8'));
    return `pid=${lockInfo.pid ?? 'unknown'}, startedAt=${lockInfo.startedAt ?? 'unknown'}`;
  } catch {
    return 'unknown owner';
  }
}

async function acquireBuildLock() {
  mkdirSync(locksRoot, { recursive: true });
  const startedAt = Date.now();

  while (true) {
    try {
      mkdirSync(buildLockDir);
      writeFileSync(lockInfoPath, JSON.stringify({
        pid: process.pid,
        startedAt: new Date().toISOString(),
        workspaceRoot: generatedRoot,
      }, null, 2), 'utf8');
      buildLockHeld = true;
      return;
    } catch (error) {
      if (error && error.code !== 'EEXIST') {
        fail(`Failed to acquire TypeScript generated build lock: ${error.message}`);
      }
    }

    if (Date.now() - startedAt >= lockTimeoutMs) {
      fail(
        `Timed out waiting for TypeScript generated build lock after ${lockTimeoutMs}ms (${describeLockOwner()}).`,
      );
    }

    await sleep(lockPollMs);
  }
}

function releaseBuildLock() {
  if (!buildLockHeld) {
    return;
  }

  if (existsSync(buildLockDir)) {
    rmSync(buildLockDir, { recursive: true, force: true });
  }
  buildLockHeld = false;
}

async function resetBuildOutputs() {
  mkdirSync(cacheDir, { recursive: true });
  mkdirSync(tmpWorkspaceRoot, { recursive: true });
  await tryPruneRetainedBuildRoots();
  await tryPruneRetainedStaleOutputs();
  await disposeStagedOutput(distRoot, 'dist');
  await disposeStagedOutput(browserRoot, 'browser');
  mkdirSync(esmTmpRoot, { recursive: true });
}

function toPosixRelative(input) {
  return input.replaceAll('\\', '/');
}

function sanitizeSourceMapSource(sourcePath) {
  const normalizedSourcePath = toPosixRelative(sourcePath);
  const stableTempMatch = normalizedSourcePath.match(
    /^(?:\.\.\/)+\.sdkwork\/tmp\/stable-typescript-build\/(?:run-[^/]+\/)?esm\/(.+)\.js$/,
  );

  if (!stableTempMatch) {
    return normalizedSourcePath;
  }

  return `../src/${stableTempMatch[1]}.ts`;
}

function sanitizeCjsSourceMap() {
  const sourceMapPath = path.join(distRoot, 'index.cjs.map');
  if (!existsSync(sourceMapPath)) {
    fail('TypeScript generated dist/index.cjs.map was not produced.');
  }

  const sourceMap = JSON.parse(readFileSync(sourceMapPath, 'utf8'));
  if (!Array.isArray(sourceMap.sources)) {
    fail('TypeScript generated dist/index.cjs.map must contain a sources array.');
  }

  sourceMap.sources = sourceMap.sources.map((sourcePath) => sanitizeSourceMapSource(String(sourcePath)));
  writeFileSync(sourceMapPath, `${JSON.stringify(sourceMap)}\n`, 'utf8');
}

async function cleanupTmpWorkspaceRoot() {
  if (existsSync(tmpRoot)) {
    const removed = await removePathWithRetries(
      tmpRoot,
      'TypeScript generated temporary build workspace',
      true,
    );
    if (!removed) {
      console.warn(
        `[sdkwork-craw-chat-sdk] Retaining TypeScript generated temporary build workspace for later cleanup: ${tmpRoot}`,
      );
    }
  }

  await tryPruneRetainedBuildRoots();
  await tryPruneRetainedStaleOutputs();

  if (existsSync(tmpWorkspaceRoot) && readdirSync(tmpWorkspaceRoot).length === 0) {
    await removePathWithRetries(
      tmpWorkspaceRoot,
      'TypeScript generated temporary build root',
      true,
    );
  }
  if (existsSync(staleOutputsRoot) && readdirSync(staleOutputsRoot).length === 0) {
    await removePathWithRetries(
      staleOutputsRoot,
      'TypeScript generated stale output root',
      true,
    );
  }
}

function compileDeclarations() {
  run(process.execPath, [tscBin, '-p', 'tsconfig.json', '--emitDeclarationOnly', '--outDir', distRoot], {
    step: 'typescript-generated:tsc-declarations',
  });
}

function compileEsmTree() {
  run(
    process.execPath,
    [
      tscBin,
      '-p',
      'tsconfig.json',
      '--emitDeclarationOnly',
      'false',
      '--declaration',
      'false',
      '--declarationMap',
      'false',
      '--outDir',
      esmTmpRoot,
    ],
    {
      step: 'typescript-generated:tsc-esm-tree',
    },
  );
  rewriteEsmSpecifiers(esmTmpRoot);
}

function bundleCjs() {
  run(
    process.execPath,
    [
      rollupBin,
      path.join(esmTmpRoot, 'index.js'),
      '--format',
      'cjs',
      '--file',
      path.join(distRoot, 'index.cjs'),
      '--external',
      '@sdkwork/sdk-common',
      '--sourcemap',
    ],
    {
      step: 'typescript-generated:rollup-cjs',
    },
  );
}

function bundleEsm() {
  run(
    process.execPath,
    [
      rollupBin,
      path.join(esmTmpRoot, 'index.js'),
      '--format',
      'esm',
      '--file',
      path.join(distRoot, 'index.js'),
      '--external',
      '@sdkwork/sdk-common',
    ],
    {
      step: 'typescript-generated:rollup-esm',
    },
  );
}

function publishBrowserTree() {
  copyDirectory(esmTmpRoot, browserRoot);
}

for (const signal of ['SIGINT', 'SIGTERM', 'SIGHUP']) {
  process.on(signal, () => {
    releaseBuildLock();
    process.exit(1);
  });
}
process.on('exit', () => {
  releaseBuildLock();
});

await acquireBuildLock();

try {
  ensureToolingInstalled();
  await resetBuildOutputs();
  compileDeclarations();
  compileEsmTree();
  bundleCjs();
  bundleEsm();
  publishBrowserTree();
  sanitizeCjsSourceMap();

  if (!existsSync(path.join(distRoot, 'index.js'))) {
    fail('TypeScript generated dist/index.js was not produced.');
  }
  if (!existsSync(path.join(distRoot, 'index.cjs'))) {
    fail('TypeScript generated dist/index.cjs was not produced.');
  }
  if (!existsSync(path.join(distRoot, 'index.d.ts'))) {
    fail('TypeScript generated dist/index.d.ts was not produced.');
  }
  if (!existsSync(path.join(browserRoot, 'index.js'))) {
    fail('TypeScript generated browser/index.js was not produced.');
  }

  await cleanupTmpWorkspaceRoot();
} finally {
  releaseBuildLock();
}

console.log('[sdkwork-craw-chat-sdk] TypeScript generated package build completed.');
