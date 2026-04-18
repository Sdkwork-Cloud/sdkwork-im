import { fireEvent, render, screen } from '@testing-library/react';
import { describe, expect, it, vi } from 'vitest';
import { WorkspaceTabs } from './index';

describe('WorkspaceTabs', () => {
  it('renders closable workspace tabs and separates tab activation from close actions', () => {
    const onCloseTab = vi.fn();
    const onValueChange = vi.fn();

    render(
      <WorkspaceTabs
        items={[
          { id: 'notes', label: 'Notes.md', modified: true },
          { id: 'memory', label: 'Memory.md' },
        ]}
        onCloseTab={onCloseTab}
        onValueChange={onValueChange}
        value="notes"
      />,
    );

    expect(screen.getByRole('tab', { name: 'Notes.md' })).toHaveAttribute('aria-selected', 'true');

    fireEvent.click(screen.getByRole('tab', { name: 'Memory.md' }));
    expect(onValueChange).toHaveBeenCalledWith('memory');

    fireEvent.click(screen.getByRole('button', { name: 'Close Memory.md' }));
    expect(onCloseTab).toHaveBeenCalledWith('memory');
    expect(onValueChange).toHaveBeenCalledTimes(1);
  });
});
