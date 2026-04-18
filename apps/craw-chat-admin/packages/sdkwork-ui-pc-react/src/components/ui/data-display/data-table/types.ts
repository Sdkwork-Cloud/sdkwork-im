import type * as React from 'react';
import type { BulkActionBarProps } from '../../actions';
import type { TableCellProps, TableHeadProps, TableProps, TableRowProps } from '../../table';
import type { SlotProps } from '../../../../lib/slot-props';

export type DataTableDensity = 'comfortable' | 'compact';
export type DataTableAlign = 'left' | 'center' | 'right';
export type DataTablePaginationMode = 'client' | 'server';
export type DataTableSortingMode = 'client' | 'server';

export interface DataTableSortingStateItem {
  desc: boolean;
  id: string;
}

export type DataTableSortingState = DataTableSortingStateItem[];

export type DataTableRegionSlotProps = SlotProps<Omit<React.ComponentPropsWithoutRef<'div'>, 'children'>>;
export type DataTableTableSlotProps = SlotProps<TableProps>;
export type DataTableHeaderProps = SlotProps<Omit<TableHeadProps, 'children'>>;
export type DataTableCellProps = SlotProps<Omit<TableCellProps, 'children'>>;
export type DataTableRowProps = SlotProps<Omit<TableRowProps, 'children'>>;
export type DataTableCellRenderer<T = any> = (row: T, index: number) => React.ReactNode;
export type DataTableAccessorResolver<T = any> = (row: T) => unknown;
export type DataTableCellPropsResolver<T = any> = (row: T, index: number) => DataTableCellProps | undefined;
export type DataTableRowIdResolver<T = any> = (row: T, index: number) => React.Key;
export type DataTableRowPropsResolver<T = any> = (row: T, index: number) => DataTableRowProps | undefined;
export type DataTableRowSelectionLabelResolver<T = any> = (row: T, index: number) => string;
export type DataTableRowActionsRenderer<T = any> = (row: T, index: number) => React.ReactNode;
export type DataTableRowClickHandler<T = any> = (row: T, index: number) => void;
export type DataTableSelectedRowIdsChangeHandler = (ids: React.Key[]) => void;
export type DataTablePageChangeHandler = (page: number) => void;
export type DataTablePageSizeChangeHandler = (pageSize: number) => void;
export type DataTableSortingChangeHandler = (sorting: DataTableSortingState) => void;

export interface DataTableColumn<T = any> {
  accessorFn?: DataTableAccessorResolver<T>;
  accessorKey?: Extract<keyof T, string>;
  align?: DataTableAlign;
  cell: DataTableCellRenderer<T>;
  cellProps?: DataTableCellProps | DataTableCellPropsResolver<T>;
  header: React.ReactNode;
  headerProps?: DataTableHeaderProps;
  id: string;
  sortLabel?: string;
  sortable?: boolean;
  width?: number | string;
}

export interface DataTablePaginationProps {
  defaultPage?: number;
  defaultPageSize?: number;
  mode?: DataTablePaginationMode;
  onPageChange?: DataTablePageChangeHandler;
  onPageSizeChange?: DataTablePageSizeChangeHandler;
  page?: number;
  pageSize?: number;
  pageSizeOptions?: readonly number[];
  rowCount?: number;
}

export interface DataTableSlotProps {
  footer?: DataTableRegionSlotProps;
  header?: DataTableRegionSlotProps;
  pagination?: DataTableRegionSlotProps;
  surface?: DataTableRegionSlotProps;
  table?: DataTableTableSlotProps;
  toolbar?: DataTableRegionSlotProps;
}

export interface DataTableProps<T = any> extends Omit<React.HTMLAttributes<HTMLDivElement>, 'title'> {
  columnVisibility?: Partial<Record<string, boolean>>;
  columns: DataTableColumn<T>[];
  defaultSorting?: DataTableSortingState;
  density?: DataTableDensity;
  description?: React.ReactNode;
  emptyDescription?: React.ReactNode;
  emptyState?: React.ReactNode;
  emptyTitle?: React.ReactNode;
  footer?: React.ReactNode;
  getRowId?: DataTableRowIdResolver<T>;
  getRowProps?: DataTableRowPropsResolver<T>;
  getRowSelectionLabel?: DataTableRowSelectionLabelResolver<T>;
  loading?: boolean;
  loadingLabel?: React.ReactNode;
  onRowClick?: DataTableRowClickHandler<T>;
  onSelectedRowIdsChange?: DataTableSelectedRowIdsChangeHandler;
  onSortingChange?: DataTableSortingChangeHandler;
  pagination?: DataTablePaginationProps;
  rowActions?: DataTableRowActionsRenderer<T>;
  rowActionsLabel?: React.ReactNode;
  rows: T[];
  selectable?: boolean;
  selectedRowIds?: React.Key[];
  selectionBar?: Omit<BulkActionBarProps, 'count' | 'onClear'>;
  slotProps?: DataTableSlotProps;
  sorting?: DataTableSortingState;
  sortingMode?: DataTableSortingMode;
  stickyHeader?: boolean;
  title?: React.ReactNode;
  toolbar?: React.ReactNode;
}
