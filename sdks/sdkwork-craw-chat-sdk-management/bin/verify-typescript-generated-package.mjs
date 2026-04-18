#!/usr/bin/env node
import {
  closeSync,
  existsSync,
  mkdirSync,
  openSync,
  readFileSync,
  rmSync,
  rmdirSync,
} from 'node:fs';
import { createRequire } from 'node:module';
import path from 'node:path';
import { spawnSync } from 'node:child_process';
import { fileURLToPath, pathToFileURL } from 'node:url';

const scriptDir = path.dirname(fileURLToPath(import.meta.url));
const workspaceRoot = path.resolve(scriptDir, '..');
const generatedRoot = path.join(
  workspaceRoot,
  'sdkwork-craw-chat-sdk-management-typescript',
  'generated',
  'server-openapi',
);
const packageJsonPath = path.join(generatedRoot, 'package.json');
const distRoot = path.join(generatedRoot, 'dist');
const cacheDir = path.join(generatedRoot, '.npm-cache');
const requiredArtifacts = [
  'index.js',
  'index.cjs',
  'index.d.ts',
];

function fail(message) {
  console.error(`[sdkwork-craw-chat-sdk-management] ${message}`);
  process.exit(1);
}

function run(command, args, options = {}) {
  const spawnOptions = {
    cwd: generatedRoot,
    shell: false,
    env: options.env,
  };
  let capturePath = null;
  let captureFd = null;
  let captureRoot = null;
  let captureRootParent = null;

  if (options.captureOutput) {
    captureRootParent = path.join(workspaceRoot, '.sdkwork', 'tmp', 'verify-typescript-generated-package');
    captureRoot = path.join(
      captureRootParent,
      `run-${process.pid}-${Date.now()}-${Math.random().toString(16).slice(2, 10)}`,
    );
    mkdirSync(captureRoot, { recursive: true });
    capturePath = path.join(captureRoot, `stdout-${process.pid}-${Date.now()}.log`);
    captureFd = openSync(capturePath, 'w');
    spawnOptions.stdio = ['ignore', captureFd, 'inherit'];
  } else {
    spawnOptions.stdio = 'inherit';
  }

  let result;
  try {
    result = spawnSync(command, args, spawnOptions);
  } finally {
    if (captureFd != null) {
      closeSync(captureFd);
    }
  }

  try {
    if (result.error) {
      fail(`${options.step || command} failed to start: ${result.error.message}`);
    }
    if ((result.status ?? 1) !== 0) {
      const stderr = (result.stderr || '').trim();
      fail(`${options.step || command} failed with exit code ${result.status}${stderr ? `\n${stderr}` : ''}`);
    }
    if (result.signal) {
      fail(`${options.step || command} terminated with signal ${result.signal}`);
    }

    if (!options.captureOutput || capturePath == null) {
      return '';
    }

    return readFileSync(capturePath, 'utf8').trim();
  } finally {
    if (capturePath != null && existsSync(capturePath)) {
      rmSync(capturePath, { force: true });
    }
    if (captureRoot != null && existsSync(captureRoot)) {
      try {
        rmSync(captureRoot, { recursive: true, force: true });
      } catch {
        // Keep the directory when unexpected files are present.
      }
    }
    if (captureRootParent != null && existsSync(captureRootParent)) {
      try {
        rmdirSync(captureRootParent);
      } catch {
        // Keep the parent directory when concurrent runs or unexpected files are present.
      }
    }
  }
}

function runNpm(args, options = {}) {
  if (process.platform === 'win32') {
    return run('cmd.exe', ['/d', '/s', '/c', 'npm', ...args], options);
  }
  return run('npm', args, options);
}

function parseJson(step, source) {
  const trimmed = source.trim();
  if (!trimmed) {
    fail(`${step} produced no output.`);
  }

  const candidates = [trimmed];
  const firstArrayIndex = trimmed.indexOf('[');
  const lastArrayIndex = trimmed.lastIndexOf(']');
  if (firstArrayIndex > 0 && lastArrayIndex > firstArrayIndex) {
    candidates.push(trimmed.slice(firstArrayIndex, lastArrayIndex + 1));
  }

  for (const candidate of candidates) {
    try {
      return JSON.parse(candidate);
    } catch {
      // Fall through to the next candidate shape.
    }
  }

  fail(`${step} produced invalid JSON output.`);
}

