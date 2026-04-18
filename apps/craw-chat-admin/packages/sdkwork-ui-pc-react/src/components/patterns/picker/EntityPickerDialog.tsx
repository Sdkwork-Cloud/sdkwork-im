import * as React from 'react';
import {
  mergePatternSlotProps,
  type PatternSlotProps,
} from '../_internal/slot-props';
import type { ButtonProps } from '../../ui/button';
import {
  PickerDialog,
  type PickerDialogProps,
  type PickerDialogSlotProps,
} from './PickerDialog';
import { PickerSelectionFooter } from './PickerSelectionFooter';

export type EntityPickerSelectionMode = 'single' | 'multiple';
export type EntityPickerDialogSelectionActionHandler = () => void;

export interface EntityPickerDialogSelectionContext<T> {
  clearSelection: EntityPickerDialogSelectionActionHandler;
  selectedCount: number;
  selectedIds: React.Key[];
  selectedItems: T[];
  selectionMode: EntityPickerSelectionMode;
}

export interface EntityPickerDialogRenderItemContext<T>
  extends EntityPickerDialogSelectionContext<T> {
  deselect: EntityPickerDialogSelectionActionHandler;
  index: number;
  item: T;
  itemId: React.Key;
  select: EntityPickerDialogSelectionActionHandler;
  selected: boolean;
  toggleSelection: EntityPickerDialogSelectionActionHandler;
}

export type EntityPickerDialogConfirmLabelRenderer<T> = (
  context: EntityPickerDialogSelectionContext<T>,
) => React.ReactNode;
export type EntityPickerDialogConfirmHandler<T> = (
  selectedItems: T[],
  selectedIds: React.Key[],
) => void;
export type EntityPickerDialogItemIdResolver<T> = (item: T, index: number) => React.Key;
export type EntityPickerDialogItemLabelRenderer<T> = (item: T, index: number) => React.ReactNode;
export type EntityPickerDialogItemRenderer<T> = (
  context: EntityPickerDialogRenderItemContext<T>,
) => React.ReactNode;
export type EntityPickerDialogSelectedIdsChangeHandler<T> = (
  selectedIds: React.Key[],
  selectedItems: T[],
) => void;
export type EntityPickerDialogSelectionSummaryRenderer<T> = (
  context: EntityPickerDialogSelectionContext<T>,
) => React.ReactNode;

export type EntityPickerDialogRegionSlotProps = PatternSlotProps<
  Omit<React.ComponentPropsWithoutRef<'div'>, 'children'>
>;

export interface EntityPickerDialogSlotProps extends PickerDialogSlotProps {
  items?: EntityPickerDialogRegionSlotProps;
  pagination?: EntityPickerDialogRegionSlotProps;
}

export interface EntityPickerDialogProps<T = any>
  extends Omit<PickerDialogProps, 'children' | 'empty' | 'footer' | 'slotProps'> {
  cancelLabel?: React.ReactNode;
  clearLabel?: React.ReactNode;
  confirmLabel?:
    | React.ReactNode
    | EntityPickerDialogConfirmLabelRenderer<T>;
  confirmLoading?: boolean;
  confirmVariant?: ButtonProps['variant'];
  defaultSelectedIds?: React.Key[];
  getItemId?: EntityPickerDialogItemIdResolver<T>;
  getItemLabel?: EntityPickerDialogItemLabelRenderer<T>;
  items: T[];
  layout?: 'grid' | 'list';
  onConfirm?: EntityPickerDialogConfirmHandler<T>;
  onSelectedIdsChange?: EntityPickerDialogSelectedIdsChangeHandler<T>;
  pagination?: React.ReactNode;
  renderItem: EntityPickerDialogItemRenderer<T>;
  selectedIds?: React.Key[];
  selectionMode?: EntityPickerSelectionMode;
  selectionSummary?:
    | React.ReactNode
    | EntityPickerDialogSelectionSummaryRenderer<T>;
  slotProps?: EntityPickerDialogSlotProps;
}

