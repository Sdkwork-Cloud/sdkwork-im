import * as React from 'react';
import { mergeOptionalSlotProps, mergeSlotProps, type SlotProps } from '../../../lib/slot-props';
import { cn } from '../../../lib/utils';

export type StepperOrientation = 'horizontal' | 'vertical';
export type StepperItemStatus = 'complete' | 'current' | 'upcoming' | 'error';

export type StepperItemRootProps = SlotProps<
  Omit<React.ComponentPropsWithoutRef<'li'>, 'children' | 'title'>
>;
export type StepperItemRegionSlotProps = SlotProps<Omit<React.ComponentPropsWithoutRef<'div'>, 'children'>>;
export type StepperItemIndicatorSlotProps = SlotProps<Omit<React.ComponentPropsWithoutRef<'span'>, 'children'>>;

export interface StepperCollectionItemState {
  index: number;
  last: boolean;
  orientation: StepperOrientation;
  status: StepperItemStatus;
}

export type StepperItemPropsResolver = (state: StepperCollectionItemState) => StepperItemRootProps | undefined;
export type StepperItemSlotPropsResolver = (
  state: StepperCollectionItemState,
) => StepperItemSlotProps | undefined;

export interface StepperItemSlotProps {
  connector?: StepperItemIndicatorSlotProps;
  content?: StepperItemRegionSlotProps;
  indicator?: StepperItemIndicatorSlotProps;
}

export interface StepperProps extends React.OlHTMLAttributes<HTMLOListElement> {
  getItemProps?: StepperItemPropsResolver;
  getItemSlotProps?: StepperItemSlotPropsResolver;
  orientation?: StepperOrientation;
}

export interface StepperItemProps extends Omit<React.LiHTMLAttributes<HTMLLIElement>, 'title'> {
  description?: React.ReactNode;
  icon?: React.ReactNode;
  last?: boolean;
  meta?: React.ReactNode;
  stepNumber?: number;
  status?: StepperItemStatus;
  slotProps?: StepperItemSlotProps;
  title: React.ReactNode;
}

const StepperContext = React.createContext<StepperOrientation>('horizontal');

const indicatorShellClassName: Record<StepperItemStatus, string> = {
  complete:
    'border-[var(--sdk-color-state-success)] bg-[color-mix(in_srgb,var(--sdk-color-state-success)_14%,var(--sdk-color-surface-panel))] text-[var(--sdk-color-state-success)]',
  current:
    'border-[var(--sdk-color-brand-primary)] bg-[var(--sdk-color-brand-primary-soft)] text-[var(--sdk-color-brand-primary)]',
  error:
    'border-[var(--sdk-color-state-danger)] bg-[color-mix(in_srgb,var(--sdk-color-state-danger)_14%,var(--sdk-color-surface-panel))] text-[var(--sdk-color-state-danger)]',
  upcoming:
    'border-[var(--sdk-color-border-default)] bg-[var(--sdk-color-surface-panel-muted)] text-[var(--sdk-color-text-muted)]',
};

const connectorClassName: Record<Exclude<StepperItemStatus, 'error'>, string> = {
  complete: 'bg-[var(--sdk-color-state-success)]/35',
  current: 'bg-[var(--sdk-color-brand-primary)]/25',
  upcoming: 'bg-[var(--sdk-color-border-default)]',
};

function mergeStepperItemSlotProps(
  baseSlotProps?: StepperItemSlotProps,
  overrideSlotProps?: StepperItemSlotProps,
): StepperItemSlotProps | undefined {
  const mergedSlotProps: StepperItemSlotProps = {};

  const connector = mergeOptionalSlotProps(baseSlotProps?.connector, overrideSlotProps?.connector);
  const content = mergeOptionalSlotProps(baseSlotProps?.content, overrideSlotProps?.content);
  const indicator = mergeOptionalSlotProps(baseSlotProps?.indicator, overrideSlotProps?.indicator);

  if (connector) {
    mergedSlotProps.connector = connector;
  }

  if (content) {
    mergedSlotProps.content = content;
  }

  if (indicator) {
    mergedSlotProps.indicator = indicator;
  }

  return Object.keys(mergedSlotProps).length > 0 ? mergedSlotProps : undefined;
}

const Stepper = React.forwardRef<HTMLOListElement, StepperProps>(
  ({ children, className, getItemProps, getItemSlotProps, orientation = 'horizontal', ...props }, ref) => {
    const items = React.Children.toArray(children).map((child, index, collection) => {
      if (!React.isValidElement<StepperItemProps>(child)) {
        return child;
      }

      const childProps = child.props as StepperItemProps;
      const itemState: StepperCollectionItemState = {
        index,
        last: index === collection.length - 1,
        orientation,
        status: childProps.status ?? 'upcoming',
      };
      const parentItemProps = getItemProps?.(itemState);

      return React.cloneElement(child, {
        ...parentItemProps,
        ...childProps,
        className: cn(parentItemProps?.className, childProps.className),
        last: itemState.last,
        slotProps: mergeStepperItemSlotProps(getItemSlotProps?.(itemState), childProps.slotProps),
        style:
          parentItemProps?.style || childProps.style
            ? {
                ...parentItemProps?.style,
                ...childProps.style,
              }
            : undefined,
        stepNumber: index + 1,
      });
    });

    return (
      <StepperContext.Provider value={orientation}>
        <ol
          ref={ref}
          className={cn(
            orientation === 'horizontal' ? 'flex flex-wrap items-start gap-3' : 'space-y-3',
            className,
          )}
          data-orientation={orientation}
          data-sdk-ui="stepper"
          {...props}
        >
          {items}
        </ol>
      </StepperContext.Provider>
    );
  },
);

