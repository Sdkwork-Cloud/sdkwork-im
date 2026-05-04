#!/usr/bin/env node
import { spawnSync } from 'node:child_process';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

import { resolveGeneratorModulePath } from '../../../bin/generator-runtime.mjs';

const scriptDir = path.dirname(fileURLToPath(import.meta.url));
const packageRoot = path.resolve(scriptDir, '..');
const workspaceRoot = path.resolve(packageRoot, '..', '..');
const tscPath = resolveGeneratorModulePath(workspaceRoot, 'typescript', 'bin', 'tsc');

const result = spawnSync(process.execPath, [tscPath, ...process.argv.slice(2)], {
  cwd: packageRoot,
  stdio: 'inherit',
  shell: false,
});

if (result.error) {
  throw result.error;
}

process.exit(result.status ?? 1);
