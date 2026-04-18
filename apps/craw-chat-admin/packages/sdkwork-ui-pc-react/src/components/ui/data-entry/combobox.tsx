import * as React from 'react';
import { Check, ChevronsUpDown, X } from 'lucide-react';
import { useControllableState } from '../../../lib/core';
import { mergeSlotProps, type SlotProps } from '../../../lib/slot-props';
import { cn } from '../../../lib/utils';
import {
  Command,
  type CommandEmptyProps,
  CommandEmpty,
  type CommandGroupProps,
  CommandGroup,
  type CommandInputProps,
  CommandInput,
  type CommandListProps,
  CommandItem,
  CommandList,
} from '../actions/command';
import { inputBaseClassName } from '../input';
import { Popover, PopoverContent, PopoverTrigger, type PopoverContentProps } from '../popover';

export interface ComboboxOption {
  value: string;
  label: string;
  description?: string;
  disabled?: boolean;
  keywords?: string[];
}

export type ComboboxValueChangeHandler = (value: string) => void;
export type ComboboxRootSlotProps = SlotProps<Omit<React.ComponentPropsWithoutRef<'div'>, 'children'>>;
export type ComboboxClearButtonSlotProps = SlotProps<Omit<React.ComponentPropsWithoutRef<'button'>, 'children'>>;
export type ComboboxContentSlotProps = SlotProps<Omit<PopoverContentProps, 'children'>>;
export type ComboboxInputSlotProps = SlotProps<CommandInputProps>;
export type ComboboxListSlotProps = SlotProps<CommandListProps>;
export type ComboboxEmptySlotProps = SlotProps<CommandEmptyProps>;
export type ComboboxGroupSlotProps = SlotProps<CommandGroupProps>;

export interface ComboboxSlotProps {
  clearButton?: ComboboxClearButtonSlotProps;
  content?: ComboboxContentSlotProps;
  empty?: ComboboxEmptySlotProps;
  group?: ComboboxGroupSlotProps;
  input?: ComboboxInputSlotProps;
  list?: ComboboxListSlotProps;
  root?: ComboboxRootSlotProps;
}

export interface ComboboxProps extends Omit<React.ButtonHTMLAttributes<HTMLButtonElement>, 'defaultValue' | 'onChange' | 'value'> {
  clearable?: boolean;
  defaultValue?: string;
  emptyText?: string;
  options: ComboboxOption[];
  onValueChange?: ComboboxValueChangeHandler;
  placeholder?: string;
  searchPlaceholder?: string;
  slotProps?: ComboboxSlotProps;
  value?: string;
}

