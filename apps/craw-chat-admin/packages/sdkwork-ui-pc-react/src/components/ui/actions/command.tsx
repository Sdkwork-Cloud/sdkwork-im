import * as React from 'react';
import { Command as CommandPrimitive } from 'cmdk';
import { Search } from 'lucide-react';
import { mergeSlotProps, type SlotProps } from '../../../lib/slot-props';
import { cn } from '../../../lib/utils';
import { Dialog, DialogContent, type DialogContentProps } from '../dialog';

export type CommandProps = React.ComponentPropsWithoutRef<typeof CommandPrimitive>;
export type CommandDialogContentProps = SlotProps<Omit<DialogContentProps, 'children'>>;
export interface CommandDialogSlotProps {
  command?: SlotProps<CommandProps>;
  content?: CommandDialogContentProps;
}
export type CommandDialogProps = React.ComponentProps<typeof Dialog> & {
  slotProps?: CommandDialogSlotProps;
};
export type CommandInputProps = React.ComponentPropsWithoutRef<typeof CommandPrimitive.Input>;
export type CommandListProps = React.ComponentPropsWithoutRef<typeof CommandPrimitive.List>;
export type CommandEmptyProps = React.ComponentPropsWithoutRef<typeof CommandPrimitive.Empty>;
export type CommandGroupProps = React.ComponentPropsWithoutRef<typeof CommandPrimitive.Group>;
export type CommandSeparatorProps = React.ComponentPropsWithoutRef<typeof CommandPrimitive.Separator>;
export type CommandItemProps = React.ComponentPropsWithoutRef<typeof CommandPrimitive.Item>;
export type CommandShortcutProps = React.HTMLAttributes<HTMLSpanElement>;

const Command = React.forwardRef<
  React.ElementRef<typeof CommandPrimitive>,
  CommandProps
>(({ className, ...props }, ref) => (
  <CommandPrimitive
    ref={ref}
    className={cn(
      'flex h-full w-full flex-col overflow-hidden rounded-[var(--sdk-radius-panel)] border border-[var(--sdk-color-border-default)] bg-[var(--sdk-color-surface-panel)] text-[var(--sdk-color-text-primary)]',
      className,
    )}
    data-sdk-ui="command"
    data-slot="command"
    {...props}
  />
));

Command.displayName = 'Command';

function CommandDialog({ children, slotProps, ...props }: CommandDialogProps) {
  return (
    <Dialog {...props}>
      <DialogContent
        data-sdk-ui="command-dialog"
        {...mergeSlotProps<CommandDialogContentProps>(
          {
            'aria-describedby': undefined,
            className: 'overflow-hidden p-0 [&>button]:hidden',
          },
          slotProps?.content,
        )}
      >
        <Command
          {...mergeSlotProps<SlotProps<CommandProps>>(
            {
              className:
                '[&_[cmdk-group-heading]]:px-3 [&_[cmdk-group-heading]]:pb-1 [&_[cmdk-group-heading]]:pt-3 [&_[cmdk-group-heading]]:text-xs [&_[cmdk-group-heading]]:font-medium [&_[cmdk-group-heading]]:uppercase [&_[cmdk-group-heading]]:tracking-[0.08em] [&_[cmdk-group-heading]]:text-[var(--sdk-color-text-muted)]',
            },
            slotProps?.command,
          )}
        >
          {children}
        </Command>
      </DialogContent>
    </Dialog>
  );
}

const CommandInput = React.forwardRef<
  React.ElementRef<typeof CommandPrimitive.Input>,
  CommandInputProps
