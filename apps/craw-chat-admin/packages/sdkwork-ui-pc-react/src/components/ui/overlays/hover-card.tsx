import * as React from 'react';
import * as HoverCardPrimitive from '@radix-ui/react-hover-card';
import { cn } from '../../../lib/utils';

const HoverCard = HoverCardPrimitive.Root;

export type HoverCardProps = React.ComponentPropsWithoutRef<typeof HoverCardPrimitive.Root>;
export type HoverCardTriggerProps = React.ComponentPropsWithoutRef<typeof HoverCardPrimitive.Trigger>;
export type HoverCardContentProps = React.ComponentPropsWithoutRef<typeof HoverCardPrimitive.Content>;

const HoverCardTrigger = React.forwardRef<
  React.ElementRef<typeof HoverCardPrimitive.Trigger>,
  HoverCardTriggerProps
>(({ ...props }, ref) => (
  <HoverCardPrimitive.Trigger
    ref={ref}
    data-sdk-ui="hover-card-trigger"
    {...props}
  />
));

HoverCardTrigger.displayName = 'HoverCardTrigger';

const HoverCardContent = React.forwardRef<
  React.ElementRef<typeof HoverCardPrimitive.Content>,
  HoverCardContentProps
>(({ align = 'center', className, sideOffset = 8, ...props }, ref) => (
  <HoverCardPrimitive.Portal>
    <HoverCardPrimitive.Content
      align={align}
      className={cn(
        'z-50 w-80 rounded-[var(--sdk-radius-panel)] border border-[var(--sdk-color-border-default)] bg-[var(--sdk-color-surface-panel)] p-4 text-[var(--sdk-color-text-primary)] shadow-[var(--sdk-shadow-md)] outline-none',
        className,
      )}
      data-sdk-ui="hover-card-content"
      ref={ref}
      sideOffset={sideOffset}
      {...props}
    />
  </HoverCardPrimitive.Portal>
));

HoverCardContent.displayName = 'HoverCardContent';

export { HoverCard, HoverCardContent, HoverCardTrigger };
HoverCard.displayName = 'HoverCard';
