import assert from 'node:assert/strict';
import { spawn } from 'node:child_process';
import { access } from 'node:fs/promises';
import { test } from 'node:test';
import { appRoot, fromAppRoot } from './support/testPaths.mjs';

const distIndexHtml = fromAppRoot('dist', 'index.html');
const buildScriptPath = fromAppRoot('scripts', 'build.mjs');

function runBuild() {
  return new Promise((resolve, reject) => {
    const child = spawn(process.execPath, [buildScriptPath], {
      cwd: appRoot,
      stdio: 'ignore',
    });

    child.on('error', reject);
    child.on('close', resolve);
  });
}

test('portal builds into a standalone distributable without external installs', async () => {
  const exitCode = await runBuild();

  assert.equal(exitCode, 0);
  await access(distIndexHtml);
});

test('portal build tolerates concurrent invocations targeting the same dist directory', async () => {
  const exitCodes = await Promise.all([runBuild(), runBuild(), runBuild(), runBuild()]);

  assert.deepEqual(exitCodes, [0, 0, 0, 0]);
  await access(distIndexHtml);
});
