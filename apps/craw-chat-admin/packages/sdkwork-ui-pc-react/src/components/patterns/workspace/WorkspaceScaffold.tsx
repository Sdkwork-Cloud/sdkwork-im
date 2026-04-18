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

export type WorkspaceScaffoldDirection = NonNullable<PanelGroupProps['direction']>;

interface WorkspaceScaffoldPaneSizing {
  collapsedSize?: PanelProps['collapsedSize'];
  collapsible?: PanelProps['collapsible'];
  defaultSize?: PanelProps['defaultSize'];
  maxSize?: PanelProps['maxSize'];
  minSize?: PanelProps['minSize'];
}

export type WorkspaceScaffoldRegionSlotProps = PatternSlotProps<
  Omit<React.ComponentPropsWithoutRef<'div'>, 'children'>
>;
export type WorkspaceScaffoldPanelSlotProps = Omit<
  PanelProps,
  'children' | 'collapsedSize' | 'collapsible' | 'defaultSize' | 'maxSize' | 'minSize'
>;
export type WorkspaceScaffoldPanelGroupSlotProps = Omit<PanelGroupProps, 'children' | 'direction'>;

export interface WorkspaceScaffoldPaneSlotProps extends WorkspacePanelSlotProps {
  panel?: WorkspaceScaffoldPanelSlotProps;
  region?: WorkspaceScaffoldRegionSlotProps;
}

export interface WorkspaceScaffoldInspectorSlotProps extends InspectorRailSlotProps {
  panel?: WorkspaceScaffoldPanelSlotProps;
  region?: WorkspaceScaffoldRegionSlotProps;
}

export interface WorkspaceScaffoldSidebarProps
  extends WorkspaceScaffoldPaneSizing,
    Omit<WorkspacePanelProps, 'className' | 'slotProps'> {
  className?: string;
  slotProps?: WorkspaceScaffoldPaneSlotProps;
}

export interface WorkspaceScaffoldMainProps
  extends WorkspaceScaffoldPaneSizing,
    Omit<WorkspacePanelProps, 'className' | 'slotProps'> {
  className?: string;
  slotProps?: WorkspaceScaffoldPaneSlotProps;
}

export interface WorkspaceScaffoldBottomProps
  extends WorkspaceScaffoldPaneSizing,
    Omit<WorkspacePanelProps, 'className' | 'slotProps'> {
  className?: string;
  slotProps?: WorkspaceScaffoldPaneSlotProps;
}

export interface WorkspaceScaffoldInspectorProps
  extends WorkspaceScaffoldPaneSizing,
    Omit<InspectorRailProps, 'className' | 'slotProps'> {
  className?: string;
  slotProps?: WorkspaceScaffoldInspectorSlotProps;
}

export interface WorkspaceScaffoldSlotProps {
  banner?: WorkspaceScaffoldRegionSlotProps;
  bottomGroup?: WorkspaceScaffoldPanelGroupSlotProps;
  header?: WorkspaceScaffoldRegionSlotProps;
  mainGroup?: WorkspaceScaffoldPanelGroupSlotProps;
  mainSplit?: WorkspaceScaffoldRegionSlotProps;
  statusBar?: WorkspaceScaffoldRegionSlotProps;
  toolbar?: WorkspaceScaffoldRegionSlotProps;
}

export interface WorkspaceScaffoldProps
  extends Omit<React.HTMLAttributes<HTMLDivElement>, 'children' | 'content' | 'title'> {
  banner?: React.ReactNode;
  bottomPanel?: WorkspaceScaffoldBottomProps;
  header?: React.ReactNode;
  inspector?: WorkspaceScaffoldInspectorProps;
  main: WorkspaceScaffoldMainProps;
  resizeHandleMode?: 'line' | 'grip';
  sidebar?: WorkspaceScaffoldSidebarProps;
  slotProps?: WorkspaceScaffoldSlotProps;
  statusBar?: React.ReactNode;
  toolbar?: React.ReactNode;
  workspaceDirection?: WorkspaceScaffoldDirection;
}

function renderWorkspacePane(
  region: 'sidebar' | 'main' | 'bottom',
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
    ...panelProps
  }: WorkspaceScaffoldSidebarProps | WorkspaceScaffoldMainProps | WorkspaceScaffoldBottomProps,
  defaults: Pick<PanelProps, 'defaultSize' | 'minSize'>,
) {
  const {
    panel: panelSlotProps,
    region: regionSlotProps,
    ...workspacePanelSlotProps
  } = slotProps ?? {};

  return (
    <Panel
      {...panelSlotProps}
      collapsedSize={collapsedSize}
      collapsible={collapsible}
      defaultSize={defaultSize ?? defaults.defaultSize}
      maxSize={maxSize}
      minSize={minSize ?? defaults.minSize}
    >
      <div
        {...mergePatternSlotProps<WorkspaceScaffoldRegionSlotProps>(
          {
            className: 'h-full',
            'data-sdk-region': region,
          },
          regionSlotProps,
        )}
      >
        <WorkspacePanel
          actions={actions}
          className={cn('h-full', className)}
          description={description}
          slotProps={workspacePanelSlotProps}
          title={title}
          {...panelProps}
        >
          {children}
        </WorkspacePanel>
      </div>
    </Panel>
  );
}

