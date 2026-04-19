import { existsSync } from 'node:fs';
import path from 'node:path';

function resolveWorkspaceProbe(workspaceRoot, probeSegments, errorMessage) {
  let current = path.resolve(workspaceRoot);

  while (true) {
    const candidate = path.join(current, ...probeSegments);
    if (existsSync(candidate)) {
      return candidate;
    }

    const parent = path.dirname(current);
    if (parent === current) {
      break;
    }
    current = parent;
  }

  throw new Error(errorMessage(workspaceRoot));
}

export function resolveGeneratorRoot(workspaceRoot) {
  if (process.env.SDKWORK_GENERATOR_ROOT) {
    const explicitRoot = path.resolve(process.env.SDKWORK_GENERATOR_ROOT);
    if (existsSync(explicitRoot)) {
      return explicitRoot;
    }
  }

  return resolveWorkspaceProbe(
    workspaceRoot,
    ['sdk', 'sdkwork-sdk-generator'],
    (root) =>
      `Unable to locate sdkwork-sdk-generator from workspace root ${root}. ` +
      'Set SDKWORK_GENERATOR_ROOT to an explicit path.',
  );
}

export function resolveGeneratorModulePath(workspaceRoot, ...segments) {
  const generatorRoot = resolveGeneratorRoot(workspaceRoot);
  const candidates = [
    path.join(generatorRoot, 'node_modules', ...segments),
    path.join(generatorRoot, 'node_modules', '.pnpm', 'node_modules', ...segments),
  ];

  for (const candidate of candidates) {
    if (existsSync(candidate)) {
      return candidate;
    }
  }

  throw new Error(
    `Unable to locate generator module ${segments.join('/')} from ${generatorRoot}. ` +
    `Tried: ${candidates.join(', ')}`,
  );
}

export function resolveSdkCommonRoot(workspaceRoot) {
  if (process.env.SDKWORK_SDK_COMMON_ROOT) {
    const explicitRoot = path.resolve(process.env.SDKWORK_SDK_COMMON_ROOT);
    if (existsSync(explicitRoot)) {
      return explicitRoot;
    }
  }

  return resolveWorkspaceProbe(
    workspaceRoot,
    ['sdk', 'sdkwork-sdk-commons', 'sdkwork-sdk-common-typescript'],
    (root) =>
      `Unable to locate sdkwork-sdk-common-typescript from workspace root ${root}. ` +
      'Set SDKWORK_SDK_COMMON_ROOT to an explicit path.',
  );
}

export function resolveSdkCommonPath(workspaceRoot, ...segments) {
  return path.join(resolveSdkCommonRoot(workspaceRoot), ...segments);
}
