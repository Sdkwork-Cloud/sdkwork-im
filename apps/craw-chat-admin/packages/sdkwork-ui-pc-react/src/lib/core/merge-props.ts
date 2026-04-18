import type { CSSProperties } from 'react';
import { cn } from '../utils';

type MergeableProps = {
  className?: string;
  style?: CSSProperties;
  [key: string]: unknown;
};

function isEventHandlerKey(key: string) {
  return /^on[A-Z]/.test(key);
}

export function mergeProps<T extends MergeableProps>(
  baseProps: T,
  overrideProps?: Partial<T>,
): T {
  if (!overrideProps) {
    return baseProps;
  }

  const mergedEntries = Object.entries(overrideProps).map(([key, overrideValue]) => {
    const baseValue = baseProps[key];

    if (key === 'className') {
      return [key, cn(baseValue as string | undefined, overrideValue as string | undefined)];
    }

    if (key === 'style') {
      return [
        key,
        baseValue || overrideValue
          ? {
              ...(baseValue as CSSProperties | undefined),
              ...(overrideValue as CSSProperties | undefined),
            }
          : undefined,
      ];
    }

    if (
      isEventHandlerKey(key)
      && typeof baseValue === 'function'
      && typeof overrideValue === 'function'
    ) {
      return [
        key,
        (...args: unknown[]) => {
          (baseValue as (...handlerArgs: unknown[]) => void)(...args);
          (overrideValue as (...handlerArgs: unknown[]) => void)(...args);
        },
      ];
    }

    return [key, overrideValue];
  });

  return {
    ...baseProps,
    ...Object.fromEntries(mergedEntries),
  } as T;
}
