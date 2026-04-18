import * as React from 'react';
import { cn } from '../../../lib/utils';
import {
  SDKWORK_DARK_THEME,
  createThemeStyle,
  type SdkworkTheme,
} from '../../../theme/sdkwork-theme';

export interface AppShellProps extends Omit<React.ComponentPropsWithoutRef<'div'>, 'children' | 'content'> {
  content?: React.ReactNode;
  footer?: React.ReactNode;
  header?: React.ReactNode;
  sidebar?: React.ReactNode;
  sidebarWidth?: number | string;
  theme?: SdkworkTheme;
}

export const AppShell = React.forwardRef<HTMLDivElement, AppShellProps>(({
  className,
  content,
  footer,
  header,
  sidebar,
  sidebarWidth = 280,
  style,
  theme = SDKWORK_DARK_THEME,
  ...props
}, ref) => {
  return (
    <div
      ref={ref}
      className={cn(
        'flex min-h-screen w-full flex-col bg-[var(--sdk-color-surface-canvas)] text-[var(--sdk-color-text-primary)]',
        className,
      )}
      data-sdk-color-mode={theme.colorMode}
      data-sdk-pattern="app-shell"
      data-sdk-shell="app"
      {...props}
      style={{ ...createThemeStyle(theme), ...style }}
    >
      {header ? (
        <header
          className="border-b border-[var(--sdk-color-border-subtle)] bg-[var(--sdk-color-surface-panel)]/90 backdrop-blur"
          data-sdk-region="header"
        >
          {header}
        </header>
      ) : null}
      <div className="flex min-h-0 flex-1" data-sdk-region="body">
        {sidebar ? (
          <aside
            className="min-h-0 shrink-0 border-r border-[var(--sdk-color-border-subtle)] bg-[var(--sdk-color-surface-panel)]"
            data-sdk-region="sidebar"
            style={{ width: sidebarWidth }}
          >
            {sidebar}
          </aside>
        ) : null}
        <main className="min-h-0 min-w-0 flex-1" data-sdk-region="content">
          {content ?? null}
        </main>
      </div>
      {footer ? (
        <footer
          className="border-t border-[var(--sdk-color-border-subtle)] bg-[var(--sdk-color-surface-panel)]/80"
          data-sdk-region="footer"
        >
          {footer}
        </footer>
      ) : null}
    </div>
  );
});
AppShell.displayName = 'AppShell';
