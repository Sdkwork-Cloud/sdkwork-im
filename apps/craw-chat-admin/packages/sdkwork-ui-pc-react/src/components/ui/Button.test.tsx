import { render, screen } from '@testing-library/react';
import { describe, expect, it } from 'vitest';
import { Button } from './button';

describe('Button', () => {
  it('renders SDKWORK button variants with semantic defaults', () => {
    render(<Button variant="primary">Launch</Button>);

    const button = screen.getByRole('button', { name: 'Launch' });
    expect(button).toHaveClass('sdk-btn');
    expect(button).toHaveClass('sdk-btn--primary');
  });

  it('supports loading state without losing disabled safety', () => {
    render(<Button loading>Loading</Button>);

    const button = screen.getByRole('button', { name: /Loading/i });
    expect(button).toBeDisabled();
    expect(button).toHaveAttribute('data-loading', 'true');
  });
});
