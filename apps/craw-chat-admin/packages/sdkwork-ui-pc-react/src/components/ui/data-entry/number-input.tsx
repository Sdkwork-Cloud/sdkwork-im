import * as React from 'react';
import { ChevronDown, ChevronUp } from 'lucide-react';
import { composeRefs, useControllableState } from '../../../lib/core';
import { cn } from '../../../lib/utils';
import { mergeSlotProps, type SlotProps } from '../../../lib/slot-props';
import { inputBaseClassName } from '../input';

function clampValue(value: number, min?: number, max?: number) {
  if (typeof min === 'number' && value < min) {
    return min;
  }

  if (typeof max === 'number' && value > max) {
    return max;
  }

  return value;
}

function parseNumericConstraint(value: number | string | undefined) {
  if (typeof value === 'number' && Number.isFinite(value)) {
    return value;
  }

  if (typeof value === 'string') {
    const parsed = Number(value);

    if (Number.isFinite(parsed)) {
      return parsed;
    }
  }

  return undefined;
}

export type NumberInputValueChangeHandler = (value: number | null) => void;
export type NumberInputRootSlotProps = SlotProps<Omit<React.ComponentPropsWithoutRef<'div'>, 'children'>>;
export type NumberInputStepperSlotProps = SlotProps<Omit<React.ComponentPropsWithoutRef<'div'>, 'children'>>;
export type NumberInputStepperButtonSlotProps = SlotProps<Omit<React.ComponentPropsWithoutRef<'button'>, 'children'>>;

export interface NumberInputSlotProps {
  decrementButton?: NumberInputStepperButtonSlotProps;
  incrementButton?: NumberInputStepperButtonSlotProps;
  root?: NumberInputRootSlotProps;
  stepper?: NumberInputStepperSlotProps;
}

export interface NumberInputProps
  extends Omit<
    React.InputHTMLAttributes<HTMLInputElement>,
    'defaultValue' | 'onChange' | 'type' | 'value'
  > {
  defaultValue?: number;
  onChange?: React.ChangeEventHandler<HTMLInputElement>;
  onValueChange?: NumberInputValueChangeHandler;
  slotProps?: NumberInputSlotProps;
  value?: number | null;
}

const NumberInput = React.forwardRef<HTMLInputElement, NumberInputProps>(
  (
    {
      className,
      defaultValue,
      max,
      min,
      onChange,
      onValueChange,
      slotProps,
      step = 1,
      value,
      ...props
    },
    ref,
  ) => {
    const inputRef = React.useRef<HTMLInputElement>(null);
    const numericMin = parseNumericConstraint(min);
    const numericMax = parseNumericConstraint(max);
    const [currentValue, setCurrentValue] = useControllableState<number | null>({
      defaultValue: defaultValue ?? null,
      onChange: onValueChange,
      value: value === undefined ? undefined : value,
    });

    const handleChange = React.useCallback(
      (event: React.ChangeEvent<HTMLInputElement>) => {
        const nextValue = event.target.value.trim();

        onChange?.(event);

        if (nextValue === '') {
          setCurrentValue(null);
          return;
        }

        const parsed = Number(nextValue);

        if (!Number.isNaN(parsed)) {
          setCurrentValue(parsed);
        }
      },
      [onChange, setCurrentValue],
    );

    const handleStep = React.useCallback(
      (direction: 1 | -1) => {
        const stepValue = parseNumericConstraint(step);
        const safeStep = stepValue ?? 1;
        const current =
          typeof currentValue === 'number'
            ? currentValue
            : typeof defaultValue === 'number'
              ? defaultValue
              : 0;
        const next = clampValue(current + safeStep * direction, numericMin, numericMax);

        setCurrentValue(next);
      },
      [currentValue, defaultValue, numericMax, numericMin, setCurrentValue, step],
    );

    return (
      <div
        {...mergeSlotProps(
          {
            className: 'relative',
            'data-sdk-ui': 'number-input',
            'data-slot': 'number-input',
          },
          slotProps?.root,
        )}
      >
        <input
          ref={composeRefs(ref, inputRef)}
          className={cn(
            inputBaseClassName,
            'rounded-[var(--sdk-radius-field)] pr-11 [appearance:textfield] [&::-webkit-inner-spin-button]:appearance-none [&::-webkit-outer-spin-button]:appearance-none',
            className,
          )}
          data-sdk-ui="number-input-field"
          data-slot="number-input-input"
          inputMode="decimal"
          max={max}
          min={min}
          onChange={handleChange}
          step={step}
          type="number"
          value={currentValue ?? ''}
          {...props}
        />
        <div
          {...mergeSlotProps(
            {
              className:
                'absolute inset-y-1 right-1 flex w-8 flex-col overflow-hidden rounded-[var(--sdk-radius-control)] border border-[var(--sdk-color-border-default)] bg-[var(--sdk-color-surface-panel-muted)]',
              'data-sdk-ui': 'number-input-stepper',
              'data-slot': 'number-input-stepper',
            },
            slotProps?.stepper,
          )}
        >
          <button
            aria-label="Increase value"
            {...mergeSlotProps(
              {
                className:
                  'flex flex-1 items-center justify-center text-[var(--sdk-color-text-secondary)] transition-colors hover:bg-[var(--sdk-color-surface-elevated)] hover:text-[var(--sdk-color-text-primary)]',
                'data-slot': 'number-input-increment',
              },
              slotProps?.incrementButton,
            )}
            onClick={(event) => {
              slotProps?.incrementButton?.onClick?.(event);
              if (!event.defaultPrevented) {
                handleStep(1);
              }
            }}
            type="button"
          >
            <ChevronUp className="h-3.5 w-3.5" />
          </button>
          <button
            aria-label="Decrease value"
            {...mergeSlotProps(
              {
                className:
                  'flex flex-1 items-center justify-center border-t border-[var(--sdk-color-border-default)] text-[var(--sdk-color-text-secondary)] transition-colors hover:bg-[var(--sdk-color-surface-elevated)] hover:text-[var(--sdk-color-text-primary)]',
                'data-slot': 'number-input-decrement',
              },
              slotProps?.decrementButton,
            )}
            onClick={(event) => {
              slotProps?.decrementButton?.onClick?.(event);
              if (!event.defaultPrevented) {
                handleStep(-1);
              }
            }}
            type="button"
          >
            <ChevronDown className="h-3.5 w-3.5" />
          </button>
        </div>
      </div>
    );
  },
);

NumberInput.displayName = 'NumberInput';

export { NumberInput };
