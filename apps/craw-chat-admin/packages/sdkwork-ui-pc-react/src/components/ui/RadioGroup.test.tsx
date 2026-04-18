import { render, screen } from '@testing-library/react';
import { describe, expect, it } from 'vitest';
import { RadioGroup, RadioGroupItem } from './radio-group';

describe('RadioGroup', () => {
  it('renders a shared radio group with the provided default selection', () => {
    render(
      <RadioGroup defaultValue="compact">
        <RadioGroupItem value="comfortable" aria-label="Comfortable" />
        <RadioGroupItem value="compact" aria-label="Compact" />
      </RadioGroup>,
    );

    const radios = screen.getAllByRole('radio');

    expect(radios).toHaveLength(2);
    expect(screen.getByRole('radio', { name: 'Compact' })).toHaveAttribute('data-state', 'checked');
  });
});
