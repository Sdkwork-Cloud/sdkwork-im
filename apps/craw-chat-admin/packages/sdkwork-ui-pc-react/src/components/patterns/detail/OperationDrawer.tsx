import * as React from 'react';
import { cn } from '../../../lib/utils';
import {
  mergePatternSlotProps,
  type PatternSlotProps,
} from '../_internal/slot-props';
import {
  Drawer,
  DrawerBody,
  type DrawerBodyProps,
  DrawerContent,
  type DrawerContentProps,
  DrawerDescription,
  DrawerFooter,
  type DrawerFooterProps,
  DrawerHeader,
  type DrawerHeaderProps,
  DrawerTitle,
} from '../../ui/overlays';

export type OperationDrawerSide = 'left' | 'right';
export type OperationDrawerSize = 'sm' | 'md' | 'lg' | 'xl' | 'full';

export type OperationDrawerSurfaceSlotProps = PatternSlotProps<
  Omit<React.ComponentPropsWithoutRef<'div'>, 'children'>
>;
export type OperationDrawerSidebarSlotProps = PatternSlotProps<
  Omit<React.ComponentPropsWithoutRef<'aside'>, 'children'>
>;

export interface OperationDrawerSlotProps {
  body?: PatternSlotProps<Omit<DrawerBodyProps, 'children'>>;
  content?: PatternSlotProps<Omit<DrawerContentProps, 'children' | 'side' | 'size'>>;
  footer?: PatternSlotProps<Omit<DrawerFooterProps, 'children'>>;
  header?: PatternSlotProps<Omit<DrawerHeaderProps, 'children'>>;
  sidebar?: OperationDrawerSidebarSlotProps;
  surface?: OperationDrawerSurfaceSlotProps;
}

export interface OperationDrawerProps extends React.ComponentPropsWithoutRef<typeof Drawer> {
  actions?: React.ReactNode;
  badge?: React.ReactNode;
  className?: string;
  description?: React.ReactNode;
  eyebrow?: React.ReactNode;
  footer?: React.ReactNode;
  side?: OperationDrawerSide;
  sidebar?: React.ReactNode;
  size?: OperationDrawerSize;
  slotProps?: OperationDrawerSlotProps;
  title: React.ReactNode;
}

