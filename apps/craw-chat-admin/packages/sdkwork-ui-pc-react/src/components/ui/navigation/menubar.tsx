import * as React from 'react';
import * as MenubarPrimitive from '@radix-ui/react-menubar';
import { Check, ChevronRight, Circle } from 'lucide-react';
import { cn } from '../../../lib/utils';

export type MenubarProps = React.ComponentPropsWithoutRef<typeof MenubarPrimitive.Root>;
export type MenubarMenuProps = React.ComponentProps<typeof MenubarPrimitive.Menu>;
export type MenubarGroupProps = React.ComponentProps<typeof MenubarPrimitive.Group>;
export type MenubarPortalProps = React.ComponentProps<typeof MenubarPrimitive.Portal>;
export type MenubarSubProps = React.ComponentProps<typeof MenubarPrimitive.Sub>;
export type MenubarRadioGroupProps = React.ComponentPropsWithoutRef<typeof MenubarPrimitive.RadioGroup>;
export type MenubarTriggerProps = React.ComponentPropsWithoutRef<typeof MenubarPrimitive.Trigger>;

export interface MenubarSubTriggerProps extends React.ComponentPropsWithoutRef<typeof MenubarPrimitive.SubTrigger> {
  inset?: boolean;
}

export type MenubarSubContentProps = React.ComponentPropsWithoutRef<typeof MenubarPrimitive.SubContent>;
export type MenubarContentProps = React.ComponentPropsWithoutRef<typeof MenubarPrimitive.Content>;

export interface MenubarItemProps extends React.ComponentPropsWithoutRef<typeof MenubarPrimitive.Item> {
  inset?: boolean;
}

export type MenubarCheckboxItemProps = React.ComponentPropsWithoutRef<typeof MenubarPrimitive.CheckboxItem>;
export type MenubarRadioItemProps = React.ComponentPropsWithoutRef<typeof MenubarPrimitive.RadioItem>;

export interface MenubarLabelProps extends React.ComponentPropsWithoutRef<typeof MenubarPrimitive.Label> {
  inset?: boolean;
}

export type MenubarSeparatorProps = React.ComponentPropsWithoutRef<typeof MenubarPrimitive.Separator>;
export type MenubarShortcutProps = React.HTMLAttributes<HTMLSpanElement>;

const Menubar = React.forwardRef<
  React.ElementRef<typeof MenubarPrimitive.Root>,
  MenubarProps
>(({ className, ...props }, ref) => (
  <MenubarPrimitive.Root
    ref={ref}
    className={cn(
      'flex min-h-10 items-center gap-1 rounded-[var(--sdk-radius-control)] border border-[var(--sdk-color-border-default)] bg-[var(--sdk-color-surface-panel)] p-1 shadow-[var(--sdk-shadow-sm)]',
      className,
    )}
    data-sdk-ui="menubar"
    data-slot="menubar"
    {...props}
  />
));

Menubar.displayName = 'Menubar';

const MenubarMenu: typeof MenubarPrimitive.Menu = MenubarPrimitive.Menu;
const MenubarPortal: typeof MenubarPrimitive.Portal = MenubarPrimitive.Portal;
const MenubarSub: typeof MenubarPrimitive.Sub = MenubarPrimitive.Sub;

const MenubarGroup = React.forwardRef<
  React.ElementRef<typeof MenubarPrimitive.Group>,
  MenubarGroupProps
>(({ ...props }, ref) => (
  <MenubarPrimitive.Group
    ref={ref}
    data-sdk-ui="menubar-group"
    data-slot="menubar-group"
    {...props}
  />
));

MenubarGroup.displayName = 'MenubarGroup';

const MenubarRadioGroup = React.forwardRef<
  React.ElementRef<typeof MenubarPrimitive.RadioGroup>,
  MenubarRadioGroupProps
>(({ ...props }, ref) => (
  <MenubarPrimitive.RadioGroup
    ref={ref}
    data-sdk-ui="menubar-radio-group"
    data-slot="menubar-radio-group"
    {...props}
  />
));

MenubarRadioGroup.displayName = 'MenubarRadioGroup';

const MenubarTrigger = React.forwardRef<
  React.ElementRef<typeof MenubarPrimitive.Trigger>,
  MenubarTriggerProps
