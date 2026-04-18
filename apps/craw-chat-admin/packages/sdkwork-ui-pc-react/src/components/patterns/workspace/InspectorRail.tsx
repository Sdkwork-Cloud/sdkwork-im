import * as React from 'react';
import { cn } from '../../../lib/utils';
import {
  mergePatternSlotProps,
  type PatternSlotProps,
} from '../_internal/slot-props';

export type InspectorRailRegionSlotProps = PatternSlotProps<
  Omit<React.ComponentPropsWithoutRef<'div'>, 'children'>
>;

export interface InspectorRailSlotProps {
  body?: InspectorRailRegionSlotProps;
  footer?: InspectorRailRegionSlotProps;
  header?: InspectorRailRegionSlotProps;
}

export type InspectorRailSide = 'left' | 'right';
export type InspectorRailVariant = 'docked' | 'sticky' | 'drawer';
export type InspectorRailMetricTone = 'default' | 'success' | 'warning' | 'danger';

export interface InspectorRailProps extends Omit<React.ComponentPropsWithoutRef<'aside'>, 'title'> {
  actions?: React.ReactNode;
  description?: React.ReactNode;
  eyebrow?: React.ReactNode;
  footer?: React.ReactNode;
  meta?: React.ReactNode;
  side?: InspectorRailSide;
  slotProps?: InspectorRailSlotProps;
  stickyHeader?: boolean;
  summary?: React.ReactNode;
  title?: React.ReactNode;
  variant?: InspectorRailVariant;
}

export interface InspectorRailSectionProps
  extends Omit<React.HTMLAttributes<HTMLElement>, 'title'> {
  actions?: React.ReactNode;
  description?: React.ReactNode;
  title?: React.ReactNode;
}

export interface InspectorRailMetricsProps extends React.HTMLAttributes<HTMLDivElement> {
  columns?: 1 | 2 | 3;
}

export interface InspectorRailMetricProps extends React.HTMLAttributes<HTMLDivElement> {
  helper?: React.ReactNode;
  label: React.ReactNode;
  tone?: InspectorRailMetricTone;
  value: React.ReactNode;
}

const railVariantClassName: Record<InspectorRailVariant, string> = {
  docked:
    'border border-[var(--sdk-color-border-default)] bg-[var(--sdk-color-surface-panel)] shadow-[var(--sdk-shadow-sm)]',
  sticky:
    'border border-[var(--sdk-color-border-default)] bg-[var(--sdk-color-surface-panel)] shadow-[var(--sdk-shadow-md)]',
  drawer:
    'border border-[var(--sdk-color-border-default)] bg-[var(--sdk-color-surface-panel)] shadow-[var(--sdk-shadow-lg)]',
};

const metricToneClassName: Record<InspectorRailMetricTone, string> = {
  default: 'text-[var(--sdk-color-text-primary)]',
  success: 'text-[var(--sdk-color-state-success)]',
  warning: 'text-[var(--sdk-color-state-warning)]',
  danger: 'text-[var(--sdk-color-state-danger)]',
};

const metricsColumnsClassName = {
  1: 'grid-cols-1',
  2: 'grid-cols-1 sm:grid-cols-2',
  3: 'grid-cols-1 sm:grid-cols-2 xl:grid-cols-3',
} as const;

