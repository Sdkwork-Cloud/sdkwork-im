import * as React from 'react';
import { SearchX } from 'lucide-react';
import { cn } from '../../../lib/utils';
import { Button } from '../button';

export type EmptySearchClearHandler = () => void;

export interface EmptySearchProps extends Omit<React.HTMLAttributes<HTMLDivElement>, 'title'> {
  actions?: React.ReactNode;
  clearLabel?: React.ReactNode;
  description?: React.ReactNode;
  keyword?: React.ReactNode;
  onClear?: EmptySearchClearHandler;
  title?: React.ReactNode;
}

export const EmptySearch = React.forwardRef<HTMLDivElement, EmptySearchProps>(({
  actions,
  className,
  clearLabel = 'Clear search',
  description = 'Try a different keyword or clear the current search.',
  keyword,
  onClear,
  title,
  ...props
}, ref) => {
  const resolvedTitle = title ?? (keyword ? `No results found for "${keyword}"` : 'No search results');
  const resolvedActions =
    actions ?? (onClear ? <Button onClick={onClear} variant="secondary">{clearLabel}</Button> : null);

  return (
    <div
      ref={ref}
      className={cn(
        'flex flex-col items-center justify-center gap-3 rounded-[var(--sdk-radius-panel)] border border-dashed border-[var(--sdk-color-border-strong)] bg-[var(--sdk-color-surface-panel)] px-6 py-12 text-center',
        className,
      )}
      data-sdk-ui="empty-search"
      role="status"
      {...props}
    >
      <div className="flex h-14 w-14 items-center justify-center rounded-full border border-[var(--sdk-color-border-default)] bg-[var(--sdk-color-surface-panel-muted)] text-[var(--sdk-color-text-secondary)] shadow-[var(--sdk-shadow-sm)]">
        <SearchX className="h-6 w-6" />
      </div>
      <div className="text-lg font-semibold text-[var(--sdk-color-text-primary)]">{resolvedTitle}</div>
      <div className="max-w-xl text-sm text-[var(--sdk-color-text-secondary)]">{description}</div>
      {resolvedActions ? <div className="mt-2 flex items-center gap-3">{resolvedActions}</div> : null}
    </div>
  );
});

EmptySearch.displayName = 'EmptySearch';
