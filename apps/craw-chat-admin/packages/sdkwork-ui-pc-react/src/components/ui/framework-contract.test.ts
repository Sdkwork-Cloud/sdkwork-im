import { readFileSync, readdirSync } from 'node:fs';
import { join } from 'node:path';
import { describe, expect, it } from 'vitest';

const frameworkMarkerPattern = /data-sdk-(ui|pattern|region)(=|['"]\s*:)/;
const jsxRenderPattern = /<[A-Z][A-Za-z0-9.]*/;
const reexportOnlyPattern = /^\s*(export\s+\{[\s\S]*?\}\s+from\s+['"][^'"]+['"];\s*|export\s+\*\s+from\s+['"][^'"]+['"];\s*)+$/;

function collectFiles(root: string, predicate: (filePath: string) => boolean): string[] {
  const entries = readdirSync(root, { withFileTypes: true });

  return entries.flatMap((entry) => {
    const filePath = join(root, entry.name);

    if (entry.isDirectory()) {
      return collectFiles(filePath, predicate);
    }

    return predicate(filePath) ? [filePath] : [];
  });
}

describe('framework contract', () => {
  it('marks every rendered framework component with stable SDKWORK metadata', () => {
    const componentFiles = collectFiles('src/components', (filePath) => filePath.endsWith('.tsx') && !filePath.endsWith('.test.tsx'));

    const missingMarkers = componentFiles
      .filter((filePath) => {
        const content = readFileSync(filePath, 'utf8');

        if (reexportOnlyPattern.test(content.trim())) {
          return false;
        }

        if (!jsxRenderPattern.test(content)) {
          return false;
        }

        return !frameworkMarkerPattern.test(content);
      })
      .sort();

    expect(missingMarkers).toEqual([]);
  });
});
