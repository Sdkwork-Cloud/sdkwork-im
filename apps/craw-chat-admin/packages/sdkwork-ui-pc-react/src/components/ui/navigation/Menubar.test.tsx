import { cleanup, fireEvent, render, screen } from '@testing-library/react';
import { afterEach, describe, expect, it } from 'vitest';
import {
  Menubar,
  MenubarCheckboxItem,
  MenubarContent,
  MenubarItem,
  MenubarMenu,
  MenubarShortcut,
  MenubarSub,
  MenubarSubContent,
  MenubarSubTrigger,
  MenubarTrigger,
} from './index';

function queryUi(name: string) {
  return document.body.querySelector(`[data-sdk-ui="${name}"]`);
}

afterEach(() => {
  cleanup();
});

describe('Menubar', () => {
  it('renders desktop menu triggers and opens the selected menu content', () => {
    render(
      <Menubar value="file">
        <MenubarMenu value="file">
          <MenubarTrigger>File</MenubarTrigger>
          <MenubarContent>
            <MenubarItem>
              New workspace
              <MenubarShortcut>Ctrl+N</MenubarShortcut>
            </MenubarItem>
          </MenubarContent>
        </MenubarMenu>
      </Menubar>,
    );

    fireEvent.focus(screen.getByRole('menuitem', { name: 'File' }));

    expect(screen.getByRole('menubar')).toBeInTheDocument();
    expect(screen.getByText('New workspace')).toBeInTheDocument();
    expect(screen.getByText('Ctrl+N')).toBeInTheDocument();
  });

  it('publishes stable slot markers for menubar anatomy surfaces', () => {
    render(
      <Menubar value="file">
        <MenubarMenu value="file">
          <MenubarTrigger>File</MenubarTrigger>
          <MenubarContent forceMount>
            <MenubarItem>
              New workspace
              <MenubarShortcut>Ctrl+N</MenubarShortcut>
            </MenubarItem>
            <MenubarCheckboxItem checked>Auto save</MenubarCheckboxItem>
            <MenubarSub open>
              <MenubarSubTrigger>Share</MenubarSubTrigger>
              <MenubarSubContent forceMount>
                <MenubarItem>Invite team</MenubarItem>
              </MenubarSubContent>
            </MenubarSub>
          </MenubarContent>
        </MenubarMenu>
      </Menubar>,
    );

    expect(queryUi('menubar')).toHaveAttribute('data-slot', 'menubar');
    expect(queryUi('menubar-trigger')).toHaveAttribute('data-slot', 'menubar-trigger');
    expect(queryUi('menubar-content')).toHaveAttribute('data-slot', 'menubar-content');
    expect(screen.getByText('New workspace').closest('[data-sdk-ui="menubar-item"]')).toHaveAttribute(
      'data-slot',
      'menubar-item',
    );
    expect(screen.getByText('Ctrl+N').closest('[data-sdk-ui="menubar-shortcut"]')).toHaveAttribute(
      'data-slot',
      'menubar-shortcut',
    );
    expect(screen.getByText('Auto save').closest('[data-sdk-ui="menubar-checkbox-item"]')).toHaveAttribute(
      'data-slot',
      'menubar-checkbox-item',
    );
    expect(screen.getByText('Share').closest('[data-sdk-ui="menubar-sub-trigger"]')).toHaveAttribute(
      'data-slot',
      'menubar-sub-trigger',
    );
    expect(queryUi('menubar-sub-content')).toHaveAttribute('data-slot', 'menubar-sub-content');
  });
});
