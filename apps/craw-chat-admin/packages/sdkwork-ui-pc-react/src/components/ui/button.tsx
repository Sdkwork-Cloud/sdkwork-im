import * as React from 'react';
import { Slot } from '@radix-ui/react-slot';
import { cva, type VariantProps } from 'class-variance-authority';
import { LoaderCircle } from 'lucide-react';
import { cn } from '../../lib/utils';

const buttonVariants = cva(
  'sdk-btn inline-flex items-center justify-center gap-2 whitespace-nowrap rounded-[var(--sdk-radius-control)] font-medium transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-[var(--sdk-color-border-focus)] focus-visible:ring-offset-2 focus-visible:ring-offset-[var(--sdk-color-surface-canvas)] disabled:pointer-events-none disabled:opacity-60',
  {
    variants: {
      variant: {
        primary:
          'sdk-btn--primary bg-[var(--sdk-color-brand-primary)] text-[var(--sdk-color-text-inverse)] hover:bg-[var(--sdk-color-brand-primary-hover)]',
        secondary:
          'sdk-btn--secondary bg-[var(--sdk-color-surface-panel-muted)] text-[var(--sdk-color-text-primary)] hover:bg-[var(--sdk-color-surface-elevated)]',
        outline:
          'sdk-btn--outline border border-[var(--sdk-color-border-default)] bg-transparent text-[var(--sdk-color-text-primary)] hover:bg-[var(--sdk-color-brand-primary-soft)]',
        ghost:
          'sdk-btn--ghost text-[var(--sdk-color-text-secondary)] hover:bg-[var(--sdk-color-brand-primary-soft)] hover:text-[var(--sdk-color-text-primary)]',
        danger:
          'sdk-btn--danger bg-[var(--sdk-color-state-danger)] text-[var(--sdk-color-text-inverse)] hover:opacity-90',
      },
      size: {
        sm: 'h-8 px-3 text-xs',
        default: 'h-10 px-4 text-sm',
        lg: 'h-11 px-5 text-sm',
        icon: 'h-10 w-10',
      },
    },
    defaultVariants: {
      variant: 'primary',
      size: 'default',
    },
  },
);

export interface ButtonProps
  extends React.ButtonHTMLAttributes<HTMLButtonElement>,
    VariantProps<typeof buttonVariants> {
  asChild?: boolean;
  loading?: boolean;
}

const Button = React.forwardRef<HTMLButtonElement, ButtonProps>(
  ({ asChild = false, className, children, disabled, loading = false, size, variant, ...props }, ref) => {
    const Comp = asChild ? Slot : 'button';

    return (
      <Comp
        ref={ref}
        className={cn(buttonVariants({ variant, size }), className)}
        data-loading={loading ? 'true' : undefined}
        data-sdk-ui="button"
        disabled={disabled || loading}
        {...props}
      >
        {loading ? <LoaderCircle className="h-4 w-4 animate-spin" /> : null}
        {children}
      </Comp>
    );
  },
);

Button.displayName = 'Button';

export { Button, buttonVariants };
