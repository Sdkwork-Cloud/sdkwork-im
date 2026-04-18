import * as React from 'react';
import {
  type ColumnDef,
  type SortingState,
  getCoreRowModel,
  getPaginationRowModel,
  getSortedRowModel,
  useReactTable,
} from '@tanstack/react-table';
import { mergeSlotProps } from '../../../lib/slot-props';
import { cn } from '../../../lib/utils';
import { EmptyState, LoadingBlock } from '../../patterns/feedback';
import { BulkActionBar } from '../actions';
import { Checkbox } from '../checkbox';
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from '../table';
import { DataTableHeaderCell } from './data-table/header-cell';
import { DataTablePaginationControls } from './data-table/pagination-controls';
import {
  areSortingStatesEqual,
  clampPage,
  normalizePageSize,
  normalizeSortingState,
  resolvePageSizeOptions,
  resolvePaginationItems,
  toPublicSortingState,
} from './data-table/state';
import {
  dataTableFooterClassName,
  dataTableSummaryClassName,
  dataTableSurfaceClassName,
} from './data-table/styles';
import type {
  DataTableAccessorResolver,
  DataTableAlign,
  DataTableCellProps,
  DataTableColumn,
  DataTableDensity,
  DataTableProps as DataTablePropsContract,
  DataTableRowProps,
} from './data-table/types';

export type {
  DataTableAccessorResolver,
  DataTableAlign,
  DataTableCellProps,
  DataTableCellPropsResolver,
  DataTableCellRenderer,
  DataTableColumn,
  DataTableDensity,
  DataTableHeaderProps,
  DataTablePageChangeHandler,
  DataTablePageSizeChangeHandler,
  DataTablePaginationMode,
  DataTablePaginationProps,
  DataTableRegionSlotProps,
  DataTableRowActionsRenderer,
  DataTableRowClickHandler,
  DataTableRowIdResolver,
  DataTableRowProps,
  DataTableRowPropsResolver,
  DataTableRowSelectionLabelResolver,
  DataTableSelectedRowIdsChangeHandler,
  DataTableSlotProps,
  DataTableSortingChangeHandler,
  DataTableSortingMode,
  DataTableSortingState,
  DataTableSortingStateItem,
  DataTableTableSlotProps,
} from './data-table/types';

export interface DataTableProps<T = any> extends DataTablePropsContract<T> {}

const densityClassName: Record<DataTableDensity, string> = {
  comfortable: 'p-4',
  compact: 'px-4 py-2.5',
};

const alignClassName: Record<DataTableAlign, string> = {
  center: 'text-center',
  left: 'text-left',
  right: 'text-right',
};

function defaultGetRowId<T>(row: T, index: number) {
  return (row as { id?: React.Key }).id ?? index;
}

function resolveColumnAccessor<T>(column: DataTableColumn<T>): DataTableAccessorResolver<T> {
  if (column.accessorFn) {
    return column.accessorFn;
  }

  if (column.accessorKey) {
    return (row: T) => row[column.accessorKey as keyof T];
  }

  return (row: T) => (row as Record<string, unknown>)[column.id];
}

function normalizeColumnVisibility(columnVisibility?: Partial<Record<string, boolean>>) {
  return Object.fromEntries(
    Object.entries(columnVisibility ?? {}).filter((entry): entry is [string, boolean] => entry[1] !== undefined),
  );
}

function appendUniqueKeys(existing: React.Key[], additions: readonly React.Key[]) {
  const seen = new Set(existing.map((item) => String(item)));
  const next = [...existing];

  additions.forEach((item) => {
    const normalizedItem = String(item);

    if (!seen.has(normalizedItem)) {
      seen.add(normalizedItem);
      next.push(item);
    }
  });

  return next;
}

type DataTableComponent = React.ForwardRefExoticComponent<DataTableProps & React.RefAttributes<HTMLDivElement>> & {
  <T = any>(props: DataTableProps<T> & React.RefAttributes<HTMLDivElement>): React.ReactNode;
};

