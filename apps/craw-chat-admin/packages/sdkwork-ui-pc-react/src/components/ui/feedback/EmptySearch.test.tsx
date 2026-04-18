import { fireEvent, render, screen } from '@testing-library/react';
import { describe, expect, it, vi } from 'vitest';
import { EmptySearch } from './index';

describe('EmptySearch', () => {
  it('renders a search-specific empty state with the current keyword', () => {
    render(<EmptySearch keyword="invoice" />);

    expect(screen.getByText('No results found for "invoice"')).toBeInTheDocument();
    expect(screen.getByText('Try a different keyword or clear the current search.')).toBeInTheDocument();
  });

  it('renders a clear action when the consumer provides a reset handler', () => {
    const handleClear = vi.fn();

    render(<EmptySearch keyword="agent" onClear={handleClear} />);

    fireEvent.click(screen.getByRole('button', { name: 'Clear search' }));
    expect(handleClear).toHaveBeenCalledTimes(1);
  });
});
