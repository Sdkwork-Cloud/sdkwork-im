import * as React from 'react';
import { ChevronDown, ChevronRight } from 'lucide-react';
import { cn } from '../../../lib/utils';

export type SidebarSectionExpandedChangeHandler = (expanded: boolean) => void;

export interface SidebarSectionProps extends Omit<React.HTMLAttributes<HTMLDivElement>, 'title'> {
  actions?: React.ReactNode;
  defaultExpanded?: boolean;
  description?: React.ReactNode;
  expanded?: boolean;
  meta?: React.ReactNode;
  onExpandedChange?: SidebarSectionExpandedChangeHandler;
  title: React.ReactNode;
}

const SidebarSection = React.forwardRef<HTMLDivElement, SidebarSectionProps>(
  (
    {
      actions,
      children,
      className,
      defaultExpanded = true,
      description,
      expanded,
      meta,
      onExpandedChange,
      title,
      ...props
    },
    ref,
  ) => {
    const isControlled = expanded !== undefined;
    const [uncontrolledExpanded, setUncontrolledExpanded] = React.useState(defaultExpanded);
    const open = isControlled ? expanded : uncontrolledExpanded;
    const contentId = React.useId();

    const handleToggle = () => {
      const nextValue = !open;
      if (!isControlled) {
        setUncontrolledExpanded(nextValue);
      }
      onExpandedChange?.(nextValue);
    };

    return (
      <div
        ref={ref}
        className={cn(
          'rounded-[var(--sdk-radius-control)] border border-[var(--sdk-color-border-subtle)] bg-[var(--sdk-color-surface-panel)]',
          className,
        )}
        data-sdk-ui="sidebar-section"
        data-state={open ? 'open' : 'closed'}
        {...props}
      >
        <div className="flex items-start gap-2 p-2">
          <button
            aria-controls={contentId}
            aria-expanded={open}
            className="flex min-w-0 flex-1 items-center justify-between gap-3 rounded-[var(--sdk-radius-control)] px-1 py-1.5 text-left transition-colors hover:bg-[var(--sdk-color-brand-primary-soft)]"
            onClick={handleToggle}
            type="button"
          >
            <div className="min-w-0">
              <div className="flex items-center gap-2">
                <span className="text-[10px] font-semibold uppercase tracking-[0.16em] text-[var(--sdk-color-text-muted)]">
                  {title}
                </span>
                {meta ? <span className="shrink-0">{meta}</span> : null}
              </div>
              {description ? (
                <div className="mt-1 text-xs text-[var(--sdk-color-text-secondary)]">{description}</div>
              ) : null}
            </div>
            <span className="shrink-0 text-[var(--sdk-color-text-muted)]">
              {open ? <ChevronDown className="h-3.5 w-3.5" /> : <ChevronRight className="h-3.5 w-3.5" />}
            </span>
          </button>
          {actions ? <div className="flex shrink-0 items-center gap-1">{actions}</div> : null}
        </div>
        {open ? (
          <div
            className="border-t border-[var(--sdk-color-border-subtle)] px-3 pb-3 pt-2"
            id={contentId}
          >
            {children}
          </div>
        ) : null}
      </div>
    );
  },
);

SidebarSection.displayName = 'SidebarSection';

export { SidebarSection };
