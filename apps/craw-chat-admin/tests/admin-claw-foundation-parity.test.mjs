import assert from 'node:assert/strict';
import { existsSync, readFileSync } from 'node:fs';
import path from 'node:path';
import test from 'node:test';

const appRoot = path.resolve(import.meta.dirname, '..');

function read(relativePath) {
  return readFileSync(path.join(appRoot, relativePath), 'utf8');
}

test('admin root imports shared ui css while the shell package owns layout host primitives', () => {
  const main = read('src/main.tsx');
  const themeCss = read('src/theme.css');
  const shellEntry = read('packages/sdkwork-craw-chat-admin-shell/src/index.ts');
  const shellHost = read('packages/sdkwork-craw-chat-admin-shell/src/styles/shell-host.css');

  assert.match(main, /@sdkwork\/ui-pc-react\/styles\.css/);
  assert.match(main, /\.\/theme\.css/);
  assert.match(themeCss, /@source "\.\/";/);
  assert.match(themeCss, /@source "\.\.\/packages";/);
  assert.match(themeCss, /--admin-shell-background/);
  assert.match(themeCss, /--admin-sidebar-background/);
  assert.match(themeCss, /--admin-content-background/);
  assert.match(themeCss, /--admin-sidebar-text:/);
  assert.match(themeCss, /--admin-sidebar-item-hover:/);
  assert.match(themeCss, /--admin-sidebar-popover-background:/);
  assert.match(themeCss, /--admin-sidebar-edge-background:/);
  assert.match(shellEntry, /\.\/styles\/shell-host\.css/);
  assert.match(shellHost, /admin-shell-host/);
  assert.match(shellHost, /admin-shell-route-scroll/);
  assert.match(shellHost, /admin-shell-sidebar-resize-handle/);
  assert.match(shellHost, /data-sdk-shell='craw-chat-admin-desktop'/);
  assert.doesNotMatch(shellHost, /admin-shell-auth-stage/);
});

test('admin keeps localization in core and does not ship a legacy commons package manifest', () => {
  const coreIndex = read('packages/sdkwork-craw-chat-admin-core/src/index.tsx');
  const coreI18n = read('packages/sdkwork-craw-chat-admin-core/src/i18n.tsx');

  assert.match(coreIndex, /AdminI18nProvider/);
  assert.match(coreIndex, /useAdminI18n/);
  assert.match(coreI18n, /translateAdminText/);
  assert.match(coreI18n, /ADMIN_LOCALE_OPTIONS/);
  assert.equal(
    existsSync(path.join(appRoot, 'packages', 'sdkwork-craw-chat-admin-commons', 'package.json')),
    false,
  );
});

