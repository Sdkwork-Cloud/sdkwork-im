import { fireEvent, render, screen } from '@testing-library/react';
import { describe, expect, it, vi } from 'vitest';
import { ActionMenuButton } from './index';

describe('ActionMenuButton', () => {
  it('renders menu entries and triggers item handlers for desktop action menus', () => {
    const onArchive = vi.fn();

    render(
      <ActionMenuButton
        menuOpen
        menuLabel="More row actions"
        items={[
          { key: 'label', label: 'More Actions', type: 'label' },
          { key: 'separator', type: 'separator' },
          {
            key: 'archive',
            label: 'Archive item',
            onSelect: onArchive,
            shortcut: 'Ctrl+Alt+A',
          },
          {
            key: 'delete',
            label: 'Delete item',
            tone: 'danger',
          },
        ]}
        variant="secondary"
      >
        Actions
      </ActionMenuButton>,
    );

    expect(screen.getByRole('button', { name: 'Actions' })).toBeInTheDocument();
    expect(screen.getByText('More Actions')).toBeInTheDocument();
    expect(screen.getByText('Archive item')).toBeInTheDocument();
    expect(screen.getByText('Ctrl+Alt+A')).toBeInTheDocument();

    fireEvent.click(screen.getByText('Archive item'));
    expect(onArchive).toHaveBeenCalledTimes(1);
    expect(screen.getByText('Delete item').closest('[role="menuitem"]')).toHaveAttribute('data-tone', 'danger');
  });
});
