import * as React from 'react';
import * as TabsPrimitive from '@radix-ui/react-tabs';
import { cn } from '../../lib/utils';

export type TabsProps = React.ComponentPropsWithoutRef<typeof TabsPrimitive.Root>;
export type TabsListProps = React.ComponentPropsWithoutRef<typeof TabsPrimitive.List>;
export type TabsTriggerProps = React.ComponentPropsWithoutRef<typeof TabsPrimitive.Trigger>;
export type TabsContentProps = React.ComponentPropsWithoutRef<typeof TabsPrimitive.Content>;

const Tabs = React.forwardRef<
  React.ElementRef<typeof TabsPrimitive.Root>,
  TabsProps
>(({ ...props }, ref) => (
  <TabsPrimitive.Root
    ref={ref}
    data-sdk-ui="tabs"
    data-slot="tabs"
    {...props}
  />
));

const TabsList = React.forwardRef<
  React.ElementRef<typeof TabsPrimitive.List>,
  TabsListProps
>(({ className, ...props }, ref) => (
  <TabsPrimitive.List
    ref={ref}
    className={cn(
      'inline-flex h-10 items-center gap-1 rounded-[var(--sdk-radius-control)] bg-[var(--sdk-color-surface-panel-muted)] p-1 text-[var(--sdk-color-text-secondary)]',
      className,
    )}
    data-sdk-ui="tabs-list"
    data-slot="tabs-list"
    {...props}
  />
));

TabsList.displayName = 'TabsList';

const TabsTrigger = React.forwardRef<
  React.ElementRef<typeof TabsPrimitive.Trigger>,
  TabsTriggerProps
>(({ className, ...props }, ref) => (
  <TabsPrimitive.Trigger
    ref={ref}
    className={cn(
      'inline-flex min-w-[6rem] items-center justify-center rounded-[var(--sdk-radius-control)] px-3 py-1.5 text-sm font-medium transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-[var(--sdk-color-border-focus)] focus-visible:ring-offset-2 focus-visible:ring-offset-[var(--sdk-color-surface-canvas)] data-[state=active]:bg-[var(--sdk-color-surface-panel)] data-[state=active]:text-[var(--sdk-color-text-primary)] data-[state=active]:shadow-[var(--sdk-shadow-sm)]',
      className,
    )}
    data-sdk-ui="tabs-trigger"
    data-slot="tabs-trigger"
    {...props}
  />
));

TabsTrigger.displayName = 'TabsTrigger';

const TabsContent = React.forwardRef<
  React.ElementRef<typeof TabsPrimitive.Content>,
  TabsContentProps
>(({ className, ...props }, ref) => (
  <TabsPrimitive.Content
    ref={ref}
    className={cn(
      'mt-4 rounded-[var(--sdk-radius-panel)] border border-[var(--sdk-color-border-default)] bg-[var(--sdk-color-surface-panel)] p-5 text-[var(--sdk-color-text-primary)] shadow-[var(--sdk-shadow-sm)] focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-[var(--sdk-color-border-focus)]',
      className,
    )}
    data-sdk-ui="tabs-content"
    data-slot="tabs-content"
    {...props}
  />
));

TabsContent.displayName = 'TabsContent';

export { Tabs, TabsContent, TabsList, TabsTrigger };
Tabs.displayName = 'Tabs';