test('admin shell chrome keeps shared desktop shell interaction primitives for the IM console', () => {
  const layout = read('packages/sdkwork-craw-chat-admin-shell/src/application/layouts/MainLayout.tsx');
  const header = read('packages/sdkwork-craw-chat-admin-shell/src/components/AppHeader.tsx');
  const sidebar = read('packages/sdkwork-craw-chat-admin-shell/src/components/Sidebar.tsx');
  const routePrefetch = read(
    'packages/sdkwork-craw-chat-admin-shell/src/application/router/routePrefetch.ts',
  );

  assert.match(layout, /relative flex h-screen flex-col overflow-hidden/);
  assert.match(layout, /<Sidebar \/>/);
  assert.match(layout, /<AppHeader \/>/);
  assert.match(layout, /admin-shell-content/);
  assert.match(layout, /\[background:var\(--admin-shell-background\)\]/);
  assert.match(layout, /bg-\[var\(--admin-content-background\)\]/);
  assert.doesNotMatch(layout, /DesktopShellFrame|brandMark|Control plane|router-admin/);
  assert.match(header, /\[background:var\(--admin-header-background\)\]/);
  assert.match(header, /ShellStatus/);
  assert.match(header, /HeaderActionButton/);
  assert.match(header, /data-slot="app-header-leading"/);
  assert.match(header, /data-slot="app-header-brand"/);
  assert.match(header, /data-slot="app-header-trailing"/);
  assert.match(header, /dataSlot="app-header-search"/);
  assert.match(header, /dataSlot="app-header-pulse"/);
  assert.match(header, /dataSlot="app-header-refresh"/);
  assert.match(header, /t\('Craw Chat Admin'\)/);
  assert.match(header, /ROUTE_PATHS\.OVERVIEW/);
  assert.match(header, /Ctrl K/);
  assert.doesNotMatch(header, /Toolbar|ToolbarGroup|import\.meta\.url|new URL\(/);
  assert.match(sidebar, /motion\/react/);
  assert.match(sidebar, /sidebar-edge-control/);
  assert.match(sidebar, /PanelLeftOpen/);
  assert.match(sidebar, /ChevronUp/);
  assert.match(sidebar, /\[background:var\(--admin-sidebar-background\)\]/);
  assert.match(sidebar, /text-\[var\(--admin-sidebar-text\)\]/);
  assert.match(sidebar, /text-\[var\(--admin-sidebar-text-muted\)\]/);
  assert.match(sidebar, /bg-\[var\(--admin-sidebar-item-hover\)\]/);
  assert.match(sidebar, /bg-\[var\(--admin-sidebar-popover-background\)\]/);
  assert.match(sidebar, /bg-\[var\(--admin-sidebar-edge-background\)\]/);
  assert.match(sidebar, /bg-primary-500/);
  assert.match(sidebar, /text-primary-400/);
  assert.match(sidebar, /bg-primary-500\/15/);
  assert.match(
    sidebar,
    /currentSidebarWidth = isSidebarCollapsed \? COLLAPSED_SIDEBAR_WIDTH : resolvedSidebarWidth/,
  );
  assert.match(sidebar, /data-slot="sidebar-resize-handle"/);
  assert.match(sidebar, /sidebar-user-control/);
  assert.match(sidebar, /prefetchSidebarRoute/);
  assert.match(sidebar, /scheduleSidebarRoutePrefetch/);
  assert.match(sidebar, /cancelSidebarRoutePrefetch/);
  assert.match(sidebar, /onPointerDown=\{\(\) => prefetchSidebarRoute\(item\.to\)\}/);
  assert.match(sidebar, /onMouseEnter=\{\(\) => scheduleSidebarRoutePrefetch\(item\.to\)\}/);
  assert.match(sidebar, /onMouseLeave=\{\(\) => cancelSidebarRoutePrefetch\(item\.to\)\}/);
  assert.match(sidebar, /onFocus=\{\(\) => scheduleSidebarRoutePrefetch\(item\.to\)\}/);
  assert.match(sidebar, /onBlur=\{\(\) => cancelSidebarRoutePrefetch\(item\.to\)\}/);
  assert.match(sidebar, /prefetchSidebarRoute\(accountSettingsTarget\)/);
  assert.match(routePrefetch, /createSidebarRoutePrefetchController/);
  assert.match(routePrefetch, /scheduleDelayMs = 120/);
  assert.match(routePrefetch, /sdkwork-craw-chat-admin-overview/);
  assert.match(routePrefetch, /sdkwork-craw-chat-admin-users/);
  assert.match(routePrefetch, /sdkwork-craw-chat-admin-settings/);
  assert.match(routePrefetch, /sdkwork-craw-chat-admin-realtime/);
  assert.doesNotMatch(sidebar, /NavigationRail|DropdownMenu|AvatarFallback|<Avatar/);
});

test('admin sidebar collapse heuristics and persisted preference remain part of the shared shell baseline', () => {
  const adminStore = read('packages/sdkwork-craw-chat-admin-core/src/store.ts');
  const adminAutoCollapse = read('packages/sdkwork-craw-chat-admin-core/src/sidebarAutoCollapse.ts');

  const adminAutoCollapseSnippets = [
    'const COMPACT_VIEWPORT_WIDTH = 1440;',
    'const ROOMY_VIEWPORT_WIDTH = 1600;',
    'const TIGHT_VIEWPORT_HEIGHT = 900;',
    'const HIGH_SCALE_FACTOR = 1.25;',
    'const TIGHT_EFFECTIVE_SCREEN_HEIGHT = 920;',
    'export function shouldAutoCollapseSidebar',
    'export function resolveAutoSidebarCollapsed',
  ];

  for (const snippet of adminAutoCollapseSnippets) {
    assert.match(adminAutoCollapse, new RegExp(snippet.replace(/[.*+?^${}()|[\]\\]/g, '\\$&')));
  }

  const adminStoreSnippets = [
    'isSidebarCollapsed',
    'sidebarWidth',
    'toggleSidebar',
    'setSidebarCollapsed',
    'setSidebarWidth',
    'sidebarCollapsePreference',
    "sidebarCollapsePreference: 'auto'",
    'resolveAutoSidebarCollapsed()',
    "sidebarCollapsePreference: 'user'",
    "sidebarCollapsePreference === 'auto'",
  ];

  for (const snippet of adminStoreSnippets) {
    assert.match(adminStore, new RegExp(snippet.replace(/[.*+?^${}()|[\]\\]/g, '\\$&')));
  }
});
