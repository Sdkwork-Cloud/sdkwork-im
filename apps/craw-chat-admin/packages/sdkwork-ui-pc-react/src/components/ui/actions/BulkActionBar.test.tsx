import { fireEvent, render, screen } from '@testing-library/react';
import { describe, expect, it, vi } from 'vitest';
import { Button } from '../button';
import { BulkActionBar } from './index';

describe('BulkActionBar', () => {
  it('renders selection summary, actions, and clear affordance for batch operations', () => {
    const onClear = vi.fn();

    render(
      <BulkActionBar
        actions={<Button size="sm">Export</Button>}
        count={3}
        description="Apply shared operations to the current selection."
        meta="24 total results"
        onClear={onClear}
        title="Rows selected"
        tone="info"
      />,
    );

    expect(screen.getByText('3')).toBeInTheDocument();
    expect(screen.getByText('Rows selected')).toBeInTheDocument();
    expect(screen.getByText('Apply shared operations to the current selection.')).toBeInTheDocument();
    expect(screen.getByText('24 total results')).toBeInTheDocument();
    expect(screen.getByRole('button', { name: 'Export' })).toBeInTheDocument();

    fireEvent.click(screen.getByRole('button', { name: 'Clear' }));
    expect(onClear).toHaveBeenCalledTimes(1);
  });
});
