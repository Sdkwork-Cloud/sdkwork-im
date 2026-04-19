#!/usr/bin/env node
import {
  copyFileSync,
  cpSync,
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
import {
  resolveGeneratorModulePath,
  resolveSdkCommonPath,
} from './generator-runtime.mjs';

const scriptDir = path.dirname(fileURLToPath(import.meta.url));
const workspaceRoot = path.resolve(scriptDir, '..');
const generatedRoot = path.join(
  workspaceRoot,
  'sdkwork-im-admin-sdk-typescript',
  'generated',
  'server-openapi',
);
const generatedManifestPath = path.join(generatedRoot, 'package.json');
const distRoot = path.join(generatedRoot, 'dist');
const tmpWorkspaceRoot = path.join(generatedRoot, '.sdkwork', 'tmp', 'stable-typescript-build');
const buildRunId = `run-${process.pid}-${Date.now()}-${Math.random().toString(16).slice(2, 10)}`;
const tmpRoot = path.join(tmpWorkspaceRoot, buildRunId);
const esmTmpRoot = path.join(tmpRoot, 'esm');
const locksRoot = path.join(generatedRoot, '.sdkwork', 'locks');
const buildLockDir = path.join(locksRoot, 'stable-typescript-generated-build.lock');
const lockInfoPath = path.join(buildLockDir, 'owner.json');
const lockTimeoutMs = 5 * 60 * 1000;
const lockPollMs = 200;
const tscBin = resolveGeneratorModulePath(workspaceRoot, 'typescript', 'bin', 'tsc');
const viteBin = resolveGeneratorModulePath(workspaceRoot, 'vite', 'bin', 'vite.js');
const rollupBin = resolveGeneratorModulePath(workspaceRoot, 'rollup', 'dist', 'bin', 'rollup');
const sdkCommonPackageRoot = path.join(generatedRoot, 'node_modules', '@sdkwork', 'sdk-common');
const toolShimRoot = path.join(generatedRoot, 'node_modules', '.bin');
let buildLockHeld = false;

function fail(message) {
  console.error(`[sdkwork-im-admin-sdk] ${message}`);
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
      ...(options.env || {}),
    },
  });

  if (result.error) {
    fail(`${options.step || command} failed to start: ${result.error.message}`);
  }
  if ((result.status ?? 1) !== 0) {
    fail(`${options.step || command} failed with exit code ${result.status}`);
  }
  if (result.signal) {
    fail(`${options.step || command} terminated with signal ${result.signal}`);
  }
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
      writeFileSync(
        lockInfoPath,
        JSON.stringify(
          {
            pid: process.pid,
            startedAt: new Date().toISOString(),
            workspaceRoot: generatedRoot,
          },
          null,
          2,
        ),
        'utf8',
      );
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

function ensureGeneratorTooling() {
  for (const requiredPath of [tscBin, viteBin, rollupBin]) {
    if (!existsSync(requiredPath)) {
      fail(`Required generator tooling is missing: ${requiredPath}`);
    }
  }
}

function stabilizeGeneratedManifest() {
  if (!existsSync(generatedManifestPath)) {
    fail(`Generated package manifest is missing: ${generatedManifestPath}`);
  }

  const manifest = JSON.parse(readFileSync(generatedManifestPath, 'utf8'));
  const stableBuildCommand = process.platform === 'win32'
    ? '..\\..\\..\\bin\\build-typescript-generated-package.cmd'
    : '../../../bin/build-typescript-generated-package';
  const nextManifest = {
    ...manifest,
    scripts: {
      ...(manifest.scripts || {}),
      build: stableBuildCommand,
      prepublishOnly: 'npm run build',
    },
  };
  const nextManifestSource = `${JSON.stringify(nextManifest, null, 2)}\n`;

  if (readFileSync(generatedManifestPath, 'utf8') !== nextManifestSource) {
    writeFileSync(generatedManifestPath, nextManifestSource, 'utf8');
  }
}

