import * as React from 'react';
import * as LabelPrimitive from '@radix-ui/react-label';
import { cn } from '../../lib/utils';

export type LabelProps = React.ComponentPropsWithoutRef<typeof LabelPrimitive.Root>;

const Label = React.forwardRef<
  React.ElementRef<typeof LabelPrimitive.Root>,
  LabelProps
>(({ className, ...props }, ref) => {
  return (
    <LabelPrimitive.Root
      ref={ref}
      className={cn('text-sm font-medium text-[var(--sdk-color-text-primary)]', className)}
      data-sdk-ui="label"
      {...props}
    />
  );
});

Label.displayName = 'Label';

export { Label };
