import * as React from 'react';
import { cn } from '../../lib/utils';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from './card';

export interface StatCardProps extends React.HTMLAttributes<HTMLDivElement> {
  change?: React.ReactNode;
  changeTone?: 'default' | 'success' | 'warning' | 'danger';
  description?: React.ReactNode;
  icon?: React.ReactNode;
  label: React.ReactNode;
  value: React.ReactNode;
}

const changeToneClass: Record<NonNullable<StatCardProps['changeTone']>, string> = {
  default: 'text-[var(--sdk-color-text-secondary)]',
  success: 'text-[var(--sdk-color-state-success)]',
  warning: 'text-[var(--sdk-color-state-warning)]',
  danger: 'text-[var(--sdk-color-state-danger)]',
};

function StatCard({
  change,
  changeTone = 'default',
  className,
  description,
  icon,
  label,
  value,
  ...props
}: StatCardProps) {
  return (
    <Card
      className={cn('h-full', className)}
      data-sdk-ui="stat-card"
      {...props}
    >
      <CardHeader className="flex flex-row items-start justify-between gap-4 space-y-0">
        <div className="space-y-1">
          <CardDescription>{label}</CardDescription>
          <CardTitle className="text-3xl">{value}</CardTitle>
        </div>
        {icon ? <div className="text-[var(--sdk-color-text-secondary)]">{icon}</div> : null}
      </CardHeader>
      {description || change ? (
        <CardContent className="flex items-center justify-between gap-4">
          {description ? <p className="text-sm text-[var(--sdk-color-text-secondary)]">{description}</p> : <span />}
          {change ? <span className={cn('text-sm font-medium', changeToneClass[changeTone])}>{change}</span> : null}
        </CardContent>
      ) : null}
    </Card>
  );
}

StatCard.displayName = 'StatCard';

export { StatCard };