>(({ className, ...props }, ref) => (
  <MenubarPrimitive.Trigger
    ref={ref}
    className={cn(
      'inline-flex min-h-8 select-none items-center rounded-[var(--sdk-radius-control)] px-3 text-sm font-medium text-[var(--sdk-color-text-secondary)] outline-none transition-colors hover:bg-[var(--sdk-color-brand-primary-soft)] hover:text-[var(--sdk-color-text-primary)] focus:bg-[var(--sdk-color-brand-primary-soft)] data-[state=open]:bg-[var(--sdk-color-brand-primary-soft)] data-[state=open]:text-[var(--sdk-color-text-primary)]',
      className,
    )}
    data-sdk-ui="menubar-trigger"
    data-slot="menubar-trigger"
    {...props}
  />
));

MenubarTrigger.displayName = 'MenubarTrigger';

const MenubarSubTrigger = React.forwardRef<
  React.ElementRef<typeof MenubarPrimitive.SubTrigger>,
  MenubarSubTriggerProps
>(({ className, inset, children, ...props }, ref) => (
  <MenubarPrimitive.SubTrigger
    ref={ref}
    className={cn(
      'flex cursor-default select-none items-center gap-2 rounded-[var(--sdk-radius-field)] px-2 py-1.5 text-sm outline-none transition-colors focus:bg-[var(--sdk-color-brand-primary-soft)] data-[state=open]:bg-[var(--sdk-color-brand-primary-soft)]',
      inset && 'pl-8',
      className,
    )}
    data-sdk-ui="menubar-sub-trigger"
    data-slot="menubar-sub-trigger"
    {...props}
  >
    {children}
    <ChevronRight className="ml-auto h-4 w-4" />
  </MenubarPrimitive.SubTrigger>
));

MenubarSubTrigger.displayName = 'MenubarSubTrigger';

const menubarContentClassName =
  'z-50 min-w-[12rem] overflow-hidden rounded-[var(--sdk-radius-panel)] border border-[var(--sdk-color-border-default)] bg-[var(--sdk-color-surface-panel)] p-1 text-[var(--sdk-color-text-primary)] shadow-[var(--sdk-shadow-md)]';

const MenubarSubContent = React.forwardRef<
  React.ElementRef<typeof MenubarPrimitive.SubContent>,
  MenubarSubContentProps
>(({ className, ...props }, ref) => (
  <MenubarPrimitive.SubContent
    ref={ref}
    className={cn(menubarContentClassName, className)}
    data-sdk-ui="menubar-sub-content"
    data-slot="menubar-sub-content"
    {...props}
  />
));

MenubarSubContent.displayName = 'MenubarSubContent';

const MenubarContent = React.forwardRef<
  React.ElementRef<typeof MenubarPrimitive.Content>,
  MenubarContentProps
>(({ align = 'start', className, sideOffset = 8, ...props }, ref) => (
  <MenubarPrimitive.Portal>
    <MenubarPrimitive.Content
      ref={ref}
      align={align}
      sideOffset={sideOffset}
      className={cn(menubarContentClassName, className)}
      data-sdk-ui="menubar-content"
      data-slot="menubar-content"
      {...props}
    />
  </MenubarPrimitive.Portal>
));

MenubarContent.displayName = 'MenubarContent';

const MenubarItem = React.forwardRef<
  React.ElementRef<typeof MenubarPrimitive.Item>,
  MenubarItemProps
>(({ className, inset, ...props }, ref) => (
  <MenubarPrimitive.Item
    ref={ref}
    className={cn(
      'relative flex cursor-default select-none items-center gap-2 rounded-[var(--sdk-radius-field)] px-2 py-1.5 text-sm outline-none transition-colors focus:bg-[var(--sdk-color-brand-primary-soft)] focus:text-[var(--sdk-color-text-primary)] data-[disabled]:pointer-events-none data-[disabled]:opacity-50',
      inset && 'pl-8',
      className,
    )}
    data-sdk-ui="menubar-item"
    data-slot="menubar-item"
    {...props}
  />
));

MenubarItem.displayName = 'MenubarItem';

const MenubarCheckboxItem = React.forwardRef<
  React.ElementRef<typeof MenubarPrimitive.CheckboxItem>,
  MenubarCheckboxItemProps
