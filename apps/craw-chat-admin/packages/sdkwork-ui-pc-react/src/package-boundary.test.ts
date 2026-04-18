import { readdirSync, readFileSync, statSync } from 'node:fs';
import { dirname, relative, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';
import { describe, expect, it } from 'vitest';

const sourceDir = dirname(fileURLToPath(import.meta.url));
const distDir = resolve(sourceDir, '../dist');
const packageJson = JSON.parse(readFileSync(resolve(sourceDir, '../package.json'), 'utf8')) as {
  dependencies?: Record<string, string>;
  peerDependencies?: Record<string, string>;
};
const allowedRuntimeDependencies = new Set([
  ...Object.keys(packageJson.dependencies ?? {}),
  ...Object.keys(packageJson.peerDependencies ?? {}),
]);

function collectJavaScriptFiles(directory: string): string[] {
  return readdirSync(directory).flatMap((entry) => {
    const entryPath = resolve(directory, entry);
    const stats = statSync(entryPath);

    if (stats.isDirectory()) {
      return collectJavaScriptFiles(entryPath);
    }

    return entryPath.endsWith('.js') ? [entryPath] : [];
  });
}

function isAllowedRuntimeImport(specifier: string): boolean {
  return [...allowedRuntimeDependencies].some((dependency) => specifier === dependency || specifier.startsWith(`${dependency}/`));
}

function collectBareImports(source: string): string[] {
  const specifiers = new Set<string>();
  const matches = source.matchAll(
    /(?:import|export)\s+(?:[^"'()]+\s+from\s+)?["']([^"']+)["']|import\(\s*["']([^"']+)["']\s*\)/g,
  );

  for (const match of matches) {
    const specifier = match[1] ?? match[2];
    if (!specifier) {
      continue;
    }

    if (
      specifier.startsWith('.')
      || specifier.startsWith('/')
      || specifier.startsWith('node:')
      || /^[A-Za-z]:[\\/]/.test(specifier)
    ) {
      continue;
    }

    specifiers.add(specifier);
  }

  return [...specifiers];
}

describe('sdkwork-ui dist package boundary', () => {
  it('keeps runtime bare imports limited to declared runtime dependencies', () => {
    const distFiles = collectJavaScriptFiles(distDir);

    expect(distFiles.length).toBeGreaterThan(0);

    const offenders = distFiles.flatMap((filePath) => {
      const source = readFileSync(filePath, 'utf8');
      const bareImports = collectBareImports(source).filter((specifier) => !isAllowedRuntimeImport(specifier));

      return bareImports.map((specifier) => `${relative(distDir, filePath)} -> ${specifier}`);
    });

    expect(offenders).toEqual([]);
  });
});
