import assert from 'node:assert/strict';
import { mkdtempSync, mkdirSync, rmSync, writeFileSync } from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import { spawnSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';

const workspaceRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..');
const publishCorePath = path.join(
  workspaceRoot,
  'sdkwork-control-plane-sdk-typescript',
  'generated',
  'server-openapi',
  'bin',
  'publish-core.mjs',
);

const tempRoot = mkdtempSync(path.join(os.tmpdir(), 'sdkwork-control-plane-sdk-publish-core-'));
const packageRoot = path.join(tempRoot, 'package');
const npmCacheDir = path.join(tempRoot, 'npm-cache');

try {
  mkdirSync(packageRoot, { recursive: true });
  mkdirSync(npmCacheDir, { recursive: true });

  writeFileSync(
    path.join(packageRoot, 'package.json'),
    JSON.stringify(
      {
        name: '@sdkwork-internal/control-plane-publish-core-regression',
        version: '1.0.0',
        type: 'module',
      },
      null,
      2,
    ),
    'utf8',
  );
  writeFileSync(path.join(packageRoot, 'README.md'), '# control-plane publish-core regression\n', 'utf8');

  const result = spawnSync(
    process.execPath,
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
        NPM_CONFIG_CACHE: npmCacheDir,
      },
    },
  );

  const output = `${result.stdout || ''}${result.stderr || ''}`;
  assert.equal(
    result.status,
    0,
    `publish-core check must succeed without npm on PATH.\n${output}`,
  );
  assert.match(output, /\[sdk-publish\] Done\./);
} finally {
  rmSync(tempRoot, { recursive: true, force: true });
}

console.log('typescript publish-core tests passed');
