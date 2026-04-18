import * as React from 'react';
import {
  AlertCircle,
  AlertTriangle,
  Bell,
  CheckCheck,
  CheckCircle2,
  ExternalLink,
  Info,
  Trash2,
} from 'lucide-react';
import { mergeOptionalSlotProps, mergeSlotProps, type SlotProps } from '../../../lib/slot-props';
import { cn } from '../../../lib/utils';

export type NotificationCenterTone = 'default' | 'info' | 'success' | 'warning' | 'danger';

export type NotificationCenterRegionSlotProps = SlotProps<Omit<React.ComponentPropsWithoutRef<'div'>, 'children'>>;
export type NotificationCenterListSlotProps = SlotProps<Omit<React.ComponentPropsWithoutRef<'ol'>, 'children'>>;
export type NotificationCenterItemRootProps = SlotProps<
  Omit<React.ComponentPropsWithoutRef<'article'>, 'children' | 'title'>
>;
export type NotificationCenterItemLinkSlotProps = SlotProps<Omit<React.ComponentPropsWithoutRef<'a'>, 'children'>>;
export type NotificationCenterItemIconSlotProps = SlotProps<Omit<React.ComponentPropsWithoutRef<'svg'>, 'children'>>;
export type NotificationCenterItemIndicatorSlotProps = SlotProps<
  Omit<React.ComponentPropsWithoutRef<'span'>, 'children'>
>;
export type NotificationCenterItemPropsResolver = (
  item: NotificationCenterEntry,
  index: number,
) => NotificationCenterItemRootProps | undefined;
export type NotificationCenterItemSlotPropsResolver = (
  item: NotificationCenterEntry,
  index: number,
) => NotificationCenterItemSlotProps | undefined;
export type NotificationCenterClearHandler = () => void;
export type NotificationCenterItemSelectHandler = (id: string) => void;
export type NotificationCenterMarkAllAsReadHandler = () => void;

export interface NotificationCenterEntry {
  actionHref?: string;
  actionLabel?: string;
  actionTarget?: React.HTMLAttributeAnchorTarget;
  content: React.ReactNode;
  id: string;
  timeLabel?: React.ReactNode;
  title: React.ReactNode;
  tone?: NotificationCenterTone;
  unread?: boolean;
}

export interface NotificationCenterItemSlotProps {
  action?: NotificationCenterItemLinkSlotProps;
  body?: NotificationCenterRegionSlotProps;
  description?: NotificationCenterRegionSlotProps;
  icon?: NotificationCenterItemIconSlotProps;
  timeLabel?: NotificationCenterRegionSlotProps;
  title?: NotificationCenterRegionSlotProps;
  unreadIndicator?: NotificationCenterItemIndicatorSlotProps;
}

export interface NotificationCenterSlotProps {
  empty?: NotificationCenterRegionSlotProps;
  header?: NotificationCenterRegionSlotProps;
  headerActions?: NotificationCenterRegionSlotProps;
  list?: NotificationCenterListSlotProps;
  viewport?: NotificationCenterRegionSlotProps;
}

export interface NotificationCenterProps extends Omit<React.ComponentPropsWithoutRef<'section'>, 'title'> {
  clearLabel?: string;
  emptyDescription?: React.ReactNode;
  emptyTitle?: React.ReactNode;
  getItemProps?: NotificationCenterItemPropsResolver;
  getItemSlotProps?: NotificationCenterItemSlotPropsResolver;
  items: NotificationCenterEntry[];
  markAllAsReadLabel?: string;
  onClear?: NotificationCenterClearHandler;
  onItemSelect?: NotificationCenterItemSelectHandler;
  onMarkAllAsRead?: NotificationCenterMarkAllAsReadHandler;
  slotProps?: NotificationCenterSlotProps;
  title?: React.ReactNode;
}

export interface NotificationCenterItemProps
  extends Omit<React.ComponentPropsWithoutRef<'article'>, 'title'> {
  item: NotificationCenterEntry;
  onItemSelect?: NotificationCenterItemSelectHandler;
  slotProps?: NotificationCenterItemSlotProps;
}

const toneIcon = {
  default: Info,
  info: Info,
  success: CheckCircle2,
  warning: AlertTriangle,
  danger: AlertCircle,
} as const;

const toneClass = {
  default: 'text-[var(--sdk-color-state-info)]',
  info: 'text-[var(--sdk-color-state-info)]',
  success: 'text-[var(--sdk-color-state-success)]',
  warning: 'text-[var(--sdk-color-state-warning)]',
  danger: 'text-[var(--sdk-color-state-danger)]',
} as const;

function resolveItemSelectLabel(item: NotificationCenterEntry) {
  return typeof item.title === 'string' ? item.title : 'Open notification item';
}

