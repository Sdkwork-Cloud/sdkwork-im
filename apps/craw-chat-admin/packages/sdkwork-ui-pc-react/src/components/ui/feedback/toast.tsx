import type { ComponentProps, ReactElement, ReactNode } from 'react';
import { Toaster as SonnerToaster, toast as sonnerToast, type ExternalToast } from 'sonner';
import { useSdkworkTheme } from '../../../theme';

type SonnerToasterProps = ComponentProps<typeof SonnerToaster>;
type SonnerToastOptions = NonNullable<SonnerToasterProps['toastOptions']>;
type SonnerToastClassNames = NonNullable<SonnerToastOptions['classNames']>;

export type SdkworkToastId = number | string;
export type SdkworkToastResultUnwrapHandler<ToastData = unknown> = () => Promise<ToastData>;
export type SdkworkToastResult<ToastData = unknown> =
  | SdkworkToastId
  | {
      unwrap: SdkworkToastResultUnwrapHandler<ToastData>;
    };

export type SdkworkToastPromiseFactory<ToastData = unknown> = () => Promise<ToastData>;
export type SdkworkToastPromise<ToastData = unknown> =
  | Promise<ToastData>
  | SdkworkToastPromiseFactory<ToastData>;

export type SdkworkToastPromiseRenderer<ToastData = unknown> =
  | ReactNode
  | ((data: ToastData) => ReactNode | Promise<ReactNode>);
export type SdkworkToastPromiseErrorRenderer =
  | ReactNode
  | ((error: unknown) => ReactNode | Promise<ReactNode>);
export type SdkworkToastPromiseFinallyHandler = () => void | Promise<void>;
export type SdkworkToastHandler = (message: ReactNode, data?: ExternalToast) => SdkworkToastId;
export type SdkworkToastCustomRenderer = (id: SdkworkToastId) => ReactElement;
export type SdkworkToastCustomHandler = (
  jsx: SdkworkToastCustomRenderer,
  data?: ExternalToast,
) => SdkworkToastId;
export type SdkworkToastDismissHandler = (id?: SdkworkToastId) => SdkworkToastId;
export type SdkworkToastPromiseHandler = <ToastData = unknown>(
  promise: SdkworkToastPromise<ToastData>,
  data?: SdkworkToastPromiseOptions<ToastData>,
) => SdkworkToastResult<ToastData>;

export interface SdkworkToastPromiseOptions<ToastData = unknown>
  extends Omit<ExternalToast, 'description'> {
  description?: SdkworkToastPromiseRenderer<ToastData>;
  error?: SdkworkToastPromiseErrorRenderer;
  finally?: SdkworkToastPromiseFinallyHandler;
  loading?: ReactNode;
  success?: SdkworkToastPromiseRenderer<ToastData>;
}

export type SdkworkToastApi = SdkworkToastHandler & {
  custom: SdkworkToastCustomHandler;
  dismiss: SdkworkToastDismissHandler;
  error: SdkworkToastHandler;
  info: SdkworkToastHandler;
  loading: SdkworkToastHandler;
  message: SdkworkToastHandler;
  promise: SdkworkToastPromiseHandler;
  success: SdkworkToastHandler;
  warning: SdkworkToastHandler;
};

type ToasterDataAttributes = {
  [K in `data-${string}`]?: string;
};

