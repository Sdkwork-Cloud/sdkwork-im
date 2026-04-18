import { describe, expect, it } from 'vitest';
import {
  CLAW_DARK_THEME,
  CLAW_LIGHT_THEME,
  SDKWORK_DARK_THEME,
  SDKWORK_LIGHT_THEME,
  createSdkworkTheme,
  createThemeCssVariables,
} from './sdkwork-theme';

describe('sdkwork theme', () => {
  it('creates a semantic theme with SDKWORK defaults and brand overrides', () => {
    const theme = createSdkworkTheme({
      colorMode: 'dark',
      brand: {
        primary: '#2563eb',
      },
    });

    expect(theme.colorMode).toBe('dark');
    expect(theme.brand.primary).toBe('#2563eb');
    expect(theme.surface.canvas).toBe(SDKWORK_DARK_THEME.surface.canvas);
    expect(theme.radius.panel).toBe(SDKWORK_DARK_THEME.radius.panel);
  });

  it('converts theme tokens into CSS variables for runtime theming', () => {
    const variables = createThemeCssVariables(SDKWORK_LIGHT_THEME);

    expect(variables['--sdk-color-brand-primary']).toBe(SDKWORK_LIGHT_THEME.brand.primary);
    expect(variables['--sdk-color-surface-canvas']).toBe(SDKWORK_LIGHT_THEME.surface.canvas);
    expect(variables['--sdk-radius-panel']).toBe(SDKWORK_LIGHT_THEME.radius.panel);
  });

  it('uses claw shell surfaces with the lobster accent as the default SDKWORK PC reference style', () => {
    const theme = createSdkworkTheme({
      preset: 'claw',
      colorMode: 'dark',
    });

    expect(theme.surface.panel).toBe(CLAW_DARK_THEME.surface.panel);
    expect(theme.radius.control).toBe(CLAW_DARK_THEME.radius.control);
    expect(theme.brand.primary).toBe('#ef4444');
  });

  it('supports the explicit tech-blue claw accent for the classic blue variant', () => {
    const theme = createSdkworkTheme({
      preset: 'claw',
      colorMode: 'dark',
      themeColor: 'tech-blue',
    });

    expect(theme.brand.primary).toBe(CLAW_DARK_THEME.brand.primary);
  });

  it('supports claw preset with deep token overrides', () => {
    const theme = createSdkworkTheme({
      preset: 'claw',
      colorMode: 'light',
      brand: {
        primary: '#0f766e',
      },
      radius: {
        panel: '2rem',
      },
    });

    expect(theme.surface.panel).toBe(CLAW_LIGHT_THEME.surface.panel);
    expect(theme.brand.primary).toBe('#0f766e');
    expect(theme.radius.panel).toBe('2rem');
  });
});
