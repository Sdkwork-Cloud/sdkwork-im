import * as React from 'react';
import { Button } from '../../button';
import {
  Pagination,
  PaginationContent,
  PaginationEllipsis,
  PaginationItem,
} from '../../pagination';
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '../../select';
import {
  dataTablePageSizeClassName,
  dataTablePaginationButtonClassName,
  dataTablePaginationLayoutClassName,
} from './styles';
import type { DataTablePaginationItem } from './state';

export interface DataTablePaginationControlsProps {
  currentPage: number;
  hasPageSizeSelector: boolean;
  onPageChange: (page: number) => void;
  onPageSizeChange: (value: string) => void;
  pageCount: number;
  paginationItems: DataTablePaginationItem[];
  pageSizeOptions: readonly number[];
  resolvedPageSize: number;
}

export function DataTablePaginationControls({
  currentPage,
  hasPageSizeSelector,
  onPageChange,
  onPageSizeChange,
  pageCount,
  paginationItems,
  pageSizeOptions,
  resolvedPageSize,
}: DataTablePaginationControlsProps) {
  return (
    <div className={dataTablePaginationLayoutClassName} data-sdk-region="data-table-pagination-controls">
      {hasPageSizeSelector ? (
        <div className={dataTablePageSizeClassName} data-sdk-region="data-table-page-size">
          <span className="whitespace-nowrap">Rows per page</span>
          <Select value={String(resolvedPageSize)} onValueChange={onPageSizeChange}>
            <SelectTrigger
              aria-label="Rows per page"
              className="h-9 w-[5.5rem] bg-[var(--sdk-color-surface-panel-muted)] shadow-none"
            >
              <SelectValue />
            </SelectTrigger>
            <SelectContent className="min-w-[5.5rem]">
              {pageSizeOptions.map((pageSizeOption) => (
                <SelectItem key={pageSizeOption} value={String(pageSizeOption)}>
                  {pageSizeOption}
                </SelectItem>
              ))}
            </SelectContent>
          </Select>
        </div>
      ) : null}
      <Pagination className="justify-start sm:justify-end">
        <PaginationContent>
          <PaginationItem>
            <Button
              aria-label="Previous page"
              className={dataTablePaginationButtonClassName}
              disabled={currentPage <= 1}
              onClick={() => onPageChange(currentPage - 1)}
              size="sm"
              type="button"
              variant="outline"
            >
              Previous
            </Button>
          </PaginationItem>
          {paginationItems.map((item, itemIndex) => (
            <PaginationItem key={`${item}-${itemIndex}`}>
              {typeof item === 'number' ? (
                <Button
                  aria-current={item === currentPage ? 'page' : undefined}
                  aria-label={`Page ${item}`}
                  className={dataTablePaginationButtonClassName}
                  onClick={() => onPageChange(item)}
                  size="sm"
                  type="button"
                  variant={item === currentPage ? 'secondary' : 'ghost'}
                >
                  {item}
                </Button>
              ) : (
                <PaginationEllipsis />
              )}
            </PaginationItem>
          ))}
          <PaginationItem>
            <Button
              aria-label="Next page"
              className={dataTablePaginationButtonClassName}
              disabled={currentPage >= pageCount}
              onClick={() => onPageChange(currentPage + 1)}
              size="sm"
              type="button"
              variant="outline"
            >
              Next
            </Button>
          </PaginationItem>
        </PaginationContent>
      </Pagination>
    </div>
  );
}
