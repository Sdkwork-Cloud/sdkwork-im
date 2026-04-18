import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import path from 'node:path';
import test from 'node:test';

const appRoot = path.resolve(import.meta.dirname, '..');

function read(relativePath) {
  return readFileSync(path.join(appRoot, relativePath), 'utf8');
}

function collectPackageNames(routeManifestSource) {
  return [...routeManifestSource.matchAll(/packageName:\s*'([^']+)'/g)].map((match) => match[1]);
}

test('admin shell entry, layout, and theme preserve shared shell primitives for IM operations', () => {
  const shellEntry = read('packages/sdkwork-craw-chat-admin-shell/src/index.ts');
  const appRootSource = read('packages/sdkwork-craw-chat-admin-shell/src/application/app/AppRoot.tsx');
  const routes = read('packages/sdkwork-craw-chat-admin-shell/src/application/router/AppRoutes.tsx');
  const layout = read('packages/sdkwork-craw-chat-admin-shell/src/application/layouts/MainLayout.tsx');
  const header = read('packages/sdkwork-craw-chat-admin-shell/src/components/AppHeader.tsx');
  const themeCss = read('src/theme.css');
  const shellHost = read('packages/sdkwork-craw-chat-admin-shell/src/styles/shell-host.css');

  assert.match(shellEntry, /\.\/styles\/shell-host\.css/);
  assert.match(appRootSource, /AppRoutes/);
  assert.doesNotMatch(appRootSource, /<MainLayout \/>/);
  assert.match(routes, /MainLayout/);
  assert.match(routes, /AdminLoginPage/);
  assert.match(layout, /relative flex h-screen flex-col overflow-hidden/);
  assert.match(layout, /\[background:var\(--admin-shell-background\)\]/);
  assert.match(layout, /<Sidebar \/>/);
  assert.match(layout, /<AppHeader \/>/);
  assert.match(layout, /CommandPalette/);
  assert.match(layout, /OperationsPulseDrawer/);
  assert.match(layout, /admin-shell-content/);
  assert.match(layout, /bg-\[var\(--admin-content-background\)\]/);
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
  assert.match(header, /\[background:var\(--admin-header-background\)\]/);
  assert.doesNotMatch(header, /import\.meta\.url/);
  assert.doesNotMatch(header, /new URL\(/);
  assert.doesNotMatch(header, /Toolbar/);
  assert.doesNotMatch(header, /ToolbarGroup/);
  assert.match(themeCss, /@source "\.\/";/);
  assert.match(themeCss, /@source "\.\.\/packages";/);
  assert.match(themeCss, /--admin-sidebar-text:/);
  assert.match(themeCss, /--admin-sidebar-item-active:/);
  assert.match(themeCss, /--admin-sidebar-popover-background:/);
  assert.match(themeCss, /--admin-sidebar-edge-background:/);
  assert.match(shellHost, /admin-shell-host/);
  assert.match(shellHost, /admin-shell-route-scroll/);
  assert.match(shellHost, /admin-shell-sidebar-resize-handle/);
  assert.match(shellHost, /data-sdk-shell='craw-chat-admin-desktop'/);
  assert.doesNotMatch(layout, /router-admin-desktop|SDKWork Router Admin|Control plane/);
  assert.doesNotMatch(shellHost, /router-admin-desktop/);
});

test('shell package owns every IM module it routes to or prefetches', () => {
  const routeManifestSource = read('packages/sdkwork-craw-chat-admin-core/src/routeManifest.ts');
  const routePrefetchSource = read(
    'packages/sdkwork-craw-chat-admin-shell/src/application/router/routePrefetch.ts',
  );
  const routesSource = read('packages/sdkwork-craw-chat-admin-shell/src/application/router/AppRoutes.tsx');
  const packageJson = JSON.parse(
    read('packages/sdkwork-craw-chat-admin-shell/package.json'),
  );

  const packageNames = collectPackageNames(routeManifestSource);

  assert.ok(packageNames.length > 0, 'route manifest should declare shell-owned module packages');
  assert.equal(
    Boolean(packageJson.dependencies?.['sdkwork-craw-chat-admin-auth']),
    true,
    'shell package should depend on auth because AppRoutes renders AdminLoginPage',
  );

  for (const packageName of packageNames) {
    const escapedPackageName = packageName.replace(/[.*+?^${}()|[\]\\]/g, '\\$&');
    const importPattern = new RegExp(`import\\('${escapedPackageName}'\\)`);

    assert.equal(
      Boolean(packageJson.dependencies?.[packageName]),
      true,
      `shell package should depend on ${packageName}`,
    );
    assert.match(
      routePrefetchSource,
      importPattern,
      `route prefetch should lazy import ${packageName}`,
    );
    assert.match(routesSource, importPattern, `AppRoutes should lazy import ${packageName}`);
  }
});

test('theme manager keeps the root theme contract while resolving IM brand tokens from preferences', () => {
  const themeManager = read(
    'packages/sdkwork-craw-chat-admin-shell/src/application/providers/ThemeManager.tsx',
  );

  assert.match(themeManager, /createSdkworkTheme/);
  assert.match(themeManager, /export function useAdminShellTheme/);
  assert.match(themeManager, /root\.setAttribute\('data-theme', themeColor\)/);
  assert.match(themeManager, /data-sdk-color-mode/);
  assert.doesNotMatch(themeManager, /theme-light/);
  assert.doesNotMatch(themeManager, /theme-dark/);
});

test('storage is registered as a first-class admin module across types, core routing, and shell loading', () => {
  const typesSource = read('packages/sdkwork-craw-chat-admin-types/src/index.ts');
  const routePathsSource = read('packages/sdkwork-craw-chat-admin-core/src/routePaths.ts');
  const routesSource = read('packages/sdkwork-craw-chat-admin-core/src/routes.ts');
  const routeManifestSource = read('packages/sdkwork-craw-chat-admin-core/src/routeManifest.ts');
  const shellRoutesSource = read('packages/sdkwork-craw-chat-admin-shell/src/application/router/AppRoutes.tsx');
  const routePrefetchSource = read(
    'packages/sdkwork-craw-chat-admin-shell/src/application/router/routePrefetch.ts',
  );

  assert.match(typesSource, /'storage'/);
  assert.match(typesSource, /'sdkwork-craw-chat-admin-storage'/);
  assert.match(routePathsSource, /STORAGE:\s*'\/storage'/);
  assert.match(routesSource, /key:\s*'storage'/);
  assert.match(routeManifestSource, /moduleId:\s*'sdkwork-craw-chat-admin-storage'/);
  assert.match(shellRoutesSource, /sdkwork-craw-chat-admin-storage/);
  assert.match(routePrefetchSource, /sdkwork-craw-chat-admin-storage/);
});
