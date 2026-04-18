import * as React from 'react';
import { CalendarRange, ChevronDown } from 'lucide-react';
import {
  DayFlag,
  DayPicker,
  SelectionState,
  UI,
  type DateRange as DayPickerDateRange,
  type Matcher,
} from 'react-day-picker';
import { useControllableState } from '../../../lib/core';
import { mergeSlotProps, type SlotProps } from '../../../lib/slot-props';
import { cn } from '../../../lib/utils';
import { buttonVariants } from '../button';
import { inputBaseClassName } from '../input';
import { Label } from '../label';
import { Popover, PopoverContent, PopoverTrigger, type PopoverContentProps } from '../popover';
import { SegmentedControl } from './segmented-control';
import {
  DateInput,
  DateTimeInput,
  type DateInputProps,
  type TemporalInputType,
} from './date-input';

export interface DateRangeValue {
  end: string;
  start: string;
}

export interface DateRangePreset {
  description?: React.ReactNode;
  disabled?: boolean;
  label: React.ReactNode;
  range: Partial<DateRangeValue>;
  value: string;
}

export interface DateRangeSpanLimit {
  amount: number;
  unit: 'day' | 'hour';
}

export type DateRangeFieldInvalidBehavior = 'preserve' | 'swap';

export type DateRangeFieldInputProps = Omit<
  DateInputProps,
  'aria-invalid' | 'className' | 'defaultValue' | 'onChange' | 'type' | 'value'
>;

export type DateRangePresetValueChangeHandler = (value: string | null) => void;
export type DateRangeValidationChangeHandler = (message: string | null) => void;
export type DateRangeValueChangeHandler = (value: DateRangeValue) => void;
export type DateRangeValueValidationResolver = (value: DateRangeValue) => string | null;
export type DateRangePickerTriggerSlotProps = SlotProps<
  Omit<React.ComponentPropsWithoutRef<'button'>, 'children'>
>;
export type DateRangePickerContentSlotProps = SlotProps<Omit<PopoverContentProps, 'children'>>;
export type DateRangePickerCalendarSlotProps = SlotProps<
  Omit<React.ComponentPropsWithoutRef<'div'>, 'children'>
>;
export type DateRangePickerPresetsSlotProps = SlotProps<
  Omit<React.ComponentPropsWithoutRef<'div'>, 'children'>
>;
export type DateRangePickerFooterSlotProps = SlotProps<
  Omit<React.ComponentPropsWithoutRef<'div'>, 'children'>
>;
export type DateRangePickerErrorSlotProps = SlotProps<
  Omit<React.ComponentPropsWithoutRef<'p'>, 'children'>
>;

export interface DateRangePickerSlotProps {
  calendar?: DateRangePickerCalendarSlotProps;
  content?: DateRangePickerContentSlotProps;
  error?: DateRangePickerErrorSlotProps;
  footer?: DateRangePickerFooterSlotProps;
  presets?: DateRangePickerPresetsSlotProps;
  trigger?: DateRangePickerTriggerSlotProps;
}

export interface DateRangeFieldBaseProps
  extends Omit<React.HTMLAttributes<HTMLDivElement>, 'defaultValue' | 'onChange'> {
  actions?: React.ReactNode;
  defaultPresetValue?: string | null;
  defaultValue?: Partial<DateRangeValue>;
  disabled?: boolean;
  endInputProps?: DateRangeFieldInputProps;
  endLabel?: React.ReactNode;
  invalidBehavior?: DateRangeFieldInvalidBehavior;
  max?: string;
  maxSpan?: DateRangeSpanLimit;
  min?: string;
  mode?: TemporalInputType;
  onPresetValueChange?: DateRangePresetValueChangeHandler;
  onValidationChange?: DateRangeValidationChangeHandler;
  onValueChange?: DateRangeValueChangeHandler;
  presetLabel?: React.ReactNode;
  presetValue?: string | null;
  presets?: DateRangePreset[];
  startInputProps?: DateRangeFieldInputProps;
  startLabel?: React.ReactNode;
  validate?: DateRangeValueValidationResolver;
  value?: Partial<DateRangeValue>;
}