const DataTable: DataTableComponent = React.forwardRef<HTMLDivElement, DataTableProps>(({
  className,
  columnVisibility,
  columns,
  defaultSorting,
  density = 'comfortable',
  description,
  emptyDescription,
  emptyState,
  emptyTitle = 'No rows',
  footer,
  getRowId = defaultGetRowId,
  getRowProps,
  getRowSelectionLabel,
  loading = false,
  loadingLabel,
  onRowClick,
  onSelectedRowIdsChange,
  onSortingChange,
  pagination,
  rowActions,
  rowActionsLabel = 'Actions',
  rows,
  selectable = false,
  selectedRowIds = [],
  selectionBar,
  slotProps,
  sorting: controlledSorting,
  sortingMode = 'client',
  stickyHeader = false,
  title,
  toolbar,
  ...props
}, ref) => {
  const [uncontrolledPage, setUncontrolledPage] = React.useState(pagination?.defaultPage ?? 1);
  const [uncontrolledPageSize, setUncontrolledPageSize] = React.useState(
    normalizePageSize(pagination?.defaultPageSize ?? 10),
  );
  const [uncontrolledSorting, setUncontrolledSorting] = React.useState(() => normalizeSortingState(defaultSorting));

  const resolvedPageSize = normalizePageSize(pagination?.pageSize ?? uncontrolledPageSize);
  const totalRowCount = pagination?.mode === 'server'
    ? pagination.rowCount ?? rows.length
    : rows.length;
  const pageCount = pagination
    ? Math.max(1, Math.ceil(Math.max(totalRowCount, 1) / resolvedPageSize))
    : 1;
  const resolvedPage = pagination ? pagination.page ?? uncontrolledPage : 1;
  const currentPage = pagination ? clampPage(resolvedPage, pageCount) : 1;
  const resolvedPageSizeOptions = pagination
    ? resolvePageSizeOptions(pagination.pageSizeOptions, resolvedPageSize)
    : [];
  const visibilityState = React.useMemo(
    () => normalizeColumnVisibility(columnVisibility),
    [columnVisibility],
  );
  const sortingState = controlledSorting === undefined
    ? uncontrolledSorting
    : normalizeSortingState(controlledSorting);
  const hasSortableColumns = React.useMemo(
    () => columns.some((column) => column.sortable),
    [columns],
  );
  const columnMap = React.useMemo(
    () => new Map(columns.map((column) => [column.id, column])),
    [columns],
  );
  const columnDefs = React.useMemo<ColumnDef<any>[]>(
    () =>
      columns.map((column) => ({
        accessorFn: resolveColumnAccessor(column),
        enableHiding: true,
        enableSorting: !!column.sortable,
        id: column.id,
      })),
    [columns],
  );

  function handlePageChange(nextPage: number) {
    if (!pagination) {
      return;
    }

    const clampedPage = clampPage(nextPage, pageCount);

    if (pagination.page === undefined) {
      setUncontrolledPage(clampedPage);
    }

    pagination.onPageChange?.(clampedPage);
  }

  function handleSortingUpdate(updater: SortingState | ((old: SortingState) => SortingState)) {
    const nextSorting = typeof updater === 'function' ? updater(sortingState) : updater;

    if (controlledSorting === undefined && !areSortingStatesEqual(uncontrolledSorting, nextSorting)) {
      setUncontrolledSorting(nextSorting);
    }

    onSortingChange?.(toPublicSortingState(nextSorting));

    if (pagination && currentPage !== 1) {
      handlePageChange(1);
    }
  }

  function handlePageSizeChange(nextPageSizeValue: string) {
    if (!pagination) {
      return;
    }

    const nextPageSize = normalizePageSize(Number(nextPageSizeValue));

    if (nextPageSize === resolvedPageSize && currentPage === 1) {
      return;
    }

    if (pagination.pageSize === undefined) {
      setUncontrolledPageSize(nextPageSize);
    }

    if (pagination.page === undefined) {
      setUncontrolledPage(1);
    }

    pagination.onPageSizeChange?.(nextPageSize);

    if (currentPage !== 1) {
      pagination.onPageChange?.(1);
    }
  }

  const table = useReactTable({
    columns: columnDefs,
    data: rows,
    getCoreRowModel: getCoreRowModel(),
    ...(pagination && pagination.mode !== 'server'
      ? {
          getPaginationRowModel: getPaginationRowModel(),
        }
      : {}),
    ...(hasSortableColumns && sortingMode !== 'server'
      ? {
          getSortedRowModel: getSortedRowModel(),
        }
      : {}),
    getRowId: (row, index) => String(getRowId(row, index)),
    manualPagination: pagination?.mode === 'server',
    manualSorting: sortingMode === 'server',
    onSortingChange: handleSortingUpdate,
    rowCount: totalRowCount,
    state: {
      ...(pagination
        ? {
            pagination: {
              pageIndex: currentPage - 1,
              pageSize: resolvedPageSize,
            },
          }
        : {}),
      columnVisibility: visibilityState,
      sorting: sortingState,
    },
  });

  const displayedRows = table.getRowModel().rows;
  const selectedRowIdSet = new Set(selectedRowIds.map((rowId) => String(rowId)));
  const rowIds = rows.map((row, index) => getRowId(row, index));
  const displayedRowIds = displayedRows.map((row) => getRowId(row.original, row.index));
  const selectedRowCount = rowIds.filter((rowId) => selectedRowIdSet.has(String(rowId))).length;
  const allRowsSelected =
    displayedRowIds.length > 0 && displayedRowIds.every((rowId) => selectedRowIdSet.has(String(rowId)));
  const someRowsSelected =
    !allRowsSelected && displayedRowIds.some((rowId) => selectedRowIdSet.has(String(rowId)));
  const paginationSummary = displayedRows.length > 0
    ? `Showing ${(currentPage - 1) * resolvedPageSize + 1}-${(currentPage - 1) * resolvedPageSize + displayedRows.length} of ${totalRowCount}`
    : `Showing 0-0 of ${totalRowCount}`;

  React.useEffect(() => {
    if (!pagination || pagination.page !== undefined) {
      return;
    }

    if (uncontrolledPage !== currentPage) {
      setUncontrolledPage(currentPage);
    }
  }, [currentPage, pagination, uncontrolledPage]);

  React.useEffect(() => {
    if (!pagination || pagination.pageSize !== undefined) {
      return;
    }

    if (uncontrolledPageSize !== resolvedPageSize) {
      setUncontrolledPageSize(resolvedPageSize);
    }
  }, [pagination, resolvedPageSize, uncontrolledPageSize]);

  function handleSelectedRowIdsChange(nextSelectedIds: React.Key[]) {
    onSelectedRowIdsChange?.(nextSelectedIds);
  }

  function handleToggleAllRows(checked: boolean | 'indeterminate') {
    if (checked) {
      handleSelectedRowIdsChange(appendUniqueKeys(selectedRowIds, displayedRowIds));
      return;
    }

    const displayedIdSet = new Set(displayedRowIds.map((rowId) => String(rowId)));

    handleSelectedRowIdsChange(
      selectedRowIds.filter((selectedRowId) => !displayedIdSet.has(String(selectedRowId))),
    );
  }

  function handleToggleRow(rowId: React.Key, checked: boolean | 'indeterminate') {
    if (checked) {
      handleSelectedRowIdsChange(appendUniqueKeys(selectedRowIds, [rowId]));
      return;
    }

    handleSelectedRowIdsChange(
      selectedRowIds.filter((selectedRowId) => String(selectedRowId) !== String(rowId)),
    );
  }

  const hasPagination = !!pagination && totalRowCount > 0;
  const hasFooter = !!footer || hasPagination;
  const hasPageSizeSelector = hasPagination && resolvedPageSizeOptions.length > 1;
  const paginationItems = hasPagination ? resolvePaginationItems(currentPage, pageCount) : [];
  const headerGroup = table.getHeaderGroups()[0];

  return (
    <div
      ref={ref}
      className={cn('flex flex-col gap-4', className)}
      data-sdk-ui="data-table"
      {...props}
    >
      {title || description || toolbar ? (
        <div
          data-sdk-region="data-table-header"
          {...mergeSlotProps(
            {
              className: 'flex flex-wrap items-start justify-between gap-3',
            },
            slotProps?.header,
          )}
        >
          <div className="min-w-0">
            {title ? (
              <div className="text-base font-semibold text-[var(--sdk-color-text-primary)]">{title}</div>
            ) : null}
            {description ? (
              <div className="mt-1 text-sm text-[var(--sdk-color-text-secondary)]">{description}</div>
            ) : null}
          </div>
          {toolbar ? (
            <div
              data-sdk-region="data-table-toolbar"
              {...mergeSlotProps(
                {},
                slotProps?.toolbar,
              )}
            >
              {toolbar}
            </div>
          ) : null}
        </div>
      ) : null}

      {selectedRowCount > 0 ? (
        <BulkActionBar
          actions={selectionBar?.actions}
          clearLabel={selectionBar?.clearLabel}
          count={selectedRowCount}
          description={selectionBar?.description}
          meta={selectionBar?.meta}
          onClear={onSelectedRowIdsChange ? () => handleSelectedRowIdsChange([]) : undefined}
          sticky={selectionBar?.sticky}
          title={selectionBar?.title ?? 'Selected rows'}
          tone={selectionBar?.tone}
        />
      ) : null}

      <div
        data-sdk-region="data-table-surface"
        {...mergeSlotProps(
          {
            className: dataTableSurfaceClassName,
          },
          slotProps?.surface,
        )}
      >
        {loading ? (
          <div className="px-4 py-6">
            <LoadingBlock label={loadingLabel} />
          </div>
        ) : rows.length === 0 ? (
          <div className="px-4 py-6">
            {emptyState ?? <EmptyState description={emptyDescription} title={emptyTitle} />}
          </div>
        ) : (
          <Table
            {...mergeSlotProps(
              {},
              slotProps?.table,
            )}
          >
            <TableHeader>
              <TableRow>
                {selectable ? (
                  <TableHead className="w-12">
                    <Checkbox
                      aria-label="Select all rows"
                      checked={allRowsSelected ? true : someRowsSelected ? 'indeterminate' : false}
                      onCheckedChange={handleToggleAllRows}
                    />
                  </TableHead>
                ) : null}
                {headerGroup?.headers
                  .filter((header) => !header.isPlaceholder)
                  .map((header) => {
                    const column = columnMap.get(header.column.id);

                    if (!column) {
                      return null;
                    }

                    return (
                      <DataTableHeaderCell
                        column={column}
                        key={header.id}
                        sortColumn={header.column}
                        stickyHeader={stickyHeader}
                      />
                    );
                  })}
                {rowActions ? (
                  <TableHead
                    className={cn(
                      'w-1 whitespace-nowrap text-right',
                      stickyHeader ? 'sticky top-0 z-10 bg-[var(--sdk-color-surface-panel)]' : null,
                    )}
                  >
                    {rowActionsLabel}
                  </TableHead>
                ) : null}
              </TableRow>
            </TableHeader>
            <TableBody>
              {displayedRows.map((tableRow) => {
                const row = tableRow.original;
                const index = tableRow.index;
                const rowId = getRowId(row, index);
                const selected = selectedRowIdSet.has(String(rowId));
                const rowSelectionLabel = getRowSelectionLabel?.(row, index) ?? String(rowId);
                const resolvedRowProps = getRowProps?.(row, index);

                return (
                  <TableRow
                    {...mergeSlotProps<DataTableRowProps>(
                      {
                        className: cn(
                          'hover:bg-[var(--sdk-color-brand-primary-soft)]',
                          onRowClick ? 'cursor-pointer' : null,
                        ),
                        'data-sdk-row-id': String(rowId),
                        'data-state': selected ? 'selected' : 'unselected',
                      },
                      resolvedRowProps,
                    )}
                    key={String(rowId)}
                    onClick={onRowClick ? () => onRowClick(row, index) : undefined}
                  >
                    {selectable ? (
                      <TableCell className={densityClassName[density]}>
                        <Checkbox
                          aria-label={`Select row ${rowSelectionLabel}`}
                          checked={selected}
                          onCheckedChange={(checked) => handleToggleRow(rowId, checked)}
                          onClick={(event) => event.stopPropagation()}
                        />
                      </TableCell>
                    ) : null}
                    {tableRow.getVisibleCells().map((cell) => {
                      const column = columnMap.get(cell.column.id);

                      if (!column) {
                        return null;
                      }

                      const resolvedCellProps =
                        typeof column.cellProps === 'function'
                          ? column.cellProps(row, index)
                          : column.cellProps;

                      return (
                        <TableCell
                          {...mergeSlotProps<DataTableCellProps>(
                            {
                              className: cn(
                                densityClassName[density],
                                alignClassName[column.align ?? 'left'],
                              ),
                            },
                            resolvedCellProps,
                          )}
                          key={cell.id}
                        >
                          {column.cell(row, index)}
                        </TableCell>
                      );
                    })}
                    {rowActions ? (
                      <TableCell className={cn(densityClassName[density], 'text-right')}>
                        <div className="flex justify-end" onClick={(event) => event.stopPropagation()}>
                          {rowActions(row, index)}
                        </div>
                      </TableCell>
                    ) : null}
                  </TableRow>
                );
              })}
            </TableBody>
          </Table>
        )}

        {hasFooter ? (
          <div
            data-sdk-region="data-table-footer"
            {...mergeSlotProps(
              {
                className: dataTableFooterClassName,
              },
              slotProps?.footer,
            )}
          >
            <div className={dataTableSummaryClassName}>
              {footer}
              {hasPagination ? <span>{paginationSummary}</span> : null}
            </div>
            {hasPagination ? (
              <div
                data-sdk-region="data-table-pagination"
                {...mergeSlotProps(
                  {},
                  slotProps?.pagination,
                )}
              >
                <DataTablePaginationControls
                  currentPage={currentPage}
                  hasPageSizeSelector={hasPageSizeSelector}
                  onPageChange={handlePageChange}
                  onPageSizeChange={handlePageSizeChange}
                  pageCount={pageCount}
                  pageSizeOptions={resolvedPageSizeOptions}
                  paginationItems={paginationItems}
                  resolvedPageSize={resolvedPageSize}
                />
              </div>
            ) : null}
          </div>
        ) : null}
      </div>
    </div>
  );
});

export { DataTable };
DataTable.displayName = 'DataTable';
