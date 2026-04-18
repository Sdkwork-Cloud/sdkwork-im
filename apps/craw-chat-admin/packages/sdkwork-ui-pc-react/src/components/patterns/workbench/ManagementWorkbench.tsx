import * as React from 'react';
import { cn } from '../../../lib/utils';
import {
  mergePatternSlotProps,
  type PatternSlotProps,
} from '../_internal/slot-props';
import { PageHeader } from '../app-shell/PageHeader';
import { InspectorRail, type InspectorRailProps } from '../workspace/InspectorRail';
import { WorkspacePanel, type WorkspacePanelProps } from '../workspace/WorkspacePanel';

export interface ManagementWorkbenchMainProps
  extends Omit<WorkspacePanelProps, 'className'> {
  className?: string;
}

export interface ManagementWorkbenchDetailProps
  extends Omit<InspectorRailProps, 'className'> {
  className?: string;
}

export type ManagementWorkbenchRegionSlotProps = PatternSlotProps<
  Omit<React.ComponentPropsWithoutRef<'div'>, 'children'>
>;
export type ManagementWorkbenchMainRootProps = PatternSlotProps<
  Omit<React.ComponentPropsWithoutRef<'section'>, 'children'>
>;

export interface ManagementWorkbenchSlotProps {
  content?: ManagementWorkbenchRegionSlotProps;
  filters?: ManagementWorkbenchRegionSlotProps;
  footer?: ManagementWorkbenchRegionSlotProps;
  header?: PatternSlotProps<Omit<React.ComponentPropsWithoutRef<'div'>, 'children'>>;
  main?: ManagementWorkbenchMainRootProps;
  selectionBar?: ManagementWorkbenchRegionSlotProps;
}

export interface ManagementWorkbenchProps
  extends Omit<React.HTMLAttributes<HTMLDivElement>, 'children' | 'title'> {
  actions?: React.ReactNode;
  description?: React.ReactNode;
  detail?: ManagementWorkbenchDetailProps;
  detailWidth?: number | string;
  eyebrow?: React.ReactNode;
  filters?: React.ReactNode;
  footer?: React.ReactNode;
  main: ManagementWorkbenchMainProps;
  selectionBar?: React.ReactNode;
  slotProps?: ManagementWorkbenchSlotProps;
  title: React.ReactNode;
}

function resolveWorkbenchDetailWidth(detailWidth: number | string) {
  return typeof detailWidth === 'number' ? `${detailWidth}px` : detailWidth;
}

export const ManagementWorkbench = React.forwardRef<HTMLDivElement, ManagementWorkbenchProps>(({
  actions,
  className,
  description,
  detail,
  detailWidth = 320,
  eyebrow,
  filters,
  footer,
  main,
  selectionBar,
  slotProps,
  title,
  ...props
}, ref) => {
  const {
    actions: mainActions,
    children,
    className: mainClassName,
    description: mainDescription,
    slotProps: mainPanelSlotProps,
    title: mainTitle,
    ...mainRootProps
  } = main;
  const resolvedMainRootProps = mergePatternSlotProps<ManagementWorkbenchMainRootProps>(
    {
      className: cn('h-full', mainClassName),
      ...mainRootProps,
    },
    slotProps?.main,
  );
  const detailGridStyle = detail
    ? { gridTemplateColumns: `minmax(0, 1fr) ${resolveWorkbenchDetailWidth(detailWidth)}` }
    : undefined;

  return (
    <div
      ref={ref}
      className={cn('flex h-full min-h-0 min-w-0 flex-col gap-4', className)}
      data-sdk-pattern="management-workbench"
      {...props}
    >
      <PageHeader
        actions={actions}
        description={description}
        eyebrow={eyebrow}
        {...slotProps?.header}
        title={title}
      />
      {filters ? (
        <div
          {...mergePatternSlotProps<ManagementWorkbenchRegionSlotProps>(
            {
              'data-sdk-region': 'management-workbench-filters',
            },
            slotProps?.filters,
          )}
        >
          {filters}
        </div>
      ) : null}
      {selectionBar ? (
        <div
          {...mergePatternSlotProps<ManagementWorkbenchRegionSlotProps>(
            {
              'data-sdk-region': 'management-workbench-selection-bar',
            },
            slotProps?.selectionBar,
          )}
        >
          {selectionBar}
        </div>
      ) : null}
      <div
        {...mergePatternSlotProps<ManagementWorkbenchRegionSlotProps>(
          {
            className: cn('min-h-0 min-w-0', detail ? 'grid gap-4 xl:grid' : null),
            'data-sdk-region': 'management-workbench-content',
            style: detailGridStyle,
          },
          slotProps?.content,
        )}
      >
        <WorkspacePanel
          actions={mainActions}
          description={mainDescription}
          {...resolvedMainRootProps}
          slotProps={mainPanelSlotProps}
          title={mainTitle}
        >
          {children}
        </WorkspacePanel>
        {detail ? <InspectorRail {...detail} className={cn('h-full', detail.className)} /> : null}
      </div>
      {footer ? (
        <div
          {...mergePatternSlotProps<ManagementWorkbenchRegionSlotProps>(
            {
              'data-sdk-region': 'management-workbench-footer',
            },
            slotProps?.footer,
          )}
        >
          {footer}
        </div>
      ) : null}
    </div>
  );
});
ManagementWorkbench.displayName = 'ManagementWorkbench';
