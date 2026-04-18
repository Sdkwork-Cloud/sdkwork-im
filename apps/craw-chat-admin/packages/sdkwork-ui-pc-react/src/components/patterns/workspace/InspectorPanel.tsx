import * as React from 'react';
import { cn } from '../../../lib/utils';

export interface InspectorPanelProps extends Omit<React.ComponentPropsWithoutRef<'aside'>, 'title'> {
  footer?: React.ReactNode;
  title?: React.ReactNode;
}

export const InspectorPanel = React.forwardRef<HTMLElement, InspectorPanelProps>(({
  children,
  className,
  footer,
  title,
  ...props
}, ref) => {
  return (
    <aside
      ref={ref}
      className={cn(
        'flex h-full flex-col rounded-[var(--sdk-radius-panel)] border border-[var(--sdk-color-border-default)] bg-[var(--sdk-color-surface-panel)] shadow-[var(--sdk-shadow-sm)]',
        className,
      )}
      data-sdk-pattern="inspector-panel"
      {...props}
    >
      {title ? (
        <div className="border-b border-[var(--sdk-color-border-subtle)] px-5 py-4 text-sm font-semibold">
          {title}
        </div>
      ) : null}
      <div className="min-h-0 flex-1 px-5 py-4">{children}</div>
      {footer ? (
        <div className="border-t border-[var(--sdk-color-border-subtle)] px-5 py-4">{footer}</div>
      ) : null}
    </aside>
  );
});
InspectorPanel.displayName = 'InspectorPanel';
