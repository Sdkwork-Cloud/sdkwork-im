import * as React from 'react';
import { Search } from 'lucide-react';
import { cn } from '../../../lib/utils';
import { EmptySearch } from '../../ui/feedback';
import { InlineAlert, type InlineAlertProps } from '../../ui/feedback/inline-alert';
import { Input } from '../../ui/input';
import { WorkspacePanel } from '../workspace/WorkspacePanel';

export interface SettingsCenterItem {
  badge?: React.ReactNode;
  description?: React.ReactNode;
  disabled?: boolean;
  icon?: React.ReactNode;
  id: string;
  keywords?: string[];
  label: React.ReactNode;
}

export interface SettingsCenterSection {
  items: SettingsCenterItem[];
  title?: React.ReactNode;
}

export type SettingsCenterActiveItemChangeHandler = (itemId: string) => void;
export type SettingsCenterSearchChangeHandler = (value: string) => void;

export interface SettingsCenterProps extends Omit<React.HTMLAttributes<HTMLDivElement>, 'children' | 'title'> {
  actions?: React.ReactNode;
  activeItem?: string;
  children?: React.ReactNode;
  description?: React.ReactNode;
  emptyState?: React.ReactNode;
  navFooter?: React.ReactNode;
  navHeader?: React.ReactNode;
  onActiveItemChange?: SettingsCenterActiveItemChangeHandler;
  onSearchChange?: SettingsCenterSearchChangeHandler;
  searchPlaceholder?: string;
  searchValue?: string;
  sections: SettingsCenterSection[];
  title: React.ReactNode;
}

export interface DirtyStateBarProps extends Omit<InlineAlertProps, 'showIcon' | 'tone'> {
  sticky?: boolean;
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

function itemMatchesQuery(item: SettingsCenterItem, query: string) {
  if (!query) {
    return true;
  }

  const haystack = [
    toSearchableText(item.label),
    toSearchableText(item.description),
    ...(item.keywords ?? []),
  ]
    .join(' ')
    .toLowerCase();

  return haystack.includes(query);
}

export const SettingsCenter = React.forwardRef<HTMLDivElement, SettingsCenterProps>(({
  actions,
  activeItem,
  children,
  className,
  description,
  emptyState,
  navFooter,
  navHeader,
  onActiveItemChange,
  onSearchChange,
  searchPlaceholder = 'Search settings',
  searchValue = '',
  sections,
  title,
  ...props
}, ref) => {
  const normalizedQuery = searchValue.trim().toLowerCase();
  const filteredSections = sections
    .map((section) => ({
      ...section,
      items: section.items.filter((item) => itemMatchesQuery(item, normalizedQuery)),
    }))
    .filter((section) => section.items.length > 0);

  const hasSearch = searchValue.length > 0 || Boolean(onSearchChange);
  const hasResults = filteredSections.length > 0;

  return (
    <div
      ref={ref}
      className={cn('grid h-full min-h-0 min-w-0 gap-4 lg:grid-cols-[18rem_minmax(0,1fr)]', className)}
      data-sdk-pattern="settings-center"
      {...props}
    >
      <aside
        className="flex min-h-0 flex-col rounded-[var(--sdk-radius-panel)] border border-[var(--sdk-color-border-default)] bg-[var(--sdk-color-surface-panel)] shadow-[var(--sdk-shadow-sm)]"
        data-sdk-region="settings-nav"
      >
        {navHeader ? <div className="border-b border-[var(--sdk-color-border-subtle)] px-4 py-4">{navHeader}</div> : null}
        {hasSearch ? (
          <div className="border-b border-[var(--sdk-color-border-subtle)] px-4 py-4">
            <label className="block">
              <span className="sr-only">{searchPlaceholder}</span>
              <div className="relative">
                <Search className="pointer-events-none absolute left-3 top-1/2 h-4 w-4 -translate-y-1/2 text-[var(--sdk-color-text-muted)]" />
                <Input
                  className="pl-9"
                  onChange={(event) => onSearchChange?.(event.target.value)}
                  placeholder={searchPlaceholder}
                  value={searchValue}
                />
              </div>
            </label>
          </div>
        ) : null}
        <div className="min-h-0 flex-1 space-y-4 overflow-y-auto px-3 py-4">
          {hasResults ? (
            filteredSections.map((section) => (
              <div className="space-y-2" key={toSearchableText(section.title) || section.items[0]?.id}>
                {section.title ? (
                  <div className="px-2 text-[10px] font-semibold uppercase tracking-[0.16em] text-[var(--sdk-color-text-muted)]">
                    {section.title}
                  </div>
                ) : null}
                <div className="space-y-1">
                  {section.items.map((item) => {
                    const active = item.id === activeItem;

                    return (
                      <button
                        aria-label={toSearchableText(item.label)}
                        className={cn(
                          'flex w-full items-start gap-3 rounded-[var(--sdk-radius-field)] border px-3 py-3 text-left transition-colors',
                          active
                            ? 'border-[var(--sdk-color-brand-primary)] bg-[var(--sdk-color-brand-primary-soft)]'
                            : 'border-transparent hover:border-[var(--sdk-color-border-default)] hover:bg-[var(--sdk-color-surface-panel-muted)]',
                          item.disabled ? 'cursor-not-allowed opacity-60' : null,
                        )}
                        data-sdk-pattern="settings-nav-item"
                        data-slot="settings-center-nav-item"
                        data-state={active ? 'active' : 'inactive'}
                        disabled={item.disabled}
                        key={item.id}
                        onClick={() => onActiveItemChange?.(item.id)}
                        type="button"
                      >
                        {item.icon ? <span className="mt-0.5 shrink-0">{item.icon}</span> : null}
                        <span className="min-w-0 flex-1">
                          <span className="flex items-center gap-2 text-sm font-medium text-[var(--sdk-color-text-primary)]">
                            <span>{item.label}</span>
                            {item.badge ? <span className="shrink-0">{item.badge}</span> : null}
                          </span>
                          {item.description ? (
                            <span className="mt-1 block text-sm text-[var(--sdk-color-text-secondary)]">
                              {item.description}
                            </span>
                          ) : null}
                        </span>
                      </button>
                    );
                  })}
                </div>
              </div>
            ))
          ) : (
            emptyState ?? (
              <EmptySearch
                description="Try a different keyword or browse the navigation without a search term."
                keyword={searchValue}
                title="No settings match your search"
              />
            )
          )}
        </div>
        {navFooter ? <div className="border-t border-[var(--sdk-color-border-subtle)] px-4 py-4">{navFooter}</div> : null}
      </aside>

      <WorkspacePanel
        actions={actions}
        className="h-full"
        description={description}
        title={title}
      >
        {children}
      </WorkspacePanel>
    </div>
  );
});

export function DirtyStateBar({ className, sticky = true, ...props }: DirtyStateBarProps) {
  return (
    <InlineAlert
      className={cn(sticky ? 'sticky bottom-0 z-10' : null, className)}
      data-sdk-pattern="dirty-state-bar"
      showIcon
      tone="warning"
      {...props}
    />
  );
}
SettingsCenter.displayName = 'SettingsCenter';
DirtyStateBar.displayName = 'DirtyStateBar';
