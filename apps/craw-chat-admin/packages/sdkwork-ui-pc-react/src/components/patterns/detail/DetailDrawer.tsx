import * as React from 'react';
import { cn } from '../../../lib/utils';
import {
  mergePatternSlotProps,
  type PatternSlotProps,
} from '../_internal/slot-props';
import {
  Drawer,
  DrawerBody,
  type DrawerBodyProps,
  DrawerContent,
  type DrawerContentProps,
  DrawerDescription,
  DrawerFooter,
  type DrawerFooterProps,
  DrawerHeader,
  type DrawerHeaderProps,
  DrawerTitle,
} from '../../ui/overlays';

export type DetailDrawerSide = 'left' | 'right';
export type DetailDrawerSize = 'sm' | 'md' | 'lg' | 'xl' | 'full';
export type DetailDrawerMetricTone = 'default' | 'success' | 'warning' | 'danger';

export interface DetailDrawerSlotProps {
  body?: PatternSlotProps<Omit<DrawerBodyProps, 'children'>>;
  content?: PatternSlotProps<Omit<DrawerContentProps, 'children' | 'side' | 'size'>>;
  footer?: PatternSlotProps<Omit<DrawerFooterProps, 'children'>>;
  header?: PatternSlotProps<Omit<DrawerHeaderProps, 'children'>>;
}

export interface DetailDrawerProps extends React.ComponentPropsWithoutRef<typeof Drawer> {
  actions?: React.ReactNode;
  className?: string;
  description?: React.ReactNode;
  eyebrow?: React.ReactNode;
  footer?: React.ReactNode;
  side?: DetailDrawerSide;
  size?: DetailDrawerSize;
  slotProps?: DetailDrawerSlotProps;
  summary?: React.ReactNode;
  title: React.ReactNode;
}

export interface DetailDrawerSectionProps extends Omit<React.HTMLAttributes<HTMLElement>, 'title'> {
  actions?: React.ReactNode;
  description?: React.ReactNode;
  title?: React.ReactNode;
}

export interface DetailDrawerMetricsProps extends React.HTMLAttributes<HTMLDivElement> {
  columns?: 1 | 2 | 3;
}

export interface DetailDrawerMetricProps extends React.HTMLAttributes<HTMLDivElement> {
  helper?: React.ReactNode;
  label: React.ReactNode;
  tone?: DetailDrawerMetricTone;
  value: React.ReactNode;
}

const metricToneClassName: Record<DetailDrawerMetricTone, string> = {
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

function DetailDrawer({
  actions,
  children,
  className,
  description,
  eyebrow,
  footer,
  size = 'lg',
  side = 'right',
  slotProps,
  summary,
  title,
  ...props
}: DetailDrawerProps) {
  return (
    <Drawer {...props}>
      <DrawerContent
        {...(description ? {} : { 'aria-describedby': undefined })}
        {...mergePatternSlotProps<PatternSlotProps<Omit<DrawerContentProps, 'children' | 'side' | 'size'>>>(
          {
            className: cn('gap-0 overflow-hidden', className),
            'data-sdk-pattern': 'detail-drawer',
            'data-sdk-region': 'detail-drawer-content',
          },
          slotProps?.content,
        )}
        size={size}
        side={side}
      >
        <DrawerHeader
          {...mergePatternSlotProps<PatternSlotProps<Omit<DrawerHeaderProps, 'children'>>>(
            {
              'data-sdk-region': 'detail-drawer-header',
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
              <DrawerTitle className={cn(eyebrow ? 'mt-1' : null)}>{title}</DrawerTitle>
              {description ? <DrawerDescription className="mt-1.5">{description}</DrawerDescription> : null}
            </div>
            {actions ? <div className="flex shrink-0 items-center gap-2 pr-8">{actions}</div> : null}
          </div>
          {summary ? (
            <div className="rounded-[var(--sdk-radius-panel)] border border-[var(--sdk-color-border-default)] bg-[var(--sdk-color-surface-panel-muted)] px-4 py-3 text-sm text-[var(--sdk-color-text-secondary)]">
              {summary}
            </div>
          ) : null}
        </DrawerHeader>
        <DrawerBody
          {...mergePatternSlotProps<PatternSlotProps<Omit<DrawerBodyProps, 'children'>>>(
            {
              className: 'space-y-4',
              'data-sdk-region': 'detail-drawer-body',
            },
            slotProps?.body,
          )}
        >
          {children}
        </DrawerBody>
        {footer ? (
          <DrawerFooter
            {...mergePatternSlotProps<PatternSlotProps<Omit<DrawerFooterProps, 'children'>>>(
              {
                'data-sdk-region': 'detail-drawer-footer',
              },
              slotProps?.footer,
            )}
          >
            {footer}
          </DrawerFooter>
        ) : null}
      </DrawerContent>
    </Drawer>
  );
}

const DetailDrawerSection = React.forwardRef<HTMLElement, DetailDrawerSectionProps>(({
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
      data-sdk-pattern="detail-drawer-section"
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

const DetailDrawerMetrics = React.forwardRef<HTMLDivElement, DetailDrawerMetricsProps>(({
  children,
  className,
  columns = 2,
  ...props
}, ref) => {
  return (
    <div
      ref={ref}
      className={cn('grid gap-3', metricsColumnsClassName[columns], className)}
      data-sdk-pattern="detail-drawer-metrics"
      {...props}
    >
      {children}
    </div>
  );
});

const DetailDrawerMetric = React.forwardRef<HTMLDivElement, DetailDrawerMetricProps>(({
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
      data-sdk-pattern="detail-drawer-metric"
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

export { DetailDrawer, DetailDrawerMetric, DetailDrawerMetrics, DetailDrawerSection };
DetailDrawer.displayName = 'DetailDrawer';
DetailDrawerMetric.displayName = 'DetailDrawerMetric';
DetailDrawerMetrics.displayName = 'DetailDrawerMetrics';
DetailDrawerSection.displayName = 'DetailDrawerSection';
