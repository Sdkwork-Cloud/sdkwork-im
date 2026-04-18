import { createContext, useContext, useEffect, useMemo, useState, type CSSProperties, type PropsWithChildren } from 'react';
import {
  createSdkworkTheme,
  createThemeHostCssVariables,
  type SdkworkColorMode,
  type SdkworkThemeColor,
  type SdkworkThemeOverrides,
} from './sdkwork-theme';

export type SdkworkThemeSelection = SdkworkColorMode | 'system';

interface SdkworkThemeContextValue {
  colorMode: SdkworkColorMode;
  themeColor: SdkworkThemeColor;
  setThemeSelection: (next: SdkworkThemeSelection) => void;
  themeSelection: SdkworkThemeSelection;
}

const SdkworkThemeContext = createContext<SdkworkThemeContextValue | null>(null);

export interface SdkworkThemeProviderProps extends PropsWithChildren {
  className?: string;
  defaultTheme?: SdkworkThemeSelection;
  dir?: 'auto' | 'ltr' | 'rtl';
  locale?: string;
  onThemeSelectionChange?: (next: SdkworkThemeSelection) => void;
  overrides?: SdkworkThemeOverrides;
  themeColor?: SdkworkThemeColor;
  themeSelection?: SdkworkThemeSelection;
}

function resolveSystemColorMode(): SdkworkColorMode {
  if (
    typeof window !== 'undefined' &&
    typeof window.matchMedia === 'function' &&
    window.matchMedia('(prefers-color-scheme: light)').matches
  ) {
    return 'light';
  }

  return 'dark';
}

export function SdkworkThemeProvider({
  children,
  className,
  defaultTheme = 'system',
  dir,
  locale,
  onThemeSelectionChange,
  overrides,
  themeColor = 'lobster',
  themeSelection: controlledThemeSelection,
}: SdkworkThemeProviderProps) {
  const [uncontrolledThemeSelection, setUncontrolledThemeSelection] = useState<SdkworkThemeSelection>(defaultTheme);
  const [systemColorMode, setSystemColorMode] = useState<SdkworkColorMode>(() => resolveSystemColorMode());
  const themeSelection = controlledThemeSelection ?? uncontrolledThemeSelection;
  const colorMode = themeSelection === 'system' ? systemColorMode : themeSelection;
  const theme = useMemo(
    () => createSdkworkTheme({ ...overrides, colorMode, themeColor }),
    [colorMode, overrides, themeColor],
  );
  const hostStyle = useMemo(
    () => createThemeHostCssVariables(theme, themeColor),
    [theme, themeColor],
  );

  useEffect(() => {
    if (typeof window === 'undefined' || typeof window.matchMedia !== 'function') {
      return;
    }

    const mediaQuery = window.matchMedia('(prefers-color-scheme: light)');
    const handleChange = (event: MediaQueryListEvent | MediaQueryList) => {
      setSystemColorMode(event.matches ? 'light' : 'dark');
    };

    setSystemColorMode(mediaQuery.matches ? 'light' : 'dark');

    mediaQuery.addEventListener?.('change', handleChange);
    mediaQuery.addListener?.(handleChange);

    return () => {
      mediaQuery.removeEventListener?.('change', handleChange);
      mediaQuery.removeListener?.(handleChange);
    };
  }, []);

  useEffect(() => {
    if (typeof document === 'undefined') {
      return;
    }

    const host = document.documentElement;
    const previousAttributes = {
      dir: host.getAttribute('dir'),
      lang: host.getAttribute('lang'),
      theme: host.getAttribute('data-theme'),
      sdkColorMode: host.getAttribute('data-sdk-color-mode'),
    };
    const hadDarkClass = host.classList.contains('dark');
    const previousStyleEntries = Object.keys(hostStyle).map((name) => [
      name,
      host.style.getPropertyValue(name),
    ] as const);

    host.setAttribute('data-theme', themeColor);
    host.setAttribute('data-sdk-color-mode', colorMode);
    if (locale) {
      host.setAttribute('lang', locale);
    } else {
      host.removeAttribute('lang');
    }
    if (dir) {
      host.setAttribute('dir', dir);
    } else {
      host.removeAttribute('dir');
    }
    host.classList.toggle('dark', colorMode === 'dark');

    Object.entries(hostStyle).forEach(([name, value]) => {
      host.style.setProperty(name, String(value));
    });

    return () => {
      host.classList.toggle('dark', hadDarkClass);

      if (previousAttributes.sdkColorMode) {
        host.setAttribute('data-sdk-color-mode', previousAttributes.sdkColorMode);
      } else {
        host.removeAttribute('data-sdk-color-mode');
      }

      if (previousAttributes.theme) {
        host.setAttribute('data-theme', previousAttributes.theme);
      } else {
        host.removeAttribute('data-theme');
      }

      if (previousAttributes.dir) {
        host.setAttribute('dir', previousAttributes.dir);
      } else {
        host.removeAttribute('dir');
      }

      if (previousAttributes.lang) {
        host.setAttribute('lang', previousAttributes.lang);
      } else {
        host.removeAttribute('lang');
      }

      previousStyleEntries.forEach(([name, value]) => {
        if (value) {
          host.style.setProperty(name, value);
        } else {
          host.style.removeProperty(name);
        }
      });
    };
  }, [colorMode, dir, hostStyle, locale, themeColor]);

  const setThemeSelection = (next: SdkworkThemeSelection) => {
    if (controlledThemeSelection === undefined) {
      setUncontrolledThemeSelection(next);
    }

    onThemeSelectionChange?.(next);
  };

  const value = useMemo(
    () => ({
      colorMode,
      themeColor,
      setThemeSelection,
      themeSelection,
    }),
    [colorMode, setThemeSelection, themeColor, themeSelection],
  );

  return (
    <SdkworkThemeContext.Provider value={value}>
      <div
        className={className}
        dir={dir}
        data-sdk-color-mode={colorMode}
        data-theme={themeColor}
        data-sdk-theme-provider=""
        lang={locale}
        style={hostStyle as CSSProperties}
      >
        {children}
      </div>
    </SdkworkThemeContext.Provider>
  );
}

export function useSdkworkTheme() {
  return (
    useContext(SdkworkThemeContext) ?? {
      colorMode: 'dark' as const,
      themeColor: 'lobster' as const,
      setThemeSelection: () => undefined,
      themeSelection: 'system' as const,
    }
  );
}
