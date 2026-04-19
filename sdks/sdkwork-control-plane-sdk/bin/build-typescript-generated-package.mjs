#!/usr/bin/env node
import { existsSync, rmSync } from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import {
  resolveTypescriptGeneratedBuildPaths,
  runTypescriptBuildCommand,
} from '../../workspace-typescript-build-shared.mjs';

const prefix = 'sdkwork-control-plane-sdk';
const scriptDir = path.dirname(fileURLToPath(import.meta.url));
const workspaceRoot = path.resolve(scriptDir, '..');
const { generatedRoot, distRoot } = resolveTypescriptGeneratedBuildPaths({
  workspaceRoot,
  relativeGeneratedRoot: path.join('sdkwork-control-plane-sdk-typescript', 'generated', 'server-openapi'),
});
const runTscPath = path.join(generatedRoot, 'bin', 'run-tsc.mjs');
const tsconfigPath = path.join(generatedRoot, 'tsconfig.json');

if (existsSync(distRoot)) {
  rmSync(distRoot, { recursive: true, force: true });
}

runTypescriptBuildCommand({
  prefix,
  command: 'node',
  args: [runTscPath, '-p', tsconfigPath],
  cwd: workspaceRoot,
  step: 'typescript:generated-build',
});
