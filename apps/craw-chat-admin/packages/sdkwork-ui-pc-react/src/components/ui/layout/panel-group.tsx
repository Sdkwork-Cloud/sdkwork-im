import * as React from 'react';
import * as ResizablePanels from 'react-resizable-panels';
import { cn } from '../../../lib/utils';

export type PanelDirection = 'horizontal' | 'vertical';

const PanelGroupDirectionContext = React.createContext<PanelDirection>('horizontal');

export interface PanelGroupProps
  extends Omit<ResizablePanels.GroupProps, 'elementRef' | 'orientation'> {
  direction?: PanelDirection;
}

const PanelGroup = React.forwardRef<HTMLDivElement, PanelGroupProps>(
  ({ className, direction = 'horizontal', ...props }, ref) => (
    <PanelGroupDirectionContext.Provider value={direction}>
      <ResizablePanels.Group
        className={cn(
          'flex h-full min-h-0 w-full min-w-0 overflow-hidden rounded-[var(--sdk-radius-panel)] border border-[var(--sdk-color-border-default)] bg-[var(--sdk-color-surface-panel)] shadow-[var(--sdk-shadow-sm)] data-[panel-group-direction=vertical]:flex-col',
          className,
        )}
        data-panel-group-direction={direction}
        data-sdk-ui="panel-group"
        elementRef={ref}
        orientation={direction}
        {...props}
      />
    </PanelGroupDirectionContext.Provider>
  ),
);

PanelGroup.displayName = 'PanelGroup';

export interface PanelProps extends Omit<ResizablePanels.PanelProps, 'elementRef'> {}

const Panel = React.forwardRef<HTMLDivElement, PanelProps>(({ className, ...props }, ref) => (
  <ResizablePanels.Panel
    className={cn('min-h-0 min-w-0 overflow-hidden', className)}
    data-sdk-ui="panel"
    data-sdk-panel=""
    elementRef={ref}
    {...props}
  />
));

Panel.displayName = 'Panel';

export interface PanelResizeHandleProps
  extends Omit<ResizablePanels.SeparatorProps, 'elementRef'> {
  withHandle?: boolean;
}

const PanelResizeHandle = React.forwardRef<HTMLDivElement, PanelResizeHandleProps>(
  ({ className, children, withHandle = false, ...props }, ref) => {
    const direction = React.useContext(PanelGroupDirectionContext);

    return (
      <ResizablePanels.Separator
        className={cn(
          'group relative flex shrink-0 touch-none select-none items-center justify-center bg-transparent transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-[var(--sdk-color-border-focus)] focus-visible:ring-offset-2 focus-visible:ring-offset-[var(--sdk-color-surface-canvas)] hover:bg-[var(--sdk-color-border-strong)]',
          direction === 'horizontal' ? 'h-full w-2 cursor-col-resize' : 'h-2 w-full cursor-row-resize',
          className,
        )}
        data-panel-group-direction={direction}
        data-sdk-ui="panel-resize-handle"
        data-sdk-panel-resize-handle=""
        elementRef={ref}
        {...props}
      >
        {withHandle ? (
          <span
            className={cn(
              'pointer-events-none flex items-center justify-center rounded-full border border-[var(--sdk-color-border-default)] bg-[var(--sdk-color-surface-panel)] shadow-[var(--sdk-shadow-sm)]',
              direction === 'horizontal' ? 'h-10 w-5' : 'h-5 w-10',
            )}
            data-testid="sdk-panel-resize-handle-grip"
          >
            <span
              className={cn(
                'grid gap-0.5 text-[var(--sdk-color-text-muted)]',
                direction === 'horizontal' ? 'grid-flow-col' : 'grid-flow-row',
              )}
            >
              <span className="h-1 w-1 rounded-full bg-current" />
              <span className="h-1 w-1 rounded-full bg-current" />
              <span className="h-1 w-1 rounded-full bg-current" />
            </span>
          </span>
        ) : (
          <span
            aria-hidden="true"
            className={cn(
              'pointer-events-none absolute rounded-full bg-[var(--sdk-color-border-default)]',
              direction === 'horizontal' ? 'left-1/2 top-0 h-full w-px -translate-x-1/2' : 'left-0 top-1/2 h-px w-full -translate-y-1/2',
            )}
          />
        )}
        {children}
      </ResizablePanels.Separator>
    );
  },
);

PanelResizeHandle.displayName = 'PanelResizeHandle';

export { Panel, PanelGroup, PanelResizeHandle };
