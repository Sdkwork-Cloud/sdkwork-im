import type { CSSProperties } from 'react';
import { describe, expect, it, vi } from 'vitest';
import { mergeProps } from './merge-props';

describe('mergeProps', () => {
  it('merges class names, style objects, and event handlers', () => {
    const baseHandler = vi.fn();
    const overrideHandler = vi.fn();
    const merged = mergeProps<{
      className?: string;
      onClick?: (...args: unknown[]) => void;
      style?: CSSProperties;
    }>(
      {
        className: 'base',
        onClick: baseHandler,
        style: {
          color: 'red',
        },
      },
      {
        className: 'override',
        onClick: overrideHandler,
        style: {
          display: 'flex',
        },
      },
    );

    expect(merged.className).toContain('base');
    expect(merged.className).toContain('override');
    expect(merged.style).toEqual({
      color: 'red',
      display: 'flex',
    });

    merged.onClick?.({} as never);

    expect(baseHandler).toHaveBeenCalledTimes(1);
    expect(overrideHandler).toHaveBeenCalledTimes(1);
  });
});
