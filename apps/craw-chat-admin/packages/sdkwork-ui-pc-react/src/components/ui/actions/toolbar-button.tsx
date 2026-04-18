import * as React from 'react';
import { cva, type VariantProps } from 'class-variance-authority';
import { cn } from '../../../lib/utils';

const toolbarButtonVariants = cva(
  'inline-flex shrink-0 items-center justify-center gap-1.5 whitespace-nowrap rounded-[var(--sdk-radius-control)] border border-transparent bg-transparent font-medium text-[var(--sdk-color-text-secondary)] transition-colors outline-none focus-visible:ring-2 focus-visible:ring-[var(--sdk-color-border-focus)] focus-visible:ring-offset-2 focus-visible:ring-offset-[var(--sdk-color-surface-canvas)] disabled:pointer-events-none disabled:opacity-50 [&_svg]:shrink-0',
  {
    variants: {
      size: {
        compact: 'h-8 min-w-8 px-2 text-xs',
        default: 'h-9 min-w-9 px-2.5 text-sm',
      },
      pressed: {
        false:
          'hover:border-[var(--sdk-color-border-default)] hover:bg-[var(--sdk-color-surface-panel-muted)] hover:text-[var(--sdk-color-text-primary)]',
        true:
          'border-[var(--sdk-color-border-strong)] bg-[var(--sdk-color-brand-primary-soft)] text-[var(--sdk-color-brand-primary)] shadow-[var(--sdk-shadow-sm)]',
      },
    },
    defaultVariants: {
      size: 'default',
      pressed: false,
    },
  },
);

export interface ToolbarButtonProps
  extends Omit<React.ButtonHTMLAttributes<HTMLButtonElement>, 'size'>,
    VariantProps<typeof toolbarButtonVariants> {
  active?: boolean;
  preserveFocusOnMouseDown?: boolean;
  shortcut?: string;
}

function resolveToolbarButtonTitle(options: {
  ariaLabel?: string;
  shortcut?: string;
  title?: string;
}) {
  const { ariaLabel, shortcut, title } = options;

  if (!shortcut) {
    return title;
  }

  const baseTitle = title ?? ariaLabel;

  return baseTitle ? `${baseTitle} (${shortcut})` : shortcut;
}

const ToolbarButton = React.forwardRef<HTMLButtonElement, ToolbarButtonProps>(
  (
    {
      active,
      'aria-label': ariaLabel,
      className,
      onMouseDown,
      pressed,
      preserveFocusOnMouseDown = false,
      shortcut,
      size,
      title,
      type = 'button',
      ...props
    },
    ref,
  ) => {
    const resolvedPressed = pressed ?? active ?? false;
    const hasPressedState = pressed !== undefined || active !== undefined;
    const resolvedTitle = resolveToolbarButtonTitle({ ariaLabel, shortcut, title });

    const handleMouseDown = React.useCallback(
      (event: React.MouseEvent<HTMLButtonElement>) => {
        if (preserveFocusOnMouseDown && !props.disabled) {
          event.preventDefault();
        }

        onMouseDown?.(event);
      },
      [onMouseDown, preserveFocusOnMouseDown, props.disabled],
    );

    return (
      <button
        ref={ref}
        aria-label={ariaLabel}
        aria-pressed={hasPressedState ? resolvedPressed : undefined}
        className={cn(toolbarButtonVariants({ pressed: resolvedPressed, size }), className)}
        data-sdk-ui="toolbar-button"
        onMouseDown={handleMouseDown}
        title={resolvedTitle}
        type={type}
        {...props}
      />
    );
  },
);

ToolbarButton.displayName = 'ToolbarButton';

export { ToolbarButton, toolbarButtonVariants };
