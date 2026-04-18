import * as React from 'react';
import { EmptyState, LoadingBlock } from '../../patterns/feedback';
import { mergeSlotProps, type SlotProps } from '../../../lib/slot-props';
import { cn } from '../../../lib/utils';

export type CollectionGridRegionSlotProps = SlotProps<Omit<React.ComponentPropsWithoutRef<'div'>, 'children'>>;
export type CollectionGridItemKeyResolver<T = any> = (item: T, index: number) => React.Key;
export type CollectionGridItemRenderer<T = any> = (item: T, index: number) => React.ReactNode;

export interface CollectionGridSlotProps {
  grid?: CollectionGridRegionSlotProps;
  pagination?: CollectionGridRegionSlotProps;
}

export interface CollectionGridProps<T = any> extends React.HTMLAttributes<HTMLDivElement> {
  emptyActions?: React.ReactNode;
  emptyDescription?: React.ReactNode;
  emptyTitle?: React.ReactNode;
  getItemKey?: CollectionGridItemKeyResolver<T>;
  items: T[];
  loading?: boolean;
  loadingLabel?: React.ReactNode;
  pagination?: React.ReactNode;
  renderItem: CollectionGridItemRenderer<T>;
  slotProps?: CollectionGridSlotProps;
}

type CollectionGridComponent = React.ForwardRefExoticComponent<
  CollectionGridProps & React.RefAttributes<HTMLDivElement>
> & {
  <T = any>(props: CollectionGridProps<T> & React.RefAttributes<HTMLDivElement>): React.ReactNode;
};

const CollectionGrid: CollectionGridComponent = React.forwardRef<HTMLDivElement, CollectionGridProps>(({
  className,
  emptyActions,
  emptyDescription,
  emptyTitle = 'No items',
  getItemKey,
  items,
  loading = false,
  loadingLabel,
  pagination,
  renderItem,
  slotProps,
  ...props
}, ref) => {
  return (
    <div
      ref={ref}
      className={cn('flex flex-col gap-6', className)}
      data-sdk-ui="collection-grid"
      data-slot="collection-grid"
      {...props}
    >
      {loading ? (
        <LoadingBlock label={loadingLabel} />
      ) : items.length === 0 ? (
        <EmptyState actions={emptyActions} description={emptyDescription} title={emptyTitle} />
      ) : (
        <>
          <div
            {...mergeSlotProps<CollectionGridRegionSlotProps>(
              {
                className: 'grid grid-cols-1 gap-4 md:grid-cols-2 xl:grid-cols-3',
              },
              slotProps?.grid,
            )}
            data-sdk-region="collection-grid-grid"
          >
            {items.map((item, index) => (
              <React.Fragment
                key={getItemKey?.(item, index) ?? (item as { id?: React.Key }).id ?? index}
              >
                {renderItem(item, index)}
              </React.Fragment>
            ))}
          </div>
          {pagination ? (
            <div
              {...mergeSlotProps<CollectionGridRegionSlotProps>(
                {},
                slotProps?.pagination,
              )}
              data-sdk-region="collection-grid-pagination"
            >
              {pagination}
            </div>
          ) : null}
        </>
      )}
    </div>
  );
});

export { CollectionGrid };
CollectionGrid.displayName = 'CollectionGrid';
