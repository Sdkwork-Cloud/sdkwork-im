import * as React from 'react';
import * as DialogPrimitive from '@radix-ui/react-dialog';
import { cva, type VariantProps } from 'class-variance-authority';
import { X } from 'lucide-react';
import { cn } from '../../../lib/utils';

const Drawer = DialogPrimitive.Root;
const DrawerPortal = DialogPrimitive.Portal;

export type DrawerProps = React.ComponentPropsWithoutRef<typeof DialogPrimitive.Root>;
export type DrawerTriggerProps = React.ComponentPropsWithoutRef<typeof DialogPrimitive.Trigger>;
export type DrawerPortalProps = React.ComponentProps<typeof DialogPrimitive.Portal>;
export type DrawerCloseProps = React.ComponentPropsWithoutRef<typeof DialogPrimitive.Close>;
export type DrawerOverlayProps = React.ComponentPropsWithoutRef<typeof DialogPrimitive.Overlay>;

const DrawerTrigger = React.forwardRef<
  React.ElementRef<typeof DialogPrimitive.Trigger>,
  DrawerTriggerProps
>(({ ...props }, ref) => (
  <DialogPrimitive.Trigger
    ref={ref}
    data-sdk-ui="drawer-trigger"
    data-slot="drawer-trigger"
    {...props}
  />
));

DrawerTrigger.displayName = 'DrawerTrigger';

const DrawerClose = React.forwardRef<
  React.ElementRef<typeof DialogPrimitive.Close>,
  DrawerCloseProps
>(({ ...props }, ref) => (
  <DialogPrimitive.Close
    ref={ref}
    data-sdk-ui="drawer-close"
    data-slot="drawer-close"
    {...props}
  />
));

DrawerClose.displayName = 'DrawerClose';

const DrawerOverlay = React.forwardRef<
  React.ElementRef<typeof DialogPrimitive.Overlay>,
  DrawerOverlayProps
>(({ className, ...props }, ref) => (
  <DialogPrimitive.Overlay
    className={cn('fixed inset-0 z-50 bg-[var(--sdk-color-surface-overlay)] backdrop-blur-sm', className)}
    data-sdk-ui="drawer-overlay"
    data-slot="drawer-overlay"
    ref={ref}
    {...props}
  />
));

DrawerOverlay.displayName = 'DrawerOverlay';

const drawerContentVariants = cva(
  'fixed z-50 flex h-full flex-col border border-[var(--sdk-color-border-default)] bg-[var(--sdk-color-surface-panel)] text-[var(--sdk-color-text-primary)] shadow-[var(--sdk-shadow-lg)] outline-none',
  {
    defaultVariants: {
      side: 'right',
      size: 'md',
    },
    variants: {
      side: {
        left: 'inset-y-0 left-0 rounded-r-[var(--sdk-radius-panel)] border-r data-[state=closed]:-translate-x-full data-[state=open]:translate-x-0',
        right: 'inset-y-0 right-0 rounded-l-[var(--sdk-radius-panel)] border-l data-[state=closed]:translate-x-full data-[state=open]:translate-x-0',
      },
      size: {
        sm: 'w-[min(92vw,24rem)]',
        md: 'w-[min(92vw,32rem)]',
        lg: 'w-[min(92vw,40rem)]',
        xl: 'w-[min(92vw,48rem)]',
        full: 'w-screen rounded-none',
      },
    },
  },
);

export interface DrawerContentProps
  extends React.ComponentPropsWithoutRef<typeof DialogPrimitive.Content>,
    VariantProps<typeof drawerContentVariants> {}

export type DrawerHeaderProps = React.HTMLAttributes<HTMLDivElement>;
export type DrawerBodyProps = React.HTMLAttributes<HTMLDivElement>;
export type DrawerFooterProps = React.HTMLAttributes<HTMLDivElement>;

