import { spawnSync } from 'node:child_process';
import fs from 'node:fs';
import path from 'node:path';

import { pnpmProcessSpec } from './pnpm-launch-lib.mjs';

const REQUIRED_SDKWORK_UI_DIST_FILES = [
  'index.js',
  'theme.js',
  'components-ui.js',
  'ui-feedback.js',
  'patterns-app-shell.js',
  'patterns-desktop-shell.js',
  'sdkwork-ui.css',
];

function defaultFileExists(filePath) {
  return fs.existsSync(filePath);
}

function defaultRunProcess(command, args, options) {
  return spawnSync(command, args, options);
}

export function resolveSdkworkUiPackageRoot(appRoot, {
  fileExists = defaultFileExists,
} = {}) {
  if (!appRoot) {
    throw new Error('appRoot is required');
  }

  const normalizedAppRoot = path.resolve(appRoot);
  const candidateRoots = [
    path.resolve(normalizedAppRoot, '..', '..', '..', 'sdkwork-ui', 'sdkwork-ui-pc-react'),
    path.resolve(normalizedAppRoot, '..', '..', '..', '..', '..', 'sdkwork-ui', 'sdkwork-ui-pc-react'),
  ];

  return candidateRoots.find((candidateRoot) => (
    fileExists(path.join(candidateRoot, 'package.json'))
  )) ?? null;
}

function hasSdkworkUiBuildTooling(uiPackageRoot, fileExists = defaultFileExists) {
  return fileExists(path.join(uiPackageRoot, 'node_modules', 'vite', 'bin', 'vite.js'));
}

export function hasSdkworkUiDist(uiPackageRoot, {
  fileExists = defaultFileExists,
} = {}) {
  return REQUIRED_SDKWORK_UI_DIST_FILES.every((entryPath) => (
    fileExists(path.join(uiPackageRoot, 'dist', entryPath))
  ));
}

function runPnpmStep(stepArgs, {
  cwd,
  runProcess = defaultRunProcess,
} = {}) {
  const { command, args } = pnpmProcessSpec(stepArgs);
  const result = runProcess(command, args, {
    cwd,
    env: process.env,
    stdio: 'inherit',
  });

  if (result.status !== 0) {
    throw new Error(
      `pnpm ${stepArgs.join(' ')} failed in ${cwd} with exit code ${result.status ?? 'unknown'}`,
    );
  }
}

export function ensureSdkworkUiDist({
  appRoot,
  fileExists = defaultFileExists,
  runProcess = defaultRunProcess,
} = {}) {
  const uiPackageRoot = resolveSdkworkUiPackageRoot(appRoot, { fileExists });

  if (!uiPackageRoot) {
    return null;
  }

  if (!hasSdkworkUiBuildTooling(uiPackageRoot, fileExists)) {
    runPnpmStep(['install', '--ignore-scripts'], {
      cwd: uiPackageRoot,
      runProcess,
    });
  }

  if (!hasSdkworkUiDist(uiPackageRoot, { fileExists })) {
    runPnpmStep(['build'], {
      cwd: uiPackageRoot,
      runProcess,
    });
  }

  if (!hasSdkworkUiDist(uiPackageRoot, { fileExists })) {
    throw new Error(`sdkwork ui dist is still missing after build: ${uiPackageRoot}`);
  }

  return uiPackageRoot;
}
