import { describe, expect, it } from 'vitest';
import {
  createSdkworkGlassStyle,
  createSdkworkHeroStyle,
  createSdkworkPanelStyle,
  createSdkworkToneStyle,
} from './index';

describe('sdkwork theme surface recipes', () => {
  it('exports reusable Claw-aligned tone, panel, glass, and hero styles', () => {
    expect(createSdkworkToneStyle('brand').color).toBe('var(--sdk-color-brand-primary)');
    expect(createSdkworkPanelStyle('accent').backgroundImage).toContain('var(--sdk-color-brand-accent)');
    expect(createSdkworkPanelStyle('accent').backgroundImage).toContain('var(--sdk-color-surface-panel)');
    expect(createSdkworkGlassStyle('accent').backgroundImage).toContain('var(--sdk-color-brand-accent)');
    expect(createSdkworkGlassStyle('accent').backgroundImage).toContain('var(--sdk-color-surface-panel)');
    expect(createSdkworkHeroStyle().backgroundImage).toContain('var(--sdk-color-surface-canvas)');
    expect(createSdkworkHeroStyle().backgroundImage).toContain('var(--sdk-color-surface-panel)');
    expect(createSdkworkHeroStyle().backgroundImage).toContain('var(--sdk-color-surface-elevated)');
    expect(createSdkworkHeroStyle().backgroundImage).not.toContain('#18181b');
  });
});