export const InspectorRail = React.forwardRef<HTMLElement, InspectorRailProps>(({
  actions,
  children,
  className,
  description,
  eyebrow,
  footer,
  meta,
  side = 'right',
  slotProps,
  stickyHeader = false,
  summary,
  title,
  variant = 'docked',
  ...props
}, ref) => {
  const hasHeader = eyebrow || title || description || meta || actions || summary;

  return (
    <aside
      ref={ref}
      className={cn(
        'flex h-full min-h-0 flex-col overflow-hidden rounded-[var(--sdk-radius-panel)]',
        railVariantClassName[variant],
        className,
      )}
      data-sdk-pattern="inspector-rail"
      data-side={side}
      data-variant={variant}
      {...props}
    >
      {hasHeader ? (
        <div
          {...mergePatternSlotProps<InspectorRailRegionSlotProps>(
            {
              className: cn(
                'border-b border-[var(--sdk-color-border-subtle)] bg-[var(--sdk-color-surface-panel)]/95 px-5 py-4',
                stickyHeader ? 'sticky top-0 z-10 backdrop-blur-xl' : null,
              ),
              'data-sdk-region': 'inspector-rail-header',
            },
            slotProps?.header,
          )}
        >
          <div className="flex flex-wrap items-start justify-between gap-3">
            <div className="min-w-0 flex-1">
              {eyebrow ? (
                <div className="text-[10px] font-semibold uppercase tracking-[0.18em] text-[var(--sdk-color-text-muted)]">
                  {eyebrow}
                </div>
              ) : null}
              {title ? <div className="mt-1 text-sm font-semibold text-[var(--sdk-color-text-primary)]">{title}</div> : null}
              {description ? (
                <div className="mt-1 text-sm text-[var(--sdk-color-text-secondary)]">{description}</div>
              ) : null}
              {meta ? <div className="mt-3">{meta}</div> : null}
            </div>
            {actions ? <div className="flex shrink-0 items-center gap-2">{actions}</div> : null}
          </div>
          {summary ? (
            <div className="mt-4 rounded-[var(--sdk-radius-control)] border border-[var(--sdk-color-border-default)] bg-[var(--sdk-color-surface-panel-muted)] px-4 py-3 text-sm text-[var(--sdk-color-text-secondary)]">
              {summary}
            </div>
          ) : null}
        </div>
      ) : null}
      <div className="min-h-0 flex-1 overflow-y-auto">
        <div
          {...mergePatternSlotProps<InspectorRailRegionSlotProps>(
            {
              className: 'space-y-4 px-5 py-4',
              'data-sdk-region': 'inspector-rail-body',
            },
            slotProps?.body,
          )}
        >
          {children}
        </div>
      </div>
      {footer ? (
        <div
          {...mergePatternSlotProps<InspectorRailRegionSlotProps>(
            {
              className: 'border-t border-[var(--sdk-color-border-subtle)] px-5 py-4',
              'data-sdk-region': 'inspector-rail-footer',
            },
            slotProps?.footer,
          )}
        >
          {footer}
        </div>
      ) : null}
    </aside>
  );
});

export const InspectorRailSection = React.forwardRef<HTMLElement, InspectorRailSectionProps>(({
  actions,
  children,
  className,
  description,
  title,
  ...props
}, ref) => {
  return (
    <section
      ref={ref}
      className={cn(
        'space-y-3 rounded-[var(--sdk-radius-panel)] border border-[var(--sdk-color-border-default)] bg-[var(--sdk-color-surface-panel)] px-4 py-4 shadow-[var(--sdk-shadow-soft)]',
        className,
      )}
      data-sdk-pattern="inspector-rail-section"
      {...props}
    >
      {(title || description || actions) ? (
        <div className="flex flex-wrap items-start justify-between gap-3">
          <div className="min-w-0 flex-1">
            {title ? <div className="text-sm font-semibold text-[var(--sdk-color-text-primary)]">{title}</div> : null}
            {description ? (
              <div className="mt-1 text-sm text-[var(--sdk-color-text-secondary)]">{description}</div>
            ) : null}
          </div>
          {actions ? <div className="flex shrink-0 items-center gap-2">{actions}</div> : null}
        </div>
      ) : null}
      {children}
    </section>
  );
});

export const InspectorRailMetrics = React.forwardRef<HTMLDivElement, InspectorRailMetricsProps>(({
  children,
  className,
  columns = 2,
  ...props
}, ref) => {
  return (
    <div
      ref={ref}
      className={cn('grid gap-3', metricsColumnsClassName[columns], className)}
      data-sdk-pattern="inspector-rail-metrics"
      {...props}
    >
      {children}
    </div>
  );
});

export const InspectorRailMetric = React.forwardRef<HTMLDivElement, InspectorRailMetricProps>(({
  className,
  helper,
  label,
  tone = 'default',
  value,
  ...props
}, ref) => {
  return (
    <div
      ref={ref}
      className={cn(
        'rounded-[var(--sdk-radius-control)] border border-[var(--sdk-color-border-default)] bg-[var(--sdk-color-surface-panel)] px-4 py-3 shadow-[var(--sdk-shadow-soft)]',
        className,
      )}
      data-sdk-pattern="inspector-rail-metric"
      data-tone={tone}
      {...props}
    >
      <div className="text-[11px] font-semibold uppercase tracking-[0.12em] text-[var(--sdk-color-text-muted)]">
        {label}
      </div>
      <div className={cn('mt-2 text-lg font-semibold', metricToneClassName[tone])}>{value}</div>
      {helper ? <div className="mt-1 text-xs text-[var(--sdk-color-text-secondary)]">{helper}</div> : null}
    </div>
  );
});
InspectorRail.displayName = 'InspectorRail';
InspectorRailSection.displayName = 'InspectorRailSection';
InspectorRailMetrics.displayName = 'InspectorRailMetrics';
InspectorRailMetric.displayName = 'InspectorRailMetric';
