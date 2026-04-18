import * as React from 'react';
import { cn } from '../../../lib/utils';

export interface SegmentedControlOption {
  disabled?: boolean;
  icon?: React.ReactNode;
  label: React.ReactNode;
  value: string;
}

export type SegmentedControlValueChangeHandler = (value: string) => void;

export interface SegmentedControlProps extends React.HTMLAttributes<HTMLDivElement> {
  fullWidth?: boolean;
  onValueChange: SegmentedControlValueChangeHandler;
  options: SegmentedControlOption[];
  orientation?: 'horizontal' | 'vertical';
  size?: 'default' | 'sm';
  value: string;
}

const sizeClassName: Record<NonNullable<SegmentedControlProps['size']>, string> = {
  default: 'min-h-9 px-3 text-sm',
  sm: 'min-h-8 px-2.5 text-xs',
};

const SegmentedControl = React.forwardRef<HTMLDivElement, SegmentedControlProps>(
  (
    {
      className,
      fullWidth = true,
      onValueChange,
      options,
      orientation = 'horizontal',
      size = 'default',
      value,
      ...props
    },
    ref,
  ) => (
    <div
      ref={ref}
      className={cn(
        'inline-flex rounded-[var(--sdk-radius-control)] border border-[var(--sdk-color-border-default)] bg-[var(--sdk-color-surface-panel-muted)] p-1',
        orientation === 'horizontal' ? 'flex-row items-center' : 'flex-col items-stretch',
        fullWidth ? 'w-full' : 'w-auto',
        className,
      )}
      data-orientation={orientation}
      data-sdk-ui="segmented-control"
      role="group"
      {...props}
    >
      {options.map((option) => {
        const active = option.value === value;

        return (
          <button
            aria-pressed={active}
            className={cn(
              'inline-flex items-center justify-center gap-2 rounded-[var(--sdk-radius-field)] font-medium transition-colors disabled:pointer-events-none disabled:opacity-50',
              sizeClassName[size],
              orientation === 'horizontal' && fullWidth ? 'flex-1' : null,
              active
                ? 'bg-[var(--sdk-color-surface-elevated)] text-[var(--sdk-color-text-primary)] shadow-[var(--sdk-shadow-sm)]'
                : 'text-[var(--sdk-color-text-secondary)] hover:bg-[var(--sdk-color-brand-primary-soft)] hover:text-[var(--sdk-color-text-primary)]',
            )}
            disabled={option.disabled}
            key={option.value}
            onClick={() => onValueChange(option.value)}
            type="button"
          >
            {option.icon ? <span className="shrink-0">{option.icon}</span> : null}
            <span className="truncate">{option.label}</span>
          </button>
        );
      })}
    </div>
  ),
);

SegmentedControl.displayName = 'SegmentedControl';

export { SegmentedControl };