>(({ checked, children, className, ...props }, ref) => (
  <MenubarPrimitive.CheckboxItem
    ref={ref}
    checked={checked}
    className={cn(
      'relative flex cursor-default select-none items-center gap-2 rounded-[var(--sdk-radius-field)] py-1.5 pl-8 pr-2 text-sm outline-none transition-colors focus:bg-[var(--sdk-color-brand-primary-soft)] data-[disabled]:pointer-events-none data-[disabled]:opacity-50',
      className,
    )}
    data-sdk-ui="menubar-checkbox-item"
    data-slot="menubar-checkbox-item"
    {...props}
  >
    <span className="absolute left-2 flex h-4 w-4 items-center justify-center">
      <MenubarPrimitive.ItemIndicator>
        <Check className="h-4 w-4" />
      </MenubarPrimitive.ItemIndicator>
    </span>
    {children}
  </MenubarPrimitive.CheckboxItem>
));

MenubarCheckboxItem.displayName = 'MenubarCheckboxItem';

const MenubarRadioItem = React.forwardRef<
  React.ElementRef<typeof MenubarPrimitive.RadioItem>,
  MenubarRadioItemProps
>(({ children, className, ...props }, ref) => (
  <MenubarPrimitive.RadioItem
    ref={ref}
    className={cn(
      'relative flex cursor-default select-none items-center gap-2 rounded-[var(--sdk-radius-field)] py-1.5 pl-8 pr-2 text-sm outline-none transition-colors focus:bg-[var(--sdk-color-brand-primary-soft)] data-[disabled]:pointer-events-none data-[disabled]:opacity-50',
      className,
    )}
    data-sdk-ui="menubar-radio-item"
    data-slot="menubar-radio-item"
    {...props}
  >
    <span className="absolute left-2 flex h-4 w-4 items-center justify-center">
      <MenubarPrimitive.ItemIndicator>
        <Circle className="h-2.5 w-2.5 fill-current" />
      </MenubarPrimitive.ItemIndicator>
    </span>
    {children}
  </MenubarPrimitive.RadioItem>
));

MenubarRadioItem.displayName = 'MenubarRadioItem';

const MenubarLabel = React.forwardRef<
  React.ElementRef<typeof MenubarPrimitive.Label>,
  MenubarLabelProps
>(({ className, inset, ...props }, ref) => (
  <MenubarPrimitive.Label
    ref={ref}
    className={cn('px-2 py-1.5 text-sm font-semibold text-[var(--sdk-color-text-secondary)]', inset && 'pl-8', className)}
    data-sdk-ui="menubar-label"
    data-slot="menubar-label"
    {...props}
  />
));

MenubarLabel.displayName = 'MenubarLabel';

const MenubarSeparator = React.forwardRef<
  React.ElementRef<typeof MenubarPrimitive.Separator>,
  MenubarSeparatorProps
>(({ className, ...props }, ref) => (
  <MenubarPrimitive.Separator
    ref={ref}
    className={cn('-mx-1 my-1 h-px bg-[var(--sdk-color-border-subtle)]', className)}
    data-sdk-ui="menubar-separator"
    data-slot="menubar-separator"
    {...props}
  />
));

MenubarSeparator.displayName = 'MenubarSeparator';

const MenubarShortcut = React.forwardRef<HTMLSpanElement, MenubarShortcutProps>(({ className, ...props }, ref) => (
  <span
    ref={ref}
    className={cn('ml-auto text-xs tracking-[0.14em] text-[var(--sdk-color-text-muted)]', className)}
    data-sdk-ui="menubar-shortcut"
    data-slot="menubar-shortcut"
    {...props}
  />
));

MenubarShortcut.displayName = 'MenubarShortcut';

export {
  Menubar,
  MenubarCheckboxItem,
  MenubarContent,
  MenubarGroup,
  MenubarItem,
  MenubarLabel,
  MenubarMenu,
  MenubarPortal,
  MenubarRadioGroup,
  MenubarRadioItem,
  MenubarSeparator,
  MenubarShortcut,
  MenubarSub,
  MenubarSubContent,
  MenubarSubTrigger,
  MenubarTrigger,
};
MenubarMenu.displayName = 'MenubarMenu';
MenubarPortal.displayName = 'MenubarPortal';
MenubarSub.displayName = 'MenubarSub';
