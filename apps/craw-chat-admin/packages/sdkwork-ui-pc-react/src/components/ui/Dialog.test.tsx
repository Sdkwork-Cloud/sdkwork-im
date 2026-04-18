import { cleanup, fireEvent, render, screen } from '@testing-library/react';
import { afterEach, describe, expect, it, vi } from 'vitest';
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from './dialog';

function queryUi(name: string) {
  return document.body.querySelector(`[data-sdk-ui="${name}"]`);
}

afterEach(() => {
  cleanup();
});

describe('Dialog', () => {
  it('renders the shared dialog shell and closes through the shared close surface', () => {
    const handleOpenChange = vi.fn();

    render(
      <Dialog open onOpenChange={handleOpenChange}>
        <DialogTrigger>Open dialog</DialogTrigger>
        <DialogContent>
          <DialogHeader>
            <DialogTitle>Workspace Template</DialogTitle>
            <DialogDescription>Configure the reusable dialog shell for desktop workflows.</DialogDescription>
          </DialogHeader>
          <DialogFooter>
            <button type="button">Save</button>
          </DialogFooter>
        </DialogContent>
      </Dialog>,
    );

    expect(screen.getByRole('dialog')).toBeInTheDocument();
    fireEvent.click(screen.getByRole('button', { name: 'Close' }));
    expect(handleOpenChange).toHaveBeenCalledWith(false);
  });

  it('publishes stable slot markers for dialog anatomy surfaces', () => {
    render(
      <Dialog defaultOpen>
        <DialogTrigger>Open dialog</DialogTrigger>
        <DialogContent>
          <DialogHeader>
            <DialogTitle>Workspace Template</DialogTitle>
            <DialogDescription>Configure the reusable dialog shell for desktop workflows.</DialogDescription>
          </DialogHeader>
          <DialogFooter>
            <button type="button">Save</button>
          </DialogFooter>
        </DialogContent>
      </Dialog>,
    );

    expect(queryUi('dialog-trigger')).toHaveAttribute('data-slot', 'dialog-trigger');
    expect(queryUi('dialog-overlay')).toHaveAttribute('data-slot', 'dialog-overlay');
    expect(queryUi('dialog-content')).toHaveAttribute('data-slot', 'dialog-content');
    expect(queryUi('dialog-header')).toHaveAttribute('data-slot', 'dialog-header');
    expect(queryUi('dialog-footer')).toHaveAttribute('data-slot', 'dialog-footer');
    expect(screen.getByText('Workspace Template').closest('[data-sdk-ui="dialog-title"]')).toHaveAttribute(
      'data-slot',
      'dialog-title',
    );
    expect(
      screen
        .getByText('Configure the reusable dialog shell for desktop workflows.')
        .closest('[data-sdk-ui="dialog-description"]'),
    ).toHaveAttribute('data-slot', 'dialog-description');
    expect(screen.getByRole('button', { name: 'Close' }).closest('[data-sdk-ui="dialog-close"]')).toHaveAttribute(
      'data-slot',
      'dialog-close',
    );
  });
});
