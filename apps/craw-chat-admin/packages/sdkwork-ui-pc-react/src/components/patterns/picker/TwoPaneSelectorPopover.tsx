import * as React from 'react';
import { Check } from 'lucide-react';
import { cn } from '../../../lib/utils';
import {
  mergePatternSlotProps,
  type PatternSlotProps,
} from '../_internal/slot-props';
import { EmptyState } from '../../ui/feedback';
import { Popover } from '../../ui/popover';
import {
  AnchoredPickerSurface,
  type AnchoredPickerSurfaceSlotProps,
} from './AnchoredPickerSurface';

export interface TwoPaneSelectorSectionEntry<TSection, TItem> {
  items: TItem[];
  section: TSection;
}

export type TwoPaneSelectorPopoverItemActionHandler = () => void;
export type TwoPaneSelectorPopoverSectionActivateHandler = () => void;

export interface TwoPaneSelectorSectionRenderContext<TSection> {
  active: boolean;
  index: number;
  section: TSection;
  sectionId: React.Key;
  setActiveSection: TwoPaneSelectorPopoverSectionActivateHandler;
}

export interface TwoPaneSelectorItemRenderContext<TSection, TItem> {
  activeSection: TSection | undefined;
  close: TwoPaneSelectorPopoverItemActionHandler;
  index: number;
  item: TItem;
  itemId: React.Key;
  section: TSection;
  sectionIndex: number;
  select: TwoPaneSelectorPopoverItemActionHandler;
  selected: boolean;
}

export type TwoPaneSelectorPopoverItemBadgeRenderer<TSection, TItem> = (
  item: TItem,
  itemIndex: number,
  section: TSection,
  sectionIndex: number,
) => React.ReactNode;
export type TwoPaneSelectorPopoverItemDescriptionRenderer<TSection, TItem> = (
  item: TItem,
  itemIndex: number,
  section: TSection,
  sectionIndex: number,
) => React.ReactNode;
export type TwoPaneSelectorPopoverItemIdResolver<TSection, TItem> = (
  item: TItem,
  itemIndex: number,
  section: TSection,
  sectionIndex: number,
) => React.Key;
export type TwoPaneSelectorPopoverItemLabelRenderer<TSection, TItem> = (
  item: TItem,
  itemIndex: number,
  section: TSection,
  sectionIndex: number,
) => React.ReactNode;
export type TwoPaneSelectorPopoverSectionDescriptionRenderer<TSection> = (
  section: TSection,
  sectionIndex: number,
) => React.ReactNode;
export type TwoPaneSelectorPopoverSectionIdResolver<TSection> = (
  section: TSection,
  sectionIndex: number,
) => React.Key;
export type TwoPaneSelectorPopoverSectionLabelRenderer<TSection> = (
  section: TSection,
  sectionIndex: number,
) => React.ReactNode;
export type TwoPaneSelectorPopoverItemRenderer<TSection, TItem> = (
  context: TwoPaneSelectorItemRenderContext<TSection, TItem>,
) => React.ReactNode;
export type TwoPaneSelectorPopoverSectionRenderer<TSection> = (
  context: TwoPaneSelectorSectionRenderContext<TSection>,
) => React.ReactNode;
export type TwoPaneSelectorPopoverActiveSectionIdChangeHandler<TSection> = (
  sectionId: React.Key | null,
  section: TSection | undefined,
) => void;
export type TwoPaneSelectorPopoverValueChangeHandler<TSection, TItem> = (
  value: React.Key,
  item: TItem,
  section: TSection,
) => void;

export type TwoPaneSelectorPopoverRegionSlotProps = PatternSlotProps<
  Omit<React.ComponentPropsWithoutRef<'div'>, 'children'>
>;

export interface TwoPaneSelectorPopoverSlotProps
  extends Omit<AnchoredPickerSurfaceSlotProps, 'surface'> {
  items?: TwoPaneSelectorPopoverRegionSlotProps;
  pickerSurface?: NonNullable<AnchoredPickerSurfaceSlotProps['surface']>;
  sections?: TwoPaneSelectorPopoverRegionSlotProps;
  surface?: TwoPaneSelectorPopoverRegionSlotProps;
}

