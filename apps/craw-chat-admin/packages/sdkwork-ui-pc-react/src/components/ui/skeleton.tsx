import * as React from 'react';
import { cn } from '../../lib/utils';

export type SkeletonProps = React.HTMLAttributes<HTMLDivElement>;

const Skeleton = React.forwardRef<HTMLDivElement, SkeletonProps>(
  ({ className, ...props }, ref) => (
    <div
      ref={ref}
      className={cn(
        'animate-pulse rounded-[var(--sdk-radius-control)] bg-[var(--sdk-color-surface-panel-muted)]',
        className,
      )}
      data-sdk-ui="skeleton"
      {...props}
    />
  ),
);

Skeleton.displayName = 'Skeleton';

export { Skeleton };
