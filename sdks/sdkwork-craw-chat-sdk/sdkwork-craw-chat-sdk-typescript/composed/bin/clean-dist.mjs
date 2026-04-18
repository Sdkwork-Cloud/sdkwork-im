#!/usr/bin/env node
import { existsSync, readdirSync, renameSync, rmSync } from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const scriptDir = path.dirname(fileURLToPath(import.meta.url));
const packageRoot = path.resolve(scriptDir, '..');
const distRoot = path.join(packageRoot, 'dist');
const promotedBuildRoot = path.join(distRoot, 'composed', 'src');
const transientBuildDirectories = new Set(['composed', 'generated']);

if (existsSync(distRoot)) {
  for (const entry of readdirSync(distRoot, { withFileTypes: true })) {
    if (transientBuildDirectories.has(entry.name)) {
      continue;
    }
    rmSync(path.join(distRoot, entry.name), { recursive: true, force: true });
  }
}

if (existsSync(promotedBuildRoot)) {
  for (const entry of readdirSync(promotedBuildRoot, { withFileTypes: true })) {
    const sourcePath = path.join(promotedBuildRoot, entry.name);
    const targetPath = path.join(distRoot, entry.name);
    rmSync(targetPath, { recursive: true, force: true });
    renameSync(sourcePath, targetPath);
  }
}

for (const relativePath of ['dist/composed', 'dist/generated']) {
  const absolutePath = path.join(packageRoot, relativePath);
  if (existsSync(absolutePath)) {
    rmSync(absolutePath, { recursive: true, force: true });
  }
}
