import * as React from 'react';
import { Activity, AlertCircle, AlertTriangle, CheckCircle2, Info } from 'lucide-react';
import { mergeOptionalSlotProps, mergeSlotProps, type SlotProps } from '../../../lib/slot-props';
import { EmptyState } from '../../patterns/feedback';
import { cn } from '../../../lib/utils';

export type ActivityFeedTone = 'default' | 'info' | 'success' | 'warning' | 'danger';

export interface ActivityFeedEntry {
  actions?: React.ReactNode;
  content?: React.ReactNode;
  description?: React.ReactNode;
  icon?: React.ReactNode;
  id: string;
  meta?: React.ReactNode;
  timestamp?: React.ReactNode;
  title: React.ReactNode;
  tone?: ActivityFeedTone;
  unread?: boolean;
}

export type ActivityFeedRegionSlotProps = SlotProps<Omit<React.ComponentPropsWithoutRef<'div'>, 'children'>>;
export type ActivityFeedListSlotProps = SlotProps<Omit<React.ComponentPropsWithoutRef<'ol'>, 'children'>>;
export type ActivityFeedItemRootProps = SlotProps<
  Omit<React.ComponentPropsWithoutRef<'div'>, 'children' | 'title'>
>;
export type ActivityFeedIndicatorSlotProps = SlotProps<Omit<React.ComponentPropsWithoutRef<'span'>, 'children'>>;
export type ActivityFeedPanelSlotProps = SlotProps<Omit<React.ComponentPropsWithoutRef<'article'>, 'children'>>;
export type ActivityFeedItemPropsResolver = (
  item: ActivityFeedEntry,
  index: number,
) => ActivityFeedItemRootProps | undefined;
export type ActivityFeedItemSlotPropsResolver = (
  item: ActivityFeedEntry,
  index: number,
) => ActivityFeedItemSlotProps | undefined;
export type ActivityFeedItemSelectHandler = (id: string) => void;

export interface ActivityFeedItemSlotProps {
  actions?: ActivityFeedRegionSlotProps;
  connector?: ActivityFeedIndicatorSlotProps;
  indicator?: ActivityFeedIndicatorSlotProps;
  panel?: ActivityFeedPanelSlotProps;
}

export interface ActivityFeedSlotProps {
  empty?: ActivityFeedRegionSlotProps;
  header?: ActivityFeedRegionSlotProps;
  headerActions?: ActivityFeedRegionSlotProps;
  list?: ActivityFeedListSlotProps;
}

export interface ActivityFeedProps extends Omit<React.HTMLAttributes<HTMLElement>, 'title'> {
  description?: React.ReactNode;
  emptyDescription?: React.ReactNode;
  emptyTitle?: React.ReactNode;
  getItemProps?: ActivityFeedItemPropsResolver;
  getItemSlotProps?: ActivityFeedItemSlotPropsResolver;
  headerActions?: React.ReactNode;
  items: ActivityFeedEntry[];
  onItemSelect?: ActivityFeedItemSelectHandler;
  slotProps?: ActivityFeedSlotProps;
  title?: React.ReactNode;
}

export interface ActivityFeedItemProps extends Omit<React.HTMLAttributes<HTMLDivElement>, 'title'> {
  item: ActivityFeedEntry;
  onItemSelect?: ActivityFeedItemSelectHandler;
  showConnector?: boolean;
  slotProps?: ActivityFeedItemSlotProps;
}

const toneIcon = {
  default: Activity,
  info: Info,
  success: CheckCircle2,
  warning: AlertTriangle,
  danger: AlertCircle,
} as const;

const toneShellClass: Record<ActivityFeedTone, string> = {
  default:
    'border-[var(--sdk-color-border-default)] bg-[var(--sdk-color-surface-panel-muted)] text-[var(--sdk-color-text-muted)]',
  info:
    'border-[var(--sdk-color-border-default)] bg-[var(--sdk-color-surface-panel-muted)] text-[var(--sdk-color-state-info)]',
  success:
    'border-[var(--sdk-color-border-default)] bg-[var(--sdk-color-surface-panel-muted)] text-[var(--sdk-color-state-success)]',
  warning:
    'border-[var(--sdk-color-border-default)] bg-[var(--sdk-color-surface-panel-muted)] text-[var(--sdk-color-state-warning)]',
  danger:
    'border-[var(--sdk-color-border-default)] bg-[var(--sdk-color-surface-panel-muted)] text-[var(--sdk-color-state-danger)]',
};