export interface TwoPaneSelectorPopoverProps<TSection = any, TItem = any>
  extends Omit<React.ComponentPropsWithoutRef<typeof Popover>, 'children'> {
  activeSectionId?: React.Key | null;
  defaultActiveSectionId?: React.Key | null;
  defaultValue?: React.Key | null;
  emptyDescription?: React.ReactNode;
  emptyTitle?: React.ReactNode;
  getItemBadge?: TwoPaneSelectorPopoverItemBadgeRenderer<TSection, TItem>;
  getItemDescription?: TwoPaneSelectorPopoverItemDescriptionRenderer<TSection, TItem>;
  getItemId?: TwoPaneSelectorPopoverItemIdResolver<TSection, TItem>;
  getItemLabel?: TwoPaneSelectorPopoverItemLabelRenderer<TSection, TItem>;
  getSectionDescription?: TwoPaneSelectorPopoverSectionDescriptionRenderer<TSection>;
  getSectionId?: TwoPaneSelectorPopoverSectionIdResolver<TSection>;
  getSectionLabel?: TwoPaneSelectorPopoverSectionLabelRenderer<TSection>;
  itemTitle?: React.ReactNode;
  onActiveSectionIdChange?: TwoPaneSelectorPopoverActiveSectionIdChangeHandler<TSection>;
  onValueChange?: TwoPaneSelectorPopoverValueChangeHandler<TSection, TItem>;
  renderItem?: TwoPaneSelectorPopoverItemRenderer<TSection, TItem>;
  renderSection?: TwoPaneSelectorPopoverSectionRenderer<TSection>;
  sectionTitle?: React.ReactNode;
  sections: Array<TwoPaneSelectorSectionEntry<TSection, TItem>>;
  slotProps?: TwoPaneSelectorPopoverSlotProps;
  trigger: React.ReactElement;
  value?: React.Key | null;
}

interface SectionRecord<TSection, TItem> {
  items: TItem[];
  index: number;
  section: TSection;
  sectionId: React.Key;
}

function resolveDefaultLabel(source: unknown, fallbackId: React.Key) {
  if (source && typeof source === 'object') {
    const label =
      (source as { label?: React.ReactNode }).label ??
      (source as { name?: React.ReactNode }).name;

    if (label !== undefined && label !== null) {
      return label;
    }
  }

  return String(fallbackId);
}

function resolveDefaultSectionId<TSection, TItem>(
  record: TwoPaneSelectorSectionEntry<TSection, TItem>,
  index: number,
) {
  const resolvedId = (record.section as { id?: React.Key }).id;
  return resolvedId ?? index;
}

function resolveDefaultItemId<TItem>(item: TItem, index: number) {
  const resolvedId = (item as { id?: React.Key }).id;
  return resolvedId ?? index;
}

type TwoPaneSelectorPopoverComponent = React.ForwardRefExoticComponent<
  TwoPaneSelectorPopoverProps & React.RefAttributes<HTMLDivElement>
> & {
  <TSection = any, TItem = any>(
    props: TwoPaneSelectorPopoverProps<TSection, TItem> & React.RefAttributes<HTMLDivElement>,
  ): React.ReactNode;
};

