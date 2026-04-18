import { cleanup, fireEvent, render, screen } from '@testing-library/react';
import { afterEach, describe, expect, it, vi } from 'vitest';
import {
  Drawer,
  DrawerBody,
  DrawerContent,
  DrawerDescription,
  DrawerFooter,
  DrawerHeader,
  DrawerTitle,
} from './index';

function queryUi(name: string) {
  return document.body.querySelector(`[data-sdk-ui="${name}"]`);
}

afterEach(() => {
  cleanup();
});

describe('Drawer', () => {
  it('renders desktop drawer structure and closes through the shared close action', () => {
    const handleOpenChange = vi.fn();

    render(
      <Drawer open onOpenChange={handleOpenChange}>
        <DrawerContent>
          <DrawerHeader>
            <DrawerTitle>Workspace Settings</DrawerTitle>
            <DrawerDescription>Adjust desktop defaults for this workspace.</DrawerDescription>
          </DrawerHeader>
          <DrawerBody>Content Area</DrawerBody>
          <DrawerFooter>
            <button type="button">Save</button>
          </DrawerFooter>
        </DrawerContent>
      </Drawer>,
    );

    expect(screen.getByRole('dialog')).toBeInTheDocument();
    expect(screen.getByText('Content Area')).toBeInTheDocument();
    fireEvent.click(screen.getByRole('button', { name: 'Close' }));
    expect(handleOpenChange).toHaveBeenCalledWith(false);
  });

  it('publishes stable slot markers for drawer anatomy surfaces', () => {
    render(
      <Drawer open onOpenChange={() => undefined}>
        <DrawerContent>
          <DrawerHeader>
            <DrawerTitle>Workspace Settings</DrawerTitle>
            <DrawerDescription>Adjust desktop defaults for this workspace.</DrawerDescription>
          </DrawerHeader>
          <DrawerBody>Content Area</DrawerBody>
          <DrawerFooter>
            <button type="button">Save</button>
          </DrawerFooter>
        </DrawerContent>
      </Drawer>,
    );

    expect(queryUi('drawer-overlay')).toHaveAttribute('data-slot', 'drawer-overlay');
    expect(queryUi('drawer-content')).toHaveAttribute('data-slot', 'drawer-content');
    expect(queryUi('drawer-header')).toHaveAttribute('data-slot', 'drawer-header');
    expect(queryUi('drawer-body')).toHaveAttribute('data-slot', 'drawer-body');
    expect(queryUi('drawer-footer')).toHaveAttribute('data-slot', 'drawer-footer');
    expect(screen.getByText('Workspace Settings').closest('[data-sdk-ui="drawer-title"]')).toHaveAttribute(
      'data-slot',
      'drawer-title',
    );
    expect(screen.getByText('Adjust desktop defaults for this workspace.').closest('[data-sdk-ui="drawer-description"]')).toHaveAttribute(
      'data-slot',
      'drawer-description',
    );
    expect(screen.getByRole('button', { name: 'Close' }).closest('[data-sdk-ui="drawer-close"]')).toHaveAttribute(
      'data-slot',
      'drawer-close',
    );
  });
});
