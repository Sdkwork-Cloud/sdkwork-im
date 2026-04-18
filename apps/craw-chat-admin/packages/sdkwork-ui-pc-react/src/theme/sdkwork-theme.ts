import type { CSSProperties } from 'react';

export type SdkworkColorMode = 'light' | 'dark';
export type SdkworkThemePreset = 'claw';
export type SdkworkThemeColor = 'green-tech' | 'lobster' | 'rose' | 'tech-blue' | 'violet' | 'zinc';

export interface SdkworkTheme {
  colorMode: SdkworkColorMode;
  brand: {
    primary: string;
    primaryHover: string;
    primarySoft: string;
    accent: string;
  };
  surface: {
    canvas: string;
    panel: string;
    panelMuted: string;
    elevated: string;
    overlay: string;
  };
  text: {
    primary: string;
    secondary: string;
    muted: string;
    inverse: string;
  };
  border: {
    subtle: string;
    default: string;
    strong: string;
    focus: string;
  };
  state: {
    success: string;
    warning: string;
    danger: string;
    info: string;
  };
  radius: {
    control: string;
    field: string;
    panel: string;
    pill: string;
  };
  shadow: {
    soft: string;
    sm: string;
    md: string;
    lg: string;
  };
}

export interface SdkworkThemeOverrides {
  brand?: Partial<SdkworkTheme['brand']>;
  surface?: Partial<SdkworkTheme['surface']>;
  text?: Partial<SdkworkTheme['text']>;
  border?: Partial<SdkworkTheme['border']>;
  state?: Partial<SdkworkTheme['state']>;
  radius?: Partial<SdkworkTheme['radius']>;
  shadow?: Partial<SdkworkTheme['shadow']>;
}

export interface CreateSdkworkThemeOptions extends SdkworkThemeOverrides {
  colorMode?: SdkworkColorMode;
  preset?: SdkworkThemePreset;
  themeColor?: SdkworkThemeColor;
}

export const CLAW_DARK_THEME: SdkworkTheme = {
  colorMode: 'dark',
  brand: {
    primary: '#3b82f6',
    primaryHover: '#2563eb',
    primarySoft: 'rgba(59, 130, 246, 0.14)',
    accent: '#22d3ee',
  },
  surface: {
    canvas: '#09090b',
    panel: '#18181b',
    panelMuted: '#27272a',
    elevated: '#3f3f46',
    overlay: 'rgba(9, 9, 11, 0.45)',
  },
  text: {
    primary: '#fafafa',
    secondary: '#d4d4d8',
    muted: '#a1a1aa',
    inverse: '#ffffff',
  },
  border: {
    subtle: 'rgba(244, 244, 245, 0.08)',
    default: 'rgba(244, 244, 245, 0.12)',
    strong: 'rgba(244, 244, 245, 0.18)',
    focus: 'rgba(59, 130, 246, 0.4)',
  },
  state: {
    success: '#22c55e',
    warning: '#f59e0b',
    danger: '#ef4444',
    info: '#38bdf8',
  },
  radius: {
    control: '0.5rem',
    field: '0.375rem',
    panel: '1rem',
    pill: '999px',
  },
  shadow: {
    soft: '0 4px 16px rgba(9, 9, 11, 0.1)',
    sm: '0 8px 24px rgba(9, 9, 11, 0.12)',
    md: '0 18px 50px rgba(9, 9, 11, 0.16)',
    lg: '0 32px 80px rgba(9, 9, 11, 0.22)',
  },
};

export const CLAW_LIGHT_THEME: SdkworkTheme = {
  colorMode: 'light',
  brand: {
    primary: '#2563eb',
    primaryHover: '#1d4ed8',
    primarySoft: 'rgba(37, 99, 235, 0.1)',
    accent: '#0891b2',
  },
  surface: {
    canvas: '#f5f5f5',
    panel: '#ffffff',
    panelMuted: '#fafafa',
    elevated: '#f4f4f5',
    overlay: 'rgba(9, 9, 11, 0.18)',
  },
  text: {
    primary: '#18181b',
    secondary: '#3f3f46',
    muted: '#71717a',
    inverse: '#ffffff',
  },
  border: {
    subtle: 'rgba(24, 24, 27, 0.05)',
    default: 'rgba(24, 24, 27, 0.1)',
    strong: 'rgba(24, 24, 27, 0.16)',
    focus: 'rgba(37, 99, 235, 0.45)',
  },
  state: {
    success: '#16a34a',
    warning: '#d97706',
    danger: '#dc2626',
    info: '#0284c7',
  },
  radius: {
    control: '0.5rem',
    field: '0.375rem',
    panel: '1rem',
    pill: '999px',
  },
  shadow: {
    soft: '0 4px 16px rgba(24, 24, 27, 0.08)',
    sm: '0 8px 24px rgba(24, 24, 27, 0.06)',
    md: '0 18px 50px rgba(24, 24, 27, 0.1)',
    lg: '0 28px 80px rgba(24, 24, 27, 0.14)',
  },
};

