import { readFileSync } from 'node:fs';
import { resolve } from 'node:path';
import { describe, expect, it } from 'vitest';

const sdkworkUiCssSource = readFileSync(
  resolve(process.cwd(), 'src/styles/sdkwork-ui.css'),
  'utf8',
);

describe('sdkwork-ui claw shell css contract', () => {
  it('ships claw-style theme palettes and scrollbar utilities', () => {
    expect(sdkworkUiCssSource).toContain('@theme');
    expect(sdkworkUiCssSource).toContain('[data-theme="lobster"]');
    expect(sdkworkUiCssSource).toContain('[data-theme="green-tech"]');
    expect(sdkworkUiCssSource).toContain('[data-theme="violet"]');
    expect(sdkworkUiCssSource).toContain('[data-theme="rose"]');
    expect(sdkworkUiCssSource).toContain('.custom-scrollbar');
    expect(sdkworkUiCssSource).toContain('--theme-primary-500');
    expect(sdkworkUiCssSource).toContain('--scrollbar-thumb');
  });
});
