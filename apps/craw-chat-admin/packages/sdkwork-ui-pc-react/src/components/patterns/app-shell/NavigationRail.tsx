import * as React from 'react';
import { cn } from '../../../lib/utils';

export interface NavigationRailProps extends React.ComponentPropsWithoutRef<'nav'> {
  footer?: React.ReactNode;
  header?: React.ReactNode;
}

export const NavigationRail = React.forwardRef<HTMLElement, NavigationRailProps>(({
  children,
  className,
  footer,
  header,
  ...props
}, ref) => {
  return (
    <nav
      ref={ref}
      className={cn(
        'flex h-full flex-col gap-4 px-4 py-5 text-[var(--sdk-color-text-secondary)]',
        className,
      )}
      data-sdk-pattern="navigation-rail"
      {...props}
    >
      {header ? <div className="shrink-0">{header}</div> : null}
      <div className="flex min-h-0 flex-1 flex-col gap-2">{children}</div>
      {footer ? <div className="shrink-0 pt-2">{footer}</div> : null}
    </nav>
  );
});
NavigationRail.displayName = 'NavigationRail';
