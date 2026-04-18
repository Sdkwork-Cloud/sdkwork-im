import * as React from 'react';
import { cn } from '../../../lib/utils';
import { Button, type ButtonProps } from '../button';

export interface IconButtonProps extends Omit<ButtonProps, 'size'> {}

const IconButton = React.forwardRef<HTMLButtonElement, IconButtonProps>(({ className, type = 'button', ...props }, ref) => (
  <Button
    ref={ref}
    className={cn('shrink-0', className)}
    data-sdk-ui="icon-button"
    size="icon"
    type={type}
    {...props}
  />
));

IconButton.displayName = 'IconButton';

export { IconButton };