>(({ className, ...props }, ref) => (
  <div
    className="flex items-center gap-3 border-b border-[var(--sdk-color-border-default)] px-4 py-3"
    cmdk-input-wrapper=""
    data-sdk-ui="command-input-wrapper"
    data-slot="command-input-wrapper"
  >
    <Search className="h-4 w-4 text-[var(--sdk-color-text-muted)]" />
    <CommandPrimitive.Input
      ref={ref}
      className={cn(
        'flex h-10 w-full bg-transparent text-sm text-[var(--sdk-color-text-primary)] outline-none placeholder:text-[var(--sdk-color-text-muted)] disabled:cursor-not-allowed disabled:opacity-50',
        className,
      )}
      data-sdk-ui="command-input"
      data-slot="command-input"
      {...props}
    />
  </div>
));

CommandInput.displayName = 'CommandInput';

const CommandList = React.forwardRef<
  React.ElementRef<typeof CommandPrimitive.List>,
  CommandListProps
>(({ className, ...props }, ref) => (
  <CommandPrimitive.List
    ref={ref}
    className={cn('max-h-[24rem] overflow-y-auto overflow-x-hidden p-2', className)}
    data-sdk-ui="command-list"
    data-slot="command-list"
    {...props}
  />
));

CommandList.displayName = 'CommandList';

const CommandEmpty = React.forwardRef<
  React.ElementRef<typeof CommandPrimitive.Empty>,
  CommandEmptyProps
>(({ className, ...props }, ref) => (
  <CommandPrimitive.Empty
    ref={ref}
    className={cn('py-8 text-center text-sm text-[var(--sdk-color-text-secondary)]', className)}
    data-sdk-ui="command-empty"
    data-slot="command-empty"
    {...props}
  />
));

CommandEmpty.displayName = 'CommandEmpty';

const CommandGroup = React.forwardRef<
  React.ElementRef<typeof CommandPrimitive.Group>,
  CommandGroupProps
>(({ className, ...props }, ref) => (
  <CommandPrimitive.Group
    ref={ref}
    className={cn('overflow-hidden p-1 text-[var(--sdk-color-text-primary)]', className)}
    data-sdk-ui="command-group"
    data-slot="command-group"
    {...props}
  />
));

CommandGroup.displayName = 'CommandGroup';

const CommandSeparator = React.forwardRef<
  React.ElementRef<typeof CommandPrimitive.Separator>,
  CommandSeparatorProps
>(({ className, ...props }, ref) => (
  <CommandPrimitive.Separator
    ref={ref}
    className={cn('-mx-1 my-2 h-px bg-[var(--sdk-color-border-default)]', className)}
    data-sdk-ui="command-separator"
    data-slot="command-separator"
    {...props}
  />
));

CommandSeparator.displayName = 'CommandSeparator';

const CommandItem = React.forwardRef<
  React.ElementRef<typeof CommandPrimitive.Item>,
  CommandItemProps
>(({ className, ...props }, ref) => (
  <CommandPrimitive.Item
    ref={ref}
    className={cn(
      'relative flex cursor-default items-center gap-2 rounded-[var(--sdk-radius-field)] px-3 py-2 text-sm outline-none transition-colors data-[disabled=true]:pointer-events-none data-[disabled=true]:opacity-50 data-[selected=true]:bg-[var(--sdk-color-brand-primary-soft)] data-[selected=true]:text-[var(--sdk-color-text-primary)]',
      className,
    )}
    data-sdk-ui="command-item"
    data-slot="command-item"
    {...props}
  />
));

CommandItem.displayName = 'CommandItem';

const CommandShortcut = React.forwardRef<HTMLSpanElement, CommandShortcutProps>(({ className, ...props }, ref) => (
  <span
    ref={ref}
    className={cn('ml-auto text-[11px] uppercase tracking-[0.12em] text-[var(--sdk-color-text-muted)]', className)}
    data-sdk-ui="command-shortcut"
    data-slot="command-shortcut"
    {...props}
  />
));

CommandShortcut.displayName = 'CommandShortcut';

export {
  Command,
  CommandDialog,
  CommandEmpty,
  CommandGroup,
  CommandInput,
  CommandItem,
  CommandList,
  CommandSeparator,
  CommandShortcut,
};
CommandDialog.displayName = 'CommandDialog';