export interface DateRangeFieldProps extends Omit<DateRangeFieldBaseProps, 'mode'> {}

export interface DateTimeRangeFieldProps extends Omit<DateRangeFieldBaseProps, 'mode'> {}

export interface DateRangePickerProps
  extends Omit<React.HTMLAttributes<HTMLDivElement>, 'defaultValue' | 'onChange'> {
  actions?: React.ReactNode;
  defaultMonth?: Date;
  defaultPresetValue?: string | null;
  defaultValue?: Partial<DateRangeValue>;
  disabled?: boolean;
  max?: string;
  maxSpan?: DateRangeSpanLimit;
  min?: string;
  numberOfMonths?: number;
  onPresetValueChange?: DateRangePresetValueChangeHandler;
  onValidationChange?: DateRangeValidationChangeHandler;
  onValueChange?: DateRangeValueChangeHandler;
  placeholder?: React.ReactNode;
  presetLabel?: React.ReactNode;
  presetValue?: string | null;
  presets?: DateRangePreset[];
  slotProps?: DateRangePickerSlotProps;
  validate?: DateRangeValueValidationResolver;
  value?: Partial<DateRangeValue>;
}

interface BaseDateRangeFieldInternalProps extends DateRangeFieldBaseProps {
  forwardedRef?: React.ForwardedRef<HTMLDivElement>;
}

function normalizeDateRangeValue(value: Partial<DateRangeValue> | null | undefined): DateRangeValue {
  return {
    end: value?.end ?? '',
    start: value?.start ?? '',
  };
}

function isDateRangeValueEqual(left: DateRangeValue, right: DateRangeValue) {
  return left.start === right.start && left.end === right.end;
}

function findMatchingPresetValue(presets: DateRangePreset[] | undefined, value: DateRangeValue) {
  return (
    presets?.find((preset) => isDateRangeValueEqual(normalizeDateRangeValue(preset.range), value))?.value
    ?? null
  );
}

function parseTemporalValue(value: string, mode: TemporalInputType) {
  if (!value) {
    return null;
  }

  const normalizedValue = mode === 'date' ? `${value}T00:00` : value;
  const parsedValue = new Date(normalizedValue).getTime();

  return Number.isNaN(parsedValue) ? null : parsedValue;
}

function parseDateValue(value: string) {
  if (!value) {
    return undefined;
  }

  const [year, month, day] = value.split('-').map(Number);

  if (!year || !month || !day) {
    return undefined;
  }

  return new Date(year, month - 1, day);
}

function toDateValue(date: Date) {
  const year = date.getFullYear();
  const month = `${date.getMonth() + 1}`.padStart(2, '0');
  const day = `${date.getDate()}`.padStart(2, '0');

  return `${year}-${month}-${day}`;
}

function toDayPickerRange(value: DateRangeValue): DayPickerDateRange | undefined {
  const from = parseDateValue(value.start);
  const to = parseDateValue(value.end);

  if (!from && !to) {
    return undefined;
  }

  return {
    from,
    to,
  };
}

function toDateRangeValue(range: DayPickerDateRange | undefined): DateRangeValue {
  return {
    end: range?.to ? toDateValue(range.to) : '',
    start: range?.from ? toDateValue(range.from) : '',
  };
}

function formatDateLabel(dateValue: string) {
  const date = parseDateValue(dateValue);

  if (!date) {
    return '';
  }

  return new Intl.DateTimeFormat('en-US', {
    day: 'numeric',
    month: 'short',
    year: 'numeric',
  }).format(date);
}

