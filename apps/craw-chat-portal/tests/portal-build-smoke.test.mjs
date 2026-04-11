import assert from 'node:assert/strict';
import { spawn } from 'node:child_process';
import { access } from 'node:fs/promises';
import path from 'node:path';
import { test } from 'node:test';

const workspaceRoot = path.resolve('.');
const distIndexHtml = path.join(workspaceRoot, 'apps/craw-chat-portal/dist/index.html');

function runBuild() {
  return new Promise((resolve, reject) => {
    const child = spawn(process.execPath, ['apps/craw-chat-portal/scripts/build.mjs'], {
      cwd: workspaceRoot,
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
