import { fireEvent, render, screen } from '@testing-library/react';
import { describe, expect, it, vi } from 'vitest';
import { SegmentedControl } from './index';

describe('SegmentedControl', () => {
  it('renders options and triggers value changes for desktop segmented selection', () => {
    const onValueChange = vi.fn();

    render(
      <SegmentedControl
        onValueChange={onValueChange}
        options={[
          { label: 'Design', value: 'design' },
          { label: 'Code', value: 'code' },
          { label: 'Preview', value: 'preview' },
        ]}
        value="design"
      />,
    );

    expect(screen.getByRole('button', { name: 'Design' })).toHaveAttribute('aria-pressed', 'true');
    fireEvent.click(screen.getByRole('button', { name: 'Code' }));
    expect(onValueChange).toHaveBeenCalledWith('code');
  });
});
