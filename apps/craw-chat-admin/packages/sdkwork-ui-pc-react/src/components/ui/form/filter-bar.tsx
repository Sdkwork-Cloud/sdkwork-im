import * as React from 'react';
import { cn } from '../../../lib/utils';

export interface FilterBarProps extends Omit<React.HTMLAttributes<HTMLElement>, 'title'> {
  description?: React.ReactNode;
  summary?: React.ReactNode;
  title?: React.ReactNode;
}

export interface FilterBarSectionProps extends React.HTMLAttributes<HTMLDivElement> {
  grow?: boolean;
  wrap?: boolean;
}

export interface FilterBarActionsProps extends React.HTMLAttributes<HTMLDivElement> {
  justify?: 'start' | 'end';
}

const actionAlignmentClassName = {
  start: 'justify-start',
  end: 'justify-start sm:ml-auto sm:justify-end',
} as const;

const FilterBar = React.forwardRef<HTMLElement, FilterBarProps>(
  ({ children, className, description, summary, title, ...props }, ref) => {
    const titleId = React.useId();
    const descriptionId = React.useId();
    const labelledBy = title ? titleId : props['aria-labelledby'];
    const describedBy = description ? descriptionId : props['aria-describedby'];

    return (
      <section
        ref={ref}
        aria-describedby={describedBy}
        aria-labelledby={labelledBy}
        className={cn(
          'space-y-4 rounded-[var(--sdk-radius-control)] border border-[var(--sdk-color-border-default)] bg-[var(--sdk-color-surface-panel)] p-4 shadow-[var(--sdk-shadow-sm)]',
          className,
        )}
        data-sdk-ui="filter-bar"
        {...props}
      >
        {title || description || summary ? (
          <div className="flex flex-col gap-3 xl:flex-row xl:items-start xl:justify-between">
            <div className="space-y-1">
              {title ? (
                <h3
                  className="text-sm font-semibold text-[var(--sdk-color-text-primary)]"
                  id={titleId}
                >
                  {title}
                </h3>
              ) : null}
              {description ? (
                <p
                  className="text-sm text-[var(--sdk-color-text-secondary)]"
                  id={descriptionId}
                >
                  {description}
                </p>
              ) : null}
            </div>
            {summary ? (
              <div className="text-sm font-medium text-[var(--sdk-color-text-secondary)]">
                {summary}
              </div>
            ) : null}
          </div>
        ) : null}
        <div className="flex flex-wrap items-end gap-3">{children}</div>
      </section>
    );
  },
);

FilterBar.displayName = 'FilterBar';

const FilterBarSection = React.forwardRef<HTMLDivElement, FilterBarSectionProps>(
  ({ className, grow = true, wrap = true, ...props }, ref) => (
    <div
      ref={ref}
      className={cn(
        'flex min-w-[12rem] items-end gap-3',
        grow ? 'flex-1' : undefined,
        wrap ? 'flex-wrap' : 'flex-nowrap',
        className,
      )}
      data-sdk-ui="filter-bar-section"
      {...props}
    />
  ),
);

FilterBarSection.displayName = 'FilterBarSection';

const FilterBarActions = React.forwardRef<HTMLDivElement, FilterBarActionsProps>(
  ({ className, justify = 'end', ...props }, ref) => (
    <div
      ref={ref}
      className={cn(
        'flex w-full flex-wrap items-center gap-2 sm:w-auto',
        actionAlignmentClassName[justify],
        className,
      )}
      data-sdk-ui="filter-bar-actions"
      {...props}
    />
  ),
);

FilterBarActions.displayName = 'FilterBarActions';

export { FilterBar, FilterBarActions, FilterBarSection };