function formatDateRangeLabel(value: DateRangeValue, placeholder: React.ReactNode) {
  if (!value.start && !value.end) {
    return placeholder;
  }

  if (value.start && value.end) {
    return `${formatDateLabel(value.start)} - ${formatDateLabel(value.end)}`;
  }

  if (value.start) {
    return `${formatDateLabel(value.start)} - Select end date`;
  }

  return placeholder;
}

function resolveCalendarMonth(
  value: DateRangeValue,
  defaultMonth: Date | undefined,
  min: string | undefined,
  max: string | undefined,
) {
  return parseDateValue(value.start)
    ?? defaultMonth
    ?? parseDateValue(min ?? '')
    ?? parseDateValue(max ?? '')
    ?? new Date();
}

function resolveRangeValidationMessage(
  value: DateRangeValue,
  mode: TemporalInputType,
  maxSpan: DateRangeSpanLimit | undefined,
  validate: DateRangeValueValidationResolver | undefined,
) {
  if (maxSpan && value.start && value.end) {
    const start = parseTemporalValue(value.start, mode);
    const end = parseTemporalValue(value.end, mode);

    if (start !== null && end !== null) {
      const spanLimitMs = maxSpan.amount * (maxSpan.unit === 'day' ? 24 * 60 * 60 * 1000 : 60 * 60 * 1000);
      const rangeSpan = Math.abs(end - start);

      if (rangeSpan > spanLimitMs) {
        return `Range cannot exceed ${maxSpan.amount} ${maxSpan.unit}${maxSpan.amount === 1 ? '' : 's'}`;
      }
    }
  }

  return validate?.(value) ?? null;
}

function resolveNextDateRangeValue(
  currentValue: DateRangeValue,
  nextKey: keyof DateRangeValue,
  nextPartValue: string,
  invalidBehavior: DateRangeFieldInvalidBehavior,
) {
  const nextValue = {
    ...currentValue,
    [nextKey]: nextPartValue,
  };

  if (invalidBehavior === 'swap' && nextValue.start && nextValue.end && nextValue.start > nextValue.end) {
    return {
      end: nextValue.start,
      start: nextValue.end,
    };
  }

  return nextValue;
}

function useDateRangeState({
  defaultPresetValue,
  defaultValue,
  onPresetValueChange,
  onValueChange,
  presetValue,
  presets,
  value,
}: {
  defaultPresetValue?: string | null;
  defaultValue?: Partial<DateRangeValue>;
  onPresetValueChange?: DateRangePresetValueChangeHandler;
  onValueChange?: DateRangeValueChangeHandler;
  presetValue?: string | null;
  presets?: DateRangePreset[];
  value?: Partial<DateRangeValue>;
}) {
  const initialValue = React.useMemo(() => normalizeDateRangeValue(defaultValue), [defaultValue]);
  const controlledValue = value === undefined ? undefined : normalizeDateRangeValue(value);
  const suppressPresetCallbackRef = React.useRef(false);
  const [currentValue, setCurrentValue] = useControllableState<DateRangeValue>({
    defaultValue: initialValue,
    onChange: onValueChange,
    value: controlledValue,
  });
  const [currentPresetValue, setCurrentPresetValue] = useControllableState<string | null>({
    defaultValue: defaultPresetValue ?? findMatchingPresetValue(presets, initialValue),
    onChange: (nextPresetValue) => {
      if (!suppressPresetCallbackRef.current) {
        onPresetValueChange?.(nextPresetValue);
      }
    },
    value: presetValue,
  });

  React.useEffect(() => {
    if (presetValue !== undefined || value === undefined) {
      return;
    }

    const nextPresetValue = findMatchingPresetValue(presets, currentValue);

    suppressPresetCallbackRef.current = true;
    setCurrentPresetValue(nextPresetValue);
    suppressPresetCallbackRef.current = false;
  }, [currentValue, presetValue, presets, setCurrentPresetValue, value]);

  const commitValue = React.useCallback(
    (nextValue: DateRangeValue, nextPresetValue: string | null) => {
      setCurrentValue(nextValue);
      setCurrentPresetValue(nextPresetValue);
    },
    [setCurrentPresetValue, setCurrentValue],
  );

  return {
    commitValue,
    currentPresetValue,
    currentValue,
  };
}

