import { existsSync } from 'node:fs';
import path from 'node:path';
import { pathToFileURL } from 'node:url';

export function resolveGeneratorRoot(workspaceRoot) {
  if (process.env.SDKWORK_GENERATOR_ROOT) {
    const explicitRoot = path.resolve(process.env.SDKWORK_GENERATOR_ROOT);
    if (existsSync(explicitRoot)) {
      return explicitRoot;
    }
  }

  const probeNames = [
    ['sdk', 'sdkwork-sdk-generator'],
  ];
  let current = path.resolve(workspaceRoot);

  while (true) {
    for (const segments of probeNames) {
      const candidate = path.join(current, ...segments);
      if (existsSync(candidate)) {
        return candidate;
      }
    }

    const parent = path.dirname(current);
    if (parent === current) {
      break;
    }
    current = parent;
  }

  throw new Error(
    `Unable to locate sdkwork-sdk-generator from workspace root ${workspaceRoot}. ` +
      'Set SDKWORK_GENERATOR_ROOT to an explicit path.',
  );
}

export function resolveGeneratorModulePath(workspaceRoot, ...segments) {
  return path.join(resolveGeneratorRoot(workspaceRoot), 'node_modules', ...segments);
}

export async function loadYamlFromGenerator(workspaceRoot) {
  const yamlPath = resolveGeneratorModulePath(workspaceRoot, 'js-yaml', 'dist', 'js-yaml.mjs');
  if (!existsSync(yamlPath)) {
    throw new Error(`Unable to locate js-yaml from generator workspace: ${yamlPath}`);
  }

  const yamlModule = await import(pathToFileURL(yamlPath).href);
  return yamlModule.default;
}
