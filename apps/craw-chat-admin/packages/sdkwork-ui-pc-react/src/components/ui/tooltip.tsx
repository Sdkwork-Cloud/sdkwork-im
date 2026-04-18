import * as React from 'react';
import * as TooltipPrimitive from '@radix-ui/react-tooltip';
import { cn } from '../../lib/utils';

const TooltipProvider = TooltipPrimitive.Provider;
const Tooltip = TooltipPrimitive.Root;

export type TooltipProviderProps = React.ComponentPropsWithoutRef<typeof TooltipPrimitive.Provider>;
export type TooltipProps = React.ComponentPropsWithoutRef<typeof TooltipPrimitive.Root>;
export type TooltipTriggerProps = React.ComponentPropsWithoutRef<typeof TooltipPrimitive.Trigger>;
export type TooltipContentProps = React.ComponentPropsWithoutRef<typeof TooltipPrimitive.Content>;

const TooltipTrigger = React.forwardRef<
  React.ElementRef<typeof TooltipPrimitive.Trigger>,
  TooltipTriggerProps
>(({ ...props }, ref) => (
  <TooltipPrimitive.Trigger
    ref={ref}
    data-sdk-ui="tooltip-trigger"
    {...props}
  />
));

TooltipTrigger.displayName = 'TooltipTrigger';

const TooltipContent = React.forwardRef<
  React.ElementRef<typeof TooltipPrimitive.Content>,
  TooltipContentProps
>(({ className, sideOffset = 6, ...props }, ref) => (
  <TooltipPrimitive.Portal>
    <TooltipPrimitive.Content
      ref={ref}
      sideOffset={sideOffset}
      className={cn(
        'z-50 overflow-hidden rounded-[var(--sdk-radius-control)] border border-[var(--sdk-color-border-default)] bg-[var(--sdk-color-surface-panel)] px-3 py-1.5 text-xs text-[var(--sdk-color-text-primary)] shadow-[var(--sdk-shadow-md)]',
        className,
      )}
      data-sdk-ui="tooltip-content"
      {...props}
    />
  </TooltipPrimitive.Portal>
));

TooltipContent.displayName = 'TooltipContent';

export { Tooltip, TooltipContent, TooltipProvider, TooltipTrigger };
Tooltip.displayName = 'Tooltip';
TooltipProvider.displayName = 'TooltipProvider';