export const SDKWORK_DARK_THEME = CLAW_DARK_THEME;
export const SDKWORK_LIGHT_THEME = CLAW_LIGHT_THEME;

const CLAW_THEME_COLOR_CONFIGS: Record<
  SdkworkThemeColor,
  {
    brand: {
      dark: NonNullable<SdkworkThemeOverrides['brand']>;
      light: NonNullable<SdkworkThemeOverrides['brand']>;
    };
    palette: {
      50: string;
      100: string;
      200: string;
      300: string;
      400: string;
      500: string;
      600: string;
      700: string;
      800: string;
      900: string;
      950: string;
    };
  }
> = {
  'green-tech': {
    brand: {
      dark: {
        accent: '#6ee7b7',
        primary: '#10b981',
        primaryHover: '#059669',
        primarySoft: 'rgba(16, 185, 129, 0.18)',
      },
      light: {
        accent: '#10b981',
        primary: '#059669',
        primaryHover: '#047857',
        primarySoft: 'rgba(16, 185, 129, 0.12)',
      },
    },
    palette: {
      50: '#ecfdf5',
      100: '#d1fae5',
      200: '#a7f3d0',
      300: '#6ee7b7',
      400: '#34d399',
      500: '#10b981',
      600: '#059669',
      700: '#047857',
      800: '#065f46',
      900: '#064e3b',
      950: '#022c22',
    },
  },
  lobster: {
    brand: {
      dark: {
        accent: '#fca5a5',
        primary: '#ef4444',
        primaryHover: '#dc2626',
        primarySoft: 'rgba(239, 68, 68, 0.18)',
      },
      light: {
        accent: '#ef4444',
        primary: '#dc2626',
        primaryHover: '#b91c1c',
        primarySoft: 'rgba(239, 68, 68, 0.12)',
      },
    },
    palette: {
      50: '#fef2f2',
      100: '#fee2e2',
      200: '#fecaca',
      300: '#fca5a5',
      400: '#f87171',
      500: '#ef4444',
      600: '#dc2626',
      700: '#b91c1c',
      800: '#991b1b',
      900: '#7f1d1d',
      950: '#450a0a',
    },
  },
  rose: {
    brand: {
      dark: {
        accent: '#fda4af',
        primary: '#f43f5e',
        primaryHover: '#e11d48',
        primarySoft: 'rgba(244, 63, 94, 0.18)',
      },
      light: {
        accent: '#f43f5e',
        primary: '#e11d48',
        primaryHover: '#be123c',
        primarySoft: 'rgba(244, 63, 94, 0.12)',
      },
    },
    palette: {
      50: '#fff1f2',
      100: '#ffe4e6',
      200: '#fecdd3',
      300: '#fda4af',
      400: '#fb7185',
      500: '#f43f5e',
      600: '#e11d48',
      700: '#be123c',
      800: '#9f1239',
      900: '#881337',
      950: '#4c0519',
    },
  },
  'tech-blue': {
    brand: {
      dark: {
        accent: '#22d3ee',
        primary: '#3b82f6',
        primaryHover: '#2563eb',
        primarySoft: 'rgba(59, 130, 246, 0.18)',
      },
      light: {
        accent: '#0891b2',
        primary: '#2563eb',
        primaryHover: '#1d4ed8',
        primarySoft: 'rgba(37, 99, 235, 0.12)',
      },
    },
    palette: {
      50: '#eff6ff',
      100: '#dbeafe',
      200: '#bfdbfe',
      300: '#93c5fd',
      400: '#60a5fa',
      500: '#3b82f6',
      600: '#2563eb',
      700: '#1d4ed8',
      800: '#1e40af',
      900: '#1e3a8a',
      950: '#172554',
    },
  },
  violet: {
    brand: {
      dark: {
        accent: '#c4b5fd',
        primary: '#8b5cf6',
        primaryHover: '#7c3aed',
        primarySoft: 'rgba(139, 92, 246, 0.18)',
      },
      light: {
        accent: '#8b5cf6',
        primary: '#7c3aed',
        primaryHover: '#6d28d9',
        primarySoft: 'rgba(139, 92, 246, 0.12)',
      },
    },
    palette: {
      50: '#f5f3ff',
      100: '#ede9fe',
      200: '#ddd6fe',
      300: '#c4b5fd',
      400: '#a78bfa',
      500: '#8b5cf6',
      600: '#7c3aed',
      700: '#6d28d9',
      800: '#5b21b6',
      900: '#4c1d95',
      950: '#2e1065',
    },
  },
  zinc: {
    brand: {
      dark: {
        accent: '#d4d4d8',
        primary: '#71717a',
        primaryHover: '#52525b',
        primarySoft: 'rgba(113, 113, 122, 0.18)',
      },
      light: {
        accent: '#71717a',
        primary: '#52525b',
        primaryHover: '#3f3f46',
        primarySoft: 'rgba(113, 113, 122, 0.12)',
      },
    },
    palette: {
      50: '#fafafa',
      100: '#f4f4f5',
      200: '#e4e4e7',
      300: '#d4d4d8',
      400: '#a1a1aa',
      500: '#71717a',
      600: '#52525b',
      700: '#3f3f46',
      800: '#27272a',
      900: '#18181b',
      950: '#09090b',
    },
  },
};

