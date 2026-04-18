import { fireEvent, render, screen } from '@testing-library/react';
import { describe, expect, it, vi } from 'vitest';
import { PickerDialog } from './PickerDialog';

describe('PickerDialog', () => {
  it('renders a reusable picker shell with filters, sidebar, footer, and result summary', () => {
    render(
      <PickerDialog
        actions={<button type="button">Refresh</button>}
        description="Use the shared picker shell for selecting assets, prompts, or workspace entities."
        filters={<button type="button">Images</button>}
        footer={<button type="button">Confirm selection</button>}
        open
        resultsSummary="24 results"
        sidebar={<div>Collections</div>}
        title="Choose asset"
      >
        <div>Asset grid</div>
      </PickerDialog>,
    );

    const dialog = screen.getByRole('dialog');

    expect(dialog).toHaveClass('w-[min(94vw,72rem)]');
    expect(screen.getByText('Choose asset')).toBeInTheDocument();
    expect(
      screen.getByText('Use the shared picker shell for selecting assets, prompts, or workspace entities.'),
    ).toBeInTheDocument();
    expect(screen.getByRole('button', { name: 'Refresh' })).toBeInTheDocument();
    expect(screen.getByRole('button', { name: 'Images' })).toBeInTheDocument();
    expect(screen.getByText('24 results')).toBeInTheDocument();
    expect(screen.getByText('Collections')).toBeInTheDocument();
    expect(screen.getByText('Asset grid')).toBeInTheDocument();
    expect(screen.getByRole('button', { name: 'Confirm selection' })).toBeInTheDocument();
  });

  it('renders loading, error, and empty states through the shared feedback layer', () => {
    const { rerender } = render(
      <PickerDialog
        loading
        loadingLabel="Loading assets..."
        open
        title="Choose asset"
      />,
    );

    expect(screen.getByText('Loading assets...')).toBeInTheDocument();

    rerender(
      <PickerDialog
        error="The asset library could not be loaded."
        errorTitle="Load failed"
        open
        title="Choose asset"
      />,
    );

    expect(screen.getByText('Load failed')).toBeInTheDocument();
    expect(screen.getByText('The asset library could not be loaded.')).toBeInTheDocument();

    rerender(
      <PickerDialog
        empty
        emptyDescription="Try a different search or create a new asset."
        emptyTitle="No assets available"
        open
        title="Choose asset"
      />,
    );

    expect(screen.getByText('No assets available')).toBeInTheDocument();
    expect(screen.getByText('Try a different search or create a new asset.')).toBeInTheDocument();
  });

  it('routes the shared close affordance through the Radix open-state callback', () => {
    const handleOpenChange = vi.fn();

    render(
      <PickerDialog
        onOpenChange={handleOpenChange}
        open
        title="Choose asset"
      >
        <div>Asset grid</div>
      </PickerDialog>,
    );

    fireEvent.click(screen.getByRole('button', { name: 'Close' }));

    expect(handleOpenChange).toHaveBeenCalledWith(false);
  });
});