function mergeNotificationCenterItemSlotProps(
  baseSlotProps?: NotificationCenterItemSlotProps,
  overrideSlotProps?: NotificationCenterItemSlotProps,
): NotificationCenterItemSlotProps | undefined {
  const mergedSlotProps: NotificationCenterItemSlotProps = {};

  const action = mergeOptionalSlotProps(baseSlotProps?.action, overrideSlotProps?.action);
  const body = mergeOptionalSlotProps(baseSlotProps?.body, overrideSlotProps?.body);
  const description = mergeOptionalSlotProps(baseSlotProps?.description, overrideSlotProps?.description);
  const icon = mergeOptionalSlotProps(baseSlotProps?.icon, overrideSlotProps?.icon);
  const timeLabel = mergeOptionalSlotProps(baseSlotProps?.timeLabel, overrideSlotProps?.timeLabel);
  const title = mergeOptionalSlotProps(baseSlotProps?.title, overrideSlotProps?.title);
  const unreadIndicator = mergeOptionalSlotProps(
    baseSlotProps?.unreadIndicator,
    overrideSlotProps?.unreadIndicator,
  );

  if (action) {
    mergedSlotProps.action = action;
  }

  if (body) {
    mergedSlotProps.body = body;
  }

  if (description) {
    mergedSlotProps.description = description;
  }

  if (icon) {
    mergedSlotProps.icon = icon;
  }

  if (timeLabel) {
    mergedSlotProps.timeLabel = timeLabel;
  }

  if (title) {
    mergedSlotProps.title = title;
  }

  if (unreadIndicator) {
    mergedSlotProps.unreadIndicator = unreadIndicator;
  }

  return Object.keys(mergedSlotProps).length > 0 ? mergedSlotProps : undefined;
}

const NotificationCenterItem = React.forwardRef<HTMLElement, NotificationCenterItemProps>(
  ({ className, item, onItemSelect, slotProps, ...props }, ref) => {
    const tone = item.tone ?? 'default';
    const Icon = toneIcon[tone];

    const itemBody = (
      <>
        <Icon
          {...mergeSlotProps<NotificationCenterItemIconSlotProps>(
            {
              className: cn('mt-0.5 h-4 w-4 shrink-0', toneClass[tone]),
              'data-sdk-region': 'notification-center-item-icon',
            },
            slotProps?.icon,
          )}
        />
        <div
          {...mergeSlotProps<NotificationCenterRegionSlotProps>(
            {
              className: 'min-w-0 flex-1',
              'data-sdk-region': 'notification-center-item-body',
            },
            slotProps?.body,
          )}
        >
          <div className="flex items-start justify-between gap-3">
            <div
              {...mergeSlotProps<NotificationCenterRegionSlotProps>(
                {
                  className: 'min-w-0 text-sm font-medium text-[var(--sdk-color-text-primary)]',
                  'data-sdk-region': 'notification-center-item-title',
                },
                slotProps?.title,
              )}
            >
              {item.title}
            </div>
            {item.timeLabel ? (
              <div
                {...mergeSlotProps<NotificationCenterRegionSlotProps>(
                  {
                    className: 'shrink-0 text-[11px] text-[var(--sdk-color-text-muted)]',
                    'data-sdk-region': 'notification-center-item-time-label',
                  },
                  slotProps?.timeLabel,
                )}
              >
                {item.timeLabel}
              </div>
            ) : null}
          </div>
          <div
            {...mergeSlotProps<NotificationCenterRegionSlotProps>(
              {
                className: 'mt-1 text-sm leading-relaxed text-[var(--sdk-color-text-secondary)]',
                'data-sdk-region': 'notification-center-item-description',
              },
              slotProps?.description,
            )}
          >
            {item.content}
          </div>
        </div>
      </>
    );

    return (
      <article
        ref={ref}
        className={cn(
          'relative px-4 py-4 transition-colors',
          onItemSelect ? 'hover:bg-[var(--sdk-color-surface-panel-muted)]' : undefined,
          className,
        )}
        data-sdk-ui="notification-center-item"
        data-tone={tone}
        data-unread={item.unread ? 'true' : 'false'}
        {...props}
      >
        {item.unread ? (
          <span
            {...mergeSlotProps<NotificationCenterItemIndicatorSlotProps>(
              {
                'aria-hidden': 'true',
                className:
                  'absolute bottom-3 left-0 top-3 w-[3px] rounded-r-full bg-[var(--sdk-color-brand-primary)]',
                'data-sdk-region': 'notification-center-item-unread-indicator',
              },
              slotProps?.unreadIndicator,
            )}
          />
        ) : null}

        {onItemSelect ? (
          <button
            aria-label={resolveItemSelectLabel(item)}
            className="flex w-full items-start gap-3 text-left"
            onClick={() => onItemSelect(item.id)}
            type="button"
          >
            {itemBody}
          </button>
        ) : (
          <div className="flex items-start gap-3">{itemBody}</div>
        )}

        {item.actionHref ? (
          <a
            {...mergeSlotProps<NotificationCenterItemLinkSlotProps>(
              {
                className:
                  'mt-3 inline-flex items-center gap-1 text-xs font-medium text-[var(--sdk-color-brand-primary)] transition-colors hover:text-[var(--sdk-color-brand-primary-hover)]',
                'data-sdk-region': 'notification-center-item-action',
                href: item.actionHref,
                rel: 'noreferrer',
                target: item.actionTarget ?? '_blank',
              },
              slotProps?.action,
            )}
          >
            {item.actionLabel ?? 'View details'}
            <ExternalLink className="h-3.5 w-3.5" />
          </a>
        ) : null}
      </article>
    );
  },
);

