import { cleanup, fireEvent, render, screen } from '@testing-library/react';
import { afterEach, describe, expect, it, vi } from 'vitest';
import { DateRangeField, DateRangePicker, DateTimeRangeField } from './index';

afterEach(() => {
  cleanup();
});

describe('DateTimeRangeField', () => {
  it('applies desktop range presets and updates both temporal inputs', () => {
    const handleValueChange = vi.fn();
    const handlePresetValueChange = vi.fn();

    render(
      <DateTimeRangeField
        endLabel="End time"
        onPresetValueChange={handlePresetValueChange}
        onValueChange={handleValueChange}
        presets={[
          {
            label: 'Last 24 hours',
            range: {
              end: '2026-04-02T12:00',
              start: '2026-04-01T12:00',
            },
            value: 'last-24-hours',
          },
          {
            label: 'Last 7 days',
            range: {
              end: '2026-04-08T12:00',
              start: '2026-04-01T12:00',
            },
            value: 'last-7-days',
          },
        ]}
        startLabel="Start time"
      />,
    );

    fireEvent.click(screen.getByRole('button', { name: 'Last 24 hours' }));

    expect(screen.getByLabelText('Start time')).toHaveValue('2026-04-01T12:00');
    expect(screen.getByLabelText('End time')).toHaveValue('2026-04-02T12:00');
    expect(handlePresetValueChange).toHaveBeenCalledWith('last-24-hours');
    expect(handleValueChange).toHaveBeenCalledWith({
      end: '2026-04-02T12:00',
      start: '2026-04-01T12:00',
    });
  });
});

describe('DateRangeField', () => {
  it('clears the active preset and surfaces validation feedback for invalid manual ranges', () => {
    const handlePresetValueChange = vi.fn();

    render(
      <DateRangeField
        defaultPresetValue="today"
        defaultValue={{
          end: '2026-04-02',
          start: '2026-04-01',
        }}
        onPresetValueChange={handlePresetValueChange}
        presets={[
          {
            label: 'Today',
            range: {
              end: '2026-04-02',
              start: '2026-04-01',
            },
            value: 'today',
          },
        ]}
        validate={(value) =>
          value.start && value.end && value.start > value.end
            ? 'Start date must be before end date'
            : null
        }
      />,
    );

    const startInput = screen.getByLabelText('Start date');
    const endInput = screen.getByLabelText('End date');

    expect(screen.getByRole('button', { name: 'Today' })).toHaveAttribute('aria-pressed', 'true');

    fireEvent.change(startInput, {
      target: { value: '2026-04-04' },
    });

    expect(screen.getByRole('button', { name: 'Today' })).toHaveAttribute('aria-pressed', 'false');
    expect(handlePresetValueChange).toHaveBeenCalledWith(null);
    expect(endInput).toHaveAttribute('aria-invalid', 'true');
    expect(screen.getByText('Start date must be before end date')).toBeInTheDocument();
  });
});

describe('DateRangePicker', () => {
  it('renders a trigger button instead of inline temporal fields and opens a calendar popover', () => {
    render(<DateRangePicker placeholder="Select range" />);

    const trigger = screen.getByRole('button', { name: 'Select range' });

    expect(trigger).toHaveAttribute('data-slot', 'date-range-picker-trigger');
    expect(screen.queryByLabelText('Start date')).not.toBeInTheDocument();
    expect(screen.queryByLabelText('End date')).not.toBeInTheDocument();

    fireEvent.click(trigger);

    expect(document.body.querySelector('[data-slot="date-range-picker-calendar"]')).toBeInTheDocument();
  });
});
