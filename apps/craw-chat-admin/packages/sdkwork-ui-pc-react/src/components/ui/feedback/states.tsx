import * as React from 'react';
import { LoaderCircle } from 'lucide-react';
import { cn } from '../../../lib/utils';

export interface EmptyStateProps extends Omit<React.HTMLAttributes<HTMLDivElement>, 'title'> {
  actions?: React.ReactNode;
  description?: React.ReactNode;
  title?: React.ReactNode;
}

export const EmptyState = React.forwardRef<HTMLDivElement, EmptyStateProps>(({
  actions,
  className,
  description,
  title,
  ...props
}, ref) => {
  return (
    <div
      ref={ref}
      className={cn(
        'flex flex-col items-center justify-center gap-3 rounded-[var(--sdk-radius-panel)] border border-dashed border-[var(--sdk-color-border-strong)] bg-[var(--sdk-color-surface-panel)] px-6 py-12 text-center',
        className,
      )}
      data-sdk-ui="empty-state"
      {...props}
    >
      {title ? <div className="text-lg font-semibold">{title}</div> : null}
      {description ? (
        <div className="max-w-xl text-sm text-[var(--sdk-color-text-secondary)]">{description}</div>
      ) : null}
      {actions ? <div className="mt-2 flex items-center gap-3">{actions}</div> : null}
    </div>
  );
});

export interface LoadingBlockProps extends React.HTMLAttributes<HTMLDivElement> {
  label?: React.ReactNode;
}

export const LoadingBlock = React.forwardRef<HTMLDivElement, LoadingBlockProps>(({ className, label = 'Loading...', ...props }, ref) => {
  return (
    <div
      ref={ref}
      className={cn(
        'flex items-center justify-center gap-3 rounded-[var(--sdk-radius-panel)] border border-[var(--sdk-color-border-subtle)] bg-[var(--sdk-color-surface-panel)] px-6 py-8 text-sm text-[var(--sdk-color-text-secondary)]',
        className,
      )}
      data-sdk-ui="loading-block"
      {...props}
    >
      <LoaderCircle className="h-4 w-4 animate-spin" />
      <span>{label}</span>
    </div>
  );
});

export interface StatusNoticeProps extends Omit<React.HTMLAttributes<HTMLDivElement>, 'title'> {
  children?: React.ReactNode;
  tone?: 'default' | 'success' | 'warning' | 'danger';
  title?: React.ReactNode;
}

const toneClass: Record<NonNullable<StatusNoticeProps['tone']>, string> = {
  default:
    'border-[var(--sdk-color-border-default)] bg-[var(--sdk-color-brand-primary-soft)] text-[var(--sdk-color-text-primary)]',
  success:
    'border-[color-mix(in_srgb,var(--sdk-color-state-success)_32%,transparent)] bg-[color-mix(in_srgb,var(--sdk-color-state-success)_14%,transparent)] text-[var(--sdk-color-text-primary)]',
  warning:
    'border-[color-mix(in_srgb,var(--sdk-color-state-warning)_32%,transparent)] bg-[color-mix(in_srgb,var(--sdk-color-state-warning)_14%,transparent)] text-[var(--sdk-color-text-primary)]',
  danger:
    'border-[color-mix(in_srgb,var(--sdk-color-state-danger)_32%,transparent)] bg-[color-mix(in_srgb,var(--sdk-color-state-danger)_14%,transparent)] text-[var(--sdk-color-text-primary)]',
};

export const StatusNotice = React.forwardRef<HTMLDivElement, StatusNoticeProps>(({
  children,
  className,
  title,
  tone = 'default',
  ...props
}, ref) => {
  return (
    <div
      ref={ref}
      className={cn(
        'rounded-[var(--sdk-radius-panel)] border px-4 py-3 shadow-[var(--sdk-shadow-sm)]',
        toneClass[tone],
        className,
      )}
      data-sdk-ui="status-notice"
      {...props}
    >
      {title ? <div className="text-sm font-semibold">{title}</div> : null}
      {children ? <div className={cn(title ? 'mt-1 text-sm' : 'text-sm')}>{children}</div> : null}
    </div>
  );
});

EmptyState.displayName = 'EmptyState';
LoadingBlock.displayName = 'LoadingBlock';
StatusNotice.displayName = 'StatusNotice';
