import type { CSSProperties } from 'react';
import { cn } from './utils';

type DataAttributeValue = string | number | boolean | undefined;

export type DataAttributes = {
  [K in `data-${string}`]?: DataAttributeValue;
};

export type SlotProps<T extends object> = T & DataAttributes;

type MergeableSlotProps = {
  className?: string;
  style?: CSSProperties;
};

export function mergeSlotProps<T extends object>(
  baseProps: T,
  overrideProps?: Partial<T>,
): T {
  const basePropsWithMerge = baseProps as T & MergeableSlotProps;
  const overridePropsWithMerge = overrideProps as Partial<T & MergeableSlotProps> | undefined;

  if (!overrideProps) {
    return baseProps;
  }

  const mergedStyle =
    basePropsWithMerge.style || overridePropsWithMerge?.style
      ? {
          ...basePropsWithMerge.style,
          ...overridePropsWithMerge?.style,
        }
      : undefined;

  return {
    ...baseProps,
    ...overrideProps,
    className: cn(basePropsWithMerge.className, overridePropsWithMerge?.className),
    style: mergedStyle,
  } as T;
}

export function mergeOptionalSlotProps<T extends object>(
  baseProps?: T,
  overrideProps?: Partial<T>,
): T | undefined {
  if (baseProps && overrideProps) {
    return mergeSlotProps(baseProps, overrideProps);
  }

  return overrideProps ? ({ ...overrideProps } as T) : baseProps;
}
