import { render, screen } from '@testing-library/react';
import { describe, expect, it } from 'vitest';
import { Button } from '../button';
import { InlineAlert } from './index';

describe('InlineAlert', () => {
  it('renders tone-aware inline feedback for desktop forms and panels', () => {
    render(
      <InlineAlert description="Capacity is almost full." title="Storage warning" tone="warning" />,
    );

    const alert = screen.getByRole('alert');

    expect(alert).toHaveTextContent('Storage warning');
    expect(alert).toHaveTextContent('Capacity is almost full.');
    expect(alert).toHaveClass('border-[color-mix(in_srgb,var(--sdk-color-state-warning)_32%,transparent)]');
  });

  it('supports contextual actions without coupling to page layout', () => {
    render(
      <InlineAlert
        actions={<Button size="sm">Resolve</Button>}
        description="Connection lost."
        tone="danger"
      />,
    );

    expect(screen.getByRole('button', { name: 'Resolve' })).toBeInTheDocument();
    expect(screen.getAllByRole('alert').at(-1)).toHaveTextContent('Connection lost.');
  });
});
