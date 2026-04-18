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
  DropdownMenuLabel,
  DropdownMenuSeparator,
  DropdownMenuShortcut,
  DropdownMenuTrigger,
} from '../dropdown-menu';

export type ActionMenuButtonItemTone = 'default' | 'danger';
export type ActionMenuButtonItemSelectHandler = () => void;

export interface ActionMenuButtonItem {
  description?: React.ReactNode;
  disabled?: boolean;
  icon?: React.ReactNode;
  key: string;
  label: React.ReactNode;
  onSelect?: ActionMenuButtonItemSelectHandler;
  shortcut?: React.ReactNode;
  tone?: ActionMenuButtonItemTone;
  type?: 'item';
}

export interface ActionMenuButtonLabelEntry {
  inset?: boolean;
  key: string;
  label: React.ReactNode;
  type: 'label';
}

export interface ActionMenuButtonSeparatorEntry {
  key: string;
  type: 'separator';
}

export type ActionMenuButtonEntry =
  | ActionMenuButtonItem
  | ActionMenuButtonLabelEntry
  | ActionMenuButtonSeparatorEntry;

export interface ActionMenuButtonSlotProps {
  content?: SlotProps<DropdownMenuContentProps>;
}

export type ActionMenuButtonMenuOpenChangeHandler = (open: boolean) => void;

export interface ActionMenuButtonProps extends Omit<ButtonProps, 'children'> {
  children: React.ReactNode;
  defaultMenuOpen?: boolean;
  items: ActionMenuButtonEntry[];
  menuAlign?: 'start' | 'center' | 'end';
  menuLabel?: string;
  menuModal?: boolean;
  menuOpen?: boolean;
  onMenuOpenChange?: ActionMenuButtonMenuOpenChangeHandler;
  showChevron?: boolean;
  slotProps?: ActionMenuButtonSlotProps;
}

const itemToneClassName: Record<ActionMenuButtonItemTone, string> = {
  default: '',
  danger:
    'text-[var(--sdk-color-state-danger)] focus:text-[var(--sdk-color-state-danger)] focus:bg-[color-mix(in_srgb,var(--sdk-color-state-danger)_12%,transparent)]',
};

function isSeparatorEntry(entry: ActionMenuButtonEntry): entry is ActionMenuButtonSeparatorEntry {
  return entry.type === 'separator';
}

function isLabelEntry(entry: ActionMenuButtonEntry): entry is ActionMenuButtonLabelEntry {
  return entry.type === 'label';
}

const ActionMenuButton = React.forwardRef<HTMLButtonElement, ActionMenuButtonProps>(
  (
    {
      children,
      className,
      defaultMenuOpen,
      items,
      menuAlign = 'end',
      menuLabel = 'Open actions menu',
      menuModal = false,
      menuOpen,
      onMenuOpenChange,
      showChevron = true,
      slotProps,
      ...props
    },
    ref,
  ) => (
    <DropdownMenu
      defaultOpen={defaultMenuOpen}
      modal={menuModal}
      onOpenChange={onMenuOpenChange}
      open={menuOpen}
    >
      <DropdownMenuTrigger asChild>
        <Button
          ref={ref}
          className={cn('justify-between', className)}
          data-sdk-ui="action-menu-button"
          type="button"
          {...props}
        >
          <span className="inline-flex min-w-0 items-center gap-2">{children}</span>
          {showChevron ? <ChevronDown className="h-4 w-4 shrink-0" /> : null}
        </Button>
      </DropdownMenuTrigger>
      <DropdownMenuContent
        {...mergeSlotProps<SlotProps<DropdownMenuContentProps>>(
          {
            align: menuAlign,
          },
          slotProps?.content,
        )}
      >
        {items.map((entry) => {
          if (isSeparatorEntry(entry)) {
            return <DropdownMenuSeparator key={entry.key} />;
          }

          if (isLabelEntry(entry)) {
            return (
              <DropdownMenuLabel inset={entry.inset} key={entry.key}>
                {entry.label}
              </DropdownMenuLabel>
            );
          }

          return (
            <DropdownMenuItem
              className={itemToneClassName[entry.tone ?? 'default']}
              data-tone={entry.tone ?? 'default'}
              disabled={entry.disabled}
              key={entry.key}
              onSelect={() => {
                entry.onSelect?.();
              }}
            >
              {entry.icon}
              {entry.description ? (
                <span className="grid gap-0.5">
                  <span>{entry.label}</span>
                  <span className="text-xs text-[var(--sdk-color-text-muted)]">{entry.description}</span>
                </span>
              ) : (
                <span>{entry.label}</span>
              )}
              {entry.shortcut ? <DropdownMenuShortcut>{entry.shortcut}</DropdownMenuShortcut> : null}
            </DropdownMenuItem>
          );
        })}
      </DropdownMenuContent>
    </DropdownMenu>
  ),
);

ActionMenuButton.displayName = 'ActionMenuButton';

export { ActionMenuButton };
