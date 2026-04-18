import * as React from 'react';
import { cn } from '../../../lib/utils';
import {
  mergePatternSlotProps,
  type PatternSlotProps,
} from '../_internal/slot-props';
import {
  DesktopWindowControls,
  type DesktopWindowControlLabels,
  type DesktopWindowController,
} from './DesktopWindowControls';

export type DesktopTitleBarRegionSlotProps = PatternSlotProps<
  Omit<React.HTMLAttributes<HTMLDivElement>, 'children'>
>;

export interface DesktopTitleBarSlotProps {
  center?: DesktopTitleBarRegionSlotProps;
  centerShell?: DesktopTitleBarRegionSlotProps;
  leading?: DesktopTitleBarRegionSlotProps;
  trailing?: DesktopTitleBarRegionSlotProps;
}

export interface DesktopTitleBarProps
  extends Omit<React.HTMLAttributes<HTMLElement>, 'children' | 'style'> {
  bordered?: boolean;
  center?: React.ReactNode;
  centerInteractive?: boolean;
  centerMaxWidth?: React.CSSProperties['maxWidth'];
  leading?: React.ReactNode;
  size?: 'comfortable' | 'compact' | 'default';
  slotProps?: DesktopTitleBarSlotProps;
  style?: React.CSSProperties;
  trailing?: React.ReactNode;
  translucent?: boolean;
  windowController?: DesktopWindowController | null;
  windowControlLabels?: Partial<DesktopWindowControlLabels>;
  windowControls?: React.ReactNode;
}

const sizeClassName: Record<NonNullable<DesktopTitleBarProps['size']>, string> = {
  comfortable: 'h-14 px-4 sm:px-5',
  compact: 'h-11 px-3 sm:px-4',
  default: 'h-12 px-3 sm:px-4',
};

export const DesktopTitleBar = React.forwardRef<HTMLElement, DesktopTitleBarProps>(({
  bordered = true,
  center,
  centerInteractive = true,
  centerMaxWidth = '36rem',
  className,
  leading,
  size = 'default',
  slotProps,
  style,
  trailing,
  translucent = true,
  windowController,
  windowControlLabels,
  windowControls,
  ...props
}, ref) => {
  const resolvedWindowControls =
    windowControls ??
    (windowController ? (
      <DesktopWindowControls controller={windowController} labels={windowControlLabels} />
    ) : null);
  const hasTrailingRegion = Boolean(trailing || resolvedWindowControls);

  return (
    <header
      ref={ref}
      className={cn(
        'relative flex items-center',
        sizeClassName[size],
        bordered ? 'border-b border-[var(--sdk-color-border-subtle)]' : null,
        translucent
          ? 'bg-[var(--sdk-color-surface-panel)]/90 backdrop-blur-xl'
          : 'bg-[var(--sdk-color-surface-panel)]',
        className,
      )}
      data-sdk-pattern="desktop-title-bar"
      style={style}
      {...props}
    >
      {leading ? (
        <div
          {...mergePatternSlotProps<DesktopTitleBarRegionSlotProps>(
            {
              className: 'flex min-w-0 flex-1 items-center gap-3',
              'data-sdk-region': 'desktop-title-bar-leading',
              'data-sdk-slot': 'leading',
              'data-tauri-drag-region': true,
            },
            slotProps?.leading,
          )}
        >
          {leading}
        </div>
      ) : (
        <div
          className="flex min-w-0 flex-1"
          data-sdk-region="desktop-title-bar-leading-spacer"
          data-sdk-slot="leading-spacer"
          data-tauri-drag-region
        />
      )}

      {center ? (
        <div
          {...mergePatternSlotProps<DesktopTitleBarRegionSlotProps>(
            {
              className:
                'pointer-events-none absolute left-1/2 top-1/2 flex w-full -translate-x-1/2 -translate-y-1/2 items-center justify-center px-24 lg:px-32',
              'data-sdk-region': 'desktop-title-bar-center-shell',
              'data-sdk-slot': 'center-shell',
              style: { maxWidth: centerMaxWidth },
            },
            slotProps?.centerShell,
          )}
        >
          <div
            {...mergePatternSlotProps<DesktopTitleBarRegionSlotProps>(
              {
                className: cn('w-full', centerInteractive ? 'pointer-events-auto' : null),
                'data-sdk-region': 'desktop-title-bar-center',
                'data-sdk-slot': 'center',
                'data-tauri-drag-region': centerInteractive ? 'false' : true,
              },
              slotProps?.center,
            )}
          >
            {center}
          </div>
        </div>
      ) : null}

      {hasTrailingRegion ? (
        <div
          {...mergePatternSlotProps<DesktopTitleBarRegionSlotProps>(
            {
              className: 'ml-auto flex h-full shrink-0 items-center justify-end gap-2',
              'data-sdk-region': 'desktop-title-bar-trailing',
              'data-sdk-slot': 'trailing',
              'data-tauri-drag-region': 'false',
            },
            slotProps?.trailing,
          )}
        >
          {trailing}
          {resolvedWindowControls}
        </div>
      ) : null}
    </header>
  );
});
DesktopTitleBar.displayName = 'DesktopTitleBar';
