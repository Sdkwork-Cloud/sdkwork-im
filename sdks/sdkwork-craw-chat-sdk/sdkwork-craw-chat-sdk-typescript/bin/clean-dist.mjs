#!/usr/bin/env node
import { rmSync } from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const scriptDir = path.dirname(fileURLToPath(import.meta.url));
const packageRoot = path.resolve(scriptDir, '..');

rmSync(path.join(packageRoot, 'dist'), { recursive: true, force: true });
