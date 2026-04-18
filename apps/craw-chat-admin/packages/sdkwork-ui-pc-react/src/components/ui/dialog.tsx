import * as React from 'react';
import * as DialogPrimitive from '@radix-ui/react-dialog';
import { X } from 'lucide-react';
import { cn } from '../../lib/utils';

const Dialog = DialogPrimitive.Root;
const DialogPortal = DialogPrimitive.Portal;

export type DialogProps = React.ComponentPropsWithoutRef<typeof DialogPrimitive.Root>;
export type DialogTriggerProps = React.ComponentPropsWithoutRef<typeof DialogPrimitive.Trigger>;
export type DialogPortalProps = React.ComponentProps<typeof DialogPrimitive.Portal>;
export type DialogCloseProps = React.ComponentPropsWithoutRef<typeof DialogPrimitive.Close>;
export type DialogOverlayProps = React.ComponentPropsWithoutRef<typeof DialogPrimitive.Overlay>;
export type DialogContentProps = React.ComponentPropsWithoutRef<typeof DialogPrimitive.Content>;
export type DialogTitleProps = React.ComponentPropsWithoutRef<typeof DialogPrimitive.Title>;
export type DialogDescriptionProps = React.ComponentPropsWithoutRef<typeof DialogPrimitive.Description>;
export type DialogHeaderProps = React.HTMLAttributes<HTMLDivElement>;
export type DialogFooterProps = React.HTMLAttributes<HTMLDivElement>;

const DialogTrigger = React.forwardRef<
  React.ElementRef<typeof DialogPrimitive.Trigger>,
  DialogTriggerProps
>(({ ...props }, ref) => (
  <DialogPrimitive.Trigger
    ref={ref}
    data-sdk-ui="dialog-trigger"
    data-slot="dialog-trigger"
    {...props}
  />
));

DialogTrigger.displayName = 'DialogTrigger';

const DialogClose = React.forwardRef<
  React.ElementRef<typeof DialogPrimitive.Close>,
  DialogCloseProps
>(({ ...props }, ref) => (
  <DialogPrimitive.Close
    ref={ref}
    data-sdk-ui="dialog-close"
    data-slot="dialog-close"
    {...props}
  />
));

DialogClose.displayName = 'DialogClose';

const DialogOverlay = React.forwardRef<
  React.ElementRef<typeof DialogPrimitive.Overlay>,
  DialogOverlayProps
>(({ className, ...props }, ref) => (
  <DialogPrimitive.Overlay
    ref={ref}
    className={cn('fixed inset-0 z-50 bg-[var(--sdk-color-surface-overlay)] backdrop-blur-sm', className)}
    data-sdk-ui="dialog-overlay"
    data-slot="dialog-overlay"
    {...props}
  />
));

DialogOverlay.displayName = 'DialogOverlay';

const DialogContent = React.forwardRef<
  React.ElementRef<typeof DialogPrimitive.Content>,
  DialogContentProps
>(({ className, children, ...props }, ref) => (
  <DialogPortal>
    <DialogOverlay />
    <DialogPrimitive.Content
      ref={ref}
      className={cn(
        'fixed left-1/2 top-1/2 z-50 grid w-[min(92vw,40rem)] -translate-x-1/2 -translate-y-1/2 gap-4 rounded-[var(--sdk-radius-panel)] border border-[var(--sdk-color-border-default)] bg-[var(--sdk-color-surface-panel)] p-6 text-[var(--sdk-color-text-primary)] shadow-[var(--sdk-shadow-lg)]',
        className,
      )}
      data-sdk-ui="dialog-content"
      data-slot="dialog-content"
      {...props}
    >
      {children}
      <DialogClose className="absolute right-4 top-4 rounded-full p-1.5 text-[var(--sdk-color-text-muted)] transition-colors hover:bg-[var(--sdk-color-brand-primary-soft)] hover:text-[var(--sdk-color-text-primary)]">
        <X className="h-4 w-4" />
        <span className="sr-only">Close</span>
      </DialogClose>
    </DialogPrimitive.Content>
  </DialogPortal>
));

DialogContent.displayName = 'DialogContent';

const DialogHeader = React.forwardRef<HTMLDivElement, DialogHeaderProps>(({ className, ...props }, ref) => (
  <div
    ref={ref}
    className={cn('flex flex-col gap-1.5 pr-10', className)}
    data-sdk-ui="dialog-header"
    data-slot="dialog-header"
    {...props}
  />
));

const DialogFooter = React.forwardRef<HTMLDivElement, DialogFooterProps>(({ className, ...props }, ref) => (
  <div
    ref={ref}
    className={cn('flex justify-end gap-3', className)}
    data-sdk-ui="dialog-footer"
    data-slot="dialog-footer"
    {...props}
  />
));

const DialogTitle = React.forwardRef<
  React.ElementRef<typeof DialogPrimitive.Title>,
  DialogTitleProps
>(({ className, ...props }, ref) => (
  <DialogPrimitive.Title
    ref={ref}
    className={cn('text-lg font-semibold', className)}
    data-sdk-ui="dialog-title"
    data-slot="dialog-title"
    {...props}
  />
));

DialogTitle.displayName = 'DialogTitle';

const DialogDescription = React.forwardRef<
  React.ElementRef<typeof DialogPrimitive.Description>,
  DialogDescriptionProps
>(({ className, ...props }, ref) => (
  <DialogPrimitive.Description
    ref={ref}
    className={cn('text-sm text-[var(--sdk-color-text-secondary)]', className)}
    data-sdk-ui="dialog-description"
    data-slot="dialog-description"
    {...props}
  />
));

DialogDescription.displayName = 'DialogDescription';

export {
  Dialog,
  DialogClose,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogOverlay,
  DialogPortal,
  DialogTitle,
  DialogTrigger,
};
Dialog.displayName = 'Dialog';
DialogFooter.displayName = 'DialogFooter';
DialogHeader.displayName = 'DialogHeader';
DialogPortal.displayName = 'DialogPortal';
