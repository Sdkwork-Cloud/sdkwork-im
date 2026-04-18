import { fireEvent, render, screen } from '@testing-library/react';
import { describe, expect, it, vi } from 'vitest';
import { SplitButton } from './index';

describe('SplitButton', () => {
  it('runs the primary action and exposes secondary actions through the menu trigger', () => {
    const onPrimary = vi.fn();
    const onSecondary = vi.fn();

    render(
      <SplitButton
        menuOpen
        items={[
          {
            key: 'duplicate',
            label: 'Duplicate item',
            onSelect: onSecondary,
            shortcut: 'Ctrl+D',
          },
        ]}
        menuLabel="More publish actions"
        onClick={onPrimary}
      >
        Publish
      </SplitButton>,
    );

    fireEvent.click(screen.getByRole('button', { name: 'Publish' }));
    expect(onPrimary).toHaveBeenCalledTimes(1);

    expect(screen.getByText('Duplicate item')).toBeInTheDocument();
    expect(screen.getByText('Ctrl+D')).toBeInTheDocument();

    fireEvent.click(screen.getByText('Duplicate item'));
    expect(onSecondary).toHaveBeenCalledTimes(1);
  });
});
