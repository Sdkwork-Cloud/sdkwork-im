import { readFileSync } from 'node:fs';
import { dirname, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';
import { describe, expect, it } from 'vitest';
import { SDKWORK_DARK_THEME } from '../theme';

const currentDirectory = dirname(fileURLToPath(import.meta.url));
const stylesheetPath = resolve(currentDirectory, 'sdkwork-ui.css');

describe('sdkwork-ui stylesheet', () => {
  it('ships root CSS variables aligned with the default SDKWORK dark theme', () => {
    const stylesheet = readFileSync(stylesheetPath, 'utf8');

    expect(stylesheet).toContain(`--sdk-color-surface-canvas: ${SDKWORK_DARK_THEME.surface.canvas};`);
    expect(stylesheet).toContain(`--sdk-radius-panel: ${SDKWORK_DARK_THEME.radius.panel};`);
    expect(stylesheet).toContain(`--sdk-shadow-md: ${SDKWORK_DARK_THEME.shadow.md};`);
  });
});
