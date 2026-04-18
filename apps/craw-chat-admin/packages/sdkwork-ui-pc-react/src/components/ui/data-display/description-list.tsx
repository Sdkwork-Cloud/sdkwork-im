import * as React from 'react';
import { cn } from '../../../lib/utils';

const descriptionListColumnsClassName = {
  1: 'grid-cols-1',
  2: 'grid-cols-1 xl:grid-cols-2',
  3: 'grid-cols-1 xl:grid-cols-3',
} as const;

export interface DescriptionListProps extends React.HTMLAttributes<HTMLDListElement> {
  columns?: keyof typeof descriptionListColumnsClassName;
}

export interface DescriptionDetailsProps extends React.HTMLAttributes<HTMLElement> {
  mono?: boolean;
}

export type DescriptionItemProps = React.HTMLAttributes<HTMLDivElement>;
export type DescriptionTermProps = React.HTMLAttributes<HTMLElement>;

const DescriptionList = React.forwardRef<HTMLDListElement, DescriptionListProps>(
  ({ className, columns = 1, ...props }, ref) => (
    <dl
      ref={ref}
      className={cn('grid gap-3', descriptionListColumnsClassName[columns], className)}
      data-sdk-ui="description-list"
      {...props}
    />
  ),
);

DescriptionList.displayName = 'DescriptionList';

const DescriptionItem = React.forwardRef<HTMLDivElement, DescriptionItemProps>(
  ({ className, ...props }, ref) => (
    <div
      ref={ref}
      className={cn(
        'space-y-1.5 rounded-[var(--sdk-radius-control)] border border-[var(--sdk-color-border-default)] bg-[var(--sdk-color-surface-panel-muted)] px-3 py-3',
        className,
      )}
      data-sdk-ui="description-item"
      {...props}
    />
  ),
);

DescriptionItem.displayName = 'DescriptionItem';

const DescriptionTerm = React.forwardRef<HTMLElement, DescriptionTermProps>(
  ({ className, ...props }, ref) => (
    <dt
      ref={ref}
      className={cn(
        'text-xs font-medium uppercase tracking-[0.12em] text-[var(--sdk-color-text-muted)]',
        className,
      )}
      data-sdk-ui="description-term"
      {...props}
    />
  ),
);

DescriptionTerm.displayName = 'DescriptionTerm';

const DescriptionDetails = React.forwardRef<HTMLElement, DescriptionDetailsProps>(
  ({ className, mono = false, ...props }, ref) => (
    <dd
      ref={ref}
      className={cn(
        'm-0 text-sm text-[var(--sdk-color-text-primary)]',
        mono ? 'font-mono' : undefined,
        className,
      )}
      data-sdk-ui="description-details"
      {...props}
    />
  ),
);

DescriptionDetails.displayName = 'DescriptionDetails';

export { DescriptionDetails, DescriptionItem, DescriptionList, DescriptionTerm };