function BaseDateRangeField({
  actions,
  className,
  defaultPresetValue,
  defaultValue,
  disabled = false,
  endInputProps,
  endLabel = 'End date',
  forwardedRef,
  invalidBehavior = 'preserve',
  max,
  maxSpan,
  min,
  mode = 'date',
  onPresetValueChange,
  onValidationChange,
  onValueChange,
  presetLabel = 'Range presets',
  presetValue,
  presets,
  startInputProps,
  startLabel = 'Start date',
  validate,
  value,
  ...props
}: BaseDateRangeFieldInternalProps) {
  const { commitValue, currentPresetValue, currentValue } = useDateRangeState({
    defaultPresetValue,
    defaultValue,
    onPresetValueChange,
    onValueChange,
    presetValue,
    presets,
    value,
  });
  const validationMessage = resolveRangeValidationMessage(currentValue, mode, maxSpan, validate);
  const startInputId = React.useId();
  const endInputId = React.useId();
  const startFieldId = startInputProps?.id ?? startInputId;
  const endFieldId = endInputProps?.id ?? endInputId;
  const TemporalField = mode === 'datetime-local' ? DateTimeInput : DateInput;

  React.useEffect(() => {
    onValidationChange?.(validationMessage);
  }, [onValidationChange, validationMessage]);

  function handlePresetValueChange(nextPresetValue: string) {
    const preset = presets?.find((item) => item.value === nextPresetValue);

    if (!preset) {
      return;
    }

    commitValue(normalizeDateRangeValue(preset.range), preset.value);
  }

  function handleFieldChange(nextKey: keyof DateRangeValue, nextPartValue: string) {
    const nextValue = resolveNextDateRangeValue(currentValue, nextKey, nextPartValue, invalidBehavior);
    commitValue(nextValue, findMatchingPresetValue(presets, nextValue));
  }

  return (
    <div
      ref={forwardedRef}
      className={cn('space-y-3', className)}
      data-sdk-ui="date-range-field"
      data-slot="date-range-field"
      {...props}
    >
      {presets?.length ? (
        <div
          className="space-y-2"
          data-slot="date-range-field-presets"
        >
          <div className="text-xs font-medium uppercase tracking-[0.18em] text-[var(--sdk-color-text-muted)]">
            {presetLabel}
          </div>
          <SegmentedControl
            fullWidth={false}
            onValueChange={handlePresetValueChange}
            options={presets.map((preset) => ({
              disabled: disabled || preset.disabled,
              label: preset.label,
              value: preset.value,
            }))}
            size="sm"
            value={currentPresetValue ?? ''}
          />
        </div>
      ) : null}
      <div
        className="grid gap-3 md:grid-cols-[minmax(0,1fr)_auto_minmax(0,1fr)] md:items-end"
        data-slot="date-range-field-inputs"
      >
        <div
          className="space-y-2"
          data-slot="date-range-field-start"
        >
          <Label htmlFor={startFieldId}>{startLabel}</Label>
          <TemporalField
            {...startInputProps}
            aria-invalid={validationMessage ? 'true' : undefined}
            disabled={disabled || startInputProps?.disabled}
            id={startFieldId}
            max={max}
            min={min}
            value={currentValue.start}
            onChange={(event) => handleFieldChange('start', event.target.value)}
          />
        </div>
        <div className="hidden pb-2 text-sm text-[var(--sdk-color-text-muted)] md:block">to</div>
        <div
          className="space-y-2"
          data-slot="date-range-field-end"
        >
          <Label htmlFor={endFieldId}>{endLabel}</Label>
          <TemporalField
            {...endInputProps}
            aria-invalid={validationMessage ? 'true' : undefined}
            disabled={disabled || endInputProps?.disabled}
            id={endFieldId}
            max={max}
            min={min}
            value={currentValue.end}
            onChange={(event) => handleFieldChange('end', event.target.value)}
          />
        </div>
      </div>
      {actions ? (
        <div
          className="flex flex-wrap items-center justify-end gap-2"
          data-slot="date-range-field-actions"
        >
          {actions}
        </div>
      ) : null}
      {validationMessage ? (
        <p
          className="text-sm text-[var(--sdk-color-state-danger)]"
          data-slot="date-range-field-error"
          role="alert"
        >
          {validationMessage}
        </p>
      ) : null}
    </div>
  );
}

