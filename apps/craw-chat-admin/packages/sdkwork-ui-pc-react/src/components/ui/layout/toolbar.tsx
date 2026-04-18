import * as React from 'react';
import { cn } from '../../../lib/utils';
import { Separator } from '../separator';

const horizontalAlignClassName = {
  start: 'justify-start',
  center: 'justify-center',
  end: 'justify-end',
} as const;

export interface ToolbarProps extends React.HTMLAttributes<HTMLDivElement> {
  align?: keyof typeof horizontalAlignClassName;
  orientation?: 'horizontal' | 'vertical';
  wrap?: boolean;
}

const Toolbar = React.forwardRef<HTMLDivElement, ToolbarProps>(
  (
    {
      align = 'start',
      'aria-orientation': ariaOrientation,
      className,
      orientation = 'horizontal',
      role = 'toolbar',
      wrap = true,
      ...props
    },
    ref,
  ) => (
    <div
      ref={ref}
      aria-orientation={ariaOrientation ?? orientation}
      className={cn(
        'flex min-h-11 w-full rounded-[var(--sdk-radius-control)] border border-[var(--sdk-color-border-default)] bg-[var(--sdk-color-surface-panel)] px-3 py-2 shadow-[var(--sdk-shadow-sm)]',
        orientation === 'horizontal'
          ? ['items-center', wrap ? 'flex-wrap' : 'flex-nowrap', horizontalAlignClassName[align]]
          : 'flex-col items-stretch',
        className,
      )}
      data-sdk-ui="toolbar"
      role={role}
      {...props}
    />
  ),
);

Toolbar.displayName = 'Toolbar';

export interface ToolbarGroupProps extends React.HTMLAttributes<HTMLDivElement> {
  orientation?: 'horizontal' | 'vertical';
  wrap?: boolean;
}

export type ToolbarSpacerProps = React.HTMLAttributes<HTMLDivElement>;
export type ToolbarSeparatorProps = React.ComponentPropsWithoutRef<typeof Separator>;

const ToolbarGroup = React.forwardRef<HTMLDivElement, ToolbarGroupProps>(
  ({ className, orientation = 'horizontal', wrap = true, ...props }, ref) => (
    <div
      ref={ref}
      className={cn(
        'flex gap-1.5',
        orientation === 'horizontal' ? ['items-center', wrap ? 'flex-wrap' : 'flex-nowrap'] : 'flex-col',
        className,
      )}
      data-sdk-ui="toolbar-group"
      {...props}
    />
  ),
);

ToolbarGroup.displayName = 'ToolbarGroup';

const ToolbarSpacer = React.forwardRef<HTMLDivElement, ToolbarSpacerProps>(
  ({ className, ...props }, ref) => <div ref={ref} className={cn('min-w-4 flex-1', className)} data-sdk-ui="toolbar-spacer" {...props} />,
);

ToolbarSpacer.displayName = 'ToolbarSpacer';

const ToolbarSeparator = React.forwardRef<
  React.ElementRef<typeof Separator>,
  ToolbarSeparatorProps
>(({ className, decorative = false, orientation = 'vertical', ...props }, ref) => (
  <Separator
    ref={ref}
    className={cn(orientation === 'vertical' ? 'mx-1 h-6' : 'my-1 w-full', className)}
    data-sdk-ui="toolbar-separator"
    decorative={decorative}
    orientation={orientation}
    {...props}
  />
));

ToolbarSeparator.displayName = 'ToolbarSeparator';

export { Toolbar, ToolbarGroup, ToolbarSeparator, ToolbarSpacer };
