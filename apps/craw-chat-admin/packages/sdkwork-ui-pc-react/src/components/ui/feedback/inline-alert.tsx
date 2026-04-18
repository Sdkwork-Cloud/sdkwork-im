import * as React from 'react';
import { AlertCircle, AlertTriangle, CheckCircle2, Info } from 'lucide-react';
import { cn } from '../../../lib/utils';

export interface InlineAlertProps extends Omit<React.HTMLAttributes<HTMLDivElement>, 'title'> {
  actions?: React.ReactNode;
  description?: React.ReactNode;
  showIcon?: boolean;
  title?: React.ReactNode;
  tone?: 'default' | 'info' | 'success' | 'warning' | 'danger';
}

const toneClass: Record<NonNullable<InlineAlertProps['tone']>, string> = {
  default:
    'border-[var(--sdk-color-border-default)] bg-[var(--sdk-color-surface-panel-muted)] text-[var(--sdk-color-text-primary)]',
  info:
    'border-[color-mix(in_srgb,var(--sdk-color-state-info)_32%,transparent)] bg-[color-mix(in_srgb,var(--sdk-color-state-info)_14%,transparent)] text-[var(--sdk-color-text-primary)]',
  success:
    'border-[color-mix(in_srgb,var(--sdk-color-state-success)_32%,transparent)] bg-[color-mix(in_srgb,var(--sdk-color-state-success)_14%,transparent)] text-[var(--sdk-color-text-primary)]',
  warning:
    'border-[color-mix(in_srgb,var(--sdk-color-state-warning)_32%,transparent)] bg-[color-mix(in_srgb,var(--sdk-color-state-warning)_14%,transparent)] text-[var(--sdk-color-text-primary)]',
  danger:
    'border-[color-mix(in_srgb,var(--sdk-color-state-danger)_32%,transparent)] bg-[color-mix(in_srgb,var(--sdk-color-state-danger)_14%,transparent)] text-[var(--sdk-color-text-primary)]',
};

const toneIcon = {
  default: Info,
  info: Info,
  success: CheckCircle2,
  warning: AlertTriangle,
  danger: AlertCircle,
} as const;

export const InlineAlert = React.forwardRef<HTMLDivElement, InlineAlertProps>(({
  actions,
  className,
  description,
  showIcon = true,
  title,
  tone = 'default',
  ...props
}, ref) => {
  const Icon = toneIcon[tone];

  return (
    <div
      ref={ref}
      className={cn(
        'flex items-start gap-3 rounded-[var(--sdk-radius-control)] border px-4 py-3 shadow-[var(--sdk-shadow-sm)]',
        toneClass[tone],
        className,
      )}
      data-sdk-ui="inline-alert"
      role="alert"
      {...props}
    >
      {showIcon ? <Icon className="mt-0.5 h-4 w-4 shrink-0" /> : null}
      <div className="min-w-0 flex-1">
        {title ? <div className="text-sm font-semibold">{title}</div> : null}
        {description ? (
          <div className={cn('text-sm text-[var(--sdk-color-text-secondary)]', title ? 'mt-1' : undefined)}>
            {description}
          </div>
        ) : null}
      </div>
      {actions ? <div className="flex shrink-0 items-center gap-2">{actions}</div> : null}
    </div>
  );
});

InlineAlert.displayName = 'InlineAlert';