const Combobox = React.forwardRef<HTMLButtonElement, ComboboxProps>(({
  className,
  clearable = false,
  defaultValue,
  disabled,
  emptyText = 'No options found.',
  onValueChange,
  options,
  placeholder = 'Select option',
  searchPlaceholder = 'Search option...',
  slotProps,
  value,
  ...props
}, ref) => {
  const [open, setOpen] = React.useState(false);
  const [selectedValue, setSelectedValue] = useControllableState<string>({
    defaultValue: defaultValue ?? '',
    onChange: onValueChange,
    value,
  });
  const selectedOption = options.find((option) => option.value === selectedValue);

  function handleValueChange(nextValue: string) {
    setSelectedValue(nextValue);
  }

  function clearValue(event: React.MouseEvent<HTMLButtonElement>) {
    event.preventDefault();
    event.stopPropagation();
    handleValueChange('');
    setOpen(false);
  }

  return (
    <Popover modal open={open} onOpenChange={setOpen}>
      <div
        {...mergeSlotProps<ComboboxRootSlotProps>(
          {
            className: 'relative',
            'data-sdk-ui': 'combobox',
            'data-slot': 'combobox',
          },
          slotProps?.root,
        )}
      >
        <PopoverTrigger asChild>
          <button
            ref={ref}
            aria-expanded={open}
            aria-label={selectedOption?.label ?? placeholder}
            className={cn(
              inputBaseClassName,
              clearable && selectedValue ? 'pr-18' : 'pr-10',
              'justify-between gap-2 text-left',
              className,
            )}
            data-sdk-ui="combobox-trigger"
            data-slot="combobox-trigger"
            disabled={disabled}
            role="combobox"
            type="button"
            {...props}
          >
            <span
              className={cn('truncate', !selectedOption && 'text-[var(--sdk-color-text-muted)]')}
              data-slot="combobox-value"
            >
              {selectedOption?.label ?? placeholder}
            </span>
          </button>
        </PopoverTrigger>
        {clearable && selectedValue ? (
          <button
            aria-label="Clear selection"
            {...mergeSlotProps<ComboboxClearButtonSlotProps>(
              {
                className:
                  'absolute right-8 top-1/2 inline-flex h-7 w-7 -translate-y-1/2 items-center justify-center rounded-[var(--sdk-radius-control)] text-[var(--sdk-color-text-muted)] transition-colors hover:bg-[var(--sdk-color-brand-primary-soft)] hover:text-[var(--sdk-color-text-primary)]',
                'data-sdk-ui': 'combobox-clear-button',
                'data-slot': 'combobox-clear-button',
              },
              slotProps?.clearButton,
            )}
            disabled={disabled}
            onClick={(event) => {
              slotProps?.clearButton?.onClick?.(event);
              if (!event.defaultPrevented) {
                clearValue(event);
              }
            }}
            type="button"
          >
            <X className="h-4 w-4" />
          </button>
        ) : null}
        <span
          aria-hidden="true"
          className="pointer-events-none absolute right-3 top-1/2 flex h-4 w-4 -translate-y-1/2 items-center justify-center text-[var(--sdk-color-text-muted)]"
          data-slot="combobox-indicator"
        >
          <ChevronsUpDown className="h-4 w-4" />
        </span>
      </div>
      <PopoverContent
        align="start"
        {...mergeSlotProps<ComboboxContentSlotProps>(
          {
            className: 'w-[var(--radix-popover-trigger-width)] p-0',
            'data-sdk-ui': 'combobox-content',
            'data-slot': 'combobox-content',
            sideOffset: 6,
          },
          slotProps?.content,
        )}
      >
        <Command>
          <CommandInput
            {...mergeSlotProps<ComboboxInputSlotProps>(
              {
                'data-sdk-ui': 'combobox-input',
                'data-slot': 'combobox-input',
                placeholder: searchPlaceholder,
              },
              slotProps?.input,
            )}
          />
          <CommandList
            {...mergeSlotProps<ComboboxListSlotProps>(
              {
                'data-sdk-ui': 'combobox-list',
                'data-slot': 'combobox-list',
              },
              slotProps?.list,
            )}
          >
            <CommandEmpty
              {...mergeSlotProps<ComboboxEmptySlotProps>(
                {
                  'data-sdk-ui': 'combobox-empty',
                  'data-slot': 'combobox-empty',
                },
                slotProps?.empty,
              )}
            >
              {emptyText}
            </CommandEmpty>
            <CommandGroup
              {...mergeSlotProps<ComboboxGroupSlotProps>(
                {
                  'data-sdk-ui': 'combobox-group',
                  'data-slot': 'combobox-group',
                },
                slotProps?.group,
              )}
            >
              {options.map((option) => (
                <CommandItem
                  disabled={option.disabled}
                  key={option.value}
                  keywords={option.keywords}
                  onSelect={() => {
                    handleValueChange(option.value);
                    setOpen(false);
                  }}
                  value={option.value}
                >
                  <Check className={cn('h-4 w-4', selectedValue === option.value ? 'opacity-100' : 'opacity-0')} />
                  <div className="min-w-0 flex-1">
                    <div className="truncate">{option.label}</div>
                    {option.description ? (
                      <div className="truncate text-xs text-[var(--sdk-color-text-muted)]">{option.description}</div>
                    ) : null}
                  </div>
                </CommandItem>
              ))}
            </CommandGroup>
          </CommandList>
        </Command>
      </PopoverContent>
    </Popover>
  );
});

Combobox.displayName = 'Combobox';

export { Combobox };
