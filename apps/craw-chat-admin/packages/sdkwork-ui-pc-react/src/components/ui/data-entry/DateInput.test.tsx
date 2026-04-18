import { fireEvent, render, screen } from '@testing-library/react';
import { describe, expect, it, vi } from 'vitest';
import { DateInput, DateTimeInput } from './index';

describe('DateInput', () => {
  it('renders a native date input and opens the picker from the calendar trigger when supported', () => {
    render(<DateInput aria-label="Due date" />);

    const input = screen.getByLabelText('Due date') as HTMLInputElement & {
      showPicker?: () => void;
    };
    const showPicker = vi.fn();

    Object.defineProperty(input, 'showPicker', {
      configurable: true,
      value: showPicker,
    });

    fireEvent.click(screen.getByRole('button', { name: 'Open calendar' }));

    expect(input).toHaveAttribute('type', 'date');
    expect(showPicker).toHaveBeenCalledTimes(1);
  });

  it('renders a native datetime-local input and opens the picker from the calendar trigger when supported', () => {
    render(<DateTimeInput aria-label="Schedule time" />);

    const input = screen.getByLabelText('Schedule time') as HTMLInputElement & {
      showPicker?: () => void;
    };
    const showPicker = vi.fn();

    Object.defineProperty(input, 'showPicker', {
      configurable: true,
      value: showPicker,
    });

    fireEvent.click(screen.getByRole('button', { name: 'Open date and time picker' }));

    expect(input).toHaveAttribute('type', 'datetime-local');
    expect(showPicker).toHaveBeenCalledTimes(1);
  });
});
