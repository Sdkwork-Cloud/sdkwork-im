import * as React from 'react';
import { ChevronDown } from 'lucide-react';
import { mergeSlotProps, type SlotProps } from '../../../lib/slot-props';
import { cn } from '../../../lib/utils';
import { Button, type ButtonProps } from '../button';
import {
  DropdownMenu,
  DropdownMenuContent,
  type DropdownMenuContentProps,
  DropdownMenuItem,
  DropdownMenuShortcut,
  DropdownMenuTrigger,
} from '../dropdown-menu';

export interface SplitButtonItem {
  disabled?: boolean;
  icon?: React.ReactNode;
  key: string;
  label: React.ReactNode;
  onSelect?: SplitButtonItemSelectHandler;
  shortcut?: React.ReactNode;
}

export type SplitButtonGroupSlotProps = SlotProps<Omit<React.ComponentPropsWithoutRef<'div'>, 'children'>>;
export type SplitButtonItemSelectHandler = () => void;
export type SplitButtonMenuOpenChangeHandler = (open: boolean) => void;

export interface SplitButtonSlotProps {
  content?: SlotProps<DropdownMenuContentProps>;
  group?: SplitButtonGroupSlotProps;
  menuTrigger?: ButtonProps;
  primaryAction?: ButtonProps;
}

export interface SplitButtonProps extends Omit<ButtonProps, 'children'> {
  children: React.ReactNode;
  defaultMenuOpen?: boolean;
  items: SplitButtonItem[];
  menuAlign?: 'start' | 'center' | 'end';
  menuLabel?: string;
  menuModal?: boolean;
  menuOpen?: boolean;
  onMenuOpenChange?: SplitButtonMenuOpenChangeHandler;
  slotProps?: SplitButtonSlotProps;
}

const SplitButton = React.forwardRef<HTMLButtonElement, SplitButtonProps>(
  (
    {
      children,
      className,
      defaultMenuOpen,
      disabled,
      items,
      loading,
      menuAlign = 'end',
      menuLabel = 'Open more actions',
      menuModal = false,
      menuOpen,
      onMenuOpenChange,
      size,
      slotProps,
      variant,
      ...props
    },
    ref,
  ) => (
    <DropdownMenu defaultOpen={defaultMenuOpen} modal={menuModal} onOpenChange={onMenuOpenChange} open={menuOpen}>
      <div
        {...mergeSlotProps<SplitButtonGroupSlotProps>(
          {
            className: cn('inline-flex items-stretch', className),
          },
          slotProps?.group,
        )}
        data-sdk-ui="split-button"
      >
        <Button
          ref={ref}
          {...mergeSlotProps<ButtonProps>(
            {
              className: 'rounded-r-none',
              disabled,
              loading,
              size,
              variant,
              ...props,
            },
            slotProps?.primaryAction,
          )}
        >
          {children}
        </Button>
        <DropdownMenuTrigger asChild>
          <Button
            {...mergeSlotProps<ButtonProps>(
              {
                'aria-label': menuLabel,
                className: 'w-10 rounded-l-none border-l border-[var(--sdk-color-border-default)] px-0',
                disabled: disabled || items.length === 0,
                size,
                variant,
              },
              slotProps?.menuTrigger,
            )}
          >
            <ChevronDown className="h-4 w-4" />
          </Button>
        </DropdownMenuTrigger>
      </div>
      <DropdownMenuContent
        {...mergeSlotProps<SlotProps<DropdownMenuContentProps>>(
          {
            align: menuAlign,
          },
          slotProps?.content,
        )}
      >
        {items.map((item) => (
          <DropdownMenuItem
            key={item.key}
            disabled={item.disabled}
            onSelect={() => {
              item.onSelect?.();
            }}
          >
            {item.icon}
            <span>{item.label}</span>
            {item.shortcut ? <DropdownMenuShortcut>{item.shortcut}</DropdownMenuShortcut> : null}
          </DropdownMenuItem>
        ))}
      </DropdownMenuContent>
    </DropdownMenu>
  ),
);

SplitButton.displayName = 'SplitButton';

export { SplitButton };
