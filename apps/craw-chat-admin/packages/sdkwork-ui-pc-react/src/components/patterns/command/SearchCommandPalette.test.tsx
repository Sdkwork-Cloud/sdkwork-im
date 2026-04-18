import { fireEvent, render, screen } from '@testing-library/react';
import { Command } from 'lucide-react';
import { describe, expect, it, vi } from 'vitest';
import { SearchCommandPalette } from './SearchCommandPalette';

describe('SearchCommandPalette', () => {
  it('renders grouped commands and closes after selecting an item', () => {
    const onItemSelect = vi.fn();
    const onOpenChange = vi.fn();

    render(
      <SearchCommandPalette
        groups={[
          {
            heading: 'Workspace',
            items: [
              {
                description: 'Open desktop preferences',
                icon: <Command className="h-4 w-4" />,
                id: 'settings',
                label: 'Open settings',
                shortcut: 'Ctrl+,',
              },
            ],
          },
        ]}
        onItemSelect={onItemSelect}
        onOpenChange={onOpenChange}
        open
      />,
    );

    fireEvent.click(screen.getByText('Open settings'));

    expect(screen.getByText('Workspace')).toBeInTheDocument();
    expect(screen.getByText('Open desktop preferences')).toBeInTheDocument();
    expect(screen.getByText('Ctrl+,')).toBeInTheDocument();
    expect(screen.getByText('Navigate')).toBeInTheDocument();
    expect(onItemSelect).toHaveBeenCalledWith(
      expect.objectContaining({
        id: 'settings',
        label: 'Open settings',
      }),
    );
    expect(onOpenChange).toHaveBeenCalledWith(false);
  });
});
