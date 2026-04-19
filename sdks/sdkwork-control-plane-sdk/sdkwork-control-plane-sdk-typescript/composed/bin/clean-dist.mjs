#!/usr/bin/env node
import { existsSync, readdirSync, rmSync } from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const scriptDir = path.dirname(fileURLToPath(import.meta.url));
const packageRoot = path.resolve(scriptDir, '..');
const distRoot = path.join(packageRoot, 'dist');

if (!existsSync(distRoot)) {
  process.exit(0);
}

for (const entry of readdirSync(distRoot, { withFileTypes: true })) {
  if (entry.isDirectory()) {
    rmSync(path.join(distRoot, entry.name), { force: true, recursive: true });
    continue;
  }

  if (!entry.isFile() || !entry.name.endsWith('.map')) {
    continue;
  }
  rmSync(path.join(distRoot, entry.name), { force: true });
}
