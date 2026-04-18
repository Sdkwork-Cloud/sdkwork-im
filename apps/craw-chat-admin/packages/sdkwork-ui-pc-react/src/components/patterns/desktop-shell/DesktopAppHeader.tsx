import * as React from 'react';
import {
  mergePatternSlotProps,
  type PatternSlotProps,
} from '../_internal/slot-props';
import {
  DesktopTitleBar,
  type DesktopTitleBarProps,
  type DesktopTitleBarSlotProps,
} from './DesktopTitleBar';

type DesktopAppHeaderRootProps = PatternSlotProps<Omit<React.ComponentPropsWithoutRef<'div'>, 'children'>>;
export type DesktopAppHeaderBarSlotProps = PatternSlotProps<
  Omit<React.ComponentPropsWithoutRef<'header'>, 'children'>
>;
export type DesktopAppHeaderBrandSlotProps = PatternSlotProps<
  Omit<React.ComponentPropsWithoutRef<'div'>, 'children'>
>;
export type DesktopAppHeaderTitleSlotProps = PatternSlotProps<
  Omit<React.ComponentPropsWithoutRef<'span'>, 'children'>
>;
export type DesktopAppHeaderSubtitleSlotProps = PatternSlotProps<
  Omit<React.ComponentPropsWithoutRef<'div'>, 'children'>
>;

export interface DesktopAppHeaderSlotProps extends DesktopTitleBarSlotProps {
  bar?: DesktopAppHeaderBarSlotProps;
  brand?: DesktopAppHeaderBrandSlotProps;
  subtitle?: DesktopAppHeaderSubtitleSlotProps;
  title?: DesktopAppHeaderTitleSlotProps;
}

export interface DesktopAppHeaderProps
  extends Omit<
    DesktopTitleBarProps,
    'leading' | 'slotProps' | 'title' | 'trailing' | 'windowControls'
  > {
  actions?: React.ReactNode;
  badge?: React.ReactNode;
  brand?: React.ReactNode;
  brandMark?: React.ReactNode;
  leading?: React.ReactNode;
  slotProps?: DesktopAppHeaderSlotProps;
  subtitle?: React.ReactNode;
  title?: React.ReactNode;
  windowControls?: React.ReactNode;
}

export const DesktopAppHeader = React.forwardRef<HTMLDivElement, DesktopAppHeaderProps>(({
  actions,
  badge,
  bordered,
  brand,
  brandMark,
  center,
  centerInteractive,
  centerMaxWidth,
  className,
  id,
  leading,
  size,
  slotProps,
  style,
  subtitle,
  title,
  translucent,
  windowController,
  windowControlLabels,
  windowControls,
  ...props
}, ref) => {
  const {
    bar: barSlotProps,
    brand: brandSlotProps,
    center: centerSlotProps,
    centerShell: centerShellSlotProps,
    leading: leadingSlotProps,
    subtitle: subtitleSlotProps,
    title: titleSlotProps,
    trailing: trailingSlotProps,
  } = slotProps ?? {};

  const brandBlock =
    brand ??
    (title || subtitle || brandMark || badge ? (
      <div
        {...mergePatternSlotProps<DesktopAppHeaderBrandSlotProps>(
          {
            className: 'flex min-w-0 items-center gap-3',
            'data-sdk-region': 'desktop-app-header-brand',
          },
          brandSlotProps,
        )}
      >
        {brandMark ? <div className="shrink-0">{brandMark}</div> : null}
        {(title || subtitle || badge) ? (
          <div className="min-w-0">
            {(title || badge) ? (
              <div className="flex min-w-0 items-center gap-2">
                {title ? (
                  <span
                    {...mergePatternSlotProps<DesktopAppHeaderTitleSlotProps>(
                      {
                        className:
                          'truncate text-sm font-semibold leading-none text-[var(--sdk-color-text-primary)]',
                        'data-sdk-region': 'desktop-app-header-title',
                      },
                      titleSlotProps,
                    )}
                  >
                    {title}
                  </span>
                ) : null}
                {badge}
              </div>
            ) : null}
            {subtitle ? (
              <div
                {...mergePatternSlotProps<DesktopAppHeaderSubtitleSlotProps>(
                  {
                    className:
                      'mt-1 truncate text-[10px] font-semibold uppercase tracking-[0.2em] text-[var(--sdk-color-text-muted)]',
                    'data-sdk-region': 'desktop-app-header-subtitle',
                  },
                  subtitleSlotProps,
                )}
              >
                {subtitle}
              </div>
            ) : null}
          </div>
        ) : null}
      </div>
    ) : null);

  return (
    <div
      ref={ref}
      {...mergePatternSlotProps<DesktopAppHeaderRootProps>(
        {
          className: 'relative z-30',
          'data-sdk-pattern': 'desktop-app-header',
        },
        {
          className,
          id,
          style,
          ...props,
        },
      )}
    >
      <DesktopTitleBar
        bordered={bordered}
        center={center}
        centerInteractive={centerInteractive}
        centerMaxWidth={centerMaxWidth}
        leading={
          brandBlock || leading ? (
            <>
              {brandBlock}
              {leading}
            </>
          ) : undefined
        }
        size={size}
        slotProps={{
          center: centerSlotProps,
          centerShell: centerShellSlotProps,
          leading: leadingSlotProps,
          trailing: trailingSlotProps,
        }}
        trailing={actions}
        translucent={translucent}
        windowController={windowController}
        windowControlLabels={windowControlLabels}
        windowControls={windowControls}
        {...barSlotProps}
      />
    </div>
  );
});
DesktopAppHeader.displayName = 'DesktopAppHeader';
