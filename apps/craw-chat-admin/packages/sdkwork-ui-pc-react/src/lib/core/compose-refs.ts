import * as React from 'react';

function assignRef<T>(ref: React.ForwardedRef<T> | undefined, value: T | null) {
  if (typeof ref === 'function') {
    ref(value);
    return;
  }

  if (ref) {
    ref.current = value;
  }
}

export function composeRefs<T>(...refs: Array<React.ForwardedRef<T> | undefined>) {
  return (value: T | null) => {
    refs.forEach((ref) => assignRef(ref, value));
  };
}
