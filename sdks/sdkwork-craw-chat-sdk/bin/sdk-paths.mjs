#!/usr/bin/env node
import { existsSync } from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

function normalizeCandidate(candidate) {
  return path.resolve(String(candidate || '').trim());
}

function isSdkworkGeneratorRoot(candidate) {
  return existsSync(path.join(candidate, 'bin', 'sdkgen.js'));
}

export function resolveSdkworkGeneratorRoot(workspaceRoot) {
  const override = process.env.SDKWORK_GENERATOR_ROOT;
  if (override && override.trim()) {
    const resolvedOverride = normalizeCandidate(override);
    if (!isSdkworkGeneratorRoot(resolvedOverride)) {
      throw new Error(`SDKWORK_GENERATOR_ROOT does not point to sdkwork-sdk-generator: ${resolvedOverride}`);
    }
    return resolvedOverride;
  }

  let current = normalizeCandidate(workspaceRoot);
  while (true) {
    const candidate = path.join(current, 'sdk', 'sdkwork-sdk-generator');
    if (isSdkworkGeneratorRoot(candidate)) {
      return candidate;
    }

    const parent = path.dirname(current);
    if (parent === current) {
      break;
    }
    current = parent;
  }

  throw new Error(
    `Unable to resolve sdkwork-sdk-generator from workspace root: ${normalizeCandidate(workspaceRoot)}`,
  );
}

function parseArgs(argv) {
  const parsed = {
    workspace: path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..'),
  };

  for (let index = 0; index < argv.length; index += 1) {
    const current = argv[index];
    if (current === '--workspace') {
      const value = (argv[index + 1] || '').trim();
      if (!value) {
        throw new Error('Missing value for --workspace');
      }
      parsed.workspace = path.resolve(value);
      index += 1;
      continue;
    }
    throw new Error(`Unknown argument: ${current}`);
  }

  return parsed;
}

const isDirectExecution =
  process.argv[1] && path.resolve(process.argv[1]) === fileURLToPath(import.meta.url);

if (isDirectExecution) {
  const args = parseArgs(process.argv.slice(2));
  process.stdout.write(resolveSdkworkGeneratorRoot(args.workspace));
}
