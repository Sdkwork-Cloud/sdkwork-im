import * as React from 'react';

export interface UseControllableStateOptions<T> {
  defaultValue: T;
  onChange?: (value: T) => void;
  value?: T;
}

export type ControllableStateUpdater<T> = T | ((previousValue: T) => T);

export function useControllableState<T>({
  defaultValue,
  onChange,
  value,
}: UseControllableStateOptions<T>) {
  const [uncontrolledValue, setUncontrolledValue] = React.useState(defaultValue);
  const isControlled = value !== undefined;
  const currentValue = isControlled ? value : uncontrolledValue;

  const setValue = React.useCallback(
    (nextValueOrUpdater: ControllableStateUpdater<T>) => {
      const nextValue =
        typeof nextValueOrUpdater === 'function'
          ? (nextValueOrUpdater as (previousValue: T) => T)(currentValue)
          : nextValueOrUpdater;

      if (Object.is(nextValue, currentValue)) {
        return;
      }

      if (!isControlled) {
        setUncontrolledValue(nextValue);
      }

      onChange?.(nextValue);
    },
    [currentValue, isControlled, onChange],
  );

  return [currentValue, setValue] as const;
}