const DrawerContent = React.forwardRef<React.ElementRef<typeof DialogPrimitive.Content>, DrawerContentProps>(
  ({ children, className, side, size, ...props }, ref) => (
    <DrawerPortal>
      <DrawerOverlay />
      <DialogPrimitive.Content
        className={cn(
          drawerContentVariants({ side, size }),
          'transition-transform duration-200 ease-out',
          className,
        )}
        data-sdk-ui="drawer-content"
        data-slot="drawer-content"
        ref={ref}
        {...props}
      >
        {children}
        <DrawerClose className="absolute right-4 top-4 rounded-full p-2 text-[var(--sdk-color-text-muted)] transition-colors hover:bg-[var(--sdk-color-brand-primary-soft)] hover:text-[var(--sdk-color-text-primary)]">
          <X className="h-4 w-4" />
          <span className="sr-only">Close</span>
        </DrawerClose>
      </DialogPrimitive.Content>
    </DrawerPortal>
  ),
);

DrawerContent.displayName = 'DrawerContent';

const DrawerHeader = React.forwardRef<HTMLDivElement, DrawerHeaderProps>(({ className, ...props }, ref) => (
  <div
    ref={ref}
    className={cn('flex shrink-0 flex-col gap-1.5 border-b border-[var(--sdk-color-border-default)] px-6 py-5 pr-14', className)}
    data-sdk-ui="drawer-header"
    data-slot="drawer-header"
    {...props}
  />
));

const DrawerBody = React.forwardRef<HTMLDivElement, DrawerBodyProps>(({ className, ...props }, ref) => (
  <div
    ref={ref}
    className={cn('flex-1 overflow-y-auto px-6 py-5', className)}
    data-sdk-ui="drawer-body"
    data-slot="drawer-body"
    {...props}
  />
));

const DrawerFooter = React.forwardRef<HTMLDivElement, DrawerFooterProps>(({ className, ...props }, ref) => (
  <div
    ref={ref}
    className={cn('flex shrink-0 items-center justify-end gap-3 border-t border-[var(--sdk-color-border-default)] px-6 py-4', className)}
    data-sdk-ui="drawer-footer"
    data-slot="drawer-footer"
    {...props}
  />
));

export type DrawerTitleProps = React.ComponentPropsWithoutRef<typeof DialogPrimitive.Title>;
export type DrawerDescriptionProps = React.ComponentPropsWithoutRef<typeof DialogPrimitive.Description>;

const DrawerTitle = React.forwardRef<
  React.ElementRef<typeof DialogPrimitive.Title>,
  DrawerTitleProps
>(({ className, ...props }, ref) => (
  <DialogPrimitive.Title
    className={cn('text-lg font-semibold', className)}
    data-sdk-ui="drawer-title"
    data-slot="drawer-title"
    ref={ref}
    {...props}
  />
));

DrawerTitle.displayName = 'DrawerTitle';

const DrawerDescription = React.forwardRef<
  React.ElementRef<typeof DialogPrimitive.Description>,
  DrawerDescriptionProps
>(({ className, ...props }, ref) => (
  <DialogPrimitive.Description
    className={cn('text-sm text-[var(--sdk-color-text-secondary)]', className)}
    data-sdk-ui="drawer-description"
    data-slot="drawer-description"
    ref={ref}
    {...props}
  />
));

DrawerDescription.displayName = 'DrawerDescription';

export {
  Drawer,
  DrawerBody,
  DrawerClose,
  DrawerContent,
  DrawerDescription,
  DrawerFooter,
  DrawerHeader,
  DrawerOverlay,
  DrawerPortal,
  DrawerTitle,
  DrawerTrigger,
};
Drawer.displayName = 'Drawer';
DrawerBody.displayName = 'DrawerBody';
DrawerFooter.displayName = 'DrawerFooter';
DrawerHeader.displayName = 'DrawerHeader';
DrawerPortal.displayName = 'DrawerPortal';