Stepper.displayName = 'Stepper';

const StepperItem = React.forwardRef<HTMLLIElement, StepperItemProps>(
  (
    {
      className,
      description,
      icon,
      last = false,
      meta,
      slotProps,
      status = 'upcoming',
      stepNumber = 1,
      title,
      ...props
    },
    ref,
  ) => {
    const orientation = React.useContext(StepperContext);
    const indicator = icon ?? stepNumber;

    if (orientation === 'vertical') {
      return (
        <li
          ref={ref}
          className={cn('grid grid-cols-[auto_minmax(0,1fr)] gap-3', className)}
          data-orientation={orientation}
          data-sdk-ui="stepper-item"
          data-status={status}
          {...props}
        >
          <div className="flex flex-col items-center">
            <span
              {...mergeSlotProps<StepperItemIndicatorSlotProps>(
                {
                  className: cn(
                    'flex h-9 w-9 shrink-0 items-center justify-center rounded-full border text-sm font-semibold shadow-[var(--sdk-shadow-soft)]',
                    indicatorShellClassName[status],
                  ),
                  'data-sdk-region': 'stepper-item-indicator',
                },
                slotProps?.indicator,
              )}
            >
              {indicator}
            </span>
            {!last ? (
              <span
                {...mergeSlotProps<StepperItemIndicatorSlotProps>(
                  {
                    'aria-hidden': 'true',
                    className: cn(
                      'mt-2 w-px flex-1 rounded-full',
                      status === 'error' ? connectorClassName.upcoming : connectorClassName[status],
                    ),
                    'data-sdk-region': 'stepper-item-connector',
                  },
                  slotProps?.connector,
                )}
              />
            ) : null}
          </div>
          <div
            {...mergeSlotProps<StepperItemRegionSlotProps>(
              {
                className:
                  'min-w-0 rounded-[var(--sdk-radius-panel)] border border-[var(--sdk-color-border-default)] bg-[var(--sdk-color-surface-panel)] px-4 py-3 shadow-[var(--sdk-shadow-soft)]',
                'data-sdk-region': 'stepper-item-content',
              },
              slotProps?.content,
            )}
          >
            <div className="flex flex-wrap items-center gap-2">
              <div className="text-sm font-semibold text-[var(--sdk-color-text-primary)]">{title}</div>
              {meta ? (
                <div className="text-[11px] font-medium uppercase tracking-[0.12em] text-[var(--sdk-color-text-muted)]">
                  {meta}
                </div>
              ) : null}
            </div>
            {description ? (
              <div className="mt-1.5 text-sm text-[var(--sdk-color-text-secondary)]">{description}</div>
            ) : null}
          </div>
        </li>
      );
    }

    return (
      <li
        ref={ref}
        className={cn('flex min-w-[12rem] flex-1 items-start gap-3', className)}
        data-orientation={orientation}
        data-sdk-ui="stepper-item"
        data-status={status}
        {...props}
      >
        <div className="flex min-w-0 flex-1 items-start gap-3">
          <span
            {...mergeSlotProps<StepperItemIndicatorSlotProps>(
              {
                className: cn(
                  'flex h-9 w-9 shrink-0 items-center justify-center rounded-full border text-sm font-semibold shadow-[var(--sdk-shadow-soft)]',
                  indicatorShellClassName[status],
                ),
                'data-sdk-region': 'stepper-item-indicator',
              },
              slotProps?.indicator,
            )}
          >
            {indicator}
          </span>
          <div
            {...mergeSlotProps<StepperItemRegionSlotProps>(
              {
                className: 'min-w-0',
                'data-sdk-region': 'stepper-item-content',
              },
              slotProps?.content,
            )}
          >
            <div className="flex flex-wrap items-center gap-2">
              <div className="text-sm font-semibold text-[var(--sdk-color-text-primary)]">{title}</div>
              {meta ? (
                <div className="text-[11px] font-medium uppercase tracking-[0.12em] text-[var(--sdk-color-text-muted)]">
                  {meta}
                </div>
              ) : null}
            </div>
            {description ? (
              <div className="mt-1.5 text-sm text-[var(--sdk-color-text-secondary)]">{description}</div>
            ) : null}
          </div>
        </div>
        {!last ? (
          <span
            {...mergeSlotProps<StepperItemIndicatorSlotProps>(
              {
                'aria-hidden': 'true',
                className: cn(
                  'mt-[1.125rem] h-px min-w-6 flex-1 rounded-full',
                  status === 'error' ? connectorClassName.upcoming : connectorClassName[status],
                ),
                'data-sdk-region': 'stepper-item-connector',
              },
              slotProps?.connector,
            )}
          />
        ) : null}
      </li>
    );
  },
);

StepperItem.displayName = 'StepperItem';

export { Stepper, StepperItem };