NotificationCenterItem.displayName = 'NotificationCenterItem';

const NotificationCenter = React.forwardRef<HTMLElement, NotificationCenterProps>(
  (
    {
      className,
      clearLabel = 'Clear notifications',
      emptyDescription = 'New alerts, updates, and workflow events will appear here.',
      emptyTitle = 'No notifications',
      getItemProps,
      getItemSlotProps,
      items,
      markAllAsReadLabel = 'Mark all as read',
      onClear,
      onItemSelect,
      onMarkAllAsRead,
      slotProps,
      title = 'Notifications',
      ...props
    },
    ref,
  ) => (
    <section
      ref={ref}
      className={cn(
        'overflow-hidden rounded-[var(--sdk-radius-panel)] border border-[var(--sdk-color-border-default)] bg-[var(--sdk-color-surface-panel)] shadow-[var(--sdk-shadow-lg)]',
        className,
      )}
      data-sdk-ui="notification-center"
      {...props}
    >
      <header
        {...mergeSlotProps<NotificationCenterRegionSlotProps>(
          {
            className:
              'flex items-center justify-between gap-3 border-b border-[var(--sdk-color-border-subtle)] bg-[var(--sdk-color-surface-panel-muted)] px-4 py-3',
            'data-sdk-region': 'notification-center-header',
          },
          slotProps?.header,
        )}
      >
        <div className="flex min-w-0 items-center gap-2">
          <Bell className="h-4 w-4 text-[var(--sdk-color-text-muted)]" />
          <div className="truncate text-xs font-semibold uppercase tracking-[0.16em] text-[var(--sdk-color-text-secondary)]">
            {title}
          </div>
        </div>

        {items.length > 0 && (onMarkAllAsRead || onClear) ? (
          <div
            {...mergeSlotProps<NotificationCenterRegionSlotProps>(
              {
                className: 'flex shrink-0 items-center gap-1',
                'data-sdk-region': 'notification-center-header-actions',
              },
              slotProps?.headerActions,
            )}
          >
            {onMarkAllAsRead ? (
              <button
                aria-label={markAllAsReadLabel}
                className="rounded-[var(--sdk-radius-control)] p-2 text-[var(--sdk-color-text-muted)] transition-colors hover:bg-[var(--sdk-color-surface-elevated)] hover:text-[var(--sdk-color-text-primary)]"
                onClick={onMarkAllAsRead}
                type="button"
              >
                <CheckCheck className="h-4 w-4" />
              </button>
            ) : null}
            {onClear ? (
              <button
                aria-label={clearLabel}
                className="rounded-[var(--sdk-radius-control)] p-2 text-[var(--sdk-color-text-muted)] transition-colors hover:bg-[var(--sdk-color-surface-elevated)] hover:text-[var(--sdk-color-state-danger)]"
                onClick={onClear}
                type="button"
              >
                <Trash2 className="h-4 w-4" />
              </button>
            ) : null}
          </div>
        ) : null}
      </header>

      {items.length === 0 ? (
        <div
          {...mergeSlotProps<NotificationCenterRegionSlotProps>(
            {
              className: 'flex flex-col items-center justify-center px-6 py-10 text-center',
              'data-sdk-region': 'notification-center-empty',
            },
            slotProps?.empty,
          )}
        >
          <Bell className="h-8 w-8 text-[var(--sdk-color-text-muted)] opacity-40" />
          <div className="mt-3 text-sm font-semibold text-[var(--sdk-color-text-primary)]">{emptyTitle}</div>
          <div className="mt-1 max-w-xs text-sm text-[var(--sdk-color-text-secondary)]">{emptyDescription}</div>
        </div>
      ) : (
        <div
          {...mergeSlotProps<NotificationCenterRegionSlotProps>(
            {
              className: 'max-h-[26rem] overflow-y-auto',
              'data-sdk-region': 'notification-center-viewport',
            },
            slotProps?.viewport,
          )}
        >
          <ol
            {...mergeSlotProps<NotificationCenterListSlotProps>(
              {
                className: 'divide-y divide-[var(--sdk-color-border-subtle)]',
                'data-sdk-region': 'notification-center-list',
              },
              slotProps?.list,
            )}
          >
            {items.map((item, index) => (
              <li className="list-none" key={item.id}>
                <NotificationCenterItem
                  {...getItemProps?.(item, index)}
                  item={item}
                  onItemSelect={onItemSelect}
                  slotProps={mergeNotificationCenterItemSlotProps(getItemSlotProps?.(item, index), undefined)}
                />
              </li>
            ))}
          </ol>
        </div>
      )}
    </section>
  ),
);

NotificationCenter.displayName = 'NotificationCenter';

export { NotificationCenter, NotificationCenterItem };
