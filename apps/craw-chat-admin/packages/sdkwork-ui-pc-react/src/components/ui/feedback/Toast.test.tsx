import { createElement, type ComponentType } from 'react';
import { render } from '@testing-library/react';
import { describe, expect, it, beforeEach, vi } from 'vitest';
import { SdkworkThemeProvider } from '../../../theme';
import { uiComponentCatalog } from '../catalog';
import * as feedback from './index';

const sonnerMock = vi.hoisted(() => ({
  toastApi: {
    custom: vi.fn(),
    dismiss: vi.fn(),
    error: vi.fn(),
    info: vi.fn(),
    loading: vi.fn(),
    message: vi.fn(),
    promise: vi.fn(),
    success: vi.fn(),
    warning: vi.fn(),
  },
  toasterSpy: vi.fn((_props: unknown) => null),
}));

vi.mock(
  'sonner',
  () => ({
    Toaster: sonnerMock.toasterSpy,
    toast: sonnerMock.toastApi,
  }),
);

function getFeedbackExports() {
  return feedback as Record<string, unknown>;
}

describe('toast feedback layer', () => {
  beforeEach(() => {
    sonnerMock.toasterSpy.mockClear();
    Object.values(sonnerMock.toastApi).forEach((mock) => mock.mockClear());
  });

  it('exports a standard toaster layer from the feedback domain', () => {
    const exportsMap = getFeedbackExports();

    expect(exportsMap.Toaster).toBeDefined();
    expect(exportsMap.SdkworkToaster).toBeDefined();
    expect(exportsMap.SdkworkToaster).not.toBe(exportsMap.Toaster);
    expect(exportsMap.toast).toBeDefined();
    expect(exportsMap.sdkToast).toBe(exportsMap.toast);
    expect(uiComponentCatalog.feedback).toEqual(
      expect.arrayContaining(['Toaster', 'SdkworkToaster', 'toast', 'sdkToast']),
    );
  });

  it('renders sonner with SDKWORK desktop defaults and theme-aware mode', () => {
    const exportsMap = getFeedbackExports();

    expect(exportsMap.Toaster).toBeDefined();

    if (typeof exportsMap.Toaster !== 'function') {
      return;
    }

    render(
      <SdkworkThemeProvider defaultTheme="light">
        {createElement(exportsMap.Toaster as ComponentType)}
      </SdkworkThemeProvider>,
    );

    expect(sonnerMock.toasterSpy).toHaveBeenCalled();

    const props = sonnerMock.toasterSpy.mock.calls.at(-1)?.[0] as
      | {
          toastOptions?: {
            classNames?: Record<string, string | undefined>;
          };
          closeButton?: boolean;
          'data-sdk-ui'?: string;
          expand?: boolean;
          position?: string;
          richColors?: boolean;
          theme?: string;
        }
      | undefined;

    expect(props).toBeDefined();

    if (!props) {
      return;
    }

    expect(props).toMatchObject({
      closeButton: true,
      'data-sdk-ui': 'toaster',
      expand: true,
      position: 'bottom-right',
      richColors: true,
      theme: 'light',
    });
    expect(props.toastOptions?.classNames?.toast).toContain('bg-[var(--sdk-color-surface-panel)]');
    expect(props.toastOptions?.classNames?.title).toContain('text-[var(--sdk-color-text-primary)]');

    sonnerMock.toasterSpy.mockClear();

    if (typeof exportsMap.SdkworkToaster !== 'function') {
      return;
    }

    render(
      <SdkworkThemeProvider defaultTheme="light">
        {createElement(exportsMap.SdkworkToaster as ComponentType)}
      </SdkworkThemeProvider>,
    );

    const sdkworkProps = sonnerMock.toasterSpy.mock.calls.at(-1)?.[0] as
      | {
          'data-sdk-ui'?: string;
        }
      | undefined;

    expect(sdkworkProps?.['data-sdk-ui']).toBe('sdkwork-toaster');
  });

  it('proxies imperative toast helpers through the shared feedback API', () => {
    const exportsMap = getFeedbackExports();

    expect(exportsMap.toast).toBeDefined();

    if (!exportsMap.toast || typeof exportsMap.toast !== 'object') {
      return;
    }

    const toastApi = exportsMap.toast as Record<string, (...args: unknown[]) => unknown>;
    const promise = Promise.resolve('done');

    toastApi.success('Saved successfully', { description: 'Workspace updated' });
    toastApi.error('Save failed');
    toastApi.warning('Storage almost full');
    toastApi.info('Sync started');
    toastApi.loading('Uploading asset');
    toastApi.promise(promise, {
      error: 'Import failed',
      loading: 'Importing',
      success: 'Import completed',
    });
    toastApi.dismiss('toast-1');

    expect(sonnerMock.toastApi.success).toHaveBeenCalledWith('Saved successfully', {
      description: 'Workspace updated',
    });
    expect(sonnerMock.toastApi.error).toHaveBeenCalledWith('Save failed');
    expect(sonnerMock.toastApi.warning).toHaveBeenCalledWith('Storage almost full');
    expect(sonnerMock.toastApi.info).toHaveBeenCalledWith('Sync started');
    expect(sonnerMock.toastApi.loading).toHaveBeenCalledWith('Uploading asset');
    expect(sonnerMock.toastApi.promise).toHaveBeenCalledWith(promise, {
      error: 'Import failed',
      loading: 'Importing',
      success: 'Import completed',
    });
    expect(sonnerMock.toastApi.dismiss).toHaveBeenCalledWith('toast-1');
  });
});
