import * as React from 'react';
import { Slot } from '@radix-ui/react-slot';
import { ChevronRight, MoreHorizontal } from 'lucide-react';
import { cn } from '../../lib/utils';

export type BreadcrumbProps = React.ComponentPropsWithoutRef<'nav'>;
export type BreadcrumbListProps = React.ComponentPropsWithoutRef<'ol'>;
export type BreadcrumbItemProps = React.ComponentPropsWithoutRef<'li'>;
export type BreadcrumbPageProps = React.ComponentPropsWithoutRef<'span'>;
export type BreadcrumbSeparatorProps = React.ComponentPropsWithoutRef<'li'>;
export type BreadcrumbEllipsisProps = React.ComponentPropsWithoutRef<'span'>;

const Breadcrumb = React.forwardRef<HTMLElement, BreadcrumbProps>(
  ({ 'aria-label': ariaLabel = 'breadcrumb', className, ...props }, ref) => (
    <nav
      ref={ref}
      aria-label={ariaLabel}
      className={cn('w-full', className)}
      data-sdk-ui="breadcrumb"
      data-slot="breadcrumb"
      {...props}
    />
  ),
);

Breadcrumb.displayName = 'Breadcrumb';

const BreadcrumbList = React.forwardRef<HTMLOListElement, BreadcrumbListProps>(
  ({ className, ...props }, ref) => (
    <ol
      ref={ref}
      className={cn(
        'flex flex-wrap items-center gap-1.5 break-words text-sm text-[var(--sdk-color-text-secondary)]',
        className,
      )}
      data-sdk-ui="breadcrumb-list"
      data-slot="breadcrumb-list"
      {...props}
    />
  ),
);

BreadcrumbList.displayName = 'BreadcrumbList';

const BreadcrumbItem = React.forwardRef<HTMLLIElement, BreadcrumbItemProps>(
  ({ className, ...props }, ref) => (
    <li
      ref={ref}
      className={cn('inline-flex items-center gap-1.5', className)}
      data-sdk-ui="breadcrumb-item"
      data-slot="breadcrumb-item"
      {...props}
    />
  ),
);

BreadcrumbItem.displayName = 'BreadcrumbItem';

export interface BreadcrumbLinkProps extends React.ComponentPropsWithoutRef<'a'> {
  asChild?: boolean;
}

const BreadcrumbLink = React.forwardRef<HTMLAnchorElement, BreadcrumbLinkProps>(
  ({ asChild = false, className, ...props }, ref) => {
    const Comp = asChild ? Slot : 'a';

    return (
      <Comp
        ref={ref}
        className={cn(
          'transition-colors hover:text-[var(--sdk-color-text-primary)] focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-[var(--sdk-color-border-focus)] focus-visible:ring-offset-2 focus-visible:ring-offset-[var(--sdk-color-surface-canvas)]',
          className,
        )}
        data-sdk-ui="breadcrumb-link"
        data-slot="breadcrumb-link"
        {...props}
      />
    );
  },
);

BreadcrumbLink.displayName = 'BreadcrumbLink';

const BreadcrumbPage = React.forwardRef<HTMLSpanElement, BreadcrumbPageProps>(
  ({ className, ...props }, ref) => (
    <span
      ref={ref}
      aria-current="page"
      aria-disabled="true"
      className={cn('font-medium text-[var(--sdk-color-text-primary)]', className)}
      data-sdk-ui="breadcrumb-page"
      data-slot="breadcrumb-page"
      {...props}
    />
  ),
);

BreadcrumbPage.displayName = 'BreadcrumbPage';

const BreadcrumbSeparator = React.forwardRef<HTMLLIElement, BreadcrumbSeparatorProps>(({
  children,
  className,
  ...props
}, ref) => (
  <li
    ref={ref}
    aria-hidden="true"
    className={cn('text-[var(--sdk-color-text-muted)]', className)}
    data-sdk-ui="breadcrumb-separator"
    data-slot="breadcrumb-separator"
    role="presentation"
    {...props}
  >
    {children ?? <ChevronRight className="h-3.5 w-3.5" />}
  </li>
));

BreadcrumbSeparator.displayName = 'BreadcrumbSeparator';

const BreadcrumbEllipsis = React.forwardRef<HTMLSpanElement, BreadcrumbEllipsisProps>(({ className, ...props }, ref) => (
  <span
    ref={ref}
    aria-hidden="true"
    className={cn('inline-flex h-9 w-9 items-center justify-center', className)}
    data-sdk-ui="breadcrumb-ellipsis"
    data-slot="breadcrumb-ellipsis"
    role="presentation"
    {...props}
  >
    <MoreHorizontal className="h-4 w-4" />
    <span className="sr-only">More breadcrumb items</span>
  </span>
));

BreadcrumbEllipsis.displayName = 'BreadcrumbEllipsis';

export {
  Breadcrumb,
  BreadcrumbEllipsis,
  BreadcrumbItem,
  BreadcrumbLink,
  BreadcrumbList,
  BreadcrumbPage,
  BreadcrumbSeparator,
};
