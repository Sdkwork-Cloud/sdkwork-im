import * as React from 'react';
import { fireEvent, render, screen } from '@testing-library/react';
import { describe, expect, it, vi } from 'vitest';
import { useControllableState } from './use-controllable-state';

function HookProbe({
  defaultValue = 1,
  onChange,
  value,
}: {
  defaultValue?: number;
  onChange?: (value: number) => void;
  value?: number;
}) {
  const [currentValue, setCurrentValue] = useControllableState({
    defaultValue,
    onChange,
    value,
  });

  return (
    <button onClick={() => setCurrentValue((previousValue) => previousValue + 1)} type="button">
      {currentValue}
    </button>
  );
}

describe('useControllableState', () => {
  it('updates uncontrolled state and notifies listeners', () => {
    const handleChange = vi.fn();

    render(<HookProbe defaultValue={2} onChange={handleChange} />);

    fireEvent.click(screen.getByRole('button', { name: '2' }));

    expect(screen.getByRole('button', { name: '3' })).toBeInTheDocument();
    expect(handleChange).toHaveBeenCalledWith(3);
  });

  it('keeps controlled state external while still notifying listeners', () => {
    const handleChange = vi.fn();

    render(<HookProbe onChange={handleChange} value={4} />);

    fireEvent.click(screen.getByRole('button', { name: '4' }));

    expect(screen.getByRole('button', { name: '4' })).toBeInTheDocument();
    expect(handleChange).toHaveBeenCalledWith(5);
  });
});
