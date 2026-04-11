import type { ReactNode } from 'react';

import { AppHeader } from '../../components/AppHeader';
import { CommandPalette } from '../../components/CommandPalette';
import { OperationsPulseDrawer } from '../../components/OperationsPulseDrawer';
import { RouteContextStrip } from '../../components/RouteContextStrip';
import { Sidebar } from '../../components/Sidebar';

export function MainLayout({ children }: { children: ReactNode }) {
  return (
    <div
      className="admin-shell-host relative flex h-screen flex-col overflow-hidden [background:var(--admin-shell-background)] font-sans text-[var(--admin-text-primary)] transition-colors duration-300"
      data-sdk-shell="craw-chat-admin-desktop"
    >
      <AppHeader />
      <RouteContextStrip />
      <CommandPalette />
      <OperationsPulseDrawer />
      <div className="relative z-10 flex min-h-0 flex-1 overflow-hidden">
        <Sidebar />
        <main className="admin-shell-content relative z-10 min-w-0 flex-1 overflow-auto scrollbar-hide bg-[var(--admin-content-background)]">
          {children}
        </main>
      </div>
    </div>
  );
}
