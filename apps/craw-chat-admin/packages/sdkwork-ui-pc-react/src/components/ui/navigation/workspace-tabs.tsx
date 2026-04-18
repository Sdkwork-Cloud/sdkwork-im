import * as React from 'react';
import { X } from 'lucide-react';
import { mergeOptionalSlotProps, mergeSlotProps, type SlotProps } from '../../../lib/slot-props';
import { cn } from '../../../lib/utils';

export type WorkspaceTabsRegionSlotProps = SlotProps<Omit<React.ComponentPropsWithoutRef<'div'>, 'children'>>;
export type WorkspaceTabsEmptySlotProps = SlotProps<Omit<React.ComponentPropsWithoutRef<'span'>, 'children'>>;
export type WorkspaceTabRootProps = SlotProps<Omit<React.ComponentPropsWithoutRef<'div'>, 'children'>>;
export type WorkspaceTabTriggerSlotProps = SlotProps<Omit<React.ComponentPropsWithoutRef<'button'>, 'children'>>;
export type WorkspaceTabIndicatorSlotProps = SlotProps<Omit<React.ComponentPropsWithoutRef<'span'>, 'children'>>;

export interface WorkspaceTabItem {
  closeLabel?: string;
  disabled?: boolean;
  icon?: React.ReactNode;
  id: string;
  label: React.ReactNode;
  meta?: React.ReactNode;
  modified?: boolean;
}

export interface WorkspaceTabCollectionItemState {
  index: number;
  item: WorkspaceTabItem;
  selected: boolean;
}

export type WorkspaceTabPropsResolver = (
  state: WorkspaceTabCollectionItemState,
) => WorkspaceTabRootProps | undefined;
export type WorkspaceTabSlotPropsResolver = (
  state: WorkspaceTabCollectionItemState,
) => WorkspaceTabSlotProps | undefined;
export type WorkspaceTabCloseHandler = (id: string) => void;
export type WorkspaceTabValueChangeHandler = (id: string) => void;

export interface WorkspaceTabSlotProps {
  closeButton?: WorkspaceTabTriggerSlotProps;
  icon?: WorkspaceTabIndicatorSlotProps;
  label?: WorkspaceTabIndicatorSlotProps;
  meta?: WorkspaceTabIndicatorSlotProps;
  modifiedIndicator?: WorkspaceTabIndicatorSlotProps;
  trigger?: WorkspaceTabTriggerSlotProps;
}

export interface WorkspaceTabsSlotProps {
  actions?: WorkspaceTabsRegionSlotProps;
  empty?: WorkspaceTabsEmptySlotProps;
  tabList?: WorkspaceTabsRegionSlotProps;
}

export interface WorkspaceTabsProps extends React.HTMLAttributes<HTMLDivElement> {
  actions?: React.ReactNode;
  emptyState?: React.ReactNode;
  getItemProps?: WorkspaceTabPropsResolver;
  getItemSlotProps?: WorkspaceTabSlotPropsResolver;
  items: WorkspaceTabItem[];
  onCloseTab?: WorkspaceTabCloseHandler;
  onValueChange?: WorkspaceTabValueChangeHandler;
  slotProps?: WorkspaceTabsSlotProps;
  value?: string | null;
}

export interface WorkspaceTabProps extends React.HTMLAttributes<HTMLDivElement> {
  item: WorkspaceTabItem;
  onCloseTab?: WorkspaceTabCloseHandler;
  onValueChange?: WorkspaceTabValueChangeHandler;
  selected?: boolean;
  slotProps?: WorkspaceTabSlotProps;
}

