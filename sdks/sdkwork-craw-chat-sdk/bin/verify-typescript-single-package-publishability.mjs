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
import path from 'node:path';
import { spawnSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';

const scriptDir = path.dirname(fileURLToPath(import.meta.url));
const workspaceRoot = path.resolve(scriptDir, '..');
const typescriptRoot = path.join(workspaceRoot, 'sdkwork-craw-chat-sdk-typescript');
const verificationRunId = `run-${process.pid}-${Date.now()}-${Math.random().toString(16).slice(2, 10)}`;
const cacheDir = path.join(
  workspaceRoot,
  '.sdkwork',
  'tmp',
  'verify-typescript-single-package-publishability',
  verificationRunId,
  'npm-cache',
);
const packageJsonPath = path.join(typescriptRoot, 'package.json');
const distRoot = path.join(typescriptRoot, 'dist');
const distTypesPath = path.join(distRoot, 'types.d.ts');
const requiredDistArtifacts = [
  'index.js',
  'index.d.ts',
  'sdk.js',
  'sdk.d.ts',
  'types.d.ts',
  'generated/index.js',
  'generated/index.d.ts',
  'generated/sdk.js',
  'generated/sdk.d.ts',
  'generated/types/index.d.ts',
];
const forbiddenDeadDistArtifacts = [
  'portal-module',
  'device-module',
  'inbox-module',
  'presence-module',
  'session-module',
  'streams-module',
];

function fail(message) {
  console.error(`[sdkwork-craw-chat-sdk] ${message}`);
  process.exit(1);
}

function run(command, args, options = {}) {
  const spawnOptions = {
    cwd: typescriptRoot,
    shell: false,
    env: options.env,
  };
  let capturePath = null;
  let captureFd = null;
  let captureRoot = null;
  let captureRootParent = null;

  if (options.captureOutput) {
    captureRootParent = path.join(
      workspaceRoot,
      '.sdkwork',
      'tmp',
      'verify-typescript-single-package-publishability',
    );
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
  fail('TypeScript single-package package.json is missing.');
}

const packageJson = JSON.parse(readFileSync(packageJsonPath, 'utf8'));
if (packageJson.name !== '@sdkwork/craw-chat-sdk') {
  fail('TypeScript single-package package name must stay on @sdkwork/craw-chat-sdk.');
}
if (packageJson.main !== './dist/index.js') {
  fail('TypeScript single-package main must stay on ./dist/index.js.');
}
if (packageJson.types !== './dist/index.d.ts') {
  fail('TypeScript single-package types must stay on ./dist/index.d.ts.');
}

const missingArtifacts = requiredDistArtifacts.filter(
  (relativePath) => !existsSync(path.join(distRoot, relativePath)),
);
if (missingArtifacts.length > 0) {
  fail(
    `TypeScript single-package is missing required dist artifacts: ${missingArtifacts.join(', ')}`,
  );
}

if (!existsSync(distTypesPath)) {
  fail('TypeScript single-package dist/types.d.ts is missing.');
}

const distTypesSource = readFileSync(distTypesPath, 'utf8');
const liveConnectionBlockMatch = distTypesSource.match(
  /export interface CrawChatLiveConnection \{([\s\S]*?)\n\}/,
);
if (!liveConnectionBlockMatch) {
  fail('TypeScript single-package dist/types.d.ts must define CrawChatLiveConnection.');
}

for (const legacySignature of [
  'onMessage(',
  'onConversationMessage(',
  'onData(',
  'onSignal(',
  'onRawEvent(',
  'onStateChange(',
  'onError(',
]) {
  if (liveConnectionBlockMatch[1].includes(legacySignature)) {
    fail(
      `TypeScript single-package dist/types.d.ts must not expose legacy live flat callbacks on CrawChatLiveConnection: ${legacySignature}`,
    );
  }
}

const liveStatusMatch = distTypesSource.match(
  /export type CrawChatLiveConnectionStatus = ([^;]+);/,
);
if (!liveStatusMatch) {
  fail('TypeScript single-package dist/types.d.ts must define CrawChatLiveConnectionStatus.');
}
if (liveStatusMatch[1].includes("'connecting'")) {
  fail(
    "TypeScript single-package dist/types.d.ts must not expose 'connecting' on CrawChatLiveConnectionStatus when sdk.connect(...) resolves only after the connection is established.",
  );
}

mkdirSync(cacheDir, { recursive: true });

try {
  const npmCliPath = path.join(path.dirname(process.execPath), 'node_modules', 'npm', 'bin', 'npm-cli.js');
  if (!existsSync(npmCliPath)) {
    fail(`TypeScript single-package verification could not find npm CLI at ${npmCliPath}.`);
  }

  const packOutput = run('node', [npmCliPath, 'pack', '--dry-run', '--json'], {
    step: 'typescript-single-package:pack-dry-run',
    captureOutput: true,
    env: {
      ...process.env,
      NPM_CONFIG_CACHE: cacheDir,
    },
  });
  const packManifest = parseJson('typescript-single-package:pack-dry-run', packOutput);
  const packEntries = Array.isArray(packManifest) ? packManifest : [packManifest];
  if (packEntries.length === 0) {
    fail('TypeScript single-package npm pack manifest must include at least one package entry.');
  }

  const packedFiles = Array.isArray(packEntries[0]?.files) ? packEntries[0].files : [];
  if (packedFiles.length === 0) {
    fail('TypeScript single-package npm pack manifest must list packaged files.');
  }

  const packedPaths = packedFiles
    .map((entry) => String(entry?.path || '').replace(/\\/g, '/'))
    .filter(Boolean);
  const requiredPackedPaths = [
    'README.md',
    'package.json',
    ...requiredDistArtifacts.map((relativePath) => `dist/${relativePath}`),
  ];
  const missingPackedPaths = requiredPackedPaths.filter(
    (relativePath) => !packedPaths.includes(relativePath),
  );
  if (missingPackedPaths.length > 0) {
    fail(
      `TypeScript single-package npm pack manifest is missing required files: ${missingPackedPaths.join(', ')}`,
    );
  }

  const forbiddenPackedPaths = packedPaths.filter(
    (relativePath) =>
      relativePath.startsWith('src/') ||
      relativePath.startsWith('test/') ||
      relativePath.startsWith('bin/') ||
      relativePath.startsWith('composed/') ||
      relativePath.startsWith('generated/') ||
      relativePath.startsWith('.npm-cache/') ||
      relativePath.startsWith('.sdkwork/') ||
      relativePath.startsWith('node_modules/'),
  );
  if (forbiddenPackedPaths.length > 0) {
    fail(
      `TypeScript single-package npm pack manifest must not ship source, workspace, or cache artifacts: ${forbiddenPackedPaths.join(', ')}`,
    );
  }

  const leakedDeadArtifacts = packedPaths.filter((relativePath) =>
    forbiddenDeadDistArtifacts.some(
      (artifactBaseName) =>
        relativePath === `dist/${artifactBaseName}.js`
        || relativePath === `dist/${artifactBaseName}.d.ts`
        || relativePath === `dist/${artifactBaseName}.d.ts.map`,
    ),
  );
  if (leakedDeadArtifacts.length > 0) {
    fail(
      `TypeScript single-package npm pack manifest must not ship dead authoring modules: ${leakedDeadArtifacts.join(', ')}`,
    );
  }
} finally {
  if (existsSync(cacheDir)) {
    rmSync(cacheDir, { recursive: true, force: true });
  }
}

console.log('[sdkwork-craw-chat-sdk] TypeScript single-package publishability verification passed.');
