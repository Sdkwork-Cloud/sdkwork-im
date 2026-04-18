import type { CSSProperties } from 'react';

export type SdkworkThemeVisualTone =
  | 'accent'
  | 'brand'
  | 'danger'
  | 'neutral'
  | 'success'
  | 'warning';

function resolveSdkworkThemeToneVar(tone: SdkworkThemeVisualTone): string {
  if (tone === 'accent') {
    return 'var(--sdk-color-brand-accent)';
  }

  if (tone === 'brand') {
    return 'var(--sdk-color-brand-primary)';
  }

  if (tone === 'success') {
    return 'var(--sdk-color-state-success)';
  }

  if (tone === 'warning') {
    return 'var(--sdk-color-state-warning)';
  }

  if (tone === 'danger') {
    return 'var(--sdk-color-state-danger)';
  }

  return 'var(--sdk-color-text-muted)';
}

export function createSdkworkToneStyle(
  tone: SdkworkThemeVisualTone,
  options: {
    backgroundWeight?: number;
    borderWeight?: number;
  } = {},
): CSSProperties {
  const backgroundWeight = options.backgroundWeight ?? 14;
  const borderWeight = options.borderWeight ?? 28;
  const colorVar = resolveSdkworkThemeToneVar(tone);

  return {
    backgroundColor: `color-mix(in srgb, ${colorVar} ${backgroundWeight}%, transparent)`,
    borderColor: `color-mix(in srgb, ${colorVar} ${borderWeight}%, transparent)`,
    color: colorVar,
  };
}

export function createSdkworkPanelStyle(
  tone: SdkworkThemeVisualTone,
  options: {
    backgroundWeight?: number;
    borderWeight?: number;
    surfaceColor?: string;
    surfaceWeight?: number;
  } = {},
): CSSProperties {
  const backgroundWeight = options.backgroundWeight ?? 10;
  const borderWeight = options.borderWeight ?? 30;
  const surfaceColor = options.surfaceColor ?? 'var(--sdk-color-surface-panel)';
  const surfaceWeight = options.surfaceWeight ?? 96;
  const colorVar = resolveSdkworkThemeToneVar(tone);

  return {
    backgroundImage: `linear-gradient(180deg, color-mix(in srgb, ${colorVar} ${backgroundWeight}%, transparent), color-mix(in srgb, ${surfaceColor} ${surfaceWeight}%, white))`,
    borderColor: `color-mix(in srgb, ${colorVar} ${borderWeight}%, transparent)`,
  };
}

export function createSdkworkGlassStyle(
  tone: SdkworkThemeVisualTone,
  options: {
    backgroundWeight?: number;
    borderWeight?: number;
    surfaceColor?: string;
    surfaceWeight?: number;
  } = {},
): CSSProperties {
  const backgroundWeight = options.backgroundWeight ?? 14;
  const borderWeight = options.borderWeight ?? 28;
  const surfaceColor = options.surfaceColor ?? 'var(--sdk-color-surface-panel)';
  const surfaceWeight = options.surfaceWeight ?? 86;
  const colorVar = resolveSdkworkThemeToneVar(tone);

  return {
    backgroundColor: `color-mix(in srgb, ${surfaceColor} 18%, transparent)`,
    backgroundImage: `linear-gradient(180deg, color-mix(in srgb, ${colorVar} ${backgroundWeight}%, transparent), color-mix(in srgb, ${surfaceColor} ${surfaceWeight}%, transparent))`,
    borderColor: `color-mix(in srgb, ${colorVar} ${borderWeight}%, transparent)`,
  };
}

export function createSdkworkBackdropStyle(): CSSProperties {
  return {
    backgroundImage:
      'radial-gradient(circle at top left, color-mix(in srgb, var(--sdk-color-brand-primary) 14%, transparent), transparent 38%), radial-gradient(circle at top right, color-mix(in srgb, var(--sdk-color-brand-accent) 10%, transparent), transparent 42%)',
  };
}

export function createSdkworkHeroStyle(): CSSProperties {
  return {
    backgroundImage:
      'radial-gradient(circle at top right, color-mix(in srgb, var(--sdk-color-brand-accent) 18%, transparent), transparent 34%), radial-gradient(circle at bottom left, color-mix(in srgb, var(--sdk-color-brand-primary) 16%, transparent), transparent 38%), linear-gradient(135deg, color-mix(in srgb, var(--sdk-color-surface-canvas) 42%, black), color-mix(in srgb, var(--sdk-color-surface-panel) 44%, black) 48%, color-mix(in srgb, var(--sdk-color-surface-elevated) 56%, black))',
  };
}