function resolveItemSelectLabel(item: ActivityFeedEntry) {
  return typeof item.title === 'string' ? item.title : 'Open activity item';
}

function mergeActivityFeedItemSlotProps(
  baseSlotProps?: ActivityFeedItemSlotProps,
  overrideSlotProps?: ActivityFeedItemSlotProps,
): ActivityFeedItemSlotProps | undefined {
  const mergedSlotProps: ActivityFeedItemSlotProps = {};

  const actions = mergeOptionalSlotProps(baseSlotProps?.actions, overrideSlotProps?.actions);
  const connector = mergeOptionalSlotProps(baseSlotProps?.connector, overrideSlotProps?.connector);
  const indicator = mergeOptionalSlotProps(baseSlotProps?.indicator, overrideSlotProps?.indicator);
  const panel = mergeOptionalSlotProps(baseSlotProps?.panel, overrideSlotProps?.panel);

  if (actions) {
    mergedSlotProps.actions = actions;
  }

  if (connector) {
    mergedSlotProps.connector = connector;
  }

  if (indicator) {
    mergedSlotProps.indicator = indicator;
  }

  if (panel) {
    mergedSlotProps.panel = panel;
  }

  return Object.keys(mergedSlotProps).length > 0 ? mergedSlotProps : undefined;
}

const ActivityFeedItem = React.forwardRef<HTMLDivElement, ActivityFeedItemProps>(
  ({ className, item, onItemSelect, showConnector = true, slotProps, ...props }, ref) => {
    const tone = item.tone ?? 'default';
    const Icon = toneIcon[tone];

    return (
      <div
        ref={ref}
        className={cn('grid grid-cols-[auto_minmax(0,1fr)] gap-4', className)}
        data-sdk-ui="activity-feed-item"
        data-tone={tone}
        data-unread={item.unread ? 'true' : 'false'}
        {...props}
      >
        <div className="flex flex-col items-center">
          <span
            {...mergeSlotProps<ActivityFeedIndicatorSlotProps>(
              {
                className: cn(
                  'relative flex h-10 w-10 shrink-0 items-center justify-center rounded-[var(--sdk-radius-panel)] border shadow-[var(--sdk-shadow-sm)]',
                  toneShellClass[tone],
                ),
                'data-sdk-region': 'activity-feed-item-indicator',
              },
              slotProps?.indicator,
            )}
          >
            {item.icon ? item.icon : <Icon className="h-4.5 w-4.5" />}
            {item.unread ? (
              <span
                aria-hidden="true"
                className="absolute -right-0.5 -top-0.5 h-2.5 w-2.5 rounded-full border border-[var(--sdk-color-surface-panel)] bg-[var(--sdk-color-brand-primary)]"
              />
            ) : null}
          </span>
          {showConnector ? (
            <span
              {...mergeSlotProps<ActivityFeedIndicatorSlotProps>(
                {
                  'aria-hidden': 'true',
                  className: 'mt-2 w-px flex-1 rounded-full bg-[var(--sdk-color-border-default)]',
                  'data-sdk-region': 'activity-feed-item-connector',
                },
                slotProps?.connector,
              )}
            />
          ) : null}
        </div>

        <article
          {...mergeSlotProps<ActivityFeedPanelSlotProps>(
            {
              className:
                'min-w-0 rounded-[var(--sdk-radius-panel)] border border-[var(--sdk-color-border-default)] bg-[var(--sdk-color-surface-panel)] px-4 py-4 shadow-[var(--sdk-shadow-soft)]',
              'data-sdk-region': 'activity-feed-item-panel',
            },
            slotProps?.panel,
          )}
        >
          <div className="flex flex-wrap items-start justify-between gap-3">
            <div className="min-w-0 flex-1">
              <div className="flex flex-wrap items-center gap-2">
                {onItemSelect ? (
                  <button
                    aria-label={resolveItemSelectLabel(item)}
                    className="truncate text-left text-sm font-semibold text-[var(--sdk-color-text-primary)] transition-colors hover:text-[var(--sdk-color-brand-primary-hover)]"
                    onClick={() => onItemSelect(item.id)}
                    type="button"
                  >
                    {item.title}
                  </button>
                ) : (
                  <div className="truncate text-sm font-semibold text-[var(--sdk-color-text-primary)]">
                    {item.title}
                  </div>
                )}
                {item.meta ? (
                  <div className="inline-flex max-w-full items-center rounded-full border border-[var(--sdk-color-border-subtle)] bg-[var(--sdk-color-surface-panel-muted)] px-2.5 py-1 text-[11px] font-medium text-[var(--sdk-color-text-secondary)]">
                    <span className="truncate">{item.meta}</span>
                  </div>
                ) : null}
              </div>

              {item.description ? (
                <div className="mt-1.5 text-sm leading-relaxed text-[var(--sdk-color-text-secondary)]">
                  {item.description}
                </div>
              ) : null}
            </div>

            {item.timestamp ? (
              <div className="shrink-0 text-xs font-medium text-[var(--sdk-color-text-muted)]">
                {item.timestamp}
              </div>
            ) : null}
          </div>

          {item.content ? (
            <div className="mt-3 text-sm text-[var(--sdk-color-text-secondary)]">{item.content}</div>
          ) : null}

          {item.actions ? (
            <div
              {...mergeSlotProps<ActivityFeedRegionSlotProps>(
                {
                  className: 'mt-3 flex flex-wrap items-center gap-2',
                  'data-sdk-region': 'activity-feed-item-actions',
                },
                slotProps?.actions,
              )}
            >
              {item.actions}
            </div>
          ) : null}
        </article>
      </div>
    );
  },
);

