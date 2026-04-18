import { existsSync, realpathSync } from 'node:fs';
import { dirname, relative, resolve, sep } from 'node:path';
import { fileURLToPath } from 'node:url';
import { describe, expect, it } from 'vitest';

const packageRoot = resolve(dirname(fileURLToPath(import.meta.url)), '..');
const workspaceAppsRoot = resolve(packageRoot, '..', '..');

const criticalRuntimePackages = [
  'react',
  'react-dom',
  'cmdk',
  'lucide-react',
  '@types/react',
  '@types/react-dom',
];

function resolveDependencyEntry(packageName: string): string {
  const segments = packageName.split('/');
  return resolve(packageRoot, 'node_modules', ...segments);
}

function isInsidePackageRoot(targetPath: string): boolean {
  const relativePath = relative(packageRoot, targetPath);

  return relativePath !== ''
    && relativePath !== '.'
    && !relativePath.startsWith(`..${sep}`)
    && relativePath !== '..';
}

function isInsideWorkspaceAppsRoot(targetPath: string): boolean {
  const relativePath = relative(workspaceAppsRoot, targetPath);

  return relativePath !== ''
    && relativePath !== '.'
    && !relativePath.startsWith(`..${sep}`)
    && relativePath !== '..';
}

describe('sdkwork-ui install contract', () => {
  it('keeps critical dependency entrypoints inside this package boundary', () => {
    const offenders = criticalRuntimePackages.flatMap((packageName) => {
      const dependencyEntry = resolveDependencyEntry(packageName);

      if (!existsSync(dependencyEntry)) {
        return [`missing dependency entry: ${packageName}`];
      }

      const resolvedEntry = realpathSync(dependencyEntry);

      return (isInsidePackageRoot(resolvedEntry) || isInsideWorkspaceAppsRoot(resolvedEntry))
        ? []
        : [`${packageName} -> ${resolvedEntry}`];
    });

    expect(offenders).toEqual([]);
  });
});
