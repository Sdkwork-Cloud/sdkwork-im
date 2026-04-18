import { render, screen } from '@testing-library/react';
import { describe, expect, it } from 'vitest';
import { Input } from './index';

describe('Input', () => {
  it('renders the standardized input marker anatomy', () => {
    render(<Input aria-label="Workspace name" placeholder="SDKWORK Desktop" />);

    expect(screen.getByRole('textbox', { name: 'Workspace name' })).toHaveAttribute('data-slot', 'input');
  });
});
