import * as React from 'react';
import { cn } from '../../../lib/utils';
import {
  mergePatternSlotProps,
  type PatternSlotProps,
} from '../_internal/slot-props';

export type WorkspacePanelSectionSlotProps = PatternSlotProps<
  Omit<React.ComponentPropsWithoutRef<'div'>, 'children'>
>;

export interface WorkspacePanelSlotProps {
  body?: WorkspacePanelSectionSlotProps;
  header?: WorkspacePanelSectionSlotProps;
}

export interface WorkspacePanelProps extends Omit<React.ComponentPropsWithoutRef<'section'>, 'title'> {
  actions?: React.ReactNode;
  description?: React.ReactNode;
  slotProps?: WorkspacePanelSlotProps;
  title?: React.ReactNode;
}

export const WorkspacePanel = React.forwardRef<HTMLElement, WorkspacePanelProps>(({
  actions,
  children,
  className,
  description,
  slotProps,
  title,
  ...props
}, ref) => {
  return (
    <section
      ref={ref}
      className={cn(
        'flex h-full flex-col rounded-[var(--sdk-radius-panel)] bg-[var(--sdk-color-surface-panel)]',
        className,
      )}
      data-sdk-pattern="workspace-panel"
      {...props}
    >
      {title || description || actions ? (
        <div
          {...mergePatternSlotProps<WorkspacePanelSectionSlotProps>(
            {
              className:
                'flex flex-wrap items-start justify-between gap-4 border-b border-[var(--sdk-color-border-subtle)] px-6 py-5',
              'data-sdk-region': 'workspace-panel-header',
            },
            slotProps?.header,
          )}
        >
          <div className="space-y-1">
            {title ? <div className="text-base font-semibold">{title}</div> : null}
            {description ? (
              <div className="text-sm text-[var(--sdk-color-text-secondary)]">{description}</div>
            ) : null}
          </div>
          {actions ? <div className="flex items-center gap-3">{actions}</div> : null}
        </div>
      ) : null}
      <div
        {...mergePatternSlotProps<WorkspacePanelSectionSlotProps>(
          {
            className: 'min-h-0 flex-1 px-6 py-5',
            'data-sdk-region': 'workspace-panel-body',
          },
          slotProps?.body,
        )}
      >
        {children}
      </div>
    </section>
  );
});
WorkspacePanel.displayName = 'WorkspacePanel';
