import { cleanup, fireEvent, render, screen } from '@testing-library/react';
import { afterEach, describe, expect, it, vi } from 'vitest';
import { PickerSelectionFooter } from './PickerSelectionFooter';

afterEach(() => {
  cleanup();
});

describe('PickerSelectionFooter', () => {
  it('renders shared summary plus clear, cancel, and confirm actions for picker workflows', () => {
    const handleCancel = vi.fn();
    const handleClear = vi.fn();
    const handleConfirm = vi.fn();

    const { container } = render(
      <PickerSelectionFooter
        onCancel={handleCancel}
        onClear={handleClear}
        onConfirm={handleConfirm}
        showClear
        summary="2 items selected"
      />,
    );

    expect(
      container.querySelector('[data-sdk-pattern="picker-selection-footer"]'),
    ).not.toBeNull();
    expect(screen.getByText('2 items selected')).toBeInTheDocument();
    expect(screen.getByRole('button', { name: 'Clear selection' })).toBeInTheDocument();
    expect(screen.getByRole('button', { name: 'Cancel' })).toBeInTheDocument();
    expect(screen.getByRole('button', { name: 'Confirm selection' })).toBeInTheDocument();

    fireEvent.click(screen.getByRole('button', { name: 'Clear selection' }));
    fireEvent.click(screen.getByRole('button', { name: 'Cancel' }));
    fireEvent.click(screen.getByRole('button', { name: 'Confirm selection' }));

    expect(handleClear).toHaveBeenCalledTimes(1);
    expect(handleCancel).toHaveBeenCalledTimes(1);
    expect(handleConfirm).toHaveBeenCalledTimes(1);
  });

  it('supports confirm disable state and custom action labels without forcing the clear action', () => {
    render(
      <PickerSelectionFooter
        cancelLabel="Close"
        confirmDisabled
        confirmLabel="Apply selection"
        summary="No items selected"
      />,
    );

    expect(screen.queryByRole('button', { name: 'Clear selection' })).not.toBeInTheDocument();
    expect(screen.getByRole('button', { name: 'Close' })).toBeInTheDocument();
    expect(screen.getByRole('button', { name: 'Apply selection' })).toBeDisabled();
  });
});
