import { act, render } from '@testing-library/react';
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';
import { SdkworkThemeProvider } from './theme-provider';

interface MatchMediaController {
  dispatch: (nextMatches: boolean) => void;
  queryList: MediaQueryList;
}

function createMatchMediaController(initialMatches: boolean): MatchMediaController {
  const listeners = new Set<(event: MediaQueryListEvent) => void>();
  let matches = initialMatches;
  const queryList = {
    addEventListener: (_type: string, listener: EventListenerOrEventListenerObject | null) => {
      if (typeof listener === 'function') {
        listeners.add(listener as (event: MediaQueryListEvent) => void);
      }
    },
    addListener: (listener: ((this: MediaQueryList, ev: MediaQueryListEvent) => any) | null) => {
      if (listener) {
        listeners.add(listener as (event: MediaQueryListEvent) => void);
      }
    },
    dispatchEvent: vi.fn(() => true),
    get matches() {
      return matches;
    },
    media: '(prefers-color-scheme: light)',
    onchange: null,
    removeEventListener: (_type: string, listener: EventListenerOrEventListenerObject | null) => {
      if (typeof listener === 'function') {
        listeners.delete(listener as (event: MediaQueryListEvent) => void);
      }
    },
    removeListener: (listener: ((this: MediaQueryList, ev: MediaQueryListEvent) => any) | null) => {
      if (listener) {
        listeners.delete(listener as (event: MediaQueryListEvent) => void);
      }
    },
  } as unknown as MediaQueryList;

  return {
    dispatch(nextMatches) {
      matches = nextMatches;
      const event = {
        matches: nextMatches,
        media: queryList.media,
      } as MediaQueryListEvent;

      listeners.forEach((listener) => listener(event));
    },
    queryList,
  };
}

