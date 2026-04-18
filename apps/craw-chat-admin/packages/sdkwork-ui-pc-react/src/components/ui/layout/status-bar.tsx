import * as React from 'react';
import { cn } from '../../../lib/utils';

const statusBarAlignClassName = {
  start: 'justify-start',
  center: 'justify-center',
  end: 'justify-end',
} as const;

export interface StatusBarProps extends React.HTMLAttributes<HTMLDivElement> {}

const StatusBar = React.forwardRef<HTMLDivElement, StatusBarProps>(({ className, role = 'status', ...props }, ref) => (
  <div
    ref={ref}
    className={cn(
      'flex min-h-9 w-full items-center gap-3 border-t border-[var(--sdk-color-border-default)] bg-[var(--sdk-color-surface-panel)] px-3 py-1.5 text-xs text-[var(--sdk-color-text-secondary)]',
      className,
    )}
    data-sdk-ui="status-bar"
    role={role}
    {...props}
  />
));

StatusBar.displayName = 'StatusBar';

export interface StatusBarSectionProps extends React.HTMLAttributes<HTMLDivElement> {
  align?: keyof typeof statusBarAlignClassName;
}

const StatusBarSection = React.forwardRef<HTMLDivElement, StatusBarSectionProps>(
  ({ align = 'start', className, ...props }, ref) => (
    <div
      ref={ref}
      className={cn('flex min-w-0 flex-1 items-center gap-3', statusBarAlignClassName[align], className)}
      data-sdk-ui="status-bar-section"
      {...props}
    />
  ),
);

StatusBarSection.displayName = 'StatusBarSection';

export interface StatusBarItemProps extends React.HTMLAttributes<HTMLDivElement> {
  icon?: React.ReactNode;
  label?: React.ReactNode;
  value?: React.ReactNode;
}

const StatusBarItem = React.forwardRef<HTMLDivElement, StatusBarItemProps>(
  ({ children, className, icon, label, value, ...props }, ref) => (
    <div
      ref={ref}
      className={cn('inline-flex min-w-0 items-center gap-2', className)}
      data-sdk-ui="status-bar-item"
      {...props}
    >
      {icon ? <span className="shrink-0 text-[var(--sdk-color-text-muted)]">{icon}</span> : null}
      {label || value ? (
        <>
          {label ? <span className="truncate text-[var(--sdk-color-text-muted)]">{label}</span> : null}
          {value ? <span className="truncate font-medium text-[var(--sdk-color-text-primary)]">{value}</span> : null}
        </>
      ) : (
        <span className="truncate">{children}</span>
      )}
    </div>
  ),
);

StatusBarItem.displayName = 'StatusBarItem';

export { StatusBar, StatusBarItem, StatusBarSection };
