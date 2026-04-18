import type { SortingState } from '@tanstack/react-table';
import type { DataTableSortingState } from './types';

export type DataTablePaginationItem = number | 'start-ellipsis' | 'end-ellipsis';

export function clampPage(page: number, pageCount: number) {
  return Math.min(Math.max(page, 1), pageCount);
}

export function normalizePageSize(pageSize: number) {
  return Math.max(1, Math.trunc(pageSize) || 1);
}

export function resolvePageSizeOptions(options: readonly number[] | undefined, currentPageSize: number) {
  if (options?.length === 0) {
    return [];
  }

  return Array.from(new Set([...(options ?? [10, 20, 50]), currentPageSize]))
    .map(normalizePageSize)
    .sort((left, right) => left - right);
}

export function resolvePaginationItems(currentPage: number, pageCount: number): DataTablePaginationItem[] {
  if (pageCount <= 7) {
    return Array.from({ length: pageCount }, (_, index) => index + 1);
  }

  if (currentPage <= 4) {
    return [1, 2, 3, 4, 5, 'end-ellipsis', pageCount];
  }

  if (currentPage >= pageCount - 3) {
    return [1, 'start-ellipsis', pageCount - 4, pageCount - 3, pageCount - 2, pageCount - 1, pageCount];
  }

  return [1, 'start-ellipsis', currentPage - 1, currentPage, currentPage + 1, 'end-ellipsis', pageCount];
}

export function normalizeSortingState(sorting: DataTableSortingState | undefined): SortingState {
  if (!sorting?.length) {
    return [];
  }

  return sorting
    .filter((item) => item.id)
    .map((item) => ({
      desc: !!item.desc,
      id: item.id,
    }));
}

export function toPublicSortingState(sorting: SortingState): DataTableSortingState {
  return sorting.map((item) => ({
    desc: !!item.desc,
    id: item.id,
  }));
}

export function areSortingStatesEqual(left: SortingState, right: SortingState) {
  if (left.length !== right.length) {
    return false;
  }

  return left.every((item, index) => item.id === right[index]?.id && item.desc === right[index]?.desc);
}