describe('sdkwork theme provider', () => {
  let controller: MatchMediaController;

  beforeEach(() => {
    controller = createMatchMediaController(false);

    Object.defineProperty(window, 'matchMedia', {
      configurable: true,
      value: vi.fn().mockImplementation(() => controller.queryList),
      writable: true,
    });

    document.documentElement.className = '';
    document.documentElement.removeAttribute('data-theme');
    document.documentElement.removeAttribute('data-sdk-color-mode');
    document.documentElement.style.cssText = '';
  });

  afterEach(() => {
    document.documentElement.className = '';
    document.documentElement.removeAttribute('data-theme');
    document.documentElement.removeAttribute('data-sdk-color-mode');
    document.documentElement.style.cssText = '';
  });

  it('mirrors resolved theme mode and theme color onto the document host', () => {
    render(
      <SdkworkThemeProvider defaultTheme="light" themeColor="rose">
        <div>Theme host sync</div>
      </SdkworkThemeProvider>,
    );

    expect(document.documentElement.getAttribute('data-theme')).toBe('rose');
    expect(document.documentElement.getAttribute('data-sdk-color-mode')).toBe('light');
    expect(document.documentElement.classList.contains('dark')).toBe(false);
    expect(document.documentElement.style.getPropertyValue('--theme-primary-500')).toBe('#f43f5e');
    expect(document.documentElement.style.getPropertyValue('--sdk-color-surface-panel')).toBe('#ffffff');
  });

  it('reacts to system color-scheme changes and updates the document host', () => {
    render(
      <SdkworkThemeProvider defaultTheme="system" themeColor="green-tech">
        <div>System theme sync</div>
      </SdkworkThemeProvider>,
    );

    expect(document.documentElement.getAttribute('data-sdk-color-mode')).toBe('dark');
    expect(document.documentElement.classList.contains('dark')).toBe(true);

    act(() => {
      controller.dispatch(true);
    });

    expect(document.documentElement.getAttribute('data-sdk-color-mode')).toBe('light');
    expect(document.documentElement.classList.contains('dark')).toBe(false);
    expect(document.documentElement.style.getPropertyValue('--theme-primary-500')).toBe('#10b981');

    act(() => {
      controller.dispatch(false);
    });

    expect(document.documentElement.getAttribute('data-sdk-color-mode')).toBe('dark');
    expect(document.documentElement.classList.contains('dark')).toBe(true);
  });

  it('supports controlled theme selection and live theme color updates after rerender', () => {
    const { rerender } = render(
      <SdkworkThemeProvider themeColor="zinc" themeSelection="light">
        <div>Controlled theme selection</div>
      </SdkworkThemeProvider>,
    );

    expect(document.documentElement.getAttribute('data-sdk-color-mode')).toBe('light');
    expect(document.documentElement.getAttribute('data-theme')).toBe('zinc');
    expect(document.documentElement.style.getPropertyValue('--theme-primary-500')).toBe('#71717a');

    rerender(
      <SdkworkThemeProvider themeColor="lobster" themeSelection="dark">
        <div>Controlled theme selection</div>
      </SdkworkThemeProvider>,
    );

    expect(document.documentElement.getAttribute('data-sdk-color-mode')).toBe('dark');
    expect(document.documentElement.classList.contains('dark')).toBe(true);
    expect(document.documentElement.getAttribute('data-theme')).toBe('lobster');
    expect(document.documentElement.style.getPropertyValue('--theme-primary-500')).toBe('#ef4444');
  });

  it('syncs locale and direction onto the document host', () => {
    const { rerender, unmount } = render(
      <SdkworkThemeProvider dir="rtl" locale="ar-SA" themeSelection="light">
        <div>Locale sync</div>
      </SdkworkThemeProvider>,
    );

    expect(document.documentElement.getAttribute('lang')).toBe('ar-SA');
    expect(document.documentElement.getAttribute('dir')).toBe('rtl');

    rerender(
      <SdkworkThemeProvider dir="ltr" locale="en-US" themeSelection="dark">
        <div>Locale sync</div>
      </SdkworkThemeProvider>,
    );

    expect(document.documentElement.getAttribute('lang')).toBe('en-US');
    expect(document.documentElement.getAttribute('dir')).toBe('ltr');

    unmount();

    expect(document.documentElement.getAttribute('lang')).toBeNull();
    expect(document.documentElement.getAttribute('dir')).toBeNull();
  });

  it('restores the outer document host state when a nested provider unmounts', () => {
    const { rerender } = render(
      <SdkworkThemeProvider dir="ltr" locale="en-US" themeColor="lobster" themeSelection="dark">
        <div>Outer locale sync</div>
      </SdkworkThemeProvider>,
    );

    expect(document.documentElement.getAttribute('data-theme')).toBe('lobster');
    expect(document.documentElement.getAttribute('data-sdk-color-mode')).toBe('dark');
    expect(document.documentElement.getAttribute('lang')).toBe('en-US');
    expect(document.documentElement.getAttribute('dir')).toBe('ltr');

    rerender(
      <SdkworkThemeProvider dir="ltr" locale="en-US" themeColor="lobster" themeSelection="dark">
        <SdkworkThemeProvider dir="rtl" locale="ar-SA" themeColor="tech-blue" themeSelection="light">
          <div>Nested locale sync</div>
        </SdkworkThemeProvider>
      </SdkworkThemeProvider>,
    );

    expect(document.documentElement.getAttribute('data-theme')).toBe('tech-blue');
    expect(document.documentElement.getAttribute('data-sdk-color-mode')).toBe('light');
    expect(document.documentElement.getAttribute('lang')).toBe('ar-SA');
    expect(document.documentElement.getAttribute('dir')).toBe('rtl');

    rerender(
      <SdkworkThemeProvider dir="ltr" locale="en-US" themeColor="lobster" themeSelection="dark">
        <div>Outer locale sync again</div>
      </SdkworkThemeProvider>,
    );

    expect(document.documentElement.getAttribute('data-theme')).toBe('lobster');
    expect(document.documentElement.getAttribute('data-sdk-color-mode')).toBe('dark');
    expect(document.documentElement.getAttribute('lang')).toBe('en-US');
    expect(document.documentElement.getAttribute('dir')).toBe('ltr');
  });

  it('defaults to the claw lobster accent when no theme color is provided', () => {
    render(
      <SdkworkThemeProvider themeSelection="dark">
        <div>Default claw accent</div>
      </SdkworkThemeProvider>,
    );

    expect(document.documentElement.getAttribute('data-theme')).toBe('lobster');
    expect(document.documentElement.style.getPropertyValue('--theme-primary-500')).toBe('#ef4444');
    expect(document.documentElement.style.getPropertyValue('--sdk-color-brand-primary')).toBe('#ef4444');
  });

  it('supports the claw tech-blue theme color explicitly', () => {
    render(
      <SdkworkThemeProvider themeColor="tech-blue" themeSelection="light">
        <div>Tech blue accent</div>
      </SdkworkThemeProvider>,
    );

    expect(document.documentElement.getAttribute('data-theme')).toBe('tech-blue');
    expect(document.documentElement.style.getPropertyValue('--theme-primary-500')).toBe('#3b82f6');
    expect(document.documentElement.style.getPropertyValue('--sdk-color-brand-primary')).toBe('#2563eb');
  });
});
