#!/usr/bin/env node

import process from 'node:process';
import { pathToFileURL } from 'node:url';

import {
  applyWindowsVitePreload,
  ensureLocalNodeModules,
  resolveReadablePackageEntry,
  resolveWorkspaceDonorRoots,
} from './vite-runtime-lib.mjs';

const REQUIRED_APP_PACKAGES = [
  '@sdkwork/ui-pc-react',
  '@tailwindcss/vite',
  '@vitejs/plugin-react',
  'lucide-react',
  'react',
  'react-dom',
  'react-router-dom',
  'tailwindcss',
  'vite',
];

const appRoot = process.cwd();
const donorRoots = resolveWorkspaceDonorRoots(appRoot);
ensureLocalNodeModules({
  appRoot,
  donorRoots,
  requiredPackages: REQUIRED_APP_PACKAGES,
});
const viteCliPath = resolveReadablePackageEntry({
  appRoot,
  donorRoots,
  packageName: 'vite',
  relativeEntry: ['bin', 'vite.js'],
});

await applyWindowsVitePreload();

process.argv = [
  process.argv[0],
  viteCliPath,
  ...process.argv.slice(2),
];

await import(pathToFileURL(viteCliPath).href);