const calendarClassNames = {
  [DayFlag.disabled]:
    'text-[var(--sdk-color-text-muted)] opacity-40',
  [DayFlag.outside]:
    'text-[var(--sdk-color-text-muted)] opacity-35',
  [DayFlag.today]:
    'text-[var(--sdk-color-brand-primary)]',
  [SelectionState.range_end]:
    'rounded-[var(--sdk-radius-control)] bg-[var(--sdk-color-brand-primary)] text-[var(--sdk-color-text-inverse)]',
  [SelectionState.range_middle]:
    'bg-[var(--sdk-color-brand-primary-soft)] text-[var(--sdk-color-text-primary)]',
  [SelectionState.range_start]:
    'rounded-[var(--sdk-radius-control)] bg-[var(--sdk-color-brand-primary)] text-[var(--sdk-color-text-inverse)]',
  [SelectionState.selected]:
    'bg-[var(--sdk-color-brand-primary-soft)] text-[var(--sdk-color-text-primary)]',
  [UI.CaptionLabel]: 'text-sm font-semibold text-[var(--sdk-color-text-primary)]',
  [UI.Day]:
    'flex h-10 w-10 items-center justify-center text-sm [&:has([aria-selected=true])]:bg-[var(--sdk-color-brand-primary-soft)] [&:has([aria-selected=true])]:text-[var(--sdk-color-text-primary)]',
  [UI.DayButton]: cn(
    buttonVariants({ size: 'sm', variant: 'ghost' }),
    'h-10 w-10 rounded-[var(--sdk-radius-control)] p-0 font-normal aria-selected:opacity-100',
  ),
  [UI.Month]:
    'space-y-3 rounded-[var(--sdk-radius-field)] border border-[var(--sdk-color-border-subtle)] bg-[var(--sdk-color-surface-panel)] p-3',
  [UI.MonthCaption]: 'relative flex items-center justify-center pt-1',
  [UI.MonthGrid]: 'w-full border-collapse',
  [UI.Months]: 'flex flex-col gap-3 md:flex-row',
  [UI.Nav]: 'absolute inset-x-0 top-1 flex items-center justify-between',
  [UI.NextMonthButton]: cn(
    buttonVariants({ size: 'sm', variant: 'ghost' }),
    'h-8 w-8 rounded-[var(--sdk-radius-control)] p-0',
  ),
  [UI.PreviousMonthButton]: cn(
    buttonVariants({ size: 'sm', variant: 'ghost' }),
    'h-8 w-8 rounded-[var(--sdk-radius-control)] p-0',
  ),
  [UI.Week]: 'mt-1 flex w-full',
  [UI.Weekday]:
    'w-10 text-[11px] font-medium uppercase tracking-[0.12em] text-[var(--sdk-color-text-muted)]',
  [UI.Weekdays]: 'mb-1 flex',
  [UI.Weeks]: 'space-y-1',
} as const;

const DateRangeField = React.forwardRef<HTMLDivElement, DateRangeFieldProps>((props, ref) => (
  <BaseDateRangeField
    {...props}
    forwardedRef={ref}
    mode="date"
  />
));

