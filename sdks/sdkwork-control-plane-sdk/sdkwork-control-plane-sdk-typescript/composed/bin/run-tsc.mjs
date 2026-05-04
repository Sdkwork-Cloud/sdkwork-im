import { existsSync } from 'node:fs';
import path from 'node:path';
import { spawnSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';
import { resolveGeneratorModulePath } from '../../../bin/generator-runtime.mjs';

const scriptDir = path.dirname(fileURLToPath(import.meta.url));
const packageRoot = path.resolve(scriptDir, '..');
const workspaceRoot = path.resolve(packageRoot, '..', '..');
const tscCandidates = [
  path.join(packageRoot, 'node_modules', 'typescript', 'bin', 'tsc'),
  path.join(packageRoot, '..', 'generated', 'server-openapi', 'node_modules', 'typescript', 'bin', 'tsc'),
];

try {
  tscCandidates.push(resolveGeneratorModulePath(workspaceRoot, 'typescript', 'bin', 'tsc'));
} catch {
}

const tscPath = tscCandidates.find((candidate) => existsSync(candidate));

if (!tscPath) {
  console.error(
    '[sdkwork-control-plane-sdk] Unable to locate a TypeScript compiler. Run the SDK generator first or set SDKWORK_GENERATOR_ROOT.',
  );
  process.exit(1);
}

const result = spawnSync(process.execPath, [tscPath, ...process.argv.slice(2)], {
  cwd: packageRoot,
  stdio: 'inherit',
});

if (result.error) {
  throw result.error;
}

process.exit(result.status ?? 1);
