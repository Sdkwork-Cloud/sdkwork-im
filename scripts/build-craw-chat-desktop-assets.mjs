#!/usr/bin/env node

import { spawn } from 'node:child_process';
import { copyFile, mkdir, readdir } from 'node:fs/promises';
import path from 'node:path';
import process from 'node:process';
import { fileURLToPath } from 'node:url';

import { pnpmProcessSpec } from './dev/pnpm-launch-lib.mjs';

const scriptDir = path.dirname(fileURLToPath(import.meta.url));
const workspaceRoot = path.resolve(scriptDir, '..');

export function createDesktopAssetBuildPlan({
  workspaceRoot: buildWorkspaceRoot = workspaceRoot,
  platform = process.platform,
} = {}) {
  return [
    {
      cwd: path.join(buildWorkspaceRoot, 'apps', 'craw-chat-admin'),
      ...pnpmProcessSpec(['build'], { platform }),
    },
  ];
}

async function copyDirectory(sourceRoot, targetRoot) {
  await mkdir(targetRoot, { recursive: true });

  const entries = await readdir(sourceRoot, { withFileTypes: true });
  for (const entry of entries) {
    const sourcePath = path.join(sourceRoot, entry.name);
    const targetPath = path.join(targetRoot, entry.name);

    if (entry.isDirectory()) {
      // Recursively mirror the portal static source tree into dist.
      // The portal build is a deterministic file copy, not a Vite pipeline.
      // This keeps desktop bundling independent from portal node_modules state.
      // It intentionally mirrors the existing portal build contract.
      // The admin app remains on the router-style pnpm build path above.
      // The portal stays lightweight and static.
      // This also avoids Windows child-process lock issues observed with portal build.mjs.
      await copyDirectory(sourcePath, targetPath);
      continue;
    }

    await copyFile(sourcePath, targetPath);
  }
}

async function buildPortalAssets({
  workspaceRoot: buildWorkspaceRoot = workspaceRoot,
} = {}) {
  const adminRoot = path.join(buildWorkspaceRoot, 'apps', 'craw-chat-admin');
  const portalRoot = path.join(buildWorkspaceRoot, 'apps', 'craw-chat-portal');
  const portalDistRoot = path.join(adminRoot, 'dist-portal');

  await mkdir(portalDistRoot, { recursive: true });
  await copyFile(
    path.join(portalRoot, 'index.html'),
    path.join(portalDistRoot, 'index.html'),
  );

  for (const directoryName of ['src', 'packages']) {
    await copyDirectory(
      path.join(portalRoot, directoryName),
      path.join(portalDistRoot, directoryName),
    );
  }
}

async function runBuildStep(step) {
  await new Promise((resolve, reject) => {
    const child = spawn(step.command, step.args, {
      cwd: step.cwd,
      stdio: 'inherit',
      windowsHide: process.platform === 'win32',
    });

    child.on('error', reject);
    child.on('exit', (code, signal) => {
      if (signal) {
        reject(new Error(`build in ${step.cwd} exited with signal ${signal}`));
        return;
      }

      if ((code ?? 1) !== 0) {
        reject(new Error(`build in ${step.cwd} exited with code ${code ?? 1}`));
        return;
      }

      resolve();
    });
  });
}

async function main() {
  const plan = createDesktopAssetBuildPlan();
  for (const step of plan) {
    // eslint-disable-next-line no-await-in-loop
    await runBuildStep(step);
  }

  await buildPortalAssets();
}

if (fileURLToPath(import.meta.url) === process.argv[1]) {
  main().catch((error) => {
    console.error(`[build-craw-chat-desktop-assets] ${error.message}`);
    process.exit(1);
  });
}
