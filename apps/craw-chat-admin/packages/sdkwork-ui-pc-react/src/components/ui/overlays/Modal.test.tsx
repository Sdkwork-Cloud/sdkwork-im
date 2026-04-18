import { cleanup, fireEvent, render, screen } from '@testing-library/react';
import { afterEach, describe, expect, it, vi } from 'vitest';
import { Button } from '../button';
import {
  ConfirmDialog,
  Modal,
  ModalBody,
  ModalContent,
  ModalDescription,
  ModalFooter,
  ModalHeader,
  ModalTitle,
} from './index';

function queryUi(name: string) {
  return document.body.querySelector(`[data-sdk-ui="${name}"]`);
}

afterEach(() => {
  cleanup();
});

describe('Modal', () => {
  it('renders a desktop modal shell with structured header body footer and sizing controls', () => {
    const handleOpenChange = vi.fn();

    render(
      <Modal open onOpenChange={handleOpenChange}>
        <ModalContent align="top" size="lg">
          <ModalHeader>
            <ModalTitle>Workspace Template</ModalTitle>
            <ModalDescription>Configure the reusable modal shell for desktop workflows.</ModalDescription>
          </ModalHeader>
          <ModalBody>Modal body content</ModalBody>
          <ModalFooter>
            <Button variant="secondary">Cancel</Button>
            <Button>Save</Button>
          </ModalFooter>
        </ModalContent>
      </Modal>,
    );

    const dialog = screen.getByRole('dialog');

    expect(dialog).toHaveClass('w-[min(92vw,56rem)]');
    expect(dialog).toHaveClass('top-[max(2rem,8vh)]');
    expect(screen.getByText('Modal body content')).toBeInTheDocument();

    fireEvent.click(screen.getByRole('button', { name: 'Close' }));
    expect(handleOpenChange).toHaveBeenCalledWith(false);
  });

  it('publishes stable slot markers for modal anatomy surfaces', () => {
    render(
      <Modal open>
        <ModalContent align="top" size="lg">
          <ModalHeader>
            <ModalTitle>Workspace Template</ModalTitle>
            <ModalDescription>Configure the reusable modal shell for desktop workflows.</ModalDescription>
          </ModalHeader>
          <ModalBody>Modal body content</ModalBody>
          <ModalFooter>
            <Button variant="secondary">Cancel</Button>
            <Button>Save</Button>
          </ModalFooter>
        </ModalContent>
      </Modal>,
    );

    expect(queryUi('modal-overlay')).toHaveAttribute('data-slot', 'modal-overlay');
    expect(queryUi('modal-content')).toHaveAttribute('data-slot', 'modal-content');
    expect(queryUi('modal-header')).toHaveAttribute('data-slot', 'modal-header');
    expect(queryUi('modal-body')).toHaveAttribute('data-slot', 'modal-body');
    expect(queryUi('modal-footer')).toHaveAttribute('data-slot', 'modal-footer');
    expect(screen.getByText('Workspace Template').closest('[data-sdk-ui="modal-title"]')).toHaveAttribute(
      'data-slot',
      'modal-title',
    );
    expect(
      screen
        .getByText('Configure the reusable modal shell for desktop workflows.')
        .closest('[data-sdk-ui="modal-description"]'),
    ).toHaveAttribute('data-slot', 'modal-description');
    expect(screen.getByRole('button', { name: 'Close' }).closest('[data-sdk-ui="modal-close"]')).toHaveAttribute(
      'data-slot',
      'modal-close',
    );
  });

  it('provides a confirm dialog abstraction for high-frequency destructive confirmations', () => {
    const handleOpenChange = vi.fn();
    const handleConfirm = vi.fn();

    render(
      <ConfirmDialog
        confirmLabel="Delete workspace"
        description="This action cannot be undone."
        onConfirm={handleConfirm}
        onOpenChange={handleOpenChange}
        open
        title="Delete workspace"
        tone="danger"
      />,
    );

    expect(screen.getByRole('dialog')).toBeInTheDocument();
    expect(screen.getByText('This action cannot be undone.')).toBeInTheDocument();

    const confirmButton = screen.getByRole('button', { name: 'Delete workspace' });
    expect(confirmButton).toHaveClass('sdk-btn--danger');

    fireEvent.click(confirmButton);
    expect(handleConfirm).toHaveBeenCalledTimes(1);
    expect(handleOpenChange).toHaveBeenCalledWith(false);
  });
});