function materializeSdkCommonDependency() {
  const sdkCommonDistRoot = resolveSdkCommonPath(workspaceRoot, 'dist');
  const sdkCommonPackageJsonPath = resolveSdkCommonPath(workspaceRoot, 'package.json');
  const sdkCommonReadmePath = resolveSdkCommonPath(workspaceRoot, 'README.md');

  if (!existsSync(sdkCommonDistRoot)) {
    fail(`sdk-common dist directory is missing: ${sdkCommonDistRoot}`);
  }
  if (!existsSync(sdkCommonPackageJsonPath)) {
    fail(`sdk-common package.json is missing: ${sdkCommonPackageJsonPath}`);
  }

  rmSync(sdkCommonPackageRoot, { recursive: true, force: true });
  mkdirSync(path.dirname(sdkCommonPackageRoot), { recursive: true });
  cpSync(sdkCommonDistRoot, path.join(sdkCommonPackageRoot, 'dist'), { recursive: true });
  copyFileSync(sdkCommonPackageJsonPath, path.join(sdkCommonPackageRoot, 'package.json'));
  if (existsSync(sdkCommonReadmePath)) {
    copyFileSync(sdkCommonReadmePath, path.join(sdkCommonPackageRoot, 'README.md'));
  }
}

function writeToolShim(stem, targetPath) {
  mkdirSync(toolShimRoot, { recursive: true });
  const normalizedTargetPath = targetPath.replaceAll('\\', '\\\\');
  const shellSource = `#!/usr/bin/env sh\nexec node "${targetPath.replaceAll('\\', '/')}" "$@"\n`;
  const cmdSource = `@echo off\r\nnode "${normalizedTargetPath}" %*\r\n`;
  const ps1Source = `node "${normalizedTargetPath}" @args\r\n`;
  writeFileSync(path.join(toolShimRoot, stem), shellSource, 'utf8');
  writeFileSync(path.join(toolShimRoot, `${stem}.cmd`), cmdSource, 'utf8');
  writeFileSync(path.join(toolShimRoot, `${stem}.ps1`), ps1Source, 'utf8');
}

function materializeToolShims() {
  writeToolShim('tsc', tscBin);
  writeToolShim('vite', viteBin);
}

function resetBuildOutputs() {
  if (existsSync(tmpWorkspaceRoot)) {
    const resolvedTmpWorkspaceRoot = path.resolve(tmpWorkspaceRoot);
    if (!resolvedTmpWorkspaceRoot.startsWith(path.resolve(generatedRoot))) {
      fail(`Refusing to remove path outside generated workspace: ${resolvedTmpWorkspaceRoot}`);
    }
    rmSync(resolvedTmpWorkspaceRoot, { recursive: true, force: true });
  }

  mkdirSync(esmTmpRoot, { recursive: true });

  if (existsSync(distRoot)) {
    const resolvedDistRoot = path.resolve(distRoot);
    const staleDistRoot = path.join(tmpWorkspaceRoot, `${buildRunId}-previous-dist`);
    if (!resolvedDistRoot.startsWith(path.resolve(generatedRoot))) {
      fail(`Refusing to move path outside generated workspace: ${resolvedDistRoot}`);
    }
    renameSync(resolvedDistRoot, staleDistRoot);
    rmSync(staleDistRoot, { recursive: true, force: true });
  }
}

function sanitizeSourceMapSource(sourcePath) {
  const normalizedSourcePath = withPosixSeparators(sourcePath);
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
    'createClient',
    'createTokenManager',
    'SdkworkBackendClient',
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
  ensureGeneratorTooling();
  stabilizeGeneratedManifest();
  materializeSdkCommonDependency();
  materializeToolShims();
  resetBuildOutputs();
  compileDeclarations();
  compileEsmTree();
  bundleCjs();
  sanitizeCjsSourceMap();
  writeEsmWrapper();

  for (const artifactPath of [
    path.join(distRoot, 'index.js'),
    path.join(distRoot, 'index.cjs'),
    path.join(distRoot, 'index.d.ts'),
  ]) {
    if (!existsSync(artifactPath)) {
      fail(`TypeScript generated build did not produce ${artifactPath}.`);
    }
  }

  cleanupTmpWorkspaceRoot();
} finally {
  releaseBuildLock();
}

console.log('[sdkwork-im-admin-sdk] TypeScript generated package build completed.');