function mergeWorkspaceTabSlotProps(
  baseSlotProps?: WorkspaceTabSlotProps,
  overrideSlotProps?: WorkspaceTabSlotProps,
): WorkspaceTabSlotProps | undefined {
  const mergedSlotProps: WorkspaceTabSlotProps = {};

  const closeButton = mergeOptionalSlotProps(baseSlotProps?.closeButton, overrideSlotProps?.closeButton);
  const icon = mergeOptionalSlotProps(baseSlotProps?.icon, overrideSlotProps?.icon);
  const label = mergeOptionalSlotProps(baseSlotProps?.label, overrideSlotProps?.label);
  const meta = mergeOptionalSlotProps(baseSlotProps?.meta, overrideSlotProps?.meta);
  const modifiedIndicator = mergeOptionalSlotProps(
    baseSlotProps?.modifiedIndicator,
    overrideSlotProps?.modifiedIndicator,
  );
  const trigger = mergeOptionalSlotProps(baseSlotProps?.trigger, overrideSlotProps?.trigger);

  if (closeButton) {
    mergedSlotProps.closeButton = closeButton;
  }

  if (icon) {
    mergedSlotProps.icon = icon;
  }

  if (label) {
    mergedSlotProps.label = label;
  }

  if (meta) {
    mergedSlotProps.meta = meta;
  }

  if (modifiedIndicator) {
    mergedSlotProps.modifiedIndicator = modifiedIndicator;
  }

  if (trigger) {
    mergedSlotProps.trigger = trigger;
  }

  return Object.keys(mergedSlotProps).length > 0 ? mergedSlotProps : undefined;
}

function resolveCloseLabel(item: WorkspaceTabItem) {
  return typeof item.label === 'string' ? `Close ${item.label}` : item.closeLabel ?? 'Close tab';
}

const WorkspaceTab = React.forwardRef<HTMLDivElement, WorkspaceTabProps>(
  ({ className, item, onCloseTab, onValueChange, selected = false, slotProps, ...props }, ref) => (
    <div
      ref={ref}
      className={cn(
        'group inline-flex items-center gap-1 rounded-[var(--sdk-radius-control)] border transition-colors',
        onCloseTab ? 'pr-1.5' : 'pr-2',
        selected
          ? 'border-[var(--sdk-color-brand-primary)]/25 bg-[var(--sdk-color-brand-primary-soft)] text-[var(--sdk-color-text-primary)]'
          : 'border-[var(--sdk-color-border-subtle)] bg-[var(--sdk-color-surface-panel)] text-[var(--sdk-color-text-secondary)]',
        className,
      )}
      data-disabled={item.disabled ? 'true' : 'false'}
      data-sdk-ui="workspace-tab"
      data-state={selected ? 'active' : 'inactive'}
      {...props}
    >
      <button
        {...mergeSlotProps<WorkspaceTabTriggerSlotProps>(
          {
            'aria-selected': selected,
            className: cn(
              'inline-flex min-w-0 items-center gap-2 rounded-[var(--sdk-radius-control)] px-3 py-2 text-sm font-medium transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-[var(--sdk-color-border-focus)]',
              selected
                ? 'text-[var(--sdk-color-text-primary)]'
                : 'hover:text-[var(--sdk-color-text-primary)]',
            ),
            'data-sdk-region': 'workspace-tab-trigger',
            disabled: item.disabled,
            onClick: () => onValueChange?.(item.id),
            role: 'tab',
            tabIndex: selected ? 0 : -1,
            type: 'button',
          },
          slotProps?.trigger,
        )}
      >
        {item.icon ? (
          <span
            {...mergeSlotProps<WorkspaceTabIndicatorSlotProps>(
              {
                className: 'shrink-0',
                'data-sdk-region': 'workspace-tab-icon',
              },
              slotProps?.icon,
            )}
          >
            {item.icon}
          </span>
        ) : null}
        <span
          {...mergeSlotProps<WorkspaceTabIndicatorSlotProps>(
            {
              className: 'truncate',
              'data-sdk-region': 'workspace-tab-label',
            },
            slotProps?.label,
          )}
        >
          {item.label}
        </span>
        {item.modified ? (
          <span
            {...mergeSlotProps<WorkspaceTabIndicatorSlotProps>(
              {
                'aria-hidden': 'true',
                className:
                  'inline-flex h-2 w-2 shrink-0 rounded-full bg-[var(--sdk-color-state-warning)]',
                'data-sdk-region': 'workspace-tab-modified-indicator',
              },
              slotProps?.modifiedIndicator,
            )}
          />
        ) : null}
        {item.meta ? (
          <span
            {...mergeSlotProps<WorkspaceTabIndicatorSlotProps>(
              {
                className: 'shrink-0 text-xs text-[var(--sdk-color-text-muted)]',
                'data-sdk-region': 'workspace-tab-meta',
              },
              slotProps?.meta,
            )}
          >
            {item.meta}
          </span>
        ) : null}
      </button>

      {onCloseTab ? (
        <button
          {...mergeSlotProps<WorkspaceTabTriggerSlotProps>(
            {
              'aria-label': item.closeLabel ?? resolveCloseLabel(item),
              className:
                'rounded-full p-1 text-[var(--sdk-color-text-muted)] transition-colors hover:bg-[var(--sdk-color-surface-elevated)] hover:text-[var(--sdk-color-text-primary)]',
              'data-sdk-region': 'workspace-tab-close-button',
              onClick: (event) => {
                event.stopPropagation();
                onCloseTab(item.id);
              },
              type: 'button',
            },
            slotProps?.closeButton,
          )}
        >
          <X className="h-3.5 w-3.5" />
        </button>
      ) : null}
    </div>
  ),
);

