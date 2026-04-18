import * as React from 'react';
import { cn } from '../../lib/utils';

export interface InputGroupProps extends React.HTMLAttributes<HTMLDivElement> {}

export interface InputGroupAddonProps extends React.HTMLAttributes<HTMLDivElement> {}

const InputGroup = React.forwardRef<HTMLDivElement, InputGroupProps>(
  ({ className, ...props }, ref) => (
    <div
      ref={ref}
      className={cn(
        'group/input-group flex w-full items-stretch overflow-hidden rounded-[var(--sdk-radius-field)] border border-[var(--sdk-color-border-default)] bg-[var(--sdk-color-surface-panel)] shadow-[var(--sdk-shadow-sm)] [&>[data-slot=input]]:rounded-none [&>[data-slot=input]]:border-0 [&>[data-slot=input]]:shadow-none [&>[data-slot=input]]:focus-visible:ring-0',
        className,
      )}
      data-sdk-ui="input-group"
      data-slot="input-group"
      {...props}
    />
  ),
);

const InputGroupAddon = React.forwardRef<HTMLDivElement, InputGroupAddonProps>(
  ({ className, ...props }, ref) => (
    <div
      ref={ref}
      className={cn(
        'inline-flex items-center border-r border-[var(--sdk-color-border-default)] bg-[var(--sdk-color-surface-panel-muted)] px-3 text-sm text-[var(--sdk-color-text-secondary)]',
        className,
      )}
      data-sdk-ui="input-group-addon"
      data-slot="input-group-addon"
      {...props}
    />
  ),
);

InputGroup.displayName = 'InputGroup';
InputGroupAddon.displayName = 'InputGroupAddon';

export {
  InputGroup,
  InputGroupAddon,
};
