import * as React from 'react';
import { cn } from '../../../lib/utils';
import {
  mergePatternSlotProps,
  type PatternSlotProps,
} from '../_internal/slot-props';
import { Panel, PanelGroup, PanelResizeHandle } from '../../ui/layout';
import type { PanelProps, PanelGroupProps } from '../../ui/layout/panel-group';
import {
  InspectorRail,
  type InspectorRailProps,
  type InspectorRailSlotProps,
} from './InspectorRail';
import {
  WorkspacePanel,
  type WorkspacePanelProps,
  type WorkspacePanelSlotProps,
} from './WorkspacePanel';

export type ListDetailWorkspaceDirection = NonNullable<PanelGroupProps['direction']>;
export type ListDetailWorkspaceRegionSlotProps = PatternSlotProps<
  Omit<React.ComponentPropsWithoutRef<'div'>, 'children'>
>;
export type ListDetailWorkspacePanelSlotProps = Omit<
  PanelProps,
  'children' | 'collapsedSize' | 'collapsible' | 'defaultSize' | 'maxSize' | 'minSize'
>;
export type ListDetailWorkspaceGroupSlotProps = Omit<PanelGroupProps, 'children' | 'direction'>;

interface ListDetailWorkspacePaneSizing {
  collapsedSize?: PanelProps['collapsedSize'];
  collapsible?: PanelProps['collapsible'];
  defaultSize?: PanelProps['defaultSize'];
  maxSize?: PanelProps['maxSize'];
  minSize?: PanelProps['minSize'];
}

export interface ListDetailWorkspacePaneSlotProps extends WorkspacePanelSlotProps {
  panel?: ListDetailWorkspacePanelSlotProps;
  region?: ListDetailWorkspaceRegionSlotProps;
}

export interface ListDetailWorkspaceDetailSlotProps extends InspectorRailSlotProps {
  panel?: ListDetailWorkspacePanelSlotProps;
  region?: ListDetailWorkspaceRegionSlotProps;
}

export interface ListDetailWorkspaceSidebarProps
  extends ListDetailWorkspacePaneSizing,
    Omit<WorkspacePanelProps, 'className' | 'slotProps'> {
  className?: string;
  slotProps?: ListDetailWorkspacePaneSlotProps;
}

export interface ListDetailWorkspaceContentProps
  extends ListDetailWorkspacePaneSizing,
    Omit<WorkspacePanelProps, 'className' | 'slotProps'> {
  className?: string;
  slotProps?: ListDetailWorkspacePaneSlotProps;
}

export interface ListDetailWorkspaceDetailProps
  extends ListDetailWorkspacePaneSizing,
    Omit<InspectorRailProps, 'className' | 'slotProps'> {
  className?: string;
  slotProps?: ListDetailWorkspaceDetailSlotProps;
}

export interface ListDetailWorkspaceSlotProps {
  panelGroup?: ListDetailWorkspaceGroupSlotProps;
}

export interface ListDetailWorkspaceProps extends Omit<React.HTMLAttributes<HTMLDivElement>, 'children' | 'content'> {
  content: ListDetailWorkspaceContentProps;
  detail?: ListDetailWorkspaceDetailProps;
  direction?: ListDetailWorkspaceDirection;
  resizeHandleMode?: 'line' | 'grip';
  sidebar?: ListDetailWorkspaceSidebarProps;
  slotProps?: ListDetailWorkspaceSlotProps;
}

function renderWorkspacePane(
  region: 'sidebar' | 'content',
  {
    actions,
    children,
    className,
    collapsedSize,
    collapsible,
    defaultSize,
    description,
    maxSize,
    minSize,
    slotProps,
    title,
    ...panelRootProps
  }: ListDetailWorkspaceSidebarProps | ListDetailWorkspaceContentProps,
  defaults: Pick<PanelProps, 'defaultSize' | 'minSize'>,
) {
  return (
    <Panel
      collapsedSize={collapsedSize}
      collapsible={collapsible}
      defaultSize={defaultSize ?? defaults.defaultSize}
      maxSize={maxSize}
      minSize={minSize ?? defaults.minSize}
      {...slotProps?.panel}
    >
      <div
        {...mergePatternSlotProps<ListDetailWorkspaceRegionSlotProps>(
          {
            className: 'h-full',
            'data-sdk-region': region,
          },
          slotProps?.region,
        )}
      >
        <WorkspacePanel
          actions={actions}
          className={cn('h-full', className)}
          description={description}
          slotProps={{
            body: slotProps?.body,
            header: slotProps?.header,
          }}
          title={title}
          {...panelRootProps}
        >
          {children}
        </WorkspacePanel>
      </div>
    </Panel>
  );
}

function renderDetailPane({
  children,
  className,
  collapsedSize,
  collapsible,
  defaultSize,
  footer,
  meta,
  side,
  stickyHeader,
  summary,
  maxSize,
  minSize,
  slotProps,
  title,
  variant,
  ...railRootProps
}: ListDetailWorkspaceDetailProps, defaults: Pick<PanelProps, 'defaultSize' | 'minSize'>) {
  return (
    <Panel
      collapsedSize={collapsedSize}
      collapsible={collapsible}
      defaultSize={defaultSize ?? defaults.defaultSize}
      maxSize={maxSize}
      minSize={minSize ?? defaults.minSize}
      {...slotProps?.panel}
    >
      <div
        {...mergePatternSlotProps<ListDetailWorkspaceRegionSlotProps>(
          {
            className: 'h-full',
            'data-sdk-region': 'detail',
          },
          slotProps?.region,
        )}
      >
        <InspectorRail
          className={cn('h-full', className)}
          footer={footer}
          meta={meta}
          side={side}
          slotProps={{
            body: slotProps?.body,
            footer: slotProps?.footer,
            header: slotProps?.header,
          }}
          stickyHeader={stickyHeader}
          summary={summary}
          title={title}
          variant={variant}
          {...railRootProps}
        >
          {children}
        </InspectorRail>
      </div>
    </Panel>
  );
}

export const ListDetailWorkspace = React.forwardRef<HTMLDivElement, ListDetailWorkspaceProps>(({
  className,
  content,
  detail,
  direction = 'horizontal',
  resizeHandleMode = 'grip',
  sidebar,
  slotProps,
  ...props
}, ref) => {
  const withHandle = resizeHandleMode === 'grip';
  const hasSidebar = Boolean(sidebar);
  const hasDetail = Boolean(detail);

  return (
    <div
      ref={ref}
      className={cn('h-full min-h-0 min-w-0', className)}
      data-sdk-pattern="list-detail-workspace"
      {...props}
    >
      <PanelGroup
        direction={direction}
        {...mergePatternSlotProps<ListDetailWorkspaceGroupSlotProps>(
          {
            className: 'h-full',
          },
          slotProps?.panelGroup,
        )}
      >
        {hasSidebar
          ? renderWorkspacePane('sidebar', sidebar!, {
              defaultSize: 24,
              minSize: 18,
            })
          : null}
        {hasSidebar ? <PanelResizeHandle withHandle={withHandle} /> : null}
        {renderWorkspacePane('content', content, {
          defaultSize: hasSidebar && hasDetail ? 52 : hasSidebar || hasDetail ? 72 : 100,
          minSize: 28,
        })}
        {hasDetail ? <PanelResizeHandle withHandle={withHandle} /> : null}
        {hasDetail
          ? renderDetailPane(detail!, {
              defaultSize: 24,
              minSize: 18,
            })
          : null}
      </PanelGroup>
    </div>
  );
});
ListDetailWorkspace.displayName = 'ListDetailWorkspace';