const sdkworkToastClassNames: Partial<SonnerToastClassNames> = {
  actionButton:
    'rounded-[var(--sdk-radius-control)] bg-[var(--sdk-color-brand-primary)] px-3 py-1 text-xs font-medium text-[var(--sdk-color-text-inverse)] transition-colors hover:bg-[var(--sdk-color-brand-primary-hover)]',
  cancelButton:
    'rounded-[var(--sdk-radius-control)] border border-[var(--sdk-color-border-default)] bg-[var(--sdk-color-surface-panel-muted)] px-3 py-1 text-xs font-medium text-[var(--sdk-color-text-secondary)] transition-colors hover:bg-[var(--sdk-color-surface-elevated)] hover:text-[var(--sdk-color-text-primary)]',
  closeButton:
    'rounded-full border border-[var(--sdk-color-border-default)] bg-[var(--sdk-color-surface-panel)] text-[var(--sdk-color-text-muted)] transition-colors hover:bg-[var(--sdk-color-surface-elevated)] hover:text-[var(--sdk-color-text-primary)]',
  description: 'text-sm leading-6 text-[var(--sdk-color-text-secondary)]',
  loader: 'text-[var(--sdk-color-brand-primary)]',
  success: 'border-[color-mix(in_srgb,var(--sdk-color-state-success)_32%,transparent)]',
  error: 'border-[color-mix(in_srgb,var(--sdk-color-state-danger)_32%,transparent)]',
  info: 'border-[color-mix(in_srgb,var(--sdk-color-state-info)_32%,transparent)]',
  title: 'text-sm font-semibold text-[var(--sdk-color-text-primary)]',
  toast:
    'group rounded-[var(--sdk-radius-control)] border border-[var(--sdk-color-border-default)] bg-[var(--sdk-color-surface-panel)] text-[var(--sdk-color-text-primary)] shadow-[var(--sdk-shadow-md)]',
  warning: 'border-[color-mix(in_srgb,var(--sdk-color-state-warning)_32%,transparent)]',
};

function mergeToastOptions(
  toastOptions?: SonnerToasterProps['toastOptions'],
): SonnerToasterProps['toastOptions'] {
  return {
    classNames: {
      ...sdkworkToastClassNames,
      ...(toastOptions?.classNames ?? {}),
    },
    ...toastOptions,
  };
}

export interface ToasterProps extends SonnerToasterProps, ToasterDataAttributes {}
export type SdkworkToasterProps = ToasterProps;

export function Toaster({
  closeButton = true,
  expand = true,
  position = 'bottom-right',
  richColors = true,
  theme,
  toastOptions,
  ...props
}: ToasterProps) {
  const { colorMode } = useSdkworkTheme();
  return (
    <SonnerToaster
      closeButton={closeButton}
      data-sdk-ui="toaster"
      expand={expand}
      position={position}
      richColors={richColors}
      theme={theme ?? colorMode}
      toastOptions={mergeToastOptions(toastOptions)}
      {...props}
    />
  );
}

export function SdkworkToaster(props: SdkworkToasterProps) {
  return (
    <Toaster
      data-sdk-ui="sdkwork-toaster"
      {...props}
    />
  );
}

const sdkworkToastApi = Object.assign(
  (message: ReactNode, data?: ExternalToast) => sonnerToast(message, data),
  {
    custom: (jsx: (id: number | string) => ReactElement, data?: ExternalToast) =>
      sonnerToast.custom(jsx, data),
    dismiss: (id?: number | string) => sonnerToast.dismiss(id),
    error: (message: ReactNode, data?: ExternalToast) => sonnerToast.error(message, data),
    info: (message: ReactNode, data?: ExternalToast) => sonnerToast.info(message, data),
    loading: (message: ReactNode, data?: ExternalToast) => sonnerToast.loading(message, data),
    message: (message: ReactNode, data?: ExternalToast) => sonnerToast.message(message, data),
    promise: <ToastData = unknown>(
      promise: SdkworkToastPromise<ToastData>,
      data?: SdkworkToastPromiseOptions<ToastData>,
    ) => sonnerToast.promise(promise, data) as SdkworkToastResult<ToastData>,
    success: (message: ReactNode, data?: ExternalToast) => sonnerToast.success(message, data),
    warning: (message: ReactNode, data?: ExternalToast) => sonnerToast.warning(message, data),
  },
) as SdkworkToastApi;

export const toast: SdkworkToastApi = sdkworkToastApi;
export const sdkToast: SdkworkToastApi = toast;
Toaster.displayName = 'Toaster';
SdkworkToaster.displayName = 'SdkworkToaster';
