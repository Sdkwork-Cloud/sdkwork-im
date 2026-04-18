import * as React from 'react';
import { Minus, Square, X } from 'lucide-react';
import { cn } from '../../../lib/utils';

export type DesktopPlatform = 'desktop' | 'web';
export type WindowUnsubscribe = () => void | Promise<void>;
export type DesktopWindowCommandHandler = () => Promise<void>;
export type DesktopWindowPlatformResolver = () => DesktopPlatform;
export type DesktopWindowMaximizedResolver = () => Promise<boolean>;
export type DesktopWindowMaximizedChangeHandler = (isMaximized: boolean) => void;
export type DesktopWindowMaximizedSubscriber = (
  callback: DesktopWindowMaximizedChangeHandler,
) => Promise<WindowUnsubscribe> | WindowUnsubscribe;

export interface DesktopWindowController {
  closeWindow: DesktopWindowCommandHandler;
  getPlatform: DesktopWindowPlatformResolver;
  isWindowMaximized: DesktopWindowMaximizedResolver;
  maximizeWindow: DesktopWindowCommandHandler;
  minimizeWindow: DesktopWindowCommandHandler;
  restoreWindow: DesktopWindowCommandHandler;
  subscribeWindowMaximized: DesktopWindowMaximizedSubscriber;
}

export interface DesktopWindowControlLabels {
  close: string;
  maximize: string;
  minimize: string;
  restore: string;
}

export interface DesktopWindowControlsProps extends Omit<React.HTMLAttributes<HTMLDivElement>, 'children'> {
  className?: string;
  controller?: DesktopWindowController | null;
  labels?: Partial<DesktopWindowControlLabels>;
  variant?: 'floating' | 'header';
}

const DEFAULT_LABELS: DesktopWindowControlLabels = {
  close: 'Close window',
  maximize: 'Maximize window',
  minimize: 'Minimize window',
  restore: 'Restore window',
};

function WindowSizeGlyph({ isMaximized }: { isMaximized: boolean }) {
  if (!isMaximized) {
    return <Square className="h-3.5 w-3.5 stroke-[2.1]" />;
  }

  return (
    <svg
      aria-hidden="true"
      className="h-3.5 w-3.5"
      fill="none"
      stroke="currentColor"
      strokeLinecap="round"
      strokeLinejoin="round"
      strokeWidth="1.8"
      viewBox="0 0 16 16"
    >
      <path d="M5 3.5h7.5V11" />
      <path d="M3.5 5H11v7.5H3.5z" />
    </svg>
  );
}

function useDesktopWindowMaximized(
  controller: DesktopWindowController | null | undefined,
  isDesktop: boolean,
) {
  const [isWindowMaximized, setIsWindowMaximized] = React.useState(false);

  React.useEffect(() => {
    if (!controller || !isDesktop) {
      setIsWindowMaximized(false);
      return;
    }

    let active = true;
    let unsubscribe: WindowUnsubscribe = () => undefined;

    void controller.isWindowMaximized().then((value) => {
      if (active) {
        setIsWindowMaximized(value);
      }
    });

    void Promise.resolve(
      controller.subscribeWindowMaximized((value) => {
        if (active) {
          setIsWindowMaximized(value);
        }
      }),
    ).then((cleanup) => {
      unsubscribe = cleanup;
    });

    return () => {
      active = false;
      void unsubscribe();
    };
  }, [controller, isDesktop]);

  return isWindowMaximized;
}

function getRootClassName(
  variant: NonNullable<DesktopWindowControlsProps['variant']>,
  className?: string,
) {
  return cn(
    'flex items-stretch',
    variant === 'header'
      ? 'h-full'
      : 'overflow-hidden rounded-[var(--sdk-radius-panel)] border border-[var(--sdk-color-border-default)] bg-[var(--sdk-color-surface-panel)] shadow-[var(--sdk-shadow-md)] backdrop-blur-xl',
    className,
  );
}

function getButtonClassName(params: {
  intent?: 'danger' | 'default';
  variant: NonNullable<DesktopWindowControlsProps['variant']>;
  withDivider?: boolean;
}) {
  const { intent = 'default', variant, withDivider = false } = params;

  return cn(
    'flex items-center justify-center transition-colors',
    variant === 'header'
      ? 'h-full w-11 text-[var(--sdk-color-text-secondary)]'
      : 'h-10 w-10 text-[var(--sdk-color-text-secondary)]',
    intent === 'danger'
      ? 'hover:bg-rose-500 hover:text-white'
      : 'hover:bg-[var(--sdk-color-brand-primary-soft)] hover:text-[var(--sdk-color-text-primary)]',
    withDivider && variant === 'floating'
      ? 'border-r border-[var(--sdk-color-border-subtle)]'
      : null,
  );
}

export const DesktopWindowControls = React.forwardRef<HTMLDivElement, DesktopWindowControlsProps>(({
  className,
  controller,
  labels,
  variant = 'header',
  ...props
}, ref) => {
  const resolvedLabels = React.useMemo(
    () => ({
      ...DEFAULT_LABELS,
      ...labels,
    }),
    [labels],
  );
  const isDesktop = controller?.getPlatform() === 'desktop';
  const isWindowMaximized = useDesktopWindowMaximized(controller, isDesktop);

  if (!controller || !isDesktop) {
    return null;
  }

  return (
    <div
      ref={ref}
      className={getRootClassName(variant, className)}
      data-sdk-pattern="desktop-window-controls"
      data-tauri-drag-region="false"
      {...props}
    >
      <button
        aria-label={resolvedLabels.minimize}
        className={getButtonClassName({
          variant,
          withDivider: true,
        })}
        data-tauri-drag-region="false"
        onClick={() => {
          void controller.minimizeWindow();
        }}
        title={resolvedLabels.minimize}
        type="button"
      >
        <Minus className="h-3.5 w-3.5 stroke-[2.4]" />
      </button>
      <button
        aria-label={isWindowMaximized ? resolvedLabels.restore : resolvedLabels.maximize}
        className={getButtonClassName({
          variant,
          withDivider: true,
        })}
        data-tauri-drag-region="false"
        onClick={() => {
          void (isWindowMaximized ? controller.restoreWindow() : controller.maximizeWindow());
        }}
        title={isWindowMaximized ? resolvedLabels.restore : resolvedLabels.maximize}
        type="button"
      >
        <WindowSizeGlyph isMaximized={isWindowMaximized} />
      </button>
      <button
        aria-label={resolvedLabels.close}
        className={getButtonClassName({
          intent: 'danger',
          variant,
        })}
        data-tauri-drag-region="false"
        onClick={() => {
          void controller.closeWindow();
        }}
        title={resolvedLabels.close}
        type="button"
      >
        <X className="h-3.5 w-3.5 stroke-[2.2]" />
      </button>
    </div>
  );
});
DesktopWindowControls.displayName = 'DesktopWindowControls';
