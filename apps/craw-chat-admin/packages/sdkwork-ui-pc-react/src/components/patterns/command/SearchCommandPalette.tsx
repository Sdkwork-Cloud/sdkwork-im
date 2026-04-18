import * as React from 'react';
import { cn } from '../../../lib/utils';
import {
  mergePatternSlotProps,
  type PatternSlotProps,
} from '../_internal/slot-props';
import {
  CommandDialog,
  type CommandDialogContentProps,
  CommandEmpty,
  CommandGroup,
  CommandInput,
  CommandItem,
  CommandList,
  CommandShortcut,
} from '../../ui/actions';
import { DialogTitle } from '../../ui/dialog';

export interface SearchCommandPaletteItem {
  description?: React.ReactNode;
  disabled?: boolean;
  icon?: React.ReactNode;
  id: string;
  keywords?: string[];
  label: React.ReactNode;
  shortcut?: React.ReactNode;
  value?: string;
}

export interface SearchCommandPaletteGroup {
  heading?: React.ReactNode;
  id?: string;
  items: SearchCommandPaletteItem[];
}

export type SearchCommandPaletteItemSelectHandler = (item: SearchCommandPaletteItem) => void;
export type SearchCommandPaletteSearchValueChangeHandler = (value: string) => void;

export type SearchCommandPaletteFooterSlotProps = PatternSlotProps<
  Omit<React.ComponentPropsWithoutRef<'div'>, 'children'>
>;

export interface SearchCommandPaletteSlotProps {
  content?: CommandDialogContentProps;
  footer?: SearchCommandPaletteFooterSlotProps;
}

export interface SearchCommandPaletteProps
  extends Omit<React.ComponentPropsWithoutRef<typeof CommandDialog>, 'children' | 'slotProps'> {
  closeOnSelect?: boolean;
  emptyState?: React.ReactNode;
  footer?: React.ReactNode;
  groups: SearchCommandPaletteGroup[];
  onItemSelect?: SearchCommandPaletteItemSelectHandler;
  onSearchValueChange?: SearchCommandPaletteSearchValueChangeHandler;
  placeholder?: string;
  searchValue?: string;
  slotProps?: SearchCommandPaletteSlotProps;
  title?: React.ReactNode;
}

function toSearchableText(node: React.ReactNode): string {
  if (node === null || node === undefined || typeof node === 'boolean') {
    return '';
  }

  if (typeof node === 'string' || typeof node === 'number') {
    return String(node);
  }

  if (Array.isArray(node)) {
    return node.map(toSearchableText).join(' ');
  }

  if (React.isValidElement<{ children?: React.ReactNode }>(node)) {
    return toSearchableText(node.props.children);
  }

  return '';
}

const defaultFooter = (
  <div className="flex flex-wrap items-center gap-3 border-t border-[var(--sdk-color-border-default)] px-4 py-3 text-xs text-[var(--sdk-color-text-secondary)]">
    <span className="inline-flex items-center gap-1">
      <kbd className="rounded border border-[var(--sdk-color-border-default)] px-1.5 py-0.5 text-[10px]">↑</kbd>
      <kbd className="rounded border border-[var(--sdk-color-border-default)] px-1.5 py-0.5 text-[10px]">↓</kbd>
      Navigate
    </span>
    <span className="inline-flex items-center gap-1">
      <kbd className="rounded border border-[var(--sdk-color-border-default)] px-1.5 py-0.5 text-[10px]">Enter</kbd>
      Run
    </span>
    <span className="inline-flex items-center gap-1">
      <kbd className="rounded border border-[var(--sdk-color-border-default)] px-1.5 py-0.5 text-[10px]">Esc</kbd>
      Close
    </span>
  </div>
);

export function SearchCommandPalette({
  closeOnSelect = true,
  emptyState,
  footer = defaultFooter,
  groups,
  onItemSelect,
  onOpenChange,
  onSearchValueChange,
  placeholder = 'Search commands',
  searchValue,
  slotProps,
  title = 'Search command palette',
  ...props
}: SearchCommandPaletteProps) {
  return (
    <CommandDialog
      onOpenChange={onOpenChange}
      slotProps={{
        content: mergePatternSlotProps<CommandDialogContentProps>(
          { 'data-sdk-pattern': 'search-command-palette' },
          slotProps?.content,
        ),
      }}
      {...props}
    >
      <DialogTitle className="sr-only">{title}</DialogTitle>
      <CommandInput
        onValueChange={onSearchValueChange}
        placeholder={placeholder}
        value={searchValue}
      />
      <CommandList>
        {emptyState ?? (
          <CommandEmpty>No matching commands.</CommandEmpty>
        )}
        {groups.map((group) => (
          <CommandGroup
            heading={group.heading}
            key={group.id ?? toSearchableText(group.heading) ?? group.items[0]?.id}
          >
            {group.items.map((item) => (
              <CommandItem
                disabled={item.disabled}
                key={item.id}
                keywords={item.keywords}
                onSelect={() => {
                  onItemSelect?.(item);
                  if (closeOnSelect) {
                    onOpenChange?.(false);
                  }
                }}
                value={item.value ?? (toSearchableText(item.label) || item.id)}
              >
                {item.icon ? (
                  <span className="shrink-0 text-[var(--sdk-color-text-muted)]">
                    {item.icon}
                  </span>
                ) : null}
                <span className="min-w-0 flex-1">
                  <span className="block truncate text-sm font-medium text-[var(--sdk-color-text-primary)]">
                    {item.label}
                  </span>
                  {item.description ? (
                    <span className="mt-0.5 block truncate text-xs text-[var(--sdk-color-text-secondary)]">
                      {item.description}
                    </span>
                  ) : null}
                </span>
                {item.shortcut ? <CommandShortcut>{item.shortcut}</CommandShortcut> : null}
              </CommandItem>
            ))}
          </CommandGroup>
        ))}
      </CommandList>
      {footer ? (
        <div
          {...mergePatternSlotProps<SearchCommandPaletteFooterSlotProps>(
            {
              className: cn('shrink-0'),
              'data-sdk-region': 'search-command-palette-footer',
            },
            slotProps?.footer,
          )}
        >
          {footer}
        </div>
      ) : null}
    </CommandDialog>
  );
}
SearchCommandPalette.displayName = 'SearchCommandPalette';