ActivityFeedItem.displayName = 'ActivityFeedItem';

const ActivityFeed = React.forwardRef<HTMLElement, ActivityFeedProps>(
  (
    {
      className,
      description,
      emptyDescription = 'Workflow updates, approvals, and automation history will appear here.',
      emptyTitle = 'No activity yet',
      getItemProps,
      getItemSlotProps,
      headerActions,
      items,
      onItemSelect,
      slotProps,
      title = 'Activity',
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
      data-sdk-ui="activity-feed"
      {...props}
    >
      {title || description || headerActions ? (
        <header
          {...mergeSlotProps<ActivityFeedRegionSlotProps>(
            {
              className:
                'flex items-start justify-between gap-4 border-b border-[var(--sdk-color-border-subtle)] bg-[var(--sdk-color-surface-panel-muted)] px-4 py-3',
              'data-sdk-region': 'activity-feed-header',
            },
            slotProps?.header,
          )}
        >
          <div className="min-w-0">
            <div className="flex items-center gap-2">
              <Activity className="h-4 w-4 text-[var(--sdk-color-text-muted)]" />
              <div className="text-xs font-semibold uppercase tracking-[0.16em] text-[var(--sdk-color-text-secondary)]">
                {title}
              </div>
            </div>
            {description ? (
              <div className="mt-1 text-sm text-[var(--sdk-color-text-secondary)]">{description}</div>
            ) : null}
          </div>

          {headerActions ? (
            <div
              {...mergeSlotProps<ActivityFeedRegionSlotProps>(
                {
                  className: 'flex shrink-0 items-center gap-2',
                  'data-sdk-region': 'activity-feed-header-actions',
                },
                slotProps?.headerActions,
              )}
            >
              {headerActions}
            </div>
          ) : null}
        </header>
      ) : null}

      {items.length === 0 ? (
        <div
          {...mergeSlotProps<ActivityFeedRegionSlotProps>(
            {
              className: 'p-4',
              'data-sdk-region': 'activity-feed-empty',
            },
            slotProps?.empty,
          )}
        >
          <EmptyState description={emptyDescription} title={emptyTitle} />
        </div>
      ) : (
        <div className="px-4 py-4">
          <ol
            {...mergeSlotProps<ActivityFeedListSlotProps>(
              {
                className: 'space-y-4',
                'data-sdk-region': 'activity-feed-list',
              },
              slotProps?.list,
            )}
          >
            {items.map((item, index) => (
              <li className="list-none" key={item.id}>
                <ActivityFeedItem
                  {...getItemProps?.(item, index)}
                  item={item}
                  onItemSelect={onItemSelect}
                  showConnector={index < items.length - 1}
                  slotProps={mergeActivityFeedItemSlotProps(getItemSlotProps?.(item, index), undefined)}
                />
              </li>
            ))}
          </ol>
        </div>
      )}
    </section>
  ),
);

ActivityFeed.displayName = 'ActivityFeed';

export { ActivityFeed, ActivityFeedItem };