WorkspaceTab.displayName = 'WorkspaceTab';

const WorkspaceTabs = React.forwardRef<HTMLDivElement, WorkspaceTabsProps>(
  (
    {
      actions,
      className,
      emptyState = 'Workspace',
      getItemProps,
      getItemSlotProps,
      items,
      onCloseTab,
      onValueChange,
      slotProps,
      value,
      ...props
    },
    ref,
  ) => (
    <div
      ref={ref}
      className={cn('flex items-center gap-3', className)}
      data-sdk-ui="workspace-tabs"
      {...props}
    >
      <div
        {...mergeSlotProps<WorkspaceTabsRegionSlotProps>(
          {
            'aria-label': 'Workspace tabs',
            'aria-orientation': 'horizontal',
            className: 'flex min-w-0 flex-1 items-center gap-2 overflow-x-auto',
            'data-sdk-region': 'workspace-tabs-tablist',
            role: 'tablist',
          },
          slotProps?.tabList,
        )}
      >
        {items.length === 0 ? (
          <span
            {...mergeSlotProps<WorkspaceTabsEmptySlotProps>(
              {
                className: 'px-1 text-sm text-[var(--sdk-color-text-muted)]',
                'data-sdk-region': 'workspace-tabs-empty',
              },
              slotProps?.empty,
            )}
          >
            {emptyState}
          </span>
        ) : (
          items.map((item, index) => {
            const itemState: WorkspaceTabCollectionItemState = {
              index,
              item,
              selected: item.id === value,
            };

            return (
              <WorkspaceTab
                {...getItemProps?.(itemState)}
                item={item}
                key={item.id}
                onCloseTab={onCloseTab}
                onValueChange={onValueChange}
                selected={itemState.selected}
                slotProps={mergeWorkspaceTabSlotProps(getItemSlotProps?.(itemState), undefined)}
              />
            );
          })
        )}
      </div>

      {actions ? (
        <div
          {...mergeSlotProps<WorkspaceTabsRegionSlotProps>(
            {
              className: 'flex shrink-0 items-center gap-2',
              'data-sdk-region': 'workspace-tabs-actions',
            },
            slotProps?.actions,
          )}
        >
          {actions}
        </div>
      ) : null}
    </div>
  ),
);

WorkspaceTabs.displayName = 'WorkspaceTabs';

export { WorkspaceTab, WorkspaceTabs };