function TwoPaneSelectorPopoverInner<TSection = any, TItem = any>({
  activeSectionId: controlledActiveSectionId,
  defaultActiveSectionId = null,
  defaultOpen = false,
  defaultValue = null,
  emptyDescription = 'Choose a different section or add more options to continue.',
  emptyTitle = 'No options available',
  getItemBadge,
  getItemDescription,
  getItemId,
  getItemLabel,
  getSectionDescription,
  getSectionId,
  getSectionLabel,
  itemTitle = 'Options',
  modal,
  onActiveSectionIdChange,
  onOpenChange,
  onValueChange,
  open: controlledOpen,
  renderItem,
  renderSection,
  sectionTitle = 'Categories',
  sections,
  slotProps,
  trigger,
  value: controlledValue,
}: TwoPaneSelectorPopoverProps<TSection, TItem>, ref: React.ForwardedRef<HTMLDivElement>) {
  const [uncontrolledOpen, setUncontrolledOpen] = React.useState(defaultOpen);
  const [uncontrolledValue, setUncontrolledValue] = React.useState<React.Key | null>(defaultValue);
  const [uncontrolledActiveSectionId, setUncontrolledActiveSectionId] =
    React.useState<React.Key | null>(defaultActiveSectionId);

  const resolvedOpen = controlledOpen ?? uncontrolledOpen;
  const resolvedValue = controlledValue ?? uncontrolledValue;

  const sectionRecords = React.useMemo<SectionRecord<TSection, TItem>[]>(
    () =>
      sections.map((record, index) => ({
        items: record.items,
        index,
        section: record.section,
        sectionId: getSectionId?.(record.section, index) ?? resolveDefaultSectionId(record, index),
      })),
    [getSectionId, sections],
  );

  const selectedSectionRecord = React.useMemo(
    () =>
      sectionRecords.find((sectionRecord) =>
        sectionRecord.items.some(
          (item, itemIndex) =>
            Object.is(
              getItemId?.(item, itemIndex, sectionRecord.section, sectionRecord.index) ??
                resolveDefaultItemId(item, itemIndex),
              resolvedValue,
            ),
        ),
      ),
    [getItemId, resolvedValue, sectionRecords],
  );

  const fallbackSectionId =
    selectedSectionRecord?.sectionId ?? sectionRecords[0]?.sectionId ?? null;

  const resolvedActiveSectionId =
    controlledActiveSectionId ?? uncontrolledActiveSectionId ?? fallbackSectionId;

  const activeSectionRecord = React.useMemo(
    () =>
      sectionRecords.find((sectionRecord) =>
        Object.is(sectionRecord.sectionId, resolvedActiveSectionId),
      ) ??
      selectedSectionRecord ??
      sectionRecords[0],
    [resolvedActiveSectionId, sectionRecords, selectedSectionRecord],
  );

  const setOpen = React.useCallback(
    (nextOpen: boolean) => {
      if (controlledOpen === undefined) {
        setUncontrolledOpen(nextOpen);
      }
      onOpenChange?.(nextOpen);
    },
    [controlledOpen, onOpenChange],
  );

  const setActiveSectionId = React.useCallback(
    (nextSectionId: React.Key | null) => {
      if (controlledActiveSectionId === undefined) {
        setUncontrolledActiveSectionId(nextSectionId);
      }
      const nextSection = sectionRecords.find((record) => Object.is(record.sectionId, nextSectionId))
        ?.section;
      onActiveSectionIdChange?.(nextSectionId, nextSection);
    },
    [controlledActiveSectionId, onActiveSectionIdChange, sectionRecords],
  );

  React.useEffect(() => {
    if (!resolvedOpen || controlledActiveSectionId !== undefined) {
      return;
    }

    const activeSectionStillExists =
      uncontrolledActiveSectionId !== null &&
      sectionRecords.some((record) => Object.is(record.sectionId, uncontrolledActiveSectionId));

    if (!activeSectionStillExists && fallbackSectionId !== null) {
      setUncontrolledActiveSectionId(fallbackSectionId);
    }
  }, [
    controlledActiveSectionId,
    fallbackSectionId,
    resolvedOpen,
    sectionRecords,
    uncontrolledActiveSectionId,
  ]);

  const selectValue = React.useCallback(
    (nextValue: React.Key, item: TItem, section: TSection) => {
      if (controlledValue === undefined) {
        setUncontrolledValue(nextValue);
      }

      onValueChange?.(nextValue, item, section);
      setOpen(false);
    },
    [controlledValue, onValueChange, setOpen],
  );

  return (
    <AnchoredPickerSurface
      bodyScrollable={false}
      modal={modal}
      onOpenChange={setOpen}
      open={resolvedOpen}
      size="lg"
      slotProps={{
        body: mergePatternSlotProps<NonNullable<AnchoredPickerSurfaceSlotProps['body']>>(
          { className: 'p-0' },
          slotProps?.body,
        ),
        content: slotProps?.content,
        filters: slotProps?.filters,
        footer: slotProps?.footer,
        header: slotProps?.header,
        surface: slotProps?.pickerSurface,
      }}
      trigger={trigger}
    >
      <div
        ref={ref}
        {...mergePatternSlotProps<TwoPaneSelectorPopoverRegionSlotProps>(
          {
            className: 'flex h-[20rem] min-h-0',
            'data-sdk-pattern': 'two-pane-selector-popover',
            'data-sdk-region': 'two-pane-selector-surface',
          },
          slotProps?.surface,
        )}
      >
        <div
          {...mergePatternSlotProps<TwoPaneSelectorPopoverRegionSlotProps>(
            {
              className:
                'flex w-[11rem] shrink-0 flex-col border-r border-[var(--sdk-color-border-default)] bg-[color-mix(in_srgb,var(--sdk-color-surface-panel-muted)_88%,transparent)]',
              'data-sdk-region': 'two-pane-selector-sections',
            },
            slotProps?.sections,
          )}
        >
          <div className="border-b border-[var(--sdk-color-border-default)] px-3 py-2 text-[11px] font-semibold uppercase tracking-[0.08em] text-[var(--sdk-color-text-muted)]">
            {sectionTitle}
          </div>
          <div className="flex-1 overflow-y-auto p-2">
            <div className="flex flex-col gap-1">
              {sectionRecords.map((sectionRecord) => {
                const active = Object.is(sectionRecord.sectionId, activeSectionRecord?.sectionId);
                const sectionContext: TwoPaneSelectorSectionRenderContext<TSection> = {
                  active,
                  index: sectionRecord.index,
                  section: sectionRecord.section,
                  sectionId: sectionRecord.sectionId,
                  setActiveSection: () => setActiveSectionId(sectionRecord.sectionId),
                };

                if (renderSection) {
                  return (
                    <React.Fragment key={sectionRecord.sectionId}>
                      {renderSection(sectionContext)}
                    </React.Fragment>
                  );
                }

                return (
                  <button
                    key={sectionRecord.sectionId}
                    className={cn(
                      'flex w-full flex-col items-start rounded-[var(--sdk-radius-field)] border px-3 py-2 text-left text-sm transition-colors',
                      active
                        ? 'border-[color-mix(in_srgb,var(--sdk-color-brand-primary)_26%,transparent)] bg-[var(--sdk-color-brand-primary-soft)] text-[var(--sdk-color-text-primary)]'
                        : 'border-transparent text-[var(--sdk-color-text-secondary)] hover:bg-[var(--sdk-color-surface-panel)] hover:text-[var(--sdk-color-text-primary)]',
                    )}
                    data-slot="two-pane-selector-section-trigger"
                    onClick={sectionContext.setActiveSection}
                    type="button"
                  >
                    <span className="truncate font-medium">
                      {getSectionLabel?.(sectionRecord.section, sectionRecord.index) ??
                        resolveDefaultLabel(sectionRecord.section, sectionRecord.sectionId)}
                    </span>
                    {getSectionDescription?.(sectionRecord.section, sectionRecord.index) ? (
                      <span className="mt-1 line-clamp-2 text-xs text-[var(--sdk-color-text-muted)]">
                        {getSectionDescription(sectionRecord.section, sectionRecord.index)}
                      </span>
                    ) : null}
                  </button>
                );
              })}
            </div>
          </div>
        </div>

        <div
          {...mergePatternSlotProps<TwoPaneSelectorPopoverRegionSlotProps>(
            {
              className: 'flex min-w-0 flex-1 flex-col',
              'data-sdk-region': 'two-pane-selector-items',
            },
            slotProps?.items,
          )}
        >
          <div className="border-b border-[var(--sdk-color-border-default)] px-4 py-2 text-[11px] font-semibold uppercase tracking-[0.08em] text-[var(--sdk-color-text-muted)]">
            {itemTitle}
          </div>
          <div className="min-h-0 flex-1 overflow-y-auto p-2">
            {activeSectionRecord && activeSectionRecord.items.length > 0 ? (
              <div className="flex flex-col gap-1.5">
                {activeSectionRecord.items.map((item, itemIndex) => {
                  const itemId =
                    getItemId?.(item, itemIndex, activeSectionRecord.section, activeSectionRecord.index) ??
                    resolveDefaultItemId(item, itemIndex);
                  const selected = Object.is(itemId, resolvedValue);
                  const itemContext: TwoPaneSelectorItemRenderContext<TSection, TItem> = {
                    activeSection: activeSectionRecord.section,
                    close: () => setOpen(false),
                    index: itemIndex,
                    item,
                    itemId,
                    section: activeSectionRecord.section,
                    sectionIndex: activeSectionRecord.index,
                    select: () => selectValue(itemId, item, activeSectionRecord.section),
                    selected,
                  };

                  if (renderItem) {
                    return (
                      <React.Fragment key={itemId}>
                        {renderItem(itemContext)}
                      </React.Fragment>
                    );
                  }

                  return (
                    <button
                      key={itemId}
                      className={cn(
                        'flex w-full items-start gap-3 rounded-[var(--sdk-radius-field)] border px-3 py-3 text-left transition-colors',
                        selected
                          ? 'border-[color-mix(in_srgb,var(--sdk-color-brand-primary)_30%,transparent)] bg-[var(--sdk-color-brand-primary-soft)] text-[var(--sdk-color-text-primary)]'
                          : 'border-transparent text-[var(--sdk-color-text-primary)] hover:border-[var(--sdk-color-border-default)] hover:bg-[var(--sdk-color-surface-panel-muted)]',
                      )}
                      data-slot="two-pane-selector-item-trigger"
                      onClick={itemContext.select}
                      type="button"
                    >
                      <div className="min-w-0 flex-1">
                        <div className="flex flex-wrap items-center gap-2">
                          <span className="truncate text-sm font-semibold">
                            {getItemLabel?.(
                              item,
                              itemIndex,
                              activeSectionRecord.section,
                              activeSectionRecord.index,
                            ) ?? resolveDefaultLabel(item, itemId)}
                          </span>
                          {getItemBadge?.(
                            item,
                            itemIndex,
                            activeSectionRecord.section,
                            activeSectionRecord.index,
                          ) ? (
                            <span className="shrink-0 rounded-full border border-[var(--sdk-color-border-default)] px-2 py-0.5 text-[10px] font-medium text-[var(--sdk-color-text-secondary)]">
                              {getItemBadge(
                                item,
                                itemIndex,
                                activeSectionRecord.section,
                                activeSectionRecord.index,
                              )}
                            </span>
                          ) : null}
                        </div>
                        {getItemDescription?.(
                          item,
                          itemIndex,
                          activeSectionRecord.section,
                          activeSectionRecord.index,
                        ) ? (
                          <div className="mt-1 text-xs leading-5 text-[var(--sdk-color-text-muted)]">
                            {getItemDescription(
                              item,
                              itemIndex,
                              activeSectionRecord.section,
                              activeSectionRecord.index,
                            )}
                          </div>
                        ) : null}
                      </div>
                      {selected ? (
                        <Check className="mt-0.5 h-4 w-4 shrink-0 text-[var(--sdk-color-brand-primary)]" />
                      ) : null}
                    </button>
                  );
                })}
              </div>
            ) : (
              <EmptyState
                className="h-full justify-center"
                description={emptyDescription}
                title={emptyTitle}
              />
            )}
          </div>
        </div>
      </div>
    </AnchoredPickerSurface>
  );
}

const TwoPaneSelectorPopover: TwoPaneSelectorPopoverComponent = React.forwardRef<
  HTMLDivElement,
  TwoPaneSelectorPopoverProps
>(
  TwoPaneSelectorPopoverInner as (
    props: TwoPaneSelectorPopoverProps,
    ref: React.ForwardedRef<HTMLDivElement>,
  ) => React.ReactNode,
);

export { TwoPaneSelectorPopover };
TwoPaneSelectorPopover.displayName = 'TwoPaneSelectorPopover';
