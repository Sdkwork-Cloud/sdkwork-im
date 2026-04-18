import * as React from 'react';
import { mergeSlotProps, type SlotProps } from '../../../lib/slot-props';
import { cn } from '../../../lib/utils';

export type TimelineItemStatus =
  | 'default'
  | 'success'
  | 'warning'
  | 'danger'
  | 'pending'
  | 'done'
  | 'current'
  | 'error';

export interface TimelineItem {
  id?: React.Key;
  title: React.ReactNode;
  description?: React.ReactNode;
  timestamp?: React.ReactNode;
  content?: React.ReactNode;
  icon?: React.ReactNode;
  status?: TimelineItemStatus;
}

export type TimelineItemRootProps = SlotProps<Omit<React.ComponentPropsWithoutRef<'li'>, 'children'>>;
export type TimelineItemRegionSlotProps = SlotProps<Omit<React.ComponentPropsWithoutRef<'div'>, 'children'>>;
export type TimelineItemIndicatorSlotProps = SlotProps<Omit<React.ComponentPropsWithoutRef<'span'>, 'children'>>;
export type TimelineItemPropsResolver = (item: TimelineItem, index: number) => TimelineItemRootProps | undefined;
export type TimelineItemSlotPropsResolver = (item: TimelineItem, index: number) => TimelineItemSlotProps | undefined;

export interface TimelineItemSlotProps {
  connector?: TimelineItemIndicatorSlotProps;
  content?: TimelineItemRegionSlotProps;
  indicator?: TimelineItemIndicatorSlotProps;
}

export interface TimelineProps extends React.OlHTMLAttributes<HTMLOListElement> {
  getItemProps?: TimelineItemPropsResolver;
  getItemSlotProps?: TimelineItemSlotPropsResolver;
  items: TimelineItem[];
}

type ResolvedTimelineStatus = 'default' | 'success' | 'warning' | 'danger' | 'pending';

const timelineStatusAlias: Record<Exclude<TimelineItemStatus, ResolvedTimelineStatus>, ResolvedTimelineStatus> = {
  current: 'warning',
  done: 'success',
  error: 'danger',
};

const timelineIndicatorShellClassName: Record<ResolvedTimelineStatus, string> = {
  default:
    'border-[var(--sdk-color-border-default)] bg-[var(--sdk-color-surface-panel-muted)] text-[var(--sdk-color-text-muted)]',
  success:
    'border-[var(--sdk-color-border-default)] bg-[var(--sdk-color-surface-panel-muted)] text-[var(--sdk-color-state-success)]',
  warning:
    'border-[var(--sdk-color-border-default)] bg-[var(--sdk-color-surface-panel-muted)] text-[var(--sdk-color-state-warning)]',
  danger:
    'border-[var(--sdk-color-border-default)] bg-[var(--sdk-color-surface-panel-muted)] text-[var(--sdk-color-state-danger)]',
  pending:
    'border-[var(--sdk-color-border-default)] bg-[var(--sdk-color-surface-panel-muted)] text-[var(--sdk-color-brand-primary)]',
};

const timelineIndicatorDotClassName: Record<ResolvedTimelineStatus, string> = {
  default: 'bg-[var(--sdk-color-text-muted)]',
  success: 'bg-[var(--sdk-color-state-success)]',
  warning: 'bg-[var(--sdk-color-state-warning)]',
  danger: 'bg-[var(--sdk-color-state-danger)]',
  pending: 'bg-[var(--sdk-color-brand-primary)]',
};

function resolveTimelineStatus(status?: TimelineItemStatus): ResolvedTimelineStatus {
  if (!status) {
    return 'default';
  }

  return timelineStatusAlias[status as keyof typeof timelineStatusAlias] ?? status;
}

const Timeline = React.forwardRef<HTMLOListElement, TimelineProps>(({
  className,
  getItemProps,
  getItemSlotProps,
  items,
  ...props
}, ref) => (
  <ol
    ref={ref}
    className={cn('space-y-0', className)}
    data-sdk-ui="timeline"
    {...props}
  >
    {items.map((item, index) => {
      const status = resolveTimelineStatus(item.status);
      const itemProps = getItemProps?.(item, index);
      const itemSlotProps = getItemSlotProps?.(item, index);

      return (
        <li
          {...mergeSlotProps<TimelineItemRootProps>(
            {
              className: 'grid grid-cols-[auto_minmax(0,1fr)] gap-4 pb-5 last:pb-0',
              'data-sdk-ui': 'timeline-item',
              'data-status': status,
            },
            itemProps,
          )}
          key={item.id ?? index}
        >
          <div className="flex flex-col items-center">
            <span
              {...mergeSlotProps<TimelineItemIndicatorSlotProps>(
                {
                  className: cn(
                    'flex h-8 w-8 shrink-0 items-center justify-center rounded-full border',
                    timelineIndicatorShellClassName[status],
                  ),
                  'data-sdk-region': 'timeline-item-indicator',
                },
                itemSlotProps?.indicator,
              )}
            >
              {item.icon ? (
                item.icon
              ) : (
                <span
                  aria-hidden="true"
                  className={cn('h-2.5 w-2.5 rounded-full', timelineIndicatorDotClassName[status])}
                />
              )}
            </span>
            {index < items.length - 1 ? (
              <span
                {...mergeSlotProps<TimelineItemIndicatorSlotProps>(
                  {
                    'aria-hidden': 'true',
                    className: 'mt-2 w-px flex-1 rounded-full bg-[var(--sdk-color-border-default)]',
                    'data-sdk-region': 'timeline-item-connector',
                  },
                  itemSlotProps?.connector,
                )}
              />
            ) : null}
          </div>
          <div
            {...mergeSlotProps<TimelineItemRegionSlotProps>(
              {
                className:
                  'min-w-0 rounded-[var(--sdk-radius-panel)] border border-[var(--sdk-color-border-default)] bg-[var(--sdk-color-surface-panel)] px-4 py-3 shadow-[var(--sdk-shadow-soft)]',
                'data-sdk-region': 'timeline-item-content',
              },
              itemSlotProps?.content,
            )}
          >
            <div className="flex flex-wrap items-start justify-between gap-2">
              <div className="text-sm font-semibold text-[var(--sdk-color-text-primary)]">{item.title}</div>
              {item.timestamp ? (
                <div className="text-xs font-medium text-[var(--sdk-color-text-muted)]">{item.timestamp}</div>
              ) : null}
            </div>
            {item.description ? (
              <div className="mt-1.5 text-sm text-[var(--sdk-color-text-secondary)]">{item.description}</div>
            ) : null}
            {item.content ? <div className="mt-3">{item.content}</div> : null}
          </div>
        </li>
      );
    })}
  </ol>
));

Timeline.displayName = 'Timeline';

export { Timeline };