DateRangeField.displayName = 'DateRangeField';

const DateTimeRangeField = React.forwardRef<HTMLDivElement, DateTimeRangeFieldProps>((props, ref) => (
  <BaseDateRangeField
    {...props}
    data-sdk-ui="date-time-range-field"
    forwardedRef={ref}
    mode="datetime-local"
  />
));

DateTimeRangeField.displayName = 'DateTimeRangeField';

const DateRangePicker = React.forwardRef<HTMLDivElement, DateRangePickerProps>(
  (
    {
      actions,
      className,
      defaultMonth,
      defaultPresetValue,
      defaultValue,
      disabled = false,
      max,
      maxSpan,
      min,
      numberOfMonths = 2,
      onPresetValueChange,
      onValidationChange,
      onValueChange,
      placeholder = 'Select date range',
      presetLabel = 'Range presets',
      presetValue,
      presets,
      slotProps,
      validate,
      value,
      ...props
    },
    ref,
  ) => {
    const { commitValue, currentPresetValue, currentValue } = useDateRangeState({
      defaultPresetValue,
      defaultValue,
      onPresetValueChange,
      onValueChange,
      presetValue,
      presets,
      value,
    });
    const [open, setOpen] = React.useState(false);
    const validationMessage = resolveRangeValidationMessage(currentValue, 'date', maxSpan, validate);
    const calendarValue = toDayPickerRange(currentValue);
    const derivedCalendarMonth = React.useMemo(
      () => resolveCalendarMonth(currentValue, defaultMonth, min, max),
      [currentValue, defaultMonth, max, min],
    );
    const [displayMonth, setDisplayMonth] = React.useState(derivedCalendarMonth);
    const disabledDays = React.useMemo(() => {
      const matchers: Matcher[] = [];
      const minDate = parseDateValue(min ?? '');
      const maxDate = parseDateValue(max ?? '');

      if (minDate) {
        matchers.push({ before: minDate });
      }

      if (maxDate) {
        matchers.push({ after: maxDate });
      }

      return matchers.length > 0 ? matchers : undefined;
    }, [max, min]);

    React.useEffect(() => {
      setDisplayMonth(derivedCalendarMonth);
    }, [derivedCalendarMonth]);

    React.useEffect(() => {
      onValidationChange?.(validationMessage);
    }, [onValidationChange, validationMessage]);

    function handlePresetSelection(nextPresetValue: string) {
      const preset = presets?.find((item) => item.value === nextPresetValue);

      if (!preset) {
        return;
      }

      const nextValue = normalizeDateRangeValue(preset.range);

      commitValue(nextValue, preset.value);
      setDisplayMonth(resolveCalendarMonth(nextValue, defaultMonth, min, max));
      setOpen(false);
    }

    function handleCalendarSelection(nextRange: DayPickerDateRange | undefined) {
      const nextValue = toDateRangeValue(nextRange);

      commitValue(nextValue, findMatchingPresetValue(presets, nextValue));
      setDisplayMonth(nextRange?.from ?? resolveCalendarMonth(nextValue, defaultMonth, min, max));

      if (nextRange?.from && nextRange?.to) {
        setOpen(false);
      }
    }

    return (
      <div
        ref={ref}
        className={cn('space-y-2', className)}
        data-sdk-ui="date-range-picker"
        data-slot="date-range-picker"
        {...props}
      >
        <Popover open={open} onOpenChange={setOpen}>
          <PopoverTrigger asChild>
            <button
              aria-expanded={open}
              aria-invalid={validationMessage ? 'true' : undefined}
              {...mergeSlotProps<DateRangePickerTriggerSlotProps>(
                {
                  className: cn(
                    inputBaseClassName,
                    'justify-between gap-3 text-left',
                    validationMessage ? 'border-[var(--sdk-color-state-danger)]' : undefined,
                  ),
                  'data-sdk-ui': 'date-range-picker-trigger',
                  'data-slot': 'date-range-picker-trigger',
                  type: 'button',
                },
                slotProps?.trigger,
              )}
              disabled={disabled}
            >
              <span
                className="flex min-w-0 items-center gap-3"
                data-slot="date-range-picker-trigger-value"
              >
                <CalendarRange className="h-4 w-4 shrink-0 text-[var(--sdk-color-text-muted)]" />
                <span
                  className={cn(
                    'truncate',
                    !currentValue.start && !currentValue.end ? 'text-[var(--sdk-color-text-muted)]' : undefined,
                  )}
                >
                  {formatDateRangeLabel(currentValue, placeholder)}
                </span>
              </span>
              <ChevronDown className="h-4 w-4 shrink-0 text-[var(--sdk-color-text-muted)]" />
            </button>
          </PopoverTrigger>
          <PopoverContent
            align="start"
            {...mergeSlotProps<DateRangePickerContentSlotProps>(
              {
                className: 'w-auto min-w-[22rem] space-y-4 p-4',
                'data-sdk-ui': 'date-range-picker-content',
                'data-slot': 'date-range-picker-content',
                sideOffset: 8,
              },
              slotProps?.content,
            )}
          >
            {presets?.length ? (
              <div
                {...mergeSlotProps<DateRangePickerPresetsSlotProps>(
                  {
                    className: 'space-y-2',
                    'data-slot': 'date-range-picker-presets',
                  },
                  slotProps?.presets,
                )}
              >
                <div className="text-xs font-medium uppercase tracking-[0.18em] text-[var(--sdk-color-text-muted)]">
                  {presetLabel}
                </div>
                <SegmentedControl
                  fullWidth={false}
                  onValueChange={handlePresetSelection}
                  options={presets.map((preset) => ({
                    disabled: disabled || preset.disabled,
                    label: preset.label,
                    value: preset.value,
                  }))}
                  size="sm"
                  value={currentPresetValue ?? ''}
                />
              </div>
            ) : null}
            <div
              {...mergeSlotProps<DateRangePickerCalendarSlotProps>(
                {
                  className: 'rounded-[var(--sdk-radius-field)] border border-[var(--sdk-color-border-subtle)] bg-[var(--sdk-color-surface-panel-muted)] p-3',
                  'data-sdk-ui': 'date-range-picker-calendar',
                  'data-slot': 'date-range-picker-calendar',
                },
                slotProps?.calendar,
              )}
            >
              <DayPicker
                animate
                autoFocus={open}
                classNames={calendarClassNames}
                disabled={disabledDays}
                mode="range"
                month={displayMonth}
                numberOfMonths={numberOfMonths}
                onMonthChange={setDisplayMonth}
                onSelect={handleCalendarSelection}
                selected={calendarValue}
                showOutsideDays={false}
              />
            </div>
            {(actions || validationMessage) && (
              <div
                {...mergeSlotProps<DateRangePickerFooterSlotProps>(
                  {
                    className: 'flex flex-wrap items-center justify-between gap-3',
                    'data-slot': 'date-range-picker-footer',
                  },
                  slotProps?.footer,
                )}
              >
                {validationMessage ? (
                  <p
                    {...mergeSlotProps<DateRangePickerErrorSlotProps>(
                      {
                        className: 'text-sm text-[var(--sdk-color-state-danger)]',
                        'data-slot': 'date-range-picker-error',
                        role: 'alert',
                      },
                      slotProps?.error,
                    )}
                  >
                    {validationMessage}
                  </p>
                ) : (
                  <span />
                )}
                {actions ? <div className="flex flex-wrap items-center gap-2">{actions}</div> : null}
              </div>
            )}
          </PopoverContent>
        </Popover>
      </div>
    );
  },
);

DateRangePicker.displayName = 'DateRangePicker';

export { DateRangeField, DateRangePicker, DateTimeRangeField };
