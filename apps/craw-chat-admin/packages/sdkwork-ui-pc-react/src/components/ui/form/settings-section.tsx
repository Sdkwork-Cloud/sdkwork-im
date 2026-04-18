import * as React from 'react';
import { cn } from '../../../lib/utils';

export interface SettingsSectionProps extends Omit<React.HTMLAttributes<HTMLElement>, 'title'> {
  actions?: React.ReactNode;
  description?: React.ReactNode;
  title?: React.ReactNode;
}

const SettingsSection = React.forwardRef<HTMLElement, SettingsSectionProps>(
  ({ actions, children, className, description, title, ...props }, ref) => {
    const titleId = React.useId();
    const descriptionId = React.useId();

    return (
      <section
        ref={ref}
        aria-describedby={description ? descriptionId : undefined}
        aria-labelledby={title ? titleId : undefined}
        className={cn('space-y-4', className)}
        data-sdk-ui="settings-section"
        {...props}
      >
        {title || description || actions ? (
          <div className="flex flex-col gap-3 border-b border-[var(--sdk-color-border-subtle)] pb-3 sm:flex-row sm:items-start sm:justify-between">
            <div className="min-w-0">
              {title ? (
                <h3
                  className="text-[11px] font-semibold uppercase tracking-[0.18em] text-[var(--sdk-color-text-muted)]"
                  id={titleId}
                >
                  {title}
                </h3>
              ) : null}
              {description ? (
                <p
                  className="mt-2 max-w-3xl text-sm text-[var(--sdk-color-text-secondary)]"
                  id={descriptionId}
                >
                  {description}
                </p>
              ) : null}
            </div>
            {actions ? <div className="flex shrink-0 items-center gap-2">{actions}</div> : null}
          </div>
        ) : null}
        <div className="space-y-3">{children}</div>
      </section>
    );
  },
);

SettingsSection.displayName = 'SettingsSection';

export { SettingsSection };
