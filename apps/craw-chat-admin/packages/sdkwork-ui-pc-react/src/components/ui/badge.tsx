import * as React from 'react';
import { cva, type VariantProps } from 'class-variance-authority';
import { cn } from '../../lib/utils';

const badgeVariants = cva(
  'inline-flex items-center rounded-[var(--sdk-radius-pill)] px-2.5 py-1 text-xs font-medium transition-colors',
  {
    variants: {
      variant: {
        default: 'bg-[var(--sdk-color-brand-primary-soft)] text-[var(--sdk-color-brand-primary)]',
        secondary: 'bg-[var(--sdk-color-surface-panel-muted)] text-[var(--sdk-color-text-secondary)]',
        success: 'bg-[color-mix(in_srgb,var(--sdk-color-state-success)_16%,transparent)] text-[var(--sdk-color-state-success)]',
        warning: 'bg-[color-mix(in_srgb,var(--sdk-color-state-warning)_16%,transparent)] text-[var(--sdk-color-state-warning)]',
        danger: 'bg-[color-mix(in_srgb,var(--sdk-color-state-danger)_16%,transparent)] text-[var(--sdk-color-state-danger)]',
        outline: 'border border-[var(--sdk-color-border-default)] text-[var(--sdk-color-text-secondary)]',
      },
    },
    defaultVariants: {
      variant: 'default',
    },
  },
);

export interface BadgeProps
  extends React.HTMLAttributes<HTMLDivElement>,
    VariantProps<typeof badgeVariants> {}

const Badge = React.forwardRef<HTMLDivElement, BadgeProps>(({ className, variant, ...props }, ref) => (
  <div ref={ref} className={cn(badgeVariants({ variant }), className)} data-sdk-ui="badge" {...props} />
));

Badge.displayName = 'Badge';

export { Badge, badgeVariants };
