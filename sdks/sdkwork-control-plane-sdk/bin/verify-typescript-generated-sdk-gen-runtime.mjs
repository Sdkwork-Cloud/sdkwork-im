#!/usr/bin/env node
import { existsSync, mkdtempSync, mkdirSync, readFileSync, rmSync, writeFileSync } from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import { spawnSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';

function fail(message) {
  console.error(`[sdkwork-control-plane-sdk] ${message}`);
  process.exit(1);
}

const scriptDir = path.dirname(fileURLToPath(import.meta.url));
const workspaceRoot = path.resolve(scriptDir, '..');
const generatedBinRoot = path.join(
  workspaceRoot,
  'sdkwork-control-plane-sdk-typescript',
  'generated',
  'server-openapi',
  'bin',
);
const sdkGenCorePath = path.join(generatedBinRoot, 'sdk-gen-core.mjs');
const sdkGenShellPath = path.join(generatedBinRoot, 'sdk-gen.sh');
const sdkGenBatchPath = path.join(generatedBinRoot, 'sdk-gen.bat');

if (!existsSync(sdkGenCorePath)) {
  fail('TypeScript generated workspace must provide sdkwork-control-plane-sdk-typescript/generated/server-openapi/bin/sdk-gen-core.mjs.');
}

const sdkGenShellSource = readFileSync(sdkGenShellPath, 'utf8');
if (!/sdk-gen-core\.mjs/.test(sdkGenShellSource)) {
  fail('TypeScript generated sdk-gen.sh must delegate to sdk-gen-core.mjs.');
}
if (/npm install && npm run build/.test(sdkGenShellSource)) {
  fail('TypeScript generated sdk-gen.sh must not inline bare npm install/build commands.');
}

const sdkGenBatchSource = readFileSync(sdkGenBatchPath, 'utf8');
if (!/sdk-gen-core\.mjs/i.test(sdkGenBatchSource)) {
  fail('TypeScript generated sdk-gen.bat must delegate to sdk-gen-core.mjs.');
}
if (/npm install && npm run build/i.test(sdkGenBatchSource)) {
  fail('TypeScript generated sdk-gen.bat must not inline bare npm install/build commands.');
}

const tempRoot = mkdtempSync(path.join(os.tmpdir(), 'sdkwork-control-plane-sdk-sdk-gen-'));
const packageRoot = path.join(tempRoot, 'generated-package');
const npmCacheDir = path.join(tempRoot, 'npm-cache');

try {
  mkdirSync(packageRoot, { recursive: true });
  mkdirSync(npmCacheDir, { recursive: true });

  writeFileSync(
    path.join(packageRoot, 'package.json'),
    JSON.stringify(
      {
        name: '@sdkwork-internal/control-plane-sdk-gen-runtime-regression',
        version: '1.0.0',
        private: true,
        type: 'module',
        scripts: {
          build: 'call "%npm_node_execpath%" ./build-ok.mjs || "$npm_node_execpath" ./build-ok.mjs || node ./build-ok.mjs',
        },
      },
      null,
      2,
    ),
    'utf8',
  );
  writeFileSync(
    path.join(packageRoot, 'build-ok.mjs'),
    "console.log('generated build ok');\n",
    'utf8',
  );

  const result = spawnSync(
    process.execPath,
    [sdkGenCorePath, 'build', '--project-dir', packageRoot],
    {
      cwd: workspaceRoot,
      encoding: 'utf8',
      shell: false,
      env: {
        ...process.env,
        PATH: '',
        Path: '',
        NPM_CONFIG_CACHE: npmCacheDir,
        npm_config_cache: npmCacheDir,
      },
    },
  );

  const output = `${result.stdout || ''}${result.stderr || ''}`;
  if ((result.status ?? 1) !== 0) {
    fail(`TypeScript generated sdk-gen runtime verification failed.\n${output}`);
  }
  if (!/\[sdk-gen\] Done\./.test(output)) {
    fail(`TypeScript generated sdk-gen runtime verification must report completion.\n${output}`);
  }
  if (!/generated build ok/.test(output)) {
    fail(`TypeScript generated sdk-gen runtime verification must execute the package build script.\n${output}`);
  }
} finally {
  rmSync(tempRoot, { recursive: true, force: true });
}

console.log('[sdkwork-control-plane-sdk] verify-typescript-generated-sdk-gen-runtime passed');
