import { fireEvent, render, screen, within } from '@testing-library/react';
import { describe, expect, it, vi } from 'vitest';
import { NumberInput } from './index';

describe('NumberInput', () => {
  it('renders a desktop number field with increment and decrement controls', () => {
    const handleValueChange = vi.fn();

    const { container } = render(
      <NumberInput
        aria-label="Temperature"
        max={2}
        min={0}
        onValueChange={handleValueChange}
        step={0.5}
        value={1}
      />,
    );

    expect(container.querySelector('[data-sdk-ui="number-input"]')).toHaveAttribute('data-slot', 'number-input');
    expect(screen.getByLabelText('Temperature')).toHaveAttribute('data-slot', 'number-input-input');
    fireEvent.click(screen.getByRole('button', { name: 'Increase value' }));
    expect(handleValueChange).toHaveBeenCalledWith(1.5);

    fireEvent.click(screen.getByRole('button', { name: 'Decrease value' }));
    expect(handleValueChange).toHaveBeenCalledWith(0.5);
  });

  it('supports direct numeric editing and reports nullable values', () => {
    const handleValueChange = vi.fn();

    render(<NumberInput aria-label="Retries" onValueChange={handleValueChange} value={3} />);

    fireEvent.change(screen.getByLabelText('Retries'), { target: { value: '12.5' } });
    expect(handleValueChange).toHaveBeenCalledWith(12.5);

    fireEvent.change(screen.getByLabelText('Retries'), { target: { value: '' } });
    expect(handleValueChange).toHaveBeenCalledWith(null);
  });

  it('updates uncontrolled values when the step controls are used', () => {
    render(<NumberInput aria-label="Workers" defaultValue={2} max={6} min={0} step={2} />);

    const input = screen.getByLabelText('Workers');
    const container = input.parentElement;

    expect(input).toHaveValue(2);

    if (!container) {
      throw new Error('Expected number input wrapper to exist.');
    }

    fireEvent.click(within(container).getByRole('button', { name: 'Increase value' }));
    expect(input).toHaveValue(4);

    fireEvent.click(within(container).getByRole('button', { name: 'Decrease value' }));
    expect(input).toHaveValue(2);
  });

  it('normalizes string bounds and steps for desktop stepper controls', () => {
    const handleValueChange = vi.fn();

    render(
      <NumberInput
        aria-label="Budget"
        max="4"
        min="0"
        onValueChange={handleValueChange}
        step="2"
        value={4}
      />,
    );

    fireEvent.click(screen.getAllByRole('button', { name: 'Increase value' }).at(-1)!);
    expect(handleValueChange).not.toHaveBeenCalled();

    fireEvent.click(screen.getAllByRole('button', { name: 'Decrease value' }).at(-1)!);
    expect(handleValueChange).toHaveBeenCalledWith(2);
  });
});
