import * as React from 'react';
import { cn } from '../../lib/utils';

export type TableProps = React.HTMLAttributes<HTMLTableElement>;
export type TableHeaderProps = React.HTMLAttributes<HTMLTableSectionElement>;
export type TableBodyProps = React.HTMLAttributes<HTMLTableSectionElement>;
export type TableFooterProps = React.HTMLAttributes<HTMLTableSectionElement>;
export type TableRowProps = React.HTMLAttributes<HTMLTableRowElement>;
export type TableHeadProps = React.ThHTMLAttributes<HTMLTableCellElement>;
export type TableCellProps = React.TdHTMLAttributes<HTMLTableCellElement>;
export type TableCaptionProps = React.HTMLAttributes<HTMLTableCaptionElement>;

export const tableViewportClassName = 'relative w-full overflow-auto';
export const tableBaseClassName = 'w-full caption-bottom text-sm';
export const tableHeaderClassName = '[&_tr]:border-b [&_tr]:border-[var(--sdk-color-border-default)]';
export const tableBodyClassName = '[&_tr:last-child]:border-0';
export const tableFooterClassName =
  'border-t border-[var(--sdk-color-border-default)] bg-[var(--sdk-color-surface-panel-muted)] font-medium [&>tr]:last:border-b-0';
export const tableRowClassName =
  'border-b border-[var(--sdk-color-border-subtle)] transition-colors data-[state=selected]:bg-[var(--sdk-color-brand-primary-soft)]';
export const tableHeadClassName =
  'h-11 px-4 text-left align-middle text-sm font-medium text-[var(--sdk-color-text-secondary)]';
export const tableCellClassName = 'p-4 align-middle text-[var(--sdk-color-text-primary)]';
export const tableCaptionClassName = 'mt-4 text-sm text-[var(--sdk-color-text-secondary)]';

const Table = React.forwardRef<HTMLTableElement, TableProps>(
  ({ className, ...props }, ref) => (
    <div className={tableViewportClassName} data-slot="table-viewport">
      <table
        ref={ref}
        className={cn(tableBaseClassName, className)}
        data-sdk-ui="table"
        data-slot="table"
        {...props}
      />
    </div>
  ),
);

Table.displayName = 'Table';

const TableHeader = React.forwardRef<HTMLTableSectionElement, TableHeaderProps>(
  ({ className, ...props }, ref) => (
    <thead
      ref={ref}
      className={cn(tableHeaderClassName, className)}
      data-sdk-ui="table-header"
      data-slot="table-header"
      {...props}
    />
  ),
);

TableHeader.displayName = 'TableHeader';

const TableBody = React.forwardRef<HTMLTableSectionElement, TableBodyProps>(
  ({ className, ...props }, ref) => (
    <tbody
      ref={ref}
      className={cn(tableBodyClassName, className)}
      data-sdk-ui="table-body"
      data-slot="table-body"
      {...props}
    />
  ),
);

TableBody.displayName = 'TableBody';

const TableFooter = React.forwardRef<HTMLTableSectionElement, TableFooterProps>(
  ({ className, ...props }, ref) => (
    <tfoot
      ref={ref}
      className={cn(tableFooterClassName, className)}
      data-sdk-ui="table-footer"
      data-slot="table-footer"
      {...props}
    />
  ),
);

TableFooter.displayName = 'TableFooter';

const TableRow = React.forwardRef<HTMLTableRowElement, TableRowProps>(
  ({ className, ...props }, ref) => (
    <tr
      ref={ref}
      className={cn(tableRowClassName, className)}
      data-sdk-ui="table-row"
      data-slot="table-row"
      {...props}
    />
  ),
);

TableRow.displayName = 'TableRow';

const TableHead = React.forwardRef<HTMLTableCellElement, TableHeadProps>(
  ({ className, ...props }, ref) => (
    <th
      ref={ref}
      className={cn(tableHeadClassName, className)}
      data-sdk-ui="table-head"
      data-slot="table-head"
      {...props}
    />
  ),
);

TableHead.displayName = 'TableHead';

const TableCell = React.forwardRef<HTMLTableCellElement, TableCellProps>(
  ({ className, ...props }, ref) => (
    <td
      ref={ref}
      className={cn(tableCellClassName, className)}
      data-sdk-ui="table-cell"
      data-slot="table-cell"
      {...props}
    />
  ),
);

TableCell.displayName = 'TableCell';

const TableCaption = React.forwardRef<HTMLTableCaptionElement, TableCaptionProps>(
  ({ className, ...props }, ref) => (
    <caption
      ref={ref}
      className={cn(tableCaptionClassName, className)}
      data-sdk-ui="table-caption"
      data-slot="table-caption"
      {...props}
    />
  ),
);

TableCaption.displayName = 'TableCaption';

export {
  Table,
  TableBody,
  TableCaption,
  TableCell,
  TableFooter,
  TableHead,
  TableHeader,
  TableRow,
};
