import { render, screen } from '@testing-library/react';
import { describe, expect, it } from 'vitest';
import { Slider } from './slider';

describe('Slider', () => {
  it('renders a shared slider thumb with the provided default value', () => {
    render(<Slider defaultValue={[40]} max={100} step={1} />);

    expect(screen.getByRole('slider')).toHaveAttribute('aria-valuenow', '40');
  });
});
