import { cleanup, fireEvent, render, screen } from '@testing-library/react';
import { afterEach, describe, expect, it, vi } from 'vitest';
import {
  ContextMenu,
  ContextMenuCheckboxItem,
  ContextMenuContent,
  ContextMenuItem,
  ContextMenuShortcut,
  ContextMenuSub,
  ContextMenuSubContent,
  ContextMenuSubTrigger,
  ContextMenuTrigger,
} from './index';

function queryUi(name: string) {
  return document.body.querySelector(`[data-sdk-ui="${name}"]`);
}

afterEach(() => {
  cleanup();
});

describe('ContextMenu', () => {
  it('renders desktop context actions and invokes selection handlers', () => {
    const handleSelect = vi.fn();

    render(
      <ContextMenu>
        <ContextMenuTrigger>Workspace Tree</ContextMenuTrigger>
        <ContextMenuContent>
          <ContextMenuItem onSelect={handleSelect}>Rename</ContextMenuItem>
        </ContextMenuContent>
      </ContextMenu>,
    );

    fireEvent.contextMenu(screen.getByText('Workspace Tree'));
    expect(screen.getByRole('menuitem', { name: 'Rename' })).toBeInTheDocument();
    fireEvent.click(screen.getByRole('menuitem', { name: 'Rename' }));

    expect(handleSelect).toHaveBeenCalledTimes(1);
  });

  it('publishes stable slot markers for context menu anatomy surfaces', () => {
    render(
      <ContextMenu>
        <ContextMenuTrigger>Workspace Tree</ContextMenuTrigger>
        <ContextMenuContent forceMount>
          <ContextMenuItem>
            Rename
            <ContextMenuShortcut>F2</ContextMenuShortcut>
          </ContextMenuItem>
          <ContextMenuCheckboxItem checked>Compact density</ContextMenuCheckboxItem>
          <ContextMenuSub open>
            <ContextMenuSubTrigger>More actions</ContextMenuSubTrigger>
            <ContextMenuSubContent forceMount>
              <ContextMenuItem>Archive</ContextMenuItem>
            </ContextMenuSubContent>
          </ContextMenuSub>
        </ContextMenuContent>
      </ContextMenu>,
    );

    fireEvent.contextMenu(screen.getByText('Workspace Tree'));

    expect(queryUi('context-menu-trigger')).toHaveAttribute('data-slot', 'context-menu-trigger');
    expect(queryUi('context-menu-content')).toHaveAttribute('data-slot', 'context-menu-content');
    expect(screen.getByText('Rename').closest('[data-sdk-ui="context-menu-item"]')).toHaveAttribute(
      'data-slot',
      'context-menu-item',
    );
    expect(screen.getByText('F2').closest('[data-sdk-ui="context-menu-shortcut"]')).toHaveAttribute(
      'data-slot',
      'context-menu-shortcut',
    );
    expect(screen.getByText('Compact density').closest('[data-sdk-ui="context-menu-checkbox-item"]')).toHaveAttribute(
      'data-slot',
      'context-menu-checkbox-item',
    );
    expect(screen.getByText('More actions').closest('[data-sdk-ui="context-menu-sub-trigger"]')).toHaveAttribute(
      'data-slot',
      'context-menu-sub-trigger',
    );
    expect(queryUi('context-menu-sub-content')).toHaveAttribute('data-slot', 'context-menu-sub-content');
  });
});
