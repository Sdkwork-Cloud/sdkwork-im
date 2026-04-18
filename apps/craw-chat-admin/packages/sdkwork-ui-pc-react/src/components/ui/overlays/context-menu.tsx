import * as React from 'react';
import * as ContextMenuPrimitive from '@radix-ui/react-context-menu';
import { Check, ChevronRight, Circle } from 'lucide-react';
import { cn } from '../../../lib/utils';

const ContextMenu = ContextMenuPrimitive.Root;
const ContextMenuPortal = ContextMenuPrimitive.Portal;
const ContextMenuSub = ContextMenuPrimitive.Sub;

export type ContextMenuProps = React.ComponentPropsWithoutRef<typeof ContextMenuPrimitive.Root>;
export type ContextMenuTriggerProps = React.ComponentPropsWithoutRef<typeof ContextMenuPrimitive.Trigger>;
export type ContextMenuGroupProps = React.ComponentProps<typeof ContextMenuPrimitive.Group>;
export type ContextMenuPortalProps = React.ComponentProps<typeof ContextMenuPrimitive.Portal>;
export type ContextMenuSubProps = React.ComponentProps<typeof ContextMenuPrimitive.Sub>;
export type ContextMenuRadioGroupProps = React.ComponentPropsWithoutRef<typeof ContextMenuPrimitive.RadioGroup>;

export interface ContextMenuSubTriggerProps extends React.ComponentPropsWithoutRef<typeof ContextMenuPrimitive.SubTrigger> {
  inset?: boolean;
}

export type ContextMenuSubContentProps = React.ComponentPropsWithoutRef<typeof ContextMenuPrimitive.SubContent>;
export type ContextMenuContentProps = React.ComponentPropsWithoutRef<typeof ContextMenuPrimitive.Content>;

export interface ContextMenuItemProps extends React.ComponentPropsWithoutRef<typeof ContextMenuPrimitive.Item> {
  inset?: boolean;
}

export type ContextMenuCheckboxItemProps = React.ComponentPropsWithoutRef<typeof ContextMenuPrimitive.CheckboxItem>;
export type ContextMenuRadioItemProps = React.ComponentPropsWithoutRef<typeof ContextMenuPrimitive.RadioItem>;

export interface ContextMenuLabelProps extends React.ComponentPropsWithoutRef<typeof ContextMenuPrimitive.Label> {
  inset?: boolean;
}

export type ContextMenuSeparatorProps = React.ComponentPropsWithoutRef<typeof ContextMenuPrimitive.Separator>;
export type ContextMenuShortcutProps = React.HTMLAttributes<HTMLSpanElement>;

const ContextMenuTrigger = React.forwardRef<
  React.ElementRef<typeof ContextMenuPrimitive.Trigger>,
  ContextMenuTriggerProps
>(({ ...props }, ref) => (
  <ContextMenuPrimitive.Trigger
    ref={ref}
    data-sdk-ui="context-menu-trigger"
    data-slot="context-menu-trigger"
    {...props}
  />
));

ContextMenuTrigger.displayName = 'ContextMenuTrigger';

const ContextMenuGroup = React.forwardRef<
  React.ElementRef<typeof ContextMenuPrimitive.Group>,
  ContextMenuGroupProps
>(({ ...props }, ref) => (
  <ContextMenuPrimitive.Group
    ref={ref}
    data-sdk-ui="context-menu-group"
    data-slot="context-menu-group"
    {...props}
  />
));

ContextMenuGroup.displayName = 'ContextMenuGroup';

const ContextMenuRadioGroup = React.forwardRef<
  React.ElementRef<typeof ContextMenuPrimitive.RadioGroup>,
  ContextMenuRadioGroupProps
>(({ ...props }, ref) => (
  <ContextMenuPrimitive.RadioGroup
    ref={ref}
    data-sdk-ui="context-menu-radio-group"
    data-slot="context-menu-radio-group"
    {...props}
  />
));

ContextMenuRadioGroup.displayName = 'ContextMenuRadioGroup';