if (!existsSync(packageJsonPath)) {
  fail('TypeScript generated package.json is missing.');
}

const packageJson = JSON.parse(readFileSync(packageJsonPath, 'utf8'));
if (packageJson.main !== './dist/index.cjs') {
  fail('TypeScript generated package main must stay on ./dist/index.cjs.');
}
if (packageJson.module !== './dist/index.js') {
  fail('TypeScript generated package module must stay on ./dist/index.js.');
}
if (packageJson.types !== './dist/index.d.ts') {
  fail('TypeScript generated package types must stay on ./dist/index.d.ts.');
}

const missingArtifacts = requiredArtifacts.filter(
  (relativePath) => !existsSync(path.join(distRoot, relativePath)),
);
if (missingArtifacts.length > 0) {
  fail(
    `TypeScript generated package is missing required dist artifacts: ${missingArtifacts.join(', ')}`,
  );
}

run('node', ['./bin/publish-core.mjs', '--language', 'typescript', '--project-dir', '.', '--action', 'check'], {
  step: 'typescript-generated:publish-core-check',
  env: {
    ...process.env,
    NPM_CONFIG_CACHE: cacheDir,
  },
});

const packOutput = runNpm(['pack', '--dry-run', '--json'], {
  step: 'typescript-generated:pack-dry-run',
  captureOutput: true,
  env: {
    ...process.env,
    NPM_CONFIG_CACHE: cacheDir,
  },
});
const packManifest = parseJson('typescript-generated:pack-dry-run', packOutput);
const packEntries = Array.isArray(packManifest) ? packManifest : [packManifest];
if (packEntries.length === 0) {
  fail('TypeScript generated npm pack manifest must include at least one package entry.');
}
const packedFiles = Array.isArray(packEntries[0]?.files) ? packEntries[0].files : [];
if (packedFiles.length === 0) {
  fail('TypeScript generated npm pack manifest must list packaged files.');
}
const packedPaths = packedFiles
  .map((entry) => String(entry?.path || '').replace(/\\/g, '/'))
  .filter(Boolean);
const requiredPackedPaths = ['README.md', 'package.json', ...requiredArtifacts.map((relativePath) => `dist/${relativePath}`)];
const missingPackedPaths = requiredPackedPaths.filter((relativePath) => !packedPaths.includes(relativePath));
if (missingPackedPaths.length > 0) {
  fail(
    `TypeScript generated npm pack manifest is missing required files: ${missingPackedPaths.join(', ')}`,
  );
}
const unexpectedPackedPaths = packedPaths.filter(
  (relativePath) =>
    relativePath !== 'README.md' &&
    relativePath !== 'package.json' &&
    !relativePath.startsWith('dist/'),
);
if (unexpectedPackedPaths.length > 0) {
  fail(
    `TypeScript generated npm pack manifest must only ship README.md, package.json, and dist/* files, but found: ${unexpectedPackedPaths.join(', ')}`,
  );
}

runNpm(['run', 'build', '--silent'], {
  step: 'typescript-generated:npm-build',
  env: {
    ...process.env,
    NPM_CONFIG_CACHE: cacheDir,
  },
});

const esmModule = await import(pathToFileURL(path.join(distRoot, 'index.js')).href);
const require = createRequire(import.meta.url);
const cjsModule = require(path.join(distRoot, 'index.cjs'));

for (const [moduleName, moduleValue] of [
  ['esm:SdkworkBackendClient', esmModule.SdkworkBackendClient],
  ['esm:createClient', esmModule.createClient],
  ['esm:DEFAULT_TIMEOUT', esmModule.DEFAULT_TIMEOUT],
  ['cjs:SdkworkBackendClient', cjsModule.SdkworkBackendClient],
  ['cjs:createClient', cjsModule.createClient],
  ['cjs:DEFAULT_TIMEOUT', cjsModule.DEFAULT_TIMEOUT],
]) {
  if (moduleValue == null) {
    fail(`TypeScript generated package smoke check missing export ${moduleName}.`);
  }
}

console.log('[sdkwork-craw-chat-sdk-management] TypeScript generated package verification passed.');
