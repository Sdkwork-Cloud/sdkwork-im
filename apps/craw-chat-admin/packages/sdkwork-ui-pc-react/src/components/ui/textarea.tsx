import * as React from 'react';
import { cn } from '../../lib/utils';

export interface TextareaProps extends React.TextareaHTMLAttributes<HTMLTextAreaElement> {}

const Textarea = React.forwardRef<HTMLTextAreaElement, TextareaProps>(({ className, ...props }, ref) => {
  return (
    <textarea
      ref={ref}
      className={cn(
        'flex min-h-24 w-full rounded-[var(--sdk-radius-field)] border border-[var(--sdk-color-border-default)] bg-[var(--sdk-color-surface-panel)] px-3 py-2 text-sm text-[var(--sdk-color-text-primary)] shadow-[var(--sdk-shadow-sm)] outline-none ring-offset-[var(--sdk-color-surface-canvas)] placeholder:text-[var(--sdk-color-text-muted)] focus-visible:ring-2 focus-visible:ring-[var(--sdk-color-border-focus)] disabled:cursor-not-allowed disabled:opacity-60',
        className,
      )}
      data-sdk-ui="textarea"
      {...props}
    />
  );
});

Textarea.displayName = 'Textarea';

export { Textarea };
