import * as React from 'react';
import * as PopoverPrimitive from '@radix-ui/react-popover';
import { cn } from '../../lib/utils';

const Popover = PopoverPrimitive.Root;

export type PopoverProps = React.ComponentPropsWithoutRef<typeof PopoverPrimitive.Root>;
export type PopoverTriggerProps = React.ComponentPropsWithoutRef<typeof PopoverPrimitive.Trigger>;
export type PopoverAnchorProps = React.ComponentPropsWithoutRef<typeof PopoverPrimitive.Anchor>;
export type PopoverContentProps = React.ComponentPropsWithoutRef<typeof PopoverPrimitive.Content>;

const PopoverTrigger = React.forwardRef<
  React.ElementRef<typeof PopoverPrimitive.Trigger>,
  PopoverTriggerProps
>(({ ...props }, ref) => (
  <PopoverPrimitive.Trigger
    ref={ref}
    data-sdk-ui="popover-trigger"
    data-slot="popover-trigger"
    {...props}
  />
));

PopoverTrigger.displayName = 'PopoverTrigger';

const PopoverAnchor = React.forwardRef<
  React.ElementRef<typeof PopoverPrimitive.Anchor>,
  PopoverAnchorProps
>(({ ...props }, ref) => (
  <PopoverPrimitive.Anchor
    ref={ref}
    data-sdk-ui="popover-anchor"
    data-slot="popover-anchor"
    {...props}
  />
));

PopoverAnchor.displayName = 'PopoverAnchor';

const PopoverContent = React.forwardRef<
  React.ElementRef<typeof PopoverPrimitive.Content>,
  PopoverContentProps
>(({ align = 'center', className, sideOffset = 8, ...props }, ref) => (
  <PopoverPrimitive.Portal>
    <PopoverPrimitive.Content
      ref={ref}
      align={align}
      className={cn(
        'z-50 w-72 rounded-[var(--sdk-radius-panel)] border border-[var(--sdk-color-border-default)] bg-[var(--sdk-color-surface-panel)] p-4 text-[var(--sdk-color-text-primary)] shadow-[var(--sdk-shadow-md)] outline-none',
        className,
      )}
      data-sdk-ui="popover-content"
      data-slot="popover-content"
      sideOffset={sideOffset}
      {...props}
    />
  </PopoverPrimitive.Portal>
));

PopoverContent.displayName = 'PopoverContent';

export { Popover, PopoverAnchor, PopoverContent, PopoverTrigger };
Popover.displayName = 'Popover';
