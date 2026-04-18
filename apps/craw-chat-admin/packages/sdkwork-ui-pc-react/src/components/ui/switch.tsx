import * as React from 'react';
import * as SwitchPrimitive from '@radix-ui/react-switch';
import { cn } from '../../lib/utils';

export type SwitchProps = React.ComponentPropsWithoutRef<typeof SwitchPrimitive.Root>;

const Switch = React.forwardRef<
  React.ElementRef<typeof SwitchPrimitive.Root>,
  SwitchProps
>(({ className, ...props }, ref) => {
  return (
    <SwitchPrimitive.Root
      ref={ref}
      className={cn(
        'peer inline-flex h-6 w-11 shrink-0 items-center rounded-full border border-transparent bg-[var(--sdk-color-border-strong)] transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-[var(--sdk-color-border-focus)] data-[state=checked]:bg-[var(--sdk-color-brand-primary)] disabled:cursor-not-allowed disabled:opacity-60',
        className,
      )}
      data-sdk-ui="switch"
      {...props}
    >
      <SwitchPrimitive.Thumb className="pointer-events-none block h-5 w-5 translate-x-0.5 rounded-full bg-white shadow-lg transition-transform data-[state=checked]:translate-x-5" />
    </SwitchPrimitive.Root>
  );
});

Switch.displayName = 'Switch';

export { Switch };
