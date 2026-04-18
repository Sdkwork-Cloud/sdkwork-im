import { cleanup, fireEvent, render, screen } from '@testing-library/react';
import { afterEach, describe, expect, it, vi } from 'vitest';
import {
  Command,
  CommandEmpty,
  CommandGroup,
  CommandInput,
  CommandItem,
  CommandList,
  CommandShortcut,
} from './index';

function queryUi(name: string) {
  return document.body.querySelector(`[data-sdk-ui="${name}"]`);
}

afterEach(() => {
  cleanup();
});

describe('Command', () => {
  it('renders grouped desktop actions and invokes item selection handlers', () => {
    const handleSelect = vi.fn();

    render(
      <Command>
        <CommandInput placeholder="Search commands" />
        <CommandList>
          <CommandEmpty>No matching actions.</CommandEmpty>
          <CommandGroup heading="Workspace">
            <CommandItem value="open-settings" onSelect={handleSelect}>
              <span>Open Settings</span>
              <CommandShortcut>Ctrl+,</CommandShortcut>
            </CommandItem>
          </CommandGroup>
        </CommandList>
      </Command>,
    );

    fireEvent.click(screen.getByText('Open Settings'));

    expect(screen.getByPlaceholderText('Search commands')).toBeInTheDocument();
    expect(screen.getByText('Workspace')).toBeInTheDocument();
    expect(screen.getByText('Ctrl+,')).toBeInTheDocument();
    expect(handleSelect).toHaveBeenCalledTimes(1);
  });

  it('publishes stable slot markers for command anatomy surfaces', () => {
    render(
      <Command>
        <CommandInput placeholder="Search commands" />
        <CommandList>
          <CommandGroup heading="Workspace">
            <CommandItem value="open-settings">
              <span>Open Settings</span>
              <CommandShortcut>Ctrl+,</CommandShortcut>
            </CommandItem>
          </CommandGroup>
        </CommandList>
      </Command>,
    );

    expect(queryUi('command')).toHaveAttribute('data-slot', 'command');
    expect(queryUi('command-input-wrapper')).toHaveAttribute('data-slot', 'command-input-wrapper');
    expect(queryUi('command-input')).toHaveAttribute('data-slot', 'command-input');
    expect(queryUi('command-list')).toHaveAttribute('data-slot', 'command-list');
    expect(queryUi('command-group')).toHaveAttribute('data-slot', 'command-group');
    expect(screen.getByText('Open Settings').closest('[data-sdk-ui="command-item"]')).toHaveAttribute(
      'data-slot',
      'command-item',
    );
    expect(screen.getByText('Ctrl+,').closest('[data-sdk-ui="command-shortcut"]')).toHaveAttribute(
      'data-slot',
      'command-shortcut',
    );
  });

  it('publishes a stable slot marker for the empty state surface', () => {
    render(
      <Command>
        <CommandInput placeholder="Search commands" />
        <CommandList>
          <CommandEmpty>No matching actions.</CommandEmpty>
        </CommandList>
      </Command>,
    );

    expect(screen.getByText('No matching actions.').closest('[data-sdk-ui="command-empty"]')).toHaveAttribute(
      'data-slot',
      'command-empty',
    );
  });
});
