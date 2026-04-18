import * as React from 'react';
import { X } from 'lucide-react';
import { cn } from '../../../lib/utils';
import { Button } from '../button';

export type BulkActionBarTone = 'default' | 'info' | 'success' | 'warning' | 'danger';
export type BulkActionBarClearHandler = () => void;

export interface BulkActionBarProps extends Omit<React.HTMLAttributes<HTMLDivElement>, 'title'> {
  actions?: React.ReactNode;
  clearLabel?: React.ReactNode;
  count?: React.ReactNode;
  description?: React.ReactNode;
  meta?: React.ReactNode;
  onClear?: BulkActionBarClearHandler;
  sticky?: boolean;
  title?: React.ReactNode;
  tone?: BulkActionBarTone;
}

const toneClassName: Record<BulkActionBarTone, string> = {
  default:
    'border-[var(--sdk-color-border-default)] bg-[var(--sdk-color-surface-panel)]',
  info:
    'border-[color-mix(in_srgb,var(--sdk-color-state-info)_28%,transparent)] bg-[color-mix(in_srgb,var(--sdk-color-state-info)_10%,transparent)]',
  success:
    'border-[color-mix(in_srgb,var(--sdk-color-state-success)_28%,transparent)] bg-[color-mix(in_srgb,var(--sdk-color-state-success)_10%,transparent)]',
  warning:
    'border-[color-mix(in_srgb,var(--sdk-color-state-warning)_28%,transparent)] bg-[color-mix(in_srgb,var(--sdk-color-state-warning)_10%,transparent)]',
  danger:
    'border-[color-mix(in_srgb,var(--sdk-color-state-danger)_28%,transparent)] bg-[color-mix(in_srgb,var(--sdk-color-state-danger)_10%,transparent)]',
};

export const BulkActionBar = React.forwardRef<HTMLDivElement, BulkActionBarProps>(({
  actions,
  className,
  clearLabel = 'Clear',
  count,
  description,
  meta,
  onClear,
  sticky = false,
  title,
  tone = 'default',
  ...props
}, ref) => {
  return (
    <div
      ref={ref}
      className={cn(
        'flex flex-wrap items-center justify-between gap-3 rounded-[var(--sdk-radius-panel)] border px-4 py-3 shadow-[var(--sdk-shadow-sm)]',
        toneClassName[tone],
        sticky ? 'sticky top-0 z-20 backdrop-blur-xl' : null,
        className,
      )}
      data-sdk-ui="bulk-action-bar"
      data-tone={tone}
      {...props}
    >
      <div className="flex min-w-0 flex-wrap items-center gap-3">
        {count ? (
          <span className="inline-flex h-8 min-w-8 items-center justify-center rounded-full bg-[var(--sdk-color-surface-panel-muted)] px-2.5 text-xs font-semibold text-[var(--sdk-color-text-primary)]">
            {count}
          </span>
        ) : null}
        <div className="min-w-0">
          {title ? <div className="text-sm font-semibold text-[var(--sdk-color-text-primary)]">{title}</div> : null}
          {description ? (
            <div className="text-sm text-[var(--sdk-color-text-secondary)]">{description}</div>
          ) : null}
        </div>
        {meta ? <div className="text-xs text-[var(--sdk-color-text-muted)]">{meta}</div> : null}
      </div>
      <div className="flex flex-wrap items-center gap-2">
        {actions}
        {onClear ? (
          <Button onClick={onClear} size="sm" type="button" variant="ghost">
            <X className="h-4 w-4" />
            {clearLabel}
          </Button>
        ) : null}
      </div>
    </div>
  );
});
BulkActionBar.displayName = 'BulkActionBar';
