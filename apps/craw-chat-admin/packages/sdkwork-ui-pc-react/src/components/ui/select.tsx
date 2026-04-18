import * as React from 'react';
import * as SelectPrimitive from '@radix-ui/react-select';
import { Check, ChevronDown, ChevronUp } from 'lucide-react';
import { cn } from '../../lib/utils';

const Select = SelectPrimitive.Root;

export type SelectProps = React.ComponentPropsWithoutRef<typeof SelectPrimitive.Root>;
export type SelectGroupProps = React.ComponentPropsWithoutRef<typeof SelectPrimitive.Group>;
export type SelectValueProps = React.ComponentPropsWithoutRef<typeof SelectPrimitive.Value>;
export type SelectTriggerProps = React.ComponentPropsWithoutRef<typeof SelectPrimitive.Trigger>;
export type SelectContentProps = React.ComponentPropsWithoutRef<typeof SelectPrimitive.Content>;
export type SelectLabelProps = React.ComponentPropsWithoutRef<typeof SelectPrimitive.Label>;
export type SelectItemProps = React.ComponentPropsWithoutRef<typeof SelectPrimitive.Item>;
export type SelectSeparatorProps = React.ComponentPropsWithoutRef<typeof SelectPrimitive.Separator>;

const SelectGroup = React.forwardRef<
  React.ElementRef<typeof SelectPrimitive.Group>,
  SelectGroupProps
>(({ ...props }, ref) => (
  <SelectPrimitive.Group
    ref={ref}
    data-sdk-ui="select-group"
    data-slot="select-group"
    {...props}
  />
));

SelectGroup.displayName = 'SelectGroup';

const SelectValue = React.forwardRef<
  React.ElementRef<typeof SelectPrimitive.Value>,
  SelectValueProps
>(({ ...props }, ref) => (
  <SelectPrimitive.Value
    ref={ref}
    data-sdk-ui="select-value"
    data-slot="select-value"
    {...props}
  />
));

SelectValue.displayName = 'SelectValue';

const SelectTrigger = React.forwardRef<
  React.ElementRef<typeof SelectPrimitive.Trigger>,
  SelectTriggerProps
>(({ className, children, ...props }, ref) => (
  <SelectPrimitive.Trigger
    ref={ref}
    className={cn(
      'flex h-10 w-full items-center justify-between gap-2 rounded-[var(--sdk-radius-field)] border border-[var(--sdk-color-border-default)] bg-[var(--sdk-color-surface-panel)] px-3 py-2 text-sm text-[var(--sdk-color-text-primary)] shadow-[var(--sdk-shadow-sm)] outline-none ring-offset-[var(--sdk-color-surface-canvas)] placeholder:text-[var(--sdk-color-text-muted)] focus:ring-2 focus:ring-[var(--sdk-color-border-focus)] disabled:cursor-not-allowed disabled:opacity-60',
      className,
    )}
    data-sdk-ui="select-trigger"
    data-slot="select-trigger"
    {...props}
  >
    {children}
    <SelectPrimitive.Icon asChild>
      <ChevronDown className="h-4 w-4 text-[var(--sdk-color-text-muted)]" />
    </SelectPrimitive.Icon>
  </SelectPrimitive.Trigger>
));

SelectTrigger.displayName = 'SelectTrigger';

const SelectContent = React.forwardRef<
  React.ElementRef<typeof SelectPrimitive.Content>,
  SelectContentProps
>(({ className, children, position = 'popper', ...props }, ref) => (
  <SelectPrimitive.Portal>
    <SelectPrimitive.Content
      ref={ref}
      position={position}
      className={cn(
        'relative z-50 min-w-[10rem] overflow-hidden rounded-[var(--sdk-radius-panel)] border border-[var(--sdk-color-border-default)] bg-[var(--sdk-color-surface-panel)] text-[var(--sdk-color-text-primary)] shadow-[var(--sdk-shadow-lg)]',
        className,
      )}
      data-sdk-ui="select-content"
      data-slot="select-content"
      {...props}
    >
      <SelectPrimitive.ScrollUpButton className="flex cursor-default items-center justify-center py-1">
        <ChevronUp className="h-4 w-4" />
      </SelectPrimitive.ScrollUpButton>
      <SelectPrimitive.Viewport className="p-1">{children}</SelectPrimitive.Viewport>
      <SelectPrimitive.ScrollDownButton className="flex cursor-default items-center justify-center py-1">
        <ChevronDown className="h-4 w-4" />
      </SelectPrimitive.ScrollDownButton>
    </SelectPrimitive.Content>
  </SelectPrimitive.Portal>
));

SelectContent.displayName = 'SelectContent';

const SelectLabel = React.forwardRef<
  React.ElementRef<typeof SelectPrimitive.Label>,
  SelectLabelProps
>(({ className, ...props }, ref) => (
  <SelectPrimitive.Label
    ref={ref}
    className={cn('px-2 py-1.5 text-xs font-semibold text-[var(--sdk-color-text-secondary)]', className)}
    data-sdk-ui="select-label"
    data-slot="select-label"
    {...props}
  />
));

SelectLabel.displayName = 'SelectLabel';

const SelectItem = React.forwardRef<
  React.ElementRef<typeof SelectPrimitive.Item>,
  SelectItemProps
>(({ className, children, ...props }, ref) => (
  <SelectPrimitive.Item
    ref={ref}
    className={cn(
      'relative flex w-full cursor-default select-none items-center rounded-[var(--sdk-radius-field)] py-2 pl-8 pr-2 text-sm outline-none transition-colors focus:bg-[var(--sdk-color-brand-primary-soft)] data-[disabled]:pointer-events-none data-[disabled]:opacity-50',
      className,
    )}
    data-sdk-ui="select-item"
    data-slot="select-item"
    {...props}
  >
    <span className="absolute left-2 flex h-3.5 w-3.5 items-center justify-center">
      <SelectPrimitive.ItemIndicator>
        <Check className="h-4 w-4" />
      </SelectPrimitive.ItemIndicator>
    </span>
    <SelectPrimitive.ItemText>{children}</SelectPrimitive.ItemText>
  </SelectPrimitive.Item>
));

SelectItem.displayName = 'SelectItem';

const SelectSeparator = React.forwardRef<
  React.ElementRef<typeof SelectPrimitive.Separator>,
  SelectSeparatorProps
>(({ className, ...props }, ref) => (
  <SelectPrimitive.Separator
    ref={ref}
    className={cn('mx-1 my-1 h-px bg-[var(--sdk-color-border-subtle)]', className)}
    data-sdk-ui="select-separator"
    data-slot="select-separator"
    {...props}
  />
));

SelectSeparator.displayName = 'SelectSeparator';

export {
  Select,
  SelectContent,
  SelectGroup,
  SelectItem,
  SelectLabel,
  SelectSeparator,
  SelectTrigger,
  SelectValue,
};
Select.displayName = 'Select';
