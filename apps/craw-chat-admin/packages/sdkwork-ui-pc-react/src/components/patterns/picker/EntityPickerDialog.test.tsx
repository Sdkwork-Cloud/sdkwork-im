import * as React from 'react';
import { cleanup, fireEvent, render, screen } from '@testing-library/react';
import { afterEach, describe, expect, it, vi } from 'vitest';
import { EntityPickerDialog } from './EntityPickerDialog';

interface PickerItem {
  id: string;
  name: string;
}

const sampleItems: PickerItem[] = [
  { id: 'asset-a', name: 'Asset A' },
  { id: 'asset-b', name: 'Asset B' },
  { id: 'asset-c', name: 'Asset C' },
];

afterEach(() => {
  cleanup();
});

describe('EntityPickerDialog', () => {
  it('manages multi-selection with shared footer summary, clear action, and confirm action', () => {
    const handleConfirm = vi.fn();

    render(
      <EntityPickerDialog
        items={sampleItems}
        onConfirm={handleConfirm}
        open
        selectionMode="multiple"
        title="Choose assets"
        renderItem={({ item, selected, toggleSelection }) => (
          <button
            aria-pressed={selected}
            onClick={toggleSelection}
            type="button"
          >
            {item.name}
          </button>
        )}
      />,
    );

    expect(
      document.querySelector('[data-sdk-pattern="picker-selection-footer"]'),
    ).not.toBeNull();
    expect(screen.getByRole('button', { name: 'Confirm selection' })).toBeDisabled();
    expect(screen.getByText('No items selected')).toBeInTheDocument();

    fireEvent.click(screen.getByRole('button', { name: 'Asset A' }));
    fireEvent.click(screen.getByRole('button', { name: 'Asset B' }));

    expect(screen.getByText('2 items selected')).toBeInTheDocument();
    expect(screen.getByRole('button', { name: 'Clear selection' })).toBeInTheDocument();

    const confirmButton = screen.getByRole('button', { name: 'Confirm selection (2)' });
    expect(confirmButton).toBeEnabled();

    fireEvent.click(confirmButton);

    expect(handleConfirm).toHaveBeenCalledWith(
      [sampleItems[0], sampleItems[1]],
      ['asset-a', 'asset-b'],
    );
  });

  it('supports controlled single selection and uses item labels in the shared footer summary', () => {
    const handleConfirm = vi.fn();
    const handleOpenChange = vi.fn();

    function ControlledExample() {
      const [selectedIds, setSelectedIds] = React.useState<React.Key[]>([sampleItems[1].id]);

      return (
        <EntityPickerDialog
          getItemId={(item) => item.id}
          getItemLabel={(item) => item.name}
          items={sampleItems}
          onConfirm={handleConfirm}
          onOpenChange={handleOpenChange}
          onSelectedIdsChange={setSelectedIds}
          open
          selectedIds={selectedIds}
          selectionMode="single"
          title="Choose workspace"
          renderItem={({ item, selected, toggleSelection }) => (
            <button
              aria-pressed={selected}
              onClick={toggleSelection}
              type="button"
            >
              {item.name}
            </button>
          )}
        />
      );
    }

    render(<ControlledExample />);

    expect(screen.getByRole('button', { name: 'Asset B' })).toHaveAttribute('aria-pressed', 'true');
    expect(screen.getByText('Selected: Asset B')).toBeInTheDocument();

    fireEvent.click(screen.getByRole('button', { name: 'Asset C' }));

    expect(screen.getByRole('button', { name: 'Asset C' })).toHaveAttribute('aria-pressed', 'true');
    expect(screen.getByText('Selected: Asset C')).toBeInTheDocument();

    fireEvent.click(screen.getByRole('button', { name: 'Confirm selection' }));

    expect(handleConfirm).toHaveBeenCalledWith([sampleItems[2]], ['asset-c']);

    fireEvent.click(screen.getByRole('button', { name: 'Cancel' }));

    expect(handleOpenChange).toHaveBeenCalledWith(false);
  });

  it('renders list layout and pagination content for paged picker flows', () => {
    render(
      <EntityPickerDialog
        items={sampleItems}
        layout="list"
        open
        pagination={<div>Page 1 of 3</div>}
        title="Choose prompts"
        renderItem={({ item, selected, toggleSelection }) => (
          <button
            aria-pressed={selected}
            onClick={toggleSelection}
            type="button"
          >
            {item.name}
          </button>
        )}
      />,
    );

    expect(screen.getByText('Page 1 of 3')).toBeInTheDocument();
    expect(screen.getByRole('button', { name: 'Asset A' })).toBeInTheDocument();
    expect(screen.getByRole('button', { name: 'Asset B' })).toBeInTheDocument();
    expect(screen.getByRole('button', { name: 'Asset C' })).toBeInTheDocument();
  });
});
