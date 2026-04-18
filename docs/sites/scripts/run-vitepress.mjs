import process from 'node:process';
import { pathToFileURL } from 'node:url';

import {
  ensureLocalNodeModules,
  resolveReadablePackageEntry,
  resolveWorkspaceDonorRoots,
} from '../../../scripts/dev/vite-runtime-lib.mjs';

const REQUIRED_DOCS_PACKAGES = [
  'vite',
  'vitepress',
];

const appRoot = process.cwd();
const donorRoots = resolveWorkspaceDonorRoots(appRoot);

ensureLocalNodeModules({
  appRoot,
  donorRoots,
  requiredPackages: REQUIRED_DOCS_PACKAGES,
});

const vitepressCliPath = resolveReadablePackageEntry({
  appRoot,
  donorRoots,
  packageName: 'vitepress',
  relativeEntry: ['bin', 'vitepress.js'],
});

const originalConsoleError = console.error.bind(console);
let printedSpawnEpermGuidance = false;

console.error = (...args) => {
  originalConsoleError(...args);

  if (printedSpawnEpermGuidance) {
    return;
  }

  const rendered = args
    .map((value) => {
      if (typeof value === 'string') {
        return value;
      }
      if (value instanceof Error) {
        return `${value.name}: ${value.message}`;
      }
      return String(value);
    })
    .join(' ');

  if (!rendered.includes('spawn EPERM')) {
    return;
  }

  printedSpawnEpermGuidance = true;
  originalConsoleError(
    [
      '',
      'VitePress is failing here because this environment blocks child-process spawning.',
      'The docs dependencies are already resolved; the remaining blocker is Vite/VitePress config loading through esbuild, which requires child_process.spawn.',
      'Use `npm run docs:verify` in this environment, or rerun `npm run docs:dev` / `npm run docs:build` on a machine that permits subprocess execution.',
    ].join('\n'),
  );
};

process.argv = [
  process.argv[0],
  vitepressCliPath,
  ...process.argv.slice(2),
];

try {
  await import(pathToFileURL(vitepressCliPath).href);
} catch (error) {
  if (
    error
    && typeof error === 'object'
    && 'code' in error
    && error.code === 'EPERM'
  ) {
    const command = process.argv[2] ?? 'vitepress';
    console.error(
      [
        `VitePress ${command} failed because this environment blocks child-process spawning (spawn EPERM).`,
        'Dependency resolution is already in place; the remaining blocker is Vite/VitePress config loading through esbuild, which requires launching a subprocess.',
        'Run docs:verify in this environment, or rerun docs:dev/docs:build on a machine that allows child_process.spawn.',
      ].join('\n'),
    );
  }

  throw error;
}
