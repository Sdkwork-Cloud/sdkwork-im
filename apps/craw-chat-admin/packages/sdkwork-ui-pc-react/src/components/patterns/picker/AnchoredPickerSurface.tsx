import * as React from 'react';
import { cn } from '../../../lib/utils';
import {
  mergePatternSlotProps,
  type PatternSlotProps,
} from '../_internal/slot-props';
import { EmptySearch, InlineAlert, LoadingBlock } from '../../ui/feedback';
import { Popover, PopoverContent, PopoverTrigger, type PopoverContentProps } from '../../ui/popover';

export type AnchoredPickerSurfaceSize = 'sm' | 'md' | 'lg' | 'xl';

interface AnchoredPickerSurfacePositioningProps {
  align?: React.ComponentPropsWithoutRef<typeof PopoverContent>['align'];
  alignOffset?: React.ComponentPropsWithoutRef<typeof PopoverContent>['alignOffset'];
  avoidCollisions?: React.ComponentPropsWithoutRef<typeof PopoverContent>['avoidCollisions'];
  collisionPadding?: React.ComponentPropsWithoutRef<typeof PopoverContent>['collisionPadding'];
  side?: React.ComponentPropsWithoutRef<typeof PopoverContent>['side'];
  sideOffset?: React.ComponentPropsWithoutRef<typeof PopoverContent>['sideOffset'];
}

export type AnchoredPickerSurfaceRegionSlotProps = PatternSlotProps<
  Omit<React.ComponentPropsWithoutRef<'div'>, 'children'>
>;
export type AnchoredPickerSurfaceSurfaceSlotProps = PatternSlotProps<
  Omit<React.ComponentPropsWithoutRef<'div'>, 'children'>
>;

export interface AnchoredPickerSurfaceSlotProps {
  body?: AnchoredPickerSurfaceRegionSlotProps;
  content?: PatternSlotProps<
    Omit<
      PopoverContentProps,
      | 'align'
      | 'alignOffset'
      | 'avoidCollisions'
      | 'children'
      | 'collisionPadding'
      | 'side'
      | 'sideOffset'
    >
  >;
  filters?: AnchoredPickerSurfaceRegionSlotProps;
  footer?: AnchoredPickerSurfaceRegionSlotProps;
  header?: AnchoredPickerSurfaceRegionSlotProps;
  surface?: AnchoredPickerSurfaceSurfaceSlotProps;
}

export interface AnchoredPickerSurfaceProps
  extends Omit<React.ComponentPropsWithoutRef<typeof Popover>, 'children'>,
    AnchoredPickerSurfacePositioningProps {
  actions?: React.ReactNode;
  badge?: React.ReactNode;
  bodyScrollable?: boolean;
  children?: React.ReactNode;
  className?: string;
  description?: React.ReactNode;
  empty?: boolean;
  emptyAction?: React.ReactNode;
  emptyDescription?: React.ReactNode;
  emptyKeyword?: React.ReactNode;
  emptyTitle?: React.ReactNode;
  error?: React.ReactNode;
  errorTitle?: React.ReactNode;
  filters?: React.ReactNode;
  footer?: React.ReactNode;
  loading?: boolean;
  loadingLabel?: React.ReactNode;
  maxHeight?: React.CSSProperties['maxHeight'];
  resultsSummary?: React.ReactNode;
  size?: AnchoredPickerSurfaceSize;
  slotProps?: AnchoredPickerSurfaceSlotProps;
  title?: React.ReactNode;
  trigger: React.ReactElement;
}

const anchoredPickerSurfaceSizeClassNames: Record<AnchoredPickerSurfaceSize, string> = {
  sm: 'w-[min(92vw,20rem)]',
  md: 'w-[min(92vw,28rem)]',
  lg: 'w-[min(92vw,36rem)]',
  xl: 'w-[min(92vw,44rem)]',
};

