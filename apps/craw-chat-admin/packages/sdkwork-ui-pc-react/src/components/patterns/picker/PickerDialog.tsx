import * as React from 'react';
import { cn } from '../../../lib/utils';
import {
  mergePatternSlotProps,
  type PatternSlotProps,
} from '../_internal/slot-props';
import { EmptySearch, InlineAlert, LoadingBlock } from '../../ui/feedback';
import {
  Modal,
  ModalContent,
  type ModalContentProps,
  ModalDescription,
  ModalFooter,
  type ModalFooterProps,
  ModalHeader,
  type ModalHeaderProps,
  ModalTitle,
} from '../../ui/overlays';

export type PickerDialogSize = 'sm' | 'md' | 'lg' | 'xl' | 'full';

export type PickerDialogRegionSlotProps = PatternSlotProps<
  Omit<React.ComponentPropsWithoutRef<'div'>, 'children'>
>;
export type PickerDialogSidebarSlotProps = PatternSlotProps<
  Omit<React.ComponentPropsWithoutRef<'aside'>, 'children'>
>;
export type PickerDialogSurfaceSlotProps = PatternSlotProps<
  Omit<React.ComponentPropsWithoutRef<'div'>, 'children'>
>;

export interface PickerDialogSlotProps {
  body?: PickerDialogRegionSlotProps;
  content?: PatternSlotProps<Omit<ModalContentProps, 'align' | 'children' | 'size'>>;
  filters?: PickerDialogRegionSlotProps;
  footer?: PatternSlotProps<Omit<ModalFooterProps, 'children'>>;
  header?: PatternSlotProps<Omit<ModalHeaderProps, 'children'>>;
  sidebar?: PickerDialogSidebarSlotProps;
  surface?: PickerDialogSurfaceSlotProps;
}

export interface PickerDialogProps extends React.ComponentPropsWithoutRef<typeof Modal> {
  actions?: React.ReactNode;
  badge?: React.ReactNode;
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
  resultsSummary?: React.ReactNode;
  sidebar?: React.ReactNode;
  size?: PickerDialogSize;
  slotProps?: PickerDialogSlotProps;
  title: React.ReactNode;
}