const ContextMenuSubTrigger = React.forwardRef<
  React.ElementRef<typeof ContextMenuPrimitive.SubTrigger>,
  ContextMenuSubTriggerProps
>(({ className, inset, children, ...props }, ref) => (
  <ContextMenuPrimitive.SubTrigger
    className={cn(
      'flex cursor-default select-none items-center rounded-[var(--sdk-radius-field)] px-2 py-2 text-sm outline-none transition-colors focus:bg-[var(--sdk-color-brand-primary-soft)] data-[state=open]:bg-[var(--sdk-color-brand-primary-soft)]',
      inset && 'pl-8',
      className,
    )}
    data-sdk-ui="context-menu-sub-trigger"
    data-slot="context-menu-sub-trigger"
    ref={ref}
    {...props}
  >
    {children}
    <ChevronRight className="ml-auto h-4 w-4" />
  </ContextMenuPrimitive.SubTrigger>
));

ContextMenuSubTrigger.displayName = 'ContextMenuSubTrigger';

const ContextMenuSubContent = React.forwardRef<
  React.ElementRef<typeof ContextMenuPrimitive.SubContent>,
  ContextMenuSubContentProps
>(({ className, ...props }, ref) => (
  <ContextMenuPrimitive.SubContent
    className={cn(
      'z-50 min-w-[12rem] overflow-hidden rounded-[var(--sdk-radius-panel)] border border-[var(--sdk-color-border-default)] bg-[var(--sdk-color-surface-panel)] p-1 text-[var(--sdk-color-text-primary)] shadow-[var(--sdk-shadow-lg)]',
      className,
    )}
    data-sdk-ui="context-menu-sub-content"
    data-slot="context-menu-sub-content"
    ref={ref}
    {...props}
  />
));

ContextMenuSubContent.displayName = 'ContextMenuSubContent';

const ContextMenuContent = React.forwardRef<
  React.ElementRef<typeof ContextMenuPrimitive.Content>,
  ContextMenuContentProps
>(({ className, ...props }, ref) => (
  <ContextMenuPrimitive.Portal>
    <ContextMenuPrimitive.Content
      className={cn(
        'z-50 min-w-[12rem] overflow-hidden rounded-[var(--sdk-radius-panel)] border border-[var(--sdk-color-border-default)] bg-[var(--sdk-color-surface-panel)] p-1 text-[var(--sdk-color-text-primary)] shadow-[var(--sdk-shadow-lg)]',
        className,
      )}
      data-sdk-ui="context-menu-content"
      data-slot="context-menu-content"
      ref={ref}
      {...props}
    />
  </ContextMenuPrimitive.Portal>
));

ContextMenuContent.displayName = 'ContextMenuContent';

const ContextMenuItem = React.forwardRef<
  React.ElementRef<typeof ContextMenuPrimitive.Item>,
  ContextMenuItemProps
>(({ className, inset, ...props }, ref) => (
  <ContextMenuPrimitive.Item
    className={cn(
      'relative flex cursor-default select-none items-center rounded-[var(--sdk-radius-field)] px-2 py-2 text-sm outline-none transition-colors focus:bg-[var(--sdk-color-brand-primary-soft)] data-[disabled]:pointer-events-none data-[disabled]:opacity-50',
      inset && 'pl-8',
      className,
    )}
    data-sdk-ui="context-menu-item"
    data-slot="context-menu-item"
    ref={ref}
    {...props}
  />
));

ContextMenuItem.displayName = 'ContextMenuItem';

const ContextMenuCheckboxItem = React.forwardRef<
  React.ElementRef<typeof ContextMenuPrimitive.CheckboxItem>,
  ContextMenuCheckboxItemProps