interface EntityPickerItemRecord<T> {
  id: React.Key;
  item: T;
}

function resolveDefaultItemId<T>(item: T, index: number): React.Key {
  const resolvedId = (item as { id?: React.Key }).id;
  return resolvedId ?? index;
}

type EntityPickerDialogComponent = React.ForwardRefExoticComponent<
  EntityPickerDialogProps & React.RefAttributes<HTMLDivElement>
> & {
  <T = any>(props: EntityPickerDialogProps<T> & React.RefAttributes<HTMLDivElement>): React.ReactNode;
};

function EntityPickerDialogInner<T = any>({
  cancelLabel = 'Cancel',
  clearLabel = 'Clear selection',
  confirmLabel,
  confirmLoading = false,
  confirmVariant = 'primary',
  defaultSelectedIds = [],
  getItemId,
  getItemLabel,
  items,
  layout = 'grid',
  onConfirm,
  onOpenChange,
  onSelectedIdsChange,
  pagination,
  renderItem,
  selectedIds: controlledSelectedIds,
  selectionMode = 'single',
  selectionSummary,
  slotProps,
  ...pickerProps
}: EntityPickerDialogProps<T>, ref: React.ForwardedRef<HTMLDivElement>) {
  const [uncontrolledSelectedIds, setUncontrolledSelectedIds] =
    React.useState<React.Key[]>(defaultSelectedIds);

  const resolveItemId = React.useCallback(
    (item: T, index: number) => getItemId?.(item, index) ?? resolveDefaultItemId(item, index),
    [getItemId],
  );

  const itemRecords = React.useMemo<EntityPickerItemRecord<T>[]>(
    () =>
      items.map((item, index) => ({
        id: resolveItemId(item, index),
        item,
      })),
    [items, resolveItemId],
  );

  const selectedIds = controlledSelectedIds ?? uncontrolledSelectedIds;

  const selectedIdSet = React.useMemo(() => new Set(selectedIds), [selectedIds]);

  const currentItemsById = React.useMemo(() => {
    const map = new Map<React.Key, T>();
    itemRecords.forEach(({ id, item }) => {
      map.set(id, item);
    });
    return map;
  }, [itemRecords]);

  const selectedItems = React.useMemo(
    () =>
      selectedIds.flatMap((id) => {
        const item = currentItemsById.get(id);
        return item === undefined ? [] : [item];
      }),
    [currentItemsById, selectedIds],
  );

  const commitSelection = React.useCallback(
    (nextSelectedIds: React.Key[]) => {
      if (controlledSelectedIds === undefined) {
        setUncontrolledSelectedIds(nextSelectedIds);
      }

      const nextSelectedItems = nextSelectedIds.flatMap((id) => {
        const item = currentItemsById.get(id);
        return item === undefined ? [] : [item];
      });

      onSelectedIdsChange?.(nextSelectedIds, nextSelectedItems);
    },
    [controlledSelectedIds, currentItemsById, onSelectedIdsChange],
  );

  const clearSelection = React.useCallback(() => {
    commitSelection([]);
  }, [commitSelection]);

  const selectionContext = React.useMemo<EntityPickerDialogSelectionContext<T>>(
    () => ({
      clearSelection,
      selectedCount: selectedIds.length,
      selectedIds,
      selectedItems,
      selectionMode,
    }),
    [clearSelection, selectedIds, selectedItems, selectionMode],
  );

  const isSelected = React.useCallback((id: React.Key) => selectedIdSet.has(id), [selectedIdSet]);

  const selectId = React.useCallback(
    (id: React.Key) => {
      if (selectionMode === 'single') {
        commitSelection([id]);
        return;
      }

      if (selectedIdSet.has(id)) {
        return;
      }

      commitSelection([...selectedIds, id]);
    },
    [commitSelection, selectedIdSet, selectedIds, selectionMode],
  );

  const deselectId = React.useCallback(
    (id: React.Key) => {
      commitSelection(selectedIds.filter((selectedId) => !Object.is(selectedId, id)));
    },
    [commitSelection, selectedIds],
  );

  const toggleId = React.useCallback(
    (id: React.Key) => {
      if (selectedIdSet.has(id)) {
        deselectId(id);
        return;
      }

      if (selectionMode === 'single') {
        commitSelection([id]);
        return;
      }

      commitSelection([...selectedIds, id]);
    },
    [commitSelection, deselectId, selectedIdSet, selectedIds, selectionMode],
  );

  const resolvedSelectionSummary =
    typeof selectionSummary === 'function'
      ? selectionSummary(selectionContext)
      : selectionSummary ?? (() => {
          if (selectedIds.length === 0) {
            return 'No items selected';
          }

          if (selectedIds.length === 1 && selectedItems.length === 1 && getItemLabel) {
            return (
              <>
                Selected: {getItemLabel(selectedItems[0], 0)}
              </>
            );
          }

          return `${selectedIds.length} item${selectedIds.length === 1 ? '' : 's'} selected`;
        })();

  const resolvedConfirmLabel =
    typeof confirmLabel === 'function'
      ? confirmLabel(selectionContext)
      : confirmLabel ??
        (selectionMode === 'multiple' && selectedIds.length > 0
          ? `Confirm selection (${selectedIds.length})`
          : 'Confirm selection');

  const resultsLayoutClassName =
    layout === 'list'
      ? 'flex flex-col gap-3'
      : 'grid grid-cols-1 gap-3 md:grid-cols-2 xl:grid-cols-3';

  return (
    <PickerDialog
      {...pickerProps}
      empty={!pickerProps.loading && !pickerProps.error && items.length === 0}
      footer={(
        <PickerSelectionFooter
          cancelLabel={cancelLabel}
          clearLabel={clearLabel}
          confirmDisabled={selectedIds.length === 0}
          confirmLabel={resolvedConfirmLabel}
          confirmLoading={confirmLoading}
          confirmVariant={confirmVariant}
          onCancel={() => onOpenChange?.(false)}
          onClear={clearSelection}
          onConfirm={() => {
            if (selectedIds.length === 0) {
              return;
            }

            onConfirm?.(selectedItems, selectedIds);
          }}
          showClear={selectedIds.length > 0}
          summary={resolvedSelectionSummary}
        />
      )}
      slotProps={slotProps}
    >
      <div
        ref={ref}
        className="flex flex-col gap-5"
        data-sdk-pattern="entity-picker-dialog"
      >
        <div
          {...mergePatternSlotProps<EntityPickerDialogRegionSlotProps>(
            {
              className: resultsLayoutClassName,
              'data-sdk-region': 'entity-picker-items',
            },
            slotProps?.items,
          )}
        >
          {itemRecords.map((record, index) => (
            <React.Fragment key={record.id}>
              {renderItem({
                ...selectionContext,
                deselect: () => deselectId(record.id),
                index,
                item: record.item,
                itemId: record.id,
                select: () => selectId(record.id),
                selected: isSelected(record.id),
                toggleSelection: () => toggleId(record.id),
              })}
            </React.Fragment>
          ))}
        </div>
        {pagination ? (
          <div
            {...mergePatternSlotProps<EntityPickerDialogRegionSlotProps>(
              {
                'data-sdk-region': 'entity-picker-pagination',
              },
              slotProps?.pagination,
            )}
          >
            {pagination}
          </div>
        ) : null}
      </div>
    </PickerDialog>
  );
}

const EntityPickerDialog: EntityPickerDialogComponent = React.forwardRef<
  HTMLDivElement,
  EntityPickerDialogProps
>(
  EntityPickerDialogInner as (
    props: EntityPickerDialogProps,
    ref: React.ForwardedRef<HTMLDivElement>,
  ) => React.ReactNode,
);

export { EntityPickerDialog };
EntityPickerDialog.displayName = 'EntityPickerDialog';
