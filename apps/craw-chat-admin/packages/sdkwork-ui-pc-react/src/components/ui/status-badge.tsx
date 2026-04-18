import * as React from 'react';
import { Activity, AlertTriangle, CheckCircle2, PauseCircle, XCircle } from 'lucide-react';
import { Badge } from './badge';
import { cn } from '../../lib/utils';

export type StatusBadgeVariant = 'success' | 'warning' | 'danger' | 'default' | 'secondary';

export interface StatusBadgeProps extends React.HTMLAttributes<HTMLDivElement> {
  label?: string;
  showIcon?: boolean;
  status: string;
  variant?: StatusBadgeVariant;
}

const statusVariantMap: Record<string, StatusBadgeVariant> = {
  active: 'success',
  completed: 'success',
  connected: 'success',
  enabled: 'success',
  healthy: 'success',
  live: 'success',
  online: 'success',
  success: 'success',
  paused: 'warning',
  pending: 'warning',
  processing: 'warning',
  queued: 'warning',
  syncing: 'warning',
  warning: 'warning',
  archived: 'secondary',
  disabled: 'secondary',
  draft: 'secondary',
  idle: 'secondary',
  inactive: 'secondary',
  rejected: 'danger',
  error: 'danger',
  failed: 'danger',
  offline: 'danger',
};

const statusIconMap: Record<StatusBadgeVariant, React.ComponentType<{ className?: string }>> = {
  success: CheckCircle2,
  warning: AlertTriangle,
  danger: XCircle,
  default: Activity,
  secondary: PauseCircle,
};

function resolveStatusVariant(status: string, variant?: StatusBadgeVariant): StatusBadgeVariant {
  if (variant) {
    return variant;
  }

  return statusVariantMap[status.trim().toLowerCase()] ?? 'default';
}

function StatusBadge({
  className,
  label,
  showIcon = false,
  status,
  variant,
  ...props
}: StatusBadgeProps) {
  const resolvedVariant = resolveStatusVariant(status, variant);
  const Icon = statusIconMap[resolvedVariant];

  return (
    <Badge
      className={cn('gap-1.5 capitalize', className)}
      data-sdk-ui="status-badge"
      data-variant={resolvedVariant}
      variant={resolvedVariant}
      {...props}
    >
      {showIcon ? <Icon className="h-3.5 w-3.5" /> : null}
      <span>{label ?? status}</span>
    </Badge>
  );
}

StatusBadge.displayName = 'StatusBadge';

export { StatusBadge };