>(({ className, children, checked, ...props }, ref) => (
  <ContextMenuPrimitive.CheckboxItem
    checked={checked}
    className={cn(
      'relative flex cursor-default select-none items-center rounded-[var(--sdk-radius-field)] py-2 pl-8 pr-2 text-sm outline-none transition-colors focus:bg-[var(--sdk-color-brand-primary-soft)] data-[disabled]:pointer-events-none data-[disabled]:opacity-50',
      className,
    )}
    data-sdk-ui="context-menu-checkbox-item"
    data-slot="context-menu-checkbox-item"
    ref={ref}
    {...props}
  >
    <span className="absolute left-2 flex h-3.5 w-3.5 items-center justify-center">
      <ContextMenuPrimitive.ItemIndicator>
        <Check className="h-4 w-4" />
      </ContextMenuPrimitive.ItemIndicator>
    </span>
    {children}
  </ContextMenuPrimitive.CheckboxItem>
));

ContextMenuCheckboxItem.displayName = 'ContextMenuCheckboxItem';

const ContextMenuRadioItem = React.forwardRef<
  React.ElementRef<typeof ContextMenuPrimitive.RadioItem>,
  ContextMenuRadioItemProps
>(({ className, children, ...props }, ref) => (
  <ContextMenuPrimitive.RadioItem
    className={cn(
      'relative flex cursor-default select-none items-center rounded-[var(--sdk-radius-field)] py-2 pl-8 pr-2 text-sm outline-none transition-colors focus:bg-[var(--sdk-color-brand-primary-soft)] data-[disabled]:pointer-events-none data-[disabled]:opacity-50',
      className,
    )}
    data-sdk-ui="context-menu-radio-item"
    data-slot="context-menu-radio-item"
    ref={ref}
    {...props}
  >
    <span className="absolute left-2 flex h-3.5 w-3.5 items-center justify-center">
      <ContextMenuPrimitive.ItemIndicator>
        <Circle className="h-2 w-2 fill-current" />
      </ContextMenuPrimitive.ItemIndicator>
    </span>
    {children}
  </ContextMenuPrimitive.RadioItem>
));

ContextMenuRadioItem.displayName = 'ContextMenuRadioItem';

const ContextMenuLabel = React.forwardRef<
  React.ElementRef<typeof ContextMenuPrimitive.Label>,
  ContextMenuLabelProps
>(({ className, inset, ...props }, ref) => (
  <ContextMenuPrimitive.Label
    className={cn('px-2 py-1.5 text-xs font-semibold text-[var(--sdk-color-text-secondary)]', inset && 'pl-8', className)}
    data-sdk-ui="context-menu-label"
    data-slot="context-menu-label"
    ref={ref}
    {...props}
  />
));

ContextMenuLabel.displayName = 'ContextMenuLabel';

const ContextMenuSeparator = React.forwardRef<
  React.ElementRef<typeof ContextMenuPrimitive.Separator>,
  ContextMenuSeparatorProps
>(({ className, ...props }, ref) => (
  <ContextMenuPrimitive.Separator
    className={cn('mx-1 my-1 h-px bg-[var(--sdk-color-border-subtle)]', className)}
    data-sdk-ui="context-menu-separator"
    data-slot="context-menu-separator"
    ref={ref}
    {...props}
  />
));

ContextMenuSeparator.displayName = 'ContextMenuSeparator';

const ContextMenuShortcut = React.forwardRef<HTMLSpanElement, ContextMenuShortcutProps>(({ className, ...props }, ref) => (
  <span
    ref={ref}
    className={cn('ml-auto text-xs tracking-[0.08em] text-[var(--sdk-color-text-muted)]', className)}
    data-sdk-ui="context-menu-shortcut"
    data-slot="context-menu-shortcut"
    {...props}
  />
));

ContextMenuShortcut.displayName = 'ContextMenuShortcut';

export {
  ContextMenu,
  ContextMenuCheckboxItem,
  ContextMenuContent,
  ContextMenuGroup,
  ContextMenuItem,
  ContextMenuLabel,
  ContextMenuPortal,
  ContextMenuRadioGroup,
  ContextMenuRadioItem,
  ContextMenuSeparator,
  ContextMenuShortcut,
  ContextMenuSub,
  ContextMenuSubContent,
  ContextMenuSubTrigger,
  ContextMenuTrigger,
};
ContextMenu.displayName = 'ContextMenu';
ContextMenuPortal.displayName = 'ContextMenuPortal';
ContextMenuSub.displayName = 'ContextMenuSub';