const PickerDialog = React.forwardRef<HTMLDivElement, PickerDialogProps>(({
  actions,
  badge,
  children,
  className,
  description,
  empty = false,
  emptyAction,
  emptyDescription,
  emptyKeyword,
  emptyTitle,
  error,
  errorTitle = 'Unable to load picker data',
  filters,
  footer,
  loading = false,
  loadingLabel = 'Loading options...',
  resultsSummary,
  sidebar,
  size = 'xl',
  slotProps,
  title,
  ...props
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

  return (
    <Modal {...props}>
      <ModalContent
        {...(description ? {} : { 'aria-describedby': undefined })}
        {...mergePatternSlotProps<PatternSlotProps<Omit<ModalContentProps, 'align' | 'children' | 'size'>>>(
          {
            className:
              'gap-0 overflow-hidden border-[color-mix(in_srgb,var(--sdk-color-border-default)_88%,transparent)] bg-[linear-gradient(180deg,color-mix(in_srgb,var(--sdk-color-surface-panel)_98%,white_2%)_0%,color-mix(in_srgb,var(--sdk-color-surface-panel-muted)_90%,transparent)_100%)]',
            'data-sdk-region': 'picker-dialog-content',
          },
          slotProps?.content,
        )}
        size={size}
      >
        <div
          ref={ref}
          {...mergePatternSlotProps<PickerDialogSurfaceSlotProps>(
            {
              className: cn('flex h-full min-h-0 min-w-0 flex-col overflow-hidden', className),
              'data-sdk-pattern': 'picker-dialog',
              'data-sdk-region': 'picker-dialog-surface',
            },
            slotProps?.surface,
          )}
        >
          <ModalHeader
            {...mergePatternSlotProps<PatternSlotProps<Omit<ModalHeaderProps, 'children'>>>(
              {
                className:
                  'gap-3 border-b-[color-mix(in_srgb,var(--sdk-color-border-default)_78%,transparent)] bg-[linear-gradient(180deg,color-mix(in_srgb,var(--sdk-color-surface-panel)_96%,white_4%)_0%,color-mix(in_srgb,var(--sdk-color-surface-panel-muted)_88%,transparent)_100%)] px-5 py-4 xl:px-6',
                'data-sdk-region': 'picker-dialog-header',
              },
              slotProps?.header,
            )}
          >
            <div className="flex items-start gap-4">
              <div className="min-w-0 flex-1">
                <div className="flex flex-wrap items-center gap-3">
                  <ModalTitle className="text-[1.35rem] tracking-tight">{title}</ModalTitle>
                  {badge ? <div className="shrink-0">{badge}</div> : null}
                  {resultsSummary ? (
                    <div className="text-sm font-medium text-[var(--sdk-color-text-secondary)]">
                      {resultsSummary}
                    </div>
                  ) : null}
                </div>
                {description ? (
                  <ModalDescription className="mt-2 max-w-4xl text-sm leading-6">
                    {description}
                  </ModalDescription>
                ) : null}
              </div>
              {actions ? <div className="flex shrink-0 items-center gap-2 pr-8">{actions}</div> : null}
            </div>
            {filters ? (
              <div
                {...mergePatternSlotProps<PickerDialogRegionSlotProps>(
                  {
                    className:
                      'flex flex-wrap items-center gap-3 rounded-[var(--sdk-radius-panel)] border border-[color-mix(in_srgb,var(--sdk-color-border-default)_72%,transparent)] bg-[color-mix(in_srgb,var(--sdk-color-surface-panel-muted)_88%,transparent)] px-3 py-3',
                    'data-sdk-region': 'picker-dialog-filters',
                  },
                  slotProps?.filters,
                )}
              >
                {filters}
              </div>
            ) : null}
          </ModalHeader>

          <div className="flex min-h-0 flex-1 overflow-hidden">
            {sidebar ? (
              <aside
                {...mergePatternSlotProps<PickerDialogSidebarSlotProps>(
                  {
                    className:
                      'hidden w-[18rem] shrink-0 border-r border-[color-mix(in_srgb,var(--sdk-color-border-default)_72%,transparent)] bg-[linear-gradient(180deg,color-mix(in_srgb,var(--sdk-color-surface-panel-muted)_92%,transparent)_0%,color-mix(in_srgb,var(--sdk-color-surface-canvas)_86%,transparent)_100%)] lg:flex lg:min-h-0 lg:flex-col',
                    'data-sdk-region': 'picker-dialog-sidebar',
                  },
                  slotProps?.sidebar,
                )}
              >
                <div className="min-h-0 flex-1 overflow-y-auto p-5">{sidebar}</div>
              </aside>
            ) : null}

            <div className="min-h-0 min-w-0 flex-1 overflow-y-auto">
              <div
                {...mergePatternSlotProps<PickerDialogRegionSlotProps>(
                  {
                    className: 'min-h-full px-5 py-5 xl:px-6',
                    'data-sdk-region': 'picker-dialog-body',
                  },
                  slotProps?.body,
                )}
              >
                {resolvedContent}
              </div>
            </div>
          </div>

          {footer ? (
            <ModalFooter
              {...mergePatternSlotProps<PatternSlotProps<Omit<ModalFooterProps, 'children'>>>(
                {
                  className:
                    'border-t-[color-mix(in_srgb,var(--sdk-color-border-default)_78%,transparent)] bg-[linear-gradient(180deg,color-mix(in_srgb,var(--sdk-color-surface-panel)_94%,white_4%)_0%,color-mix(in_srgb,var(--sdk-color-surface-panel-muted)_88%,transparent)_100%)] px-5 py-3.5 xl:px-6',
                  'data-sdk-region': 'picker-dialog-footer',
                },
                slotProps?.footer,
              )}
            >
              {footer}
            </ModalFooter>
          ) : null}
        </div>
      </ModalContent>
    </Modal>
  );
});

export { PickerDialog };
PickerDialog.displayName = 'PickerDialog';
