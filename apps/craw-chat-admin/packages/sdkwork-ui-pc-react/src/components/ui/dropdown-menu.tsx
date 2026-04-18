import * as React from 'react';
import * as DropdownMenuPrimitive from '@radix-ui/react-dropdown-menu';
import { Check, ChevronRight, Circle } from 'lucide-react';
import { cn } from '../../lib/utils';

const DropdownMenu = DropdownMenuPrimitive.Root;
const DropdownMenuPortal = DropdownMenuPrimitive.Portal;
const DropdownMenuSub = DropdownMenuPrimitive.Sub;

export type DropdownMenuProps = React.ComponentPropsWithoutRef<typeof DropdownMenuPrimitive.Root>;
export type DropdownMenuTriggerProps = React.ComponentPropsWithoutRef<typeof DropdownMenuPrimitive.Trigger>;
export type DropdownMenuGroupProps = React.ComponentProps<typeof DropdownMenuPrimitive.Group>;
export type DropdownMenuPortalProps = React.ComponentProps<typeof DropdownMenuPrimitive.Portal>;
export type DropdownMenuSubProps = React.ComponentProps<typeof DropdownMenuPrimitive.Sub>;
export type DropdownMenuRadioGroupProps = React.ComponentPropsWithoutRef<typeof DropdownMenuPrimitive.RadioGroup>;
export interface DropdownMenuSubTriggerProps extends React.ComponentPropsWithoutRef<typeof DropdownMenuPrimitive.SubTrigger> {
  inset?: boolean;
}

export type DropdownMenuSubContentProps = React.ComponentPropsWithoutRef<typeof DropdownMenuPrimitive.SubContent>;
export type DropdownMenuContentProps = React.ComponentPropsWithoutRef<typeof DropdownMenuPrimitive.Content>;

export interface DropdownMenuItemProps extends React.ComponentPropsWithoutRef<typeof DropdownMenuPrimitive.Item> {
  inset?: boolean;
}

export type DropdownMenuCheckboxItemProps = React.ComponentPropsWithoutRef<typeof DropdownMenuPrimitive.CheckboxItem>;
export type DropdownMenuRadioItemProps = React.ComponentPropsWithoutRef<typeof DropdownMenuPrimitive.RadioItem>;

export interface DropdownMenuLabelProps extends React.ComponentPropsWithoutRef<typeof DropdownMenuPrimitive.Label> {
  inset?: boolean;
}

export type DropdownMenuSeparatorProps = React.ComponentPropsWithoutRef<typeof DropdownMenuPrimitive.Separator>;
export type DropdownMenuShortcutProps = React.HTMLAttributes<HTMLSpanElement>;

const DropdownMenuTrigger = React.forwardRef<
  React.ElementRef<typeof DropdownMenuPrimitive.Trigger>,
  DropdownMenuTriggerProps
>(({ ...props }, ref) => (
  <DropdownMenuPrimitive.Trigger
    ref={ref}
    data-sdk-ui="dropdown-menu-trigger"
    data-slot="dropdown-menu-trigger"
    {...props}
  />
));

DropdownMenuTrigger.displayName = 'DropdownMenuTrigger';

const DropdownMenuGroup = React.forwardRef<
  React.ElementRef<typeof DropdownMenuPrimitive.Group>,
  DropdownMenuGroupProps
>(({ ...props }, ref) => (
  <DropdownMenuPrimitive.Group
    ref={ref}
    data-sdk-ui="dropdown-menu-group"
    data-slot="dropdown-menu-group"
    {...props}
  />
));

DropdownMenuGroup.displayName = 'DropdownMenuGroup';

const DropdownMenuRadioGroup = React.forwardRef<
  React.ElementRef<typeof DropdownMenuPrimitive.RadioGroup>,
  DropdownMenuRadioGroupProps
>(({ ...props }, ref) => (
  <DropdownMenuPrimitive.RadioGroup
    ref={ref}
    data-sdk-ui="dropdown-menu-radio-group"
    data-slot="dropdown-menu-radio-group"
    {...props}
  />
));

DropdownMenuRadioGroup.displayName = 'DropdownMenuRadioGroup';

const DropdownMenuSubTrigger = React.forwardRef<
  React.ElementRef<typeof DropdownMenuPrimitive.SubTrigger>,
  DropdownMenuSubTriggerProps