function resolveThemeColorOverrides(
  themeColor: SdkworkThemeColor,
  colorMode: SdkworkColorMode,
): SdkworkThemeOverrides {
  return {
    brand: {
      ...CLAW_THEME_COLOR_CONFIGS[themeColor].brand[colorMode],
    },
  };
}

function getPresetTheme(
  preset: SdkworkThemePreset,
  colorMode: SdkworkColorMode,
): SdkworkTheme {
  if (preset === 'claw') {
    return colorMode === 'light' ? CLAW_LIGHT_THEME : CLAW_DARK_THEME;
  }

  return colorMode === 'light' ? SDKWORK_LIGHT_THEME : SDKWORK_DARK_THEME;
}

function mergeTheme(base: SdkworkTheme, overrides?: SdkworkThemeOverrides): SdkworkTheme {
  if (!overrides) {
    return base;
  }

  return {
    ...base,
    ...overrides,
    brand: {
      ...base.brand,
      ...overrides.brand,
    },
    surface: {
      ...base.surface,
      ...overrides.surface,
    },
    text: {
      ...base.text,
      ...overrides.text,
    },
    border: {
      ...base.border,
      ...overrides.border,
    },
    state: {
      ...base.state,
      ...overrides.state,
    },
    radius: {
      ...base.radius,
      ...overrides.radius,
    },
    shadow: {
      ...base.shadow,
      ...overrides.shadow,
    },
  };
}

export function createSdkworkTheme(
  options: CreateSdkworkThemeOptions = {},
) {
  const {
    colorMode = 'dark',
    preset = 'claw',
    themeColor = 'lobster',
    ...overrides
  } = options;
  const base = getPresetTheme(preset, colorMode);
  const themeColorOverrides = resolveThemeColorOverrides(themeColor, colorMode);

  return mergeTheme(base, {
    ...themeColorOverrides,
    ...overrides,
    brand: {
      ...themeColorOverrides.brand,
      ...overrides.brand,
    },
  });
}

export function createThemeCssVariables(theme: SdkworkTheme) {
  return {
    '--sdk-color-brand-primary': theme.brand.primary,
    '--sdk-color-brand-primary-hover': theme.brand.primaryHover,
    '--sdk-color-brand-primary-soft': theme.brand.primarySoft,
    '--sdk-color-brand-accent': theme.brand.accent,
    '--sdk-color-surface-canvas': theme.surface.canvas,
    '--sdk-color-surface-panel': theme.surface.panel,
    '--sdk-color-surface-panel-muted': theme.surface.panelMuted,
    '--sdk-color-surface-elevated': theme.surface.elevated,
    '--sdk-color-surface-overlay': theme.surface.overlay,
    '--sdk-color-text-primary': theme.text.primary,
    '--sdk-color-text-secondary': theme.text.secondary,
    '--sdk-color-text-muted': theme.text.muted,
    '--sdk-color-text-inverse': theme.text.inverse,
    '--sdk-color-border-subtle': theme.border.subtle,
    '--sdk-color-border-default': theme.border.default,
    '--sdk-color-border-strong': theme.border.strong,
    '--sdk-color-border-focus': theme.border.focus,
    '--sdk-color-state-success': theme.state.success,
    '--sdk-color-state-warning': theme.state.warning,
    '--sdk-color-state-danger': theme.state.danger,
    '--sdk-color-state-info': theme.state.info,
    '--sdk-radius-control': theme.radius.control,
    '--sdk-radius-field': theme.radius.field,
    '--sdk-radius-panel': theme.radius.panel,
    '--sdk-radius-pill': theme.radius.pill,
    '--sdk-shadow-soft': theme.shadow.soft,
    '--sdk-shadow-sm': theme.shadow.sm,
    '--sdk-shadow-md': theme.shadow.md,
    '--sdk-shadow-lg': theme.shadow.lg,
  } as const;
}

export function createThemeHostCssVariables(
  theme: SdkworkTheme,
  themeColor: SdkworkThemeColor = 'lobster',
) {
  const palette = CLAW_THEME_COLOR_CONFIGS[themeColor].palette;

  return {
    ...createThemeCssVariables(theme),
    '--theme-primary-50': palette[50],
    '--theme-primary-100': palette[100],
    '--theme-primary-200': palette[200],
    '--theme-primary-300': palette[300],
    '--theme-primary-400': palette[400],
    '--theme-primary-500': palette[500],
    '--theme-primary-600': palette[600],
    '--theme-primary-700': palette[700],
    '--theme-primary-800': palette[800],
    '--theme-primary-900': palette[900],
    '--theme-primary-950': palette[950],
  } as const;
}

export function createThemeStyle(theme: SdkworkTheme): CSSProperties {
  return createThemeCssVariables(theme) as CSSProperties;
}