const AnchoredPickerSurface = React.forwardRef<HTMLDivElement, AnchoredPickerSurfaceProps>(({
  actions,
  align = 'start',
  alignOffset,
  avoidCollisions,
  badge,
  bodyScrollable = true,
  children,
  className,
  collisionPadding = 12,
  defaultOpen = false,
  description,
  empty = false,
  emptyAction,
  emptyDescription,
  emptyKeyword,
  emptyTitle,
  error,
  errorTitle = 'Unable to load options',
  filters,
  footer,
  loading = false,
  loadingLabel = 'Loading options...',
  maxHeight = 'min(30rem, var(--radix-popover-content-available-height, 30rem))',
  modal,
  onOpenChange,
  open,
  resultsSummary,
  side = 'bottom',
  sideOffset = 8,
  size = 'lg',
  slotProps,
  title,
  trigger,
}, ref) => {
  const resolvedContent = loading ? (
    <LoadingBlock label={loadingLabel} />
  ) : error ? (
    <InlineAlert
      description={error}
      title={errorTitle}
      tone="danger"
    />
  ) : empty ? (
    <EmptySearch
      actions={emptyAction}
      description={emptyDescription}
      keyword={emptyKeyword}
      title={emptyTitle}
    />
  ) : (
    children
  );

  const hasHeader =
    title !== undefined ||
    description !== undefined ||
    badge !== undefined ||
    resultsSummary !== undefined ||
    actions !== undefined ||
    filters !== undefined;

  return (
    <Popover
      defaultOpen={defaultOpen}
      modal={modal}
      onOpenChange={onOpenChange}
      open={open}
    >
      <PopoverTrigger asChild>{trigger}</PopoverTrigger>
      <PopoverContent
        align={align}
        alignOffset={alignOffset}
        avoidCollisions={avoidCollisions}
        {...mergePatternSlotProps<
          PatternSlotProps<
            Omit<
              PopoverContentProps,
              | 'align'
              | 'alignOffset'
              | 'avoidCollisions'
              | 'children'
              | 'collisionPadding'
              | 'side'
              | 'sideOffset'
            >
          >
        >(
          {
            className: cn(
              'overflow-hidden border-[color-mix(in_srgb,var(--sdk-color-border-default)_88%,transparent)] bg-[linear-gradient(180deg,color-mix(in_srgb,var(--sdk-color-surface-panel)_98%,white_2%)_0%,color-mix(in_srgb,var(--sdk-color-surface-panel-muted)_90%,transparent)_100%)] p-0 shadow-[var(--sdk-shadow-md)]',
              anchoredPickerSurfaceSizeClassNames[size],
            ),
            'data-sdk-region': 'anchored-picker-content',
          },
          slotProps?.content,
        )}
        collisionPadding={collisionPadding}
        side={side}
        sideOffset={sideOffset}
      >
        <div
          ref={ref}
          {...mergePatternSlotProps<AnchoredPickerSurfaceSurfaceSlotProps>(
            {
              className: cn('flex min-h-0 min-w-0 flex-col overflow-hidden', className),
              'data-sdk-pattern': 'anchored-picker-surface',
              'data-sdk-region': 'anchored-picker-surface',
              style: { maxHeight },
            },
            slotProps?.surface,
          )}
        >
          {hasHeader ? (
            <div
              {...mergePatternSlotProps<AnchoredPickerSurfaceRegionSlotProps>(
                {
                  className:
                    'flex shrink-0 flex-col gap-3 border-b border-[color-mix(in_srgb,var(--sdk-color-border-default)_72%,transparent)] bg-[linear-gradient(180deg,color-mix(in_srgb,var(--sdk-color-surface-panel)_96%,white_4%)_0%,color-mix(in_srgb,var(--sdk-color-surface-panel-muted)_88%,transparent)_100%)] px-4 py-3',
                  'data-sdk-region': 'anchored-picker-header',
                },
                slotProps?.header,
              )}
            >
              {title !== undefined || description !== undefined || badge !== undefined || resultsSummary !== undefined || actions !== undefined ? (
                <div className="flex items-start gap-3">
                  <div className="min-w-0 flex-1">
                    <div className="flex flex-wrap items-center gap-2">
                      {title ? (
                        <div className="text-sm font-semibold tracking-tight text-[var(--sdk-color-text-primary)]">
                          {title}
                        </div>
                      ) : null}
                      {badge ? <div className="shrink-0">{badge}</div> : null}
                      {resultsSummary ? (
                        <div className="text-xs font-medium text-[var(--sdk-color-text-secondary)]">
                          {resultsSummary}
                        </div>
                      ) : null}
                    </div>
                    {description ? (
                      <div className="mt-1.5 text-xs leading-5 text-[var(--sdk-color-text-secondary)]">
                        {description}
                      </div>
                    ) : null}
                  </div>
                  {actions ? (
                    <div className="flex shrink-0 items-center gap-2">{actions}</div>
                  ) : null}
                </div>
              ) : null}

              {filters ? (
                <div
                  {...mergePatternSlotProps<AnchoredPickerSurfaceRegionSlotProps>(
                    {
                      className:
                        'flex flex-wrap items-center gap-2 rounded-[var(--sdk-radius-panel)] border border-[color-mix(in_srgb,var(--sdk-color-border-default)_72%,transparent)] bg-[color-mix(in_srgb,var(--sdk-color-surface-panel-muted)_88%,transparent)] px-3 py-2.5',
                      'data-sdk-region': 'anchored-picker-filters',
                    },
                    slotProps?.filters,
                  )}
                >
                  {filters}
                </div>
              ) : null}
            </div>
          ) : null}

          <div
            {...mergePatternSlotProps<AnchoredPickerSurfaceRegionSlotProps>(
              {
                className: cn(
                  'min-h-0 min-w-0 flex-1 px-3 py-3',
                  bodyScrollable ? 'overflow-y-auto' : 'overflow-hidden',
                ),
                'data-sdk-region': 'anchored-picker-body',
              },
              slotProps?.body,
            )}
          >
            {resolvedContent}
          </div>

          {footer ? (
            <div
              {...mergePatternSlotProps<AnchoredPickerSurfaceRegionSlotProps>(
                {
                  className:
                    'shrink-0 border-t border-[color-mix(in_srgb,var(--sdk-color-border-default)_72%,transparent)] bg-[linear-gradient(180deg,color-mix(in_srgb,var(--sdk-color-surface-panel)_94%,white_4%)_0%,color-mix(in_srgb,var(--sdk-color-surface-panel-muted)_88%,transparent)_100%)] px-4 py-3',
                  'data-sdk-region': 'anchored-picker-footer',
                },
                slotProps?.footer,
              )}
            >
              {footer}
            </div>
          ) : null}
        </div>
      </PopoverContent>
    </Popover>
  );
});

export { AnchoredPickerSurface };
AnchoredPickerSurface.displayName = 'AnchoredPickerSurface';
