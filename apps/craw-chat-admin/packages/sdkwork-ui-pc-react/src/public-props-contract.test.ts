import { readFileSync, readdirSync } from 'node:fs';
import { join } from 'node:path';
import { describe, expect, it } from 'vitest';
import { uiComponentCatalog } from './components/ui/catalog';

function collectFiles(root: string): string[] {
  const entries = readdirSync(root, { withFileTypes: true });

  return entries.flatMap((entry) => {
    const filePath = join(root, entry.name);

    if (entry.isDirectory()) {
      return collectFiles(filePath);
    }

    return filePath.endsWith('.ts') || filePath.endsWith('.tsx') ? [filePath] : [];
  });
}

const publicComponentNames = [
  ...uiComponentCatalog.actions,
  ...uiComponentCatalog.dataEntry,
  ...uiComponentCatalog.dataDisplay,
  ...uiComponentCatalog.feedback,
  ...uiComponentCatalog.form,
  ...uiComponentCatalog.layout,
  ...uiComponentCatalog.navigation,
  ...uiComponentCatalog.overlays,
  ...uiComponentCatalog.patterns,
].filter((exportName) => /^[A-Z]/.test(exportName));

describe('public props contract', () => {
  it('defines an explicit exported ComponentNameProps type for every public component', () => {
    const sourceFiles = collectFiles('src/components');
    const sources = sourceFiles.map((filePath) => readFileSync(filePath, 'utf8'));

    const offenders = publicComponentNames.filter((componentName) => {
      const propsPattern = new RegExp(`export\\s+(?:type|interface)\\s+${componentName}Props\\b`);

      return !sources.some((source) => propsPattern.test(source));
    });

    expect(offenders).toEqual([]);
  });
});
