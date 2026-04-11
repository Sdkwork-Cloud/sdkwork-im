#!/usr/bin/env node
import {
  existsSync,
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
const cacheDir = path.join(generatedRoot, '.npm-cache');
const distRoot = path.join(generatedRoot, 'dist');
const locksRoot = path.join(generatedRoot, '.sdkwork', 'locks');
const buildLockDir = path.join(locksRoot, 'stable-typescript-generated-build.lock');
const lockInfoPath = path.join(buildLockDir, 'owner.json');
const buildRunId = `run-${process.pid}-${Date.now()}-${Math.random().toString(16).slice(2, 10)}`;
const tmpWorkspaceRoot = path.join(generatedRoot, '.sdkwork', 'tmp', 'stable-typescript-build');
const tmpRoot = path.join(tmpWorkspaceRoot, buildRunId);
const esmTmpRoot = path.join(tmpRoot, 'esm');
const tscBin = path.join(generatedRoot, 'node_modules', 'typescript', 'bin', 'tsc');
const rollupBin = path.join(generatedRoot, 'node_modules', 'rollup', 'dist', 'bin', 'rollup');
const staleDistRoot = path.join(tmpWorkspaceRoot, `${buildRunId}-previous-dist`);
const lockTimeoutMs = 5 * 60 * 1000;
const lockPollMs = 200;
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
  if (process.platform === 'win32') {
    run('cmd.exe', ['/d', '/s', '/c', 'npm', ...args], options);
    return;
  }
  run('npm', args, options);
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

function ensureToolingInstalled() {
  runNpm(['install', '--ignore-scripts'], {
    step: 'typescript-generated:npm-install-ignore-scripts',
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

function resetBuildOutputs() {
  if (existsSync(tmpWorkspaceRoot)) {
    const resolvedTmpWorkspaceRoot = path.resolve(tmpWorkspaceRoot);
    if (!resolvedTmpWorkspaceRoot.startsWith(path.resolve(generatedRoot))) {
      fail(`Refusing to remove path outside generated workspace: ${resolvedTmpWorkspaceRoot}`);
    }
    rmSync(resolvedTmpWorkspaceRoot, { recursive: true, force: true });
  }

  mkdirSync(cacheDir, { recursive: true });
  mkdirSync(esmTmpRoot, { recursive: true });

  if (existsSync(distRoot)) {
    const resolvedDistRoot = path.resolve(distRoot);
    if (!resolvedDistRoot.startsWith(path.resolve(generatedRoot))) {
      fail(`Refusing to move path outside generated workspace: ${resolvedDistRoot}`);
    }
    renameSync(resolvedDistRoot, staleDistRoot);
    rmSync(staleDistRoot, { recursive: true, force: true });
  }
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

function cleanupTmpWorkspaceRoot() {
  if (!existsSync(tmpWorkspaceRoot)) {
    return;
  }

  const resolvedTmpWorkspaceRoot = path.resolve(tmpWorkspaceRoot);
  if (!resolvedTmpWorkspaceRoot.startsWith(path.resolve(generatedRoot))) {
    fail(`Refusing to remove path outside generated workspace: ${resolvedTmpWorkspaceRoot}`);
  }
  rmSync(resolvedTmpWorkspaceRoot, { recursive: true, force: true });
}

function compileDeclarations() {
  run('node', [tscBin, '-p', 'tsconfig.json', '--emitDeclarationOnly', '--outDir', distRoot], {
    step: 'typescript-generated:tsc-declarations',
  });
}

function compileEsmTree() {
  run(
    'node',
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
    'node',
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

function collectCjsExports() {
  const cjsSource = readFileSync(path.join(distRoot, 'index.cjs'), 'utf8');
  const exportNames = new Set();
  const requiredRuntimeExports = [
    'DEFAULT_TIMEOUT',
    'DefaultAuthTokenManager',
    'SUCCESS_CODES',
    'createTokenManager',
  ];
  const exportPatterns = [
    /exports\.([A-Za-z0-9_$]+)\s*=/g,
    /Object\.defineProperty\(exports,\s*"([A-Za-z0-9_$]+)"/g,
  ];

  for (const exportPattern of exportPatterns) {
    let match = exportPattern.exec(cjsSource);
    while (match) {
      exportNames.add(match[1]);
      match = exportPattern.exec(cjsSource);
    }
  }

  for (const requiredRuntimeExport of requiredRuntimeExports) {
    if (cjsSource.includes(`"${requiredRuntimeExport}"`) || cjsSource.includes(`exports.${requiredRuntimeExport}`)) {
      exportNames.add(requiredRuntimeExport);
    }
  }

  return [...exportNames].sort();
}

function writeEsmWrapper() {
  const exportNames = collectCjsExports();
  if (exportNames.length === 0) {
    fail('TypeScript generated CJS bundle did not expose any named exports.');
  }

  const wrapperSource = [
    "import backend from './index.cjs';",
    `const { ${exportNames.join(', ')} } = backend;`,
    `export { ${exportNames.join(', ')} };`,
    '',
  ].join('\n');

  writeFileSync(path.join(distRoot, 'index.js'), wrapperSource, 'utf8');
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
  resetBuildOutputs();
  compileDeclarations();
  compileEsmTree();
  bundleCjs();
  sanitizeCjsSourceMap();
  writeEsmWrapper();

  if (!existsSync(path.join(distRoot, 'index.js'))) {
    fail('TypeScript generated dist/index.js was not produced.');
  }
  if (!existsSync(path.join(distRoot, 'index.cjs'))) {
    fail('TypeScript generated dist/index.cjs was not produced.');
  }
  if (!existsSync(path.join(distRoot, 'index.d.ts'))) {
    fail('TypeScript generated dist/index.d.ts was not produced.');
  }

  cleanupTmpWorkspaceRoot();
} finally {
  releaseBuildLock();
}

console.log('[sdkwork-craw-chat-sdk] TypeScript generated package build completed.');
