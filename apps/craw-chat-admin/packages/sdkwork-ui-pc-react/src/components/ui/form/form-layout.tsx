import * as React from 'react';
import { cn } from '../../../lib/utils';

export interface FormSectionProps extends Omit<React.HTMLAttributes<HTMLElement>, 'title'> {
  actions?: React.ReactNode;
  description?: React.ReactNode;
  title?: React.ReactNode;
}

const formGridColumnsClass = {
  1: 'grid-cols-1',
  2: 'grid-cols-1 xl:grid-cols-2',
  3: 'grid-cols-1 xl:grid-cols-3',
} as const;

export interface FormGridProps extends React.HTMLAttributes<HTMLDivElement> {
  columns?: keyof typeof formGridColumnsClass;
}

export type FormActionsProps = React.HTMLAttributes<HTMLDivElement>;

const FormSection = React.forwardRef<HTMLElement, FormSectionProps>(
  ({ actions, children, className, description, title, ...props }, ref) => {
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
          'space-y-6 rounded-[var(--sdk-radius-control)] border border-[var(--sdk-color-border-default)] bg-[var(--sdk-color-surface-panel)] p-6 shadow-[var(--sdk-shadow-sm)]',
          className,
        )}
        data-sdk-ui="form-section"
        {...props}
      >
        {title || description || actions ? (
          <div className="flex flex-col gap-4 border-b border-[var(--sdk-color-border-subtle)] pb-5 xl:flex-row xl:items-start xl:justify-between">
            <div className="space-y-1.5">
              {title ? (
                <h3
                  className="text-base font-semibold text-[var(--sdk-color-text-primary)]"
                  id={titleId}
                >
                  {title}
                </h3>
              ) : null}
              {description ? (
                <p
                  className="max-w-3xl text-sm text-[var(--sdk-color-text-secondary)]"
                  id={descriptionId}
                >
                  {description}
                </p>
              ) : null}
            </div>
            {actions ? <div className="flex items-center gap-3">{actions}</div> : null}
          </div>
        ) : null}
        <div className="space-y-6">{children}</div>
      </section>
    );
  },
);

FormSection.displayName = 'FormSection';

const FormGrid = React.forwardRef<HTMLDivElement, FormGridProps>(
  ({ children, className, columns = 2, ...props }, ref) => (
    <div
      ref={ref}
      className={cn('grid gap-4 xl:gap-6', formGridColumnsClass[columns], className)}
      data-sdk-ui="form-grid"
      {...props}
    >
      {children}
    </div>
  ),
);

FormGrid.displayName = 'FormGrid';

const FormActions = React.forwardRef<HTMLDivElement, FormActionsProps>(
  ({ className, ...props }, ref) => (
    <div
      ref={ref}
      className={cn(
        'flex flex-col-reverse gap-3 border-t border-[var(--sdk-color-border-subtle)] pt-5 sm:flex-row sm:items-center sm:justify-end',
        className,
      )}
      data-sdk-ui="form-actions"
      {...props}
    />
  ),
);

FormActions.displayName = 'FormActions';

export { FormActions, FormGrid, FormSection };
