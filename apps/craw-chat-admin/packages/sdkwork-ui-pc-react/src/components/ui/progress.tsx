import * as React from 'react';
import { cn } from '../../lib/utils';

export interface ProgressProps extends React.HTMLAttributes<HTMLDivElement> {
  value?: number;
}

function clampProgress(value?: number) {
  if (typeof value !== 'number' || Number.isNaN(value)) {
    return 0;
  }

  return Math.min(100, Math.max(0, value));
}

const Progress = React.forwardRef<HTMLDivElement, ProgressProps>(
  ({ className, value = 0, ...props }, ref) => {
    const safeValue = clampProgress(value);

    return (
      <div
        ref={ref}
        aria-valuemax={100}
        aria-valuemin={0}
        aria-valuenow={safeValue}
        className={cn(
          'relative h-2.5 w-full overflow-hidden rounded-[var(--sdk-radius-pill)] bg-[var(--sdk-color-surface-panel-muted)]',
          className,
        )}
        data-sdk-ui="progress"
        role="progressbar"
        {...props}
      >
        <div
          className="h-full rounded-[var(--sdk-radius-pill)] bg-[var(--sdk-color-brand-primary)] transition-[width] duration-200"
          data-sdk-ui="progress-indicator"
          data-testid="sdk-progress-indicator"
          style={{ width: `${safeValue}%` }}
        />
      </div>
    );
  },
);

Progress.displayName = 'Progress';

export { Progress };