>(({ className, inset, children, ...props }, ref) => (
  <DropdownMenuPrimitive.SubTrigger
    ref={ref}
    className={cn(
      'flex cursor-default select-none items-center gap-2 rounded-[var(--sdk-radius-field)] px-2 py-1.5 text-sm outline-none transition-colors focus:bg-[var(--sdk-color-brand-primary-soft)] data-[state=open]:bg-[var(--sdk-color-brand-primary-soft)]',
      inset && 'pl-8',
      className,
    )}
    data-sdk-ui="dropdown-menu-sub-trigger"
    data-slot="dropdown-menu-sub-trigger"
    {...props}
  >
    {children}
    <ChevronRight className="ml-auto h-4 w-4" />
  </DropdownMenuPrimitive.SubTrigger>
));

DropdownMenuSubTrigger.displayName = 'DropdownMenuSubTrigger';

const DropdownMenuSubContent = React.forwardRef<
  React.ElementRef<typeof DropdownMenuPrimitive.SubContent>,
  DropdownMenuSubContentProps
>(({ className, ...props }, ref) => (
  <DropdownMenuPrimitive.SubContent
    ref={ref}
    className={cn(
      'z-50 min-w-[10rem] overflow-hidden rounded-[var(--sdk-radius-panel)] border border-[var(--sdk-color-border-default)] bg-[var(--sdk-color-surface-panel)] p-1 text-[var(--sdk-color-text-primary)] shadow-[var(--sdk-shadow-md)]',
      className,
    )}
    data-sdk-ui="dropdown-menu-sub-content"
    data-slot="dropdown-menu-sub-content"
    {...props}
  />
));

DropdownMenuSubContent.displayName = 'DropdownMenuSubContent';

const DropdownMenuContent = React.forwardRef<
  React.ElementRef<typeof DropdownMenuPrimitive.Content>,
  DropdownMenuContentProps
>(({ className, sideOffset = 6, ...props }, ref) => (
  <DropdownMenuPrimitive.Portal>
    <DropdownMenuPrimitive.Content
      ref={ref}
      sideOffset={sideOffset}
      className={cn(
        'z-50 min-w-[10rem] overflow-hidden rounded-[var(--sdk-radius-panel)] border border-[var(--sdk-color-border-default)] bg-[var(--sdk-color-surface-panel)] p-1 text-[var(--sdk-color-text-primary)] shadow-[var(--sdk-shadow-md)]',
        className,
      )}
      data-sdk-ui="dropdown-menu-content"
      data-slot="dropdown-menu-content"
      {...props}
    />
  </DropdownMenuPrimitive.Portal>
));

DropdownMenuContent.displayName = 'DropdownMenuContent';

const DropdownMenuItem = React.forwardRef<
  React.ElementRef<typeof DropdownMenuPrimitive.Item>,
  DropdownMenuItemProps
>(({ className, inset, ...props }, ref) => (
  <DropdownMenuPrimitive.Item
    ref={ref}
    className={cn(
      'relative flex cursor-default select-none items-center gap-2 rounded-[var(--sdk-radius-field)] px-2 py-1.5 text-sm outline-none transition-colors focus:bg-[var(--sdk-color-brand-primary-soft)] focus:text-[var(--sdk-color-text-primary)] data-[disabled]:pointer-events-none data-[disabled]:opacity-50',
      inset && 'pl-8',
      className,
    )}
    data-sdk-ui="dropdown-menu-item"
    data-slot="dropdown-menu-item"
    {...props}
  />
));

DropdownMenuItem.displayName = 'DropdownMenuItem';

const DropdownMenuCheckboxItem = React.forwardRef<
  React.ElementRef<typeof DropdownMenuPrimitive.CheckboxItem>,
  DropdownMenuCheckboxItemProps
>(({ className, children, checked, ...props }, ref) => (
  <DropdownMenuPrimitive.CheckboxItem
    ref={ref}
    checked={checked}
    className={cn(
      'relative flex cursor-default select-none items-center gap-2 rounded-[var(--sdk-radius-field)] py-1.5 pl-8 pr-2 text-sm outline-none transition-colors focus:bg-[var(--sdk-color-brand-primary-soft)] data-[disabled]:pointer-events-none data-[disabled]:opacity-50',
      className,
    )}
    data-sdk-ui="dropdown-menu-checkbox-item"
    data-slot="dropdown-menu-checkbox-item"
    {...props}
  >
    <span className="absolute left-2 flex h-4 w-4 items-center justify-center">
      <DropdownMenuPrimitive.ItemIndicator>
        <Check className="h-4 w-4" />
      </DropdownMenuPrimitive.ItemIndicator>
    </span>
    {children}
  </DropdownMenuPrimitive.CheckboxItem>
));

