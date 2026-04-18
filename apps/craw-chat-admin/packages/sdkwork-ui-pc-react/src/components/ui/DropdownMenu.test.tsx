import { cleanup, render, screen } from '@testing-library/react';
import { afterEach, describe, expect, it } from 'vitest';
import {
  DropdownMenu,
  DropdownMenuCheckboxItem,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuShortcut,
  DropdownMenuSub,
  DropdownMenuSubContent,
  DropdownMenuSubTrigger,
  DropdownMenuTrigger,
} from './dropdown-menu';

function queryUi(name: string) {
  return document.body.querySelector(`[data-sdk-ui="${name}"]`);
}

afterEach(() => {
  cleanup();
});

describe('DropdownMenu', () => {
  it('renders shared dropdown content when opened', () => {
    render(
      <DropdownMenu open>
        <DropdownMenuTrigger>Open</DropdownMenuTrigger>
        <DropdownMenuContent>
          <DropdownMenuItem>Workspace settings</DropdownMenuItem>
        </DropdownMenuContent>
      </DropdownMenu>,
    );

    expect(screen.getByText('Workspace settings')).toBeInTheDocument();
  });

  it('publishes stable slot markers for dropdown menu anatomy surfaces', () => {
    render(
      <DropdownMenu open>
        <DropdownMenuTrigger>Open</DropdownMenuTrigger>
        <DropdownMenuContent forceMount>
          <DropdownMenuItem>
            Workspace settings
            <DropdownMenuShortcut>Ctrl+,</DropdownMenuShortcut>
          </DropdownMenuItem>
          <DropdownMenuCheckboxItem checked>Compact density</DropdownMenuCheckboxItem>
          <DropdownMenuSub open>
            <DropdownMenuSubTrigger>More tools</DropdownMenuSubTrigger>
            <DropdownMenuSubContent forceMount>
              <DropdownMenuItem>Archive workspace</DropdownMenuItem>
            </DropdownMenuSubContent>
          </DropdownMenuSub>
        </DropdownMenuContent>
      </DropdownMenu>,
    );

    expect(queryUi('dropdown-menu-trigger')).toHaveAttribute('data-slot', 'dropdown-menu-trigger');
    expect(queryUi('dropdown-menu-content')).toHaveAttribute('data-slot', 'dropdown-menu-content');
    expect(screen.getByText('Workspace settings').closest('[data-sdk-ui="dropdown-menu-item"]')).toHaveAttribute(
      'data-slot',
      'dropdown-menu-item',
    );
    expect(screen.getByText('Ctrl+,').closest('[data-sdk-ui="dropdown-menu-shortcut"]')).toHaveAttribute(
      'data-slot',
      'dropdown-menu-shortcut',
    );
    expect(screen.getByText('Compact density').closest('[data-sdk-ui="dropdown-menu-checkbox-item"]')).toHaveAttribute(
      'data-slot',
      'dropdown-menu-checkbox-item',
    );
    expect(screen.getByText('More tools').closest('[data-sdk-ui="dropdown-menu-sub-trigger"]')).toHaveAttribute(
      'data-slot',
      'dropdown-menu-sub-trigger',
    );
    expect(queryUi('dropdown-menu-sub-content')).toHaveAttribute('data-slot', 'dropdown-menu-sub-content');
  });
});
