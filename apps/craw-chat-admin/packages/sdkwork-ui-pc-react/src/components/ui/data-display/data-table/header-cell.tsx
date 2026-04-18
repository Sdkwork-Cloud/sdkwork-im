import * as React from 'react';
import type { Column } from '@tanstack/react-table';
import { ArrowDown, ArrowUp, ArrowUpDown } from 'lucide-react';
import { mergeSlotProps } from '../../../../lib/slot-props';
import { cn } from '../../../../lib/utils';
import { TableHead } from '../../table';
import { dataTableSortButtonClassName, dataTableSortIconClassName } from './styles';
import type { DataTableColumn, DataTableHeaderProps } from './types';

function resolveStyleWidth(width?: number | string) {
  if (width === undefined) {
    return undefined;
  }

  return typeof width === 'number' ? `${width}px` : width;
}

function resolveSortLabel<T>(column: DataTableColumn<T>) {
  if (column.sortLabel) {
    return column.sortLabel;
  }

  if (typeof column.header === 'string') {
    return `Sort by ${column.header}`;
  }

  return `Sort by ${column.id}`;
}

function resolveAriaSortValue(sortState: false | 'asc' | 'desc') {
  if (sortState === 'asc') {
    return 'ascending';
  }

  if (sortState === 'desc') {
    return 'descending';
  }

  return 'none';
}

function SortIcon({ sortState }: { sortState: false | 'asc' | 'desc' }) {
  if (sortState === 'asc') {
    return <ArrowUp className={dataTableSortIconClassName} />;
  }

  if (sortState === 'desc') {
    return <ArrowDown className={dataTableSortIconClassName} />;
  }

  return <ArrowUpDown className={dataTableSortIconClassName} />;
}

export interface DataTableHeaderCellProps<T> {
  column: DataTableColumn<T>;
  sortColumn?: Column<T, unknown>;
  stickyHeader?: boolean;
}

export function DataTableHeaderCell<T>({
  column,
  sortColumn,
  stickyHeader = false,
}: DataTableHeaderCellProps<T>) {
  const sortState = sortColumn?.getIsSorted() ?? false;
  const canSort = !!sortColumn?.getCanSort();

  return (
    <TableHead
      {...mergeSlotProps<DataTableHeaderProps>(
        {
          'aria-sort': canSort ? resolveAriaSortValue(sortState) : undefined,
          className: cn(
            column.align === 'center' ? 'text-center' : column.align === 'right' ? 'text-right' : 'text-left',
            stickyHeader ? 'sticky top-0 z-10 bg-[var(--sdk-color-surface-panel)]' : null,
          ),
          'data-sdk-region': 'data-table-header-cell',
          style: { width: resolveStyleWidth(column.width) },
        },
        column.headerProps,
      )}
    >
      {canSort ? (
        <button
          aria-label={resolveSortLabel(column)}
          className={cn(
            dataTableSortButtonClassName,
            column.align === 'center'
              ? 'justify-center'
              : column.align === 'right'
                ? 'justify-end'
                : 'justify-between',
          )}
          data-sdk-region="data-table-sort-trigger"
          data-slot="data-table-sort-trigger"
          onClick={() => sortColumn?.toggleSorting(sortState === 'asc')}
          type="button"
        >
          <span className="truncate">{column.header}</span>
          <SortIcon sortState={sortState} />
        </button>
      ) : (
        column.header
      )}
    </TableHead>
  );
}
