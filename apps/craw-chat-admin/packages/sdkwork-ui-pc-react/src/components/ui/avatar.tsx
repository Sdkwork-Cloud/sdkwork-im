import * as React from 'react';
import * as AvatarPrimitive from '@radix-ui/react-avatar';
import { cn } from '../../lib/utils';

export type AvatarProps = React.ComponentPropsWithoutRef<typeof AvatarPrimitive.Root>;
export type AvatarImageProps = React.ComponentPropsWithoutRef<typeof AvatarPrimitive.Image>;
export type AvatarFallbackProps = React.ComponentPropsWithoutRef<typeof AvatarPrimitive.Fallback>;

const Avatar = React.forwardRef<
  React.ElementRef<typeof AvatarPrimitive.Root>,
  AvatarProps
>(({ className, ...props }, ref) => (
  <AvatarPrimitive.Root
    ref={ref}
    className={cn(
      'relative flex h-10 w-10 shrink-0 overflow-hidden rounded-full border border-[var(--sdk-color-border-subtle)] bg-[var(--sdk-color-surface-panel-muted)] text-[var(--sdk-color-text-primary)]',
      className,
    )}
    data-sdk-ui="avatar"
    {...props}
  />
));

Avatar.displayName = 'Avatar';

const AvatarImage = React.forwardRef<
  React.ElementRef<typeof AvatarPrimitive.Image>,
  AvatarImageProps
>(({ className, ...props }, ref) => (
  <AvatarPrimitive.Image
    ref={ref}
    className={cn('aspect-square h-full w-full object-cover', className)}
    data-sdk-ui="avatar-image"
    {...props}
  />
));

AvatarImage.displayName = 'AvatarImage';

const AvatarFallback = React.forwardRef<
  React.ElementRef<typeof AvatarPrimitive.Fallback>,
  AvatarFallbackProps
>(({ className, ...props }, ref) => (
  <AvatarPrimitive.Fallback
    ref={ref}
    className={cn(
      'flex h-full w-full items-center justify-center rounded-full bg-[var(--sdk-color-brand-primary-soft)] text-xs font-semibold uppercase tracking-[0.08em] text-[var(--sdk-color-text-primary)]',
      className,
    )}
    data-sdk-ui="avatar-fallback"
    {...props}
  />
));

AvatarFallback.displayName = 'AvatarFallback';

export { Avatar, AvatarFallback, AvatarImage };
