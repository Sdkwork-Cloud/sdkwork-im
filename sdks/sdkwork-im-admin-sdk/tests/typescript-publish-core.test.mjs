import assert from 'node:assert/strict';
import { copyFileSync, existsSync, mkdtempSync, mkdirSync, rmSync, writeFileSync } from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import { spawnSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';

const workspaceRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..');
const publishCorePath = path.join(
  workspaceRoot,
  'sdkwork-im-admin-sdk-typescript',
  'generated',
  'server-openapi',
  'bin',
  'publish-core.mjs',
);

function resolveCurrentNpmCliPath() {
  const nodeDir = path.dirname(process.execPath);
  for (const candidate of [
    path.join(nodeDir, 'node_modules', 'npm', 'bin', 'npm-cli.js'),
    path.join(nodeDir, '..', 'lib', 'node_modules', 'npm', 'bin', 'npm-cli.js'),
    path.join(nodeDir, '..', 'node_modules', 'npm', 'bin', 'npm-cli.js'),
  ]) {
    if (existsSync(candidate)) {
      return candidate;
    }
  }

  throw new Error(`Unable to resolve npm-cli.js from Node runtime: ${process.execPath}`);
}

const tempRoot = mkdtempSync(path.join(os.tmpdir(), 'sdkwork-im-admin-sdk-publish-core-'));
const packageRoot = path.join(tempRoot, 'package');
const runtimeRoot = path.join(tempRoot, 'runtime');
const npmCacheDir = path.join(tempRoot, 'npm-cache');
const tempNodePath = path.join(runtimeRoot, process.platform === 'win32' ? 'node.exe' : 'node');
const npmCliPath = resolveCurrentNpmCliPath();

try {
  mkdirSync(packageRoot, { recursive: true });
  mkdirSync(runtimeRoot, { recursive: true });
  mkdirSync(npmCacheDir, { recursive: true });

  copyFileSync(process.execPath, tempNodePath);

  writeFileSync(
    path.join(packageRoot, 'package.json'),
    JSON.stringify(
      {
        name: '@sdkwork-internal/im-admin-publish-core-regression',
        version: '1.0.0',
        type: 'module',
      },
      null,
      2,
    ),
    'utf8',
  );
  writeFileSync(path.join(packageRoot, 'README.md'), '# im admin publish-core regression\n', 'utf8');

  const result = spawnSync(
    tempNodePath,
    [
      publishCorePath,
      '--language',
      'typescript',
      '--project-dir',
      packageRoot,
      '--action',
      'check',
    ],
    {
      cwd: workspaceRoot,
      encoding: 'utf8',
      shell: false,
      env: {
        ...process.env,
        PATH: '',
        Path: '',
        SDKWORK_NPM_CLI: npmCliPath,
        NPM_CONFIG_CACHE: npmCacheDir,
      },
    },
  );

  const output = `${result.stdout || ''}${result.stderr || ''}`;
  assert.equal(
    result.status,
    0,
    `publish-core check must succeed with SDKWORK_NPM_CLI even when PATH is empty.\n${output}`,
  );
  assert.match(output, /\[sdk-publish\] Done\./);
} finally {
  rmSync(tempRoot, { recursive: true, force: true });
}

console.log('typescript publish-core tests passed');