DropdownMenuCheckboxItem.displayName = 'DropdownMenuCheckboxItem';

const DropdownMenuRadioItem = React.forwardRef<
  React.ElementRef<typeof DropdownMenuPrimitive.RadioItem>,
  DropdownMenuRadioItemProps
>(({ className, children, ...props }, ref) => (
  <DropdownMenuPrimitive.RadioItem
    ref={ref}
    className={cn(
      'relative flex cursor-default select-none items-center gap-2 rounded-[var(--sdk-radius-field)] py-1.5 pl-8 pr-2 text-sm outline-none transition-colors focus:bg-[var(--sdk-color-brand-primary-soft)] data-[disabled]:pointer-events-none data-[disabled]:opacity-50',
      className,
    )}
    data-sdk-ui="dropdown-menu-radio-item"
    data-slot="dropdown-menu-radio-item"
    {...props}
  >
    <span className="absolute left-2 flex h-4 w-4 items-center justify-center">
      <DropdownMenuPrimitive.ItemIndicator>
        <Circle className="h-2.5 w-2.5 fill-current" />
      </DropdownMenuPrimitive.ItemIndicator>
    </span>
    {children}
  </DropdownMenuPrimitive.RadioItem>
));

DropdownMenuRadioItem.displayName = 'DropdownMenuRadioItem';

const DropdownMenuLabel = React.forwardRef<
  React.ElementRef<typeof DropdownMenuPrimitive.Label>,
  DropdownMenuLabelProps
>(({ className, inset, ...props }, ref) => (
  <DropdownMenuPrimitive.Label
    ref={ref}
    className={cn(
      'px-2 py-1.5 text-sm font-semibold text-[var(--sdk-color-text-secondary)]',
      inset && 'pl-8',
      className,
    )}
    data-sdk-ui="dropdown-menu-label"
    data-slot="dropdown-menu-label"
    {...props}
  />
));

DropdownMenuLabel.displayName = 'DropdownMenuLabel';

const DropdownMenuSeparator = React.forwardRef<
  React.ElementRef<typeof DropdownMenuPrimitive.Separator>,
  DropdownMenuSeparatorProps
>(({ className, ...props }, ref) => (
  <DropdownMenuPrimitive.Separator
    ref={ref}
    className={cn('-mx-1 my-1 h-px bg-[var(--sdk-color-border-subtle)]', className)}
    data-sdk-ui="dropdown-menu-separator"
    data-slot="dropdown-menu-separator"
    {...props}
  />
));

DropdownMenuSeparator.displayName = 'DropdownMenuSeparator';

const DropdownMenuShortcut = React.forwardRef<HTMLSpanElement, DropdownMenuShortcutProps>(({
  className,
  ...props
}, ref) => (
  <span
    ref={ref}
    className={cn('ml-auto text-xs tracking-[0.14em] text-[var(--sdk-color-text-muted)]', className)}
    data-sdk-ui="dropdown-menu-shortcut"
    data-slot="dropdown-menu-shortcut"
    {...props}
  />
));

DropdownMenuShortcut.displayName = 'DropdownMenuShortcut';

export {
  DropdownMenu,
  DropdownMenuCheckboxItem,
  DropdownMenuContent,
  DropdownMenuGroup,
  DropdownMenuItem,
  DropdownMenuLabel,
  DropdownMenuPortal,
  DropdownMenuRadioGroup,
  DropdownMenuRadioItem,
  DropdownMenuSeparator,
  DropdownMenuShortcut,
  DropdownMenuSub,
  DropdownMenuSubContent,
  DropdownMenuSubTrigger,
  DropdownMenuTrigger,
};
DropdownMenu.displayName = 'DropdownMenu';
DropdownMenuPortal.displayName = 'DropdownMenuPortal';
DropdownMenuSub.displayName = 'DropdownMenuSub';
