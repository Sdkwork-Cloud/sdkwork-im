import assert from 'node:assert/strict';
import { mkdtemp, mkdir, rm, writeFile } from 'node:fs/promises';
import { readFileSync } from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import test from 'node:test';
import { pathToFileURL } from 'node:url';

const repoRoot = path.resolve(import.meta.dirname, '..');
const scriptPath = path.join(repoRoot, 'scripts', 'build-craw-chat-desktop-assets.mjs');

async function loadModule() {
  return import(pathToFileURL(scriptPath).href);
}

test('desktop asset build script is aligned with the sdkwork-chat-pc app root', async () => {
  const source = readFileSync(scriptPath, 'utf8');
  assert.doesNotMatch(
    source,
    /apps[\\/](?:control-plane|craw-chat-admin|craw-chat-portal)/u,
    'desktop asset build script must not reference retired app roots',
  );

  const module = await loadModule();
  assert.equal(typeof module.createDesktopAssetBuildPlan, 'function');
  assert.equal(typeof module.assertDesktopPcWebAssetsReady, 'function');
  assert.equal(typeof module.assertDesktopEmbeddedSitesReady, 'function');

  const plan = module.createDesktopAssetBuildPlan({
    platform: 'linux',
    workspaceRoot: repoRoot,
  });
  assert.deepEqual(
    plan.map((step) => ({
      args: step.args,
      command: step.command,
      cwd: path.relative(repoRoot, step.cwd).replaceAll('\\', '/'),
    })),
    [
      {
        args: ['build'],
        command: 'pnpm',
        cwd: 'apps/sdkwork-chat-pc',
      },
    ],
  );
});

test('desktop asset readiness checks the sdkwork-chat-pc dist output', async () => {
  const module = await loadModule();
  const tempRoot = await mkdtemp(path.join(os.tmpdir(), 'craw-chat-desktop-assets-'));
  try {
    await mkdir(path.join(tempRoot, 'apps', 'sdkwork-chat-pc', 'dist'), { recursive: true });
    await writeFile(
      path.join(tempRoot, 'apps', 'sdkwork-chat-pc', 'dist', 'index.html'),
      '<!doctype html><html><body>SDKWork Chat PC</body></html>',
    );

    await module.assertDesktopPcWebAssetsReady({ workspaceRoot: tempRoot });
    await module.assertDesktopEmbeddedSitesReady({ workspaceRoot: tempRoot });
  } finally {
    await rm(tempRoot, { force: true, recursive: true });
  }
});