function renderInspectorPane(
  {
    actions,
    children,
    className,
    collapsedSize,
    collapsible,
    defaultSize,
    description,
    eyebrow,
    footer,
    maxSize,
    meta,
    minSize,
    side,
    slotProps,
    stickyHeader,
    summary,
    title,
    variant,
  }: WorkspaceScaffoldInspectorProps,
  defaults: Pick<PanelProps, 'defaultSize' | 'minSize'>,
) {
  const {
    panel: panelSlotProps,
    region: regionSlotProps,
    ...inspectorRailSlotProps
  } = slotProps ?? {};

  return (
    <Panel
      {...panelSlotProps}
      collapsedSize={collapsedSize}
      collapsible={collapsible}
      defaultSize={defaultSize ?? defaults.defaultSize}
      maxSize={maxSize}
      minSize={minSize ?? defaults.minSize}
    >
      <div
        {...mergePatternSlotProps<WorkspaceScaffoldRegionSlotProps>(
          {
            className: 'h-full',
            'data-sdk-region': 'inspector',
          },
          regionSlotProps,
        )}
      >
        <InspectorRail
          actions={actions}
          className={cn('h-full', className)}
          description={description}
          eyebrow={eyebrow}
          footer={footer}
          meta={meta}
          side={side}
          slotProps={inspectorRailSlotProps}
          stickyHeader={stickyHeader}
          summary={summary}
          title={title}
          variant={variant}
        >
          {children}
        </InspectorRail>
      </div>
    </Panel>
  );
}

export const WorkspaceScaffold = React.forwardRef<HTMLDivElement, WorkspaceScaffoldProps>(({
  banner,
  bottomPanel,
  className,
  header,
  inspector,
  main,
  resizeHandleMode = 'grip',
  sidebar,
  slotProps,
  statusBar,
  toolbar,
  workspaceDirection = 'horizontal',
  ...props
}, ref) => {
  const withHandle = resizeHandleMode === 'grip';
  const hasSidebar = Boolean(sidebar);
  const hasInspector = Boolean(inspector);
  const hasBottomPanel = Boolean(bottomPanel);

  const mainSplit = (
    <PanelGroup
      {...mergePatternSlotProps<WorkspaceScaffoldPanelGroupSlotProps>(
        {
          className: 'h-full',
        },
        slotProps?.mainGroup,
      )}
      direction={workspaceDirection}
    >
      {hasSidebar
        ? renderWorkspacePane('sidebar', sidebar!, {
            defaultSize: 22,
            minSize: 16,
          })
        : null}
      {hasSidebar ? <PanelResizeHandle withHandle={withHandle} /> : null}
      {renderWorkspacePane('main', main, {
        defaultSize: hasSidebar && hasInspector ? 56 : hasSidebar || hasInspector ? 72 : 100,
        minSize: 32,
      })}
      {hasInspector ? <PanelResizeHandle withHandle={withHandle} /> : null}
      {hasInspector
        ? renderInspectorPane(inspector!, {
            defaultSize: 22,
            minSize: 18,
          })
        : null}
    </PanelGroup>
  );

  return (
    <div
      ref={ref}
      className={cn('flex h-full min-h-0 min-w-0 flex-col gap-4', className)}
      data-sdk-pattern="workspace-scaffold"
      {...props}
    >
      {header ? (
        <div
          {...mergePatternSlotProps<WorkspaceScaffoldRegionSlotProps>(
            {
              'data-sdk-region': 'workspace-header',
            },
            slotProps?.header,
          )}
        >
          {header}
        </div>
      ) : null}
      {banner ? (
        <div
          {...mergePatternSlotProps<WorkspaceScaffoldRegionSlotProps>(
            {
              'data-sdk-region': 'workspace-banner',
            },
            slotProps?.banner,
          )}
        >
          {banner}
        </div>
      ) : null}
      {toolbar ? (
        <div
          {...mergePatternSlotProps<WorkspaceScaffoldRegionSlotProps>(
            {
              'data-sdk-region': 'workspace-toolbar',
            },
            slotProps?.toolbar,
          )}
        >
          {toolbar}
        </div>
      ) : null}
      <div className="min-h-0 min-w-0 flex-1">
        {hasBottomPanel ? (
          <PanelGroup
            {...mergePatternSlotProps<WorkspaceScaffoldPanelGroupSlotProps>(
              {
                className: 'h-full',
              },
              slotProps?.bottomGroup,
            )}
            direction="vertical"
          >
            <Panel defaultSize={72} minSize={34}>
              <div
                {...mergePatternSlotProps<WorkspaceScaffoldRegionSlotProps>(
                  {
                    className: 'h-full',
                    'data-sdk-region': 'workspace-main-split',
                  },
                  slotProps?.mainSplit,
                )}
              >
                {mainSplit}
              </div>
            </Panel>
            <PanelResizeHandle withHandle={withHandle} />
            {renderWorkspacePane('bottom', bottomPanel!, {
              defaultSize: 28,
              minSize: 18,
            })}
          </PanelGroup>
        ) : (
          <div
            {...mergePatternSlotProps<WorkspaceScaffoldRegionSlotProps>(
              {
                className: 'h-full',
                'data-sdk-region': 'workspace-main-split',
              },
              slotProps?.mainSplit,
            )}
          >
            {mainSplit}
          </div>
        )}
      </div>
      {statusBar ? (
        <div
          {...mergePatternSlotProps<WorkspaceScaffoldRegionSlotProps>(
            {
              'data-sdk-region': 'workspace-status-bar',
            },
            slotProps?.statusBar,
          )}
        >
          {statusBar}
        </div>
      ) : null}
    </div>
  );
});
WorkspaceScaffold.displayName = 'WorkspaceScaffold';
