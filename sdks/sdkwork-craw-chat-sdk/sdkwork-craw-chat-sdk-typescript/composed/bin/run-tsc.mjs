import { existsSync } from 'node:fs';
import path from 'node:path';
import { spawnSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';

function resolveCanonicalWorkspaceRoot(workspaceRoot) {
  const worktreeMarker = `${path.sep}.worktrees${path.sep}`;
  const markerIndex = workspaceRoot.lastIndexOf(worktreeMarker);
  if (markerIndex === -1) {
    return workspaceRoot;
  }
  return workspaceRoot.slice(0, markerIndex);
}

function resolveCandidateTscPaths(packageRoot) {
  const workspaceRoot = path.resolve(packageRoot, '..', '..');
  const canonicalWorkspaceRoot = resolveCanonicalWorkspaceRoot(workspaceRoot);
  const canonicalGeneratorRoot = process.env.SDKWORK_GENERATOR_ROOT
    ? path.resolve(process.env.SDKWORK_GENERATOR_ROOT)
    : path.resolve(canonicalWorkspaceRoot, '..', '..', '..', 'sdk', 'sdkwork-sdk-generator');

  return [
    path.join(packageRoot, 'node_modules', 'typescript', 'bin', 'tsc'),
    path.join(packageRoot, '..', 'generated', 'server-openapi', 'node_modules', 'typescript', 'bin', 'tsc'),
    path.join(canonicalGeneratorRoot, 'node_modules', 'typescript', 'bin', 'tsc'),
  ];
}

const scriptDir = path.dirname(fileURLToPath(import.meta.url));
const packageRoot = path.resolve(scriptDir, '..');
const tscPath = resolveCandidateTscPaths(packageRoot).find((candidate) => existsSync(candidate));

if (!tscPath) {
  console.error(
    '[sdkwork-craw-chat-sdk] Unable to locate a TypeScript compiler. Run the SDK generator first or set SDKWORK_GENERATOR_ROOT.',
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
