import { readFileSync, readdirSync } from 'node:fs';
import { join } from 'node:path';
import { describe, expect, it } from 'vitest';
import { CLAW_DARK_THEME, createThemeCssVariables } from './sdkwork-theme';

const sdkTokenPattern = /--sdk-[a-z0-9-]+/g;
const declaredTokenPattern = /--sdk-[a-z0-9-]+(?=\s*:)/g;

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

describe('theme contract', () => {
  it('declares every SDKWORK token used by the framework', () => {
    const componentFiles = collectFiles('src/components', (filePath) => filePath.endsWith('.ts') || filePath.endsWith('.tsx'));
    const frameworkSources = [...componentFiles, 'src/theme/sdkwork-theme.ts', 'src/theme/surface-recipes.ts', 'src/styles/sdkwork-ui.css'];
    const usedTokens = new Set<string>();

    frameworkSources.forEach((filePath) => {
      const content = readFileSync(filePath, 'utf8');
      const matches = content.match(sdkTokenPattern) ?? [];

      matches.forEach((token) => usedTokens.add(token));
    });

    const declaredThemeTokens = new Set(Object.keys(createThemeCssVariables(CLAW_DARK_THEME)));
    const undeclaredTokens = [...usedTokens]
      .filter((token) => !declaredThemeTokens.has(token))
      .sort();

    expect(undeclaredTokens).toEqual([]);
  });

  it('keeps the base stylesheet in sync with the runtime theme contract', () => {
    const stylesheet = readFileSync('src/styles/sdkwork-ui.css', 'utf8');
    const stylesheetTokens = [...new Set(stylesheet.match(declaredTokenPattern) ?? [])].sort();
    const runtimeTokens = Object.keys(createThemeCssVariables(CLAW_DARK_THEME)).sort();

    expect(stylesheetTokens).toEqual(runtimeTokens);
  });
});
