import { render, screen } from '@testing-library/react';
import { describe, expect, it } from 'vitest';
import { StatusBadge } from './status-badge';

describe('StatusBadge', () => {
  it('maps shared business status text into semantic SDKWORK tones', () => {
    render(<StatusBadge status="active" />);

    const badge = screen.getByText('active').closest('div');

    expect(badge).toBeInTheDocument();
    expect(badge).toHaveAttribute('data-variant', 'success');
  });
});
