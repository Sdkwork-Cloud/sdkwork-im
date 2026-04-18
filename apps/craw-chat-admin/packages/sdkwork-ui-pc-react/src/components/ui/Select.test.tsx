import { fireEvent, render, screen } from '@testing-library/react';
import { describe, expect, it } from 'vitest';
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from './index';

describe('Select', () => {
  it('renders standardized slot markers for trigger and item surfaces', () => {
    render(
      <Select defaultValue="active">
        <SelectTrigger aria-label="Status">
          <SelectValue />
        </SelectTrigger>
        <SelectContent>
          <SelectItem value="active">Active</SelectItem>
          <SelectItem value="archived">Archived</SelectItem>
        </SelectContent>
      </Select>,
    );

    const trigger = screen.getByRole('combobox', { name: 'Status' });
    expect(trigger).toHaveAttribute('data-slot', 'select-trigger');

    fireEvent.click(trigger);

    expect(screen.getByText('Archived').closest('[data-slot="select-item"]')).toBeInTheDocument();
  });
});