const OperationDrawer = React.forwardRef<HTMLDivElement, OperationDrawerProps>(({
  actions,
  badge,
  children,
  className,
  description,
  eyebrow,
  footer,
  side = 'right',
  sidebar,
  size = 'xl',
  slotProps,
  title,
  ...props
}, ref) => {
  return (
    <Drawer {...props}>
      <DrawerContent
        {...(description ? {} : { 'aria-describedby': undefined })}
        {...mergePatternSlotProps<PatternSlotProps<Omit<DrawerContentProps, 'children' | 'side' | 'size'>>>(
          {
            className:
              'gap-0 overflow-hidden border-[color-mix(in_srgb,var(--sdk-color-border-default)_88%,transparent)] bg-[linear-gradient(180deg,color-mix(in_srgb,var(--sdk-color-surface-panel)_98%,white_2%)_0%,color-mix(in_srgb,var(--sdk-color-surface-panel-muted)_90%,transparent)_100%)]',
            'data-sdk-region': 'operation-drawer-content',
          },
          slotProps?.content,
        )}
        side={side}
        size={size}
      >
        <div
          ref={ref}
          {...mergePatternSlotProps<OperationDrawerSurfaceSlotProps>(
            {
              className: cn('relative flex h-full min-h-0 min-w-0 flex-col overflow-hidden', className),
              'data-sdk-pattern': 'operation-drawer',
              'data-sdk-region': 'operation-drawer-surface',
            },
            slotProps?.surface,
          )}
        >
          <div className="pointer-events-none absolute inset-x-0 top-0 h-40 bg-[radial-gradient(circle_at_top,color-mix(in_srgb,var(--sdk-color-brand-primary)_18%,transparent),transparent_62%)]" />
          <DrawerHeader
            {...mergePatternSlotProps<PatternSlotProps<Omit<DrawerHeaderProps, 'children'>>>(
              {
                className:
                  'relative border-b-[color-mix(in_srgb,var(--sdk-color-border-default)_78%,transparent)] bg-[linear-gradient(180deg,color-mix(in_srgb,var(--sdk-color-surface-panel)_96%,white_4%)_0%,color-mix(in_srgb,var(--sdk-color-surface-panel-muted)_88%,transparent)_100%)] px-5 py-4 xl:px-6',
                'data-sdk-region': 'operation-drawer-header',
              },
              slotProps?.header,
            )}
          >
            <div className="flex items-start gap-4">
              <div className="min-w-0 flex-1">
                {eyebrow ? (
                  <div className="text-[11px] font-semibold uppercase tracking-[0.24em] text-[var(--sdk-color-text-muted)]">
                    {eyebrow}
                  </div>
                ) : null}
                <div className={cn('flex flex-wrap items-center gap-3', eyebrow ? 'mt-2' : null)}>
                  <DrawerTitle className="text-[1.45rem] tracking-tight">{title}</DrawerTitle>
                  {badge ? <div className="shrink-0">{badge}</div> : null}
                </div>
                {description ? (
                  <DrawerDescription className="mt-2 max-w-4xl text-sm leading-6">
                    {description}
                  </DrawerDescription>
                ) : null}
              </div>
              {actions ? <div className="flex shrink-0 items-center gap-2 pr-8">{actions}</div> : null}
            </div>
          </DrawerHeader>
          <div className="relative flex min-h-0 flex-1 overflow-hidden">
            <DrawerBody
              {...mergePatternSlotProps<PatternSlotProps<Omit<DrawerBodyProps, 'children'>>>(
                {
                  className: cn(
                    'min-h-0 flex-1 px-4 py-4 xl:px-5',
                    sidebar ? 'xl:border-r xl:border-[color-mix(in_srgb,var(--sdk-color-border-default)_72%,transparent)]' : null,
                  ),
                  'data-sdk-region': 'operation-drawer-body',
                },
                slotProps?.body,
              )}
            >
              {children}
            </DrawerBody>
            {sidebar ? (
              <aside
                {...mergePatternSlotProps<OperationDrawerSidebarSlotProps>(
                  {
                    className:
                      'hidden w-[320px] shrink-0 bg-[linear-gradient(180deg,color-mix(in_srgb,var(--sdk-color-surface-panel-muted)_92%,transparent)_0%,color-mix(in_srgb,var(--sdk-color-surface-canvas)_86%,transparent)_100%)] xl:flex xl:min-h-0 xl:flex-col',
                    'data-sdk-region': 'operation-drawer-sidebar',
                  },
                  slotProps?.sidebar,
                )}
              >
                <div className="min-h-0 flex-1 overflow-y-auto p-5">{sidebar}</div>
              </aside>
            ) : null}
          </div>
          {footer ? (
            <DrawerFooter
              {...mergePatternSlotProps<PatternSlotProps<Omit<DrawerFooterProps, 'children'>>>(
                {
                  className:
                    'border-t-[color-mix(in_srgb,var(--sdk-color-border-default)_78%,transparent)] bg-[linear-gradient(180deg,color-mix(in_srgb,var(--sdk-color-surface-panel)_94%,white_4%)_0%,color-mix(in_srgb,var(--sdk-color-surface-panel-muted)_88%,transparent)_100%)] px-5 py-3.5 xl:px-6',
                  'data-sdk-region': 'operation-drawer-footer',
                },
                slotProps?.footer,
              )}
            >
              {footer}
            </DrawerFooter>
          ) : null}
        </div>
      </DrawerContent>
    </Drawer>
  );
});

export { OperationDrawer };
OperationDrawer.displayName = 'OperationDrawer';
