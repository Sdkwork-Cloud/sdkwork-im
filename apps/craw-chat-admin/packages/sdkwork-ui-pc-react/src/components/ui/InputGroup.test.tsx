import { render, screen } from '@testing-library/react';
import { describe, expect, it } from 'vitest';
import { Input, InputGroup, InputGroupAddon } from './index';

describe('InputGroup', () => {
  it('renders a standardized input-group anatomy around shared field controls', () => {
    const { container } = render(
      <InputGroup>
        <InputGroupAddon>@</InputGroupAddon>
        <Input aria-label="Workspace handle" placeholder="sdkwork" />
      </InputGroup>,
    );

    expect(container.querySelector('[data-sdk-ui="input-group"]')).toHaveAttribute('data-slot', 'input-group');
    expect(screen.getByText('@').closest('[data-slot="input-group-addon"]')).toBeInTheDocument();
    expect(screen.getByRole('textbox', { name: 'Workspace handle' })).toHaveAttribute('data-slot', 'input');
  });
});
