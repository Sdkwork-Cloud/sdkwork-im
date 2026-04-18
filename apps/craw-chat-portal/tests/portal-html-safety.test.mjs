import assert from 'node:assert/strict';
import path from 'node:path';
import { pathToFileURL } from 'node:url';
import { test } from 'node:test';

import { resolvePortalAppRoot } from './helpers/portal-paths.mjs';

const appRoot = resolvePortalAppRoot(import.meta.url);

function moduleUrl(relativePath) {
  return pathToFileURL(path.join(appRoot, relativePath)).href;
}

test('desktop shell escapes tenant-controlled labels before rendering HTML', async () => {
  const shellModule = await import(
    moduleUrl('packages/craw-chat-portal-core/src/components/PortalDesktopShell.js'),
  );

  const currentRouteEntry = {
    key: 'current',
    path: '/console/current',
    productModule: {
      displayName: 'Realtime <script>alert(1)</script>',
      summary: 'Summary <img src=x onerror=alert(1)>',
      capabilityTags: ['tag <b>bold</b>'],
      navigation: {
        group: 'ops',
        groupLabel: 'Operations <iframe>',
        order: 1,
      },
      commandDeck: {
        primaryActionRoute: '/console/current',
        primaryActionLabel: 'Open <b>Playbook</b>',
        relatedRoutes: ['next'],
      },
    },
  };

  const nextRouteEntry = {
    key: 'next',
    path: '/console/next',
    productModule: {
      displayName: 'Governance <svg/onload=alert(1)>',
      summary: 'Next summary',
      capabilityTags: ['audit <script>'],
      navigation: {
        group: 'ops',
        groupLabel: 'Operations <iframe>',
        order: 2,
      },
      commandDeck: {
        primaryActionRoute: '/console/next',
        primaryActionLabel: 'Open',
        relatedRoutes: ['current'],
      },
    },
  };

  const html = shellModule.renderPortalDesktopShell({
    currentPath: currentRouteEntry.path,
    currentRouteEntry,
    currentRouteLabel: currentRouteEntry.productModule.displayName,
    pageHtml: '<section>body</section>',
    routeManifest: [currentRouteEntry, nextRouteEntry],
    shellState: {
      lastConsolePath: currentRouteEntry.path,
      settingsOpen: false,
      sidebarCollapsed: false,
      theme: 'signal',
    },
    user: {
      name: 'Lin <script>alert(1)</script>',
      role: 'Ops <b>Lead</b>',
    },
    workspace: {
      name: 'Nebula <img src=x onerror=alert(1)>',
      slug: 'tenant <b>alpha</b>',
      region: 'CN-East <svg/onload=alert(1)>',
      tier: 'Enterprise <script>',
      supportPlan: 'Platinum <script>',
      activeBrands: '12 <mark>',
      seats: '84 <mark>',
      uptime: '99.983%',
    },
  });

  assert.match(html, /Realtime &lt;script&gt;alert\(1\)&lt;\/script&gt;/);
  assert.match(html, /Summary &lt;img src=x onerror=alert\(1\)&gt;/);
  assert.match(html, /portal-topbar__domain/);
  assert.match(html, /当前值守域/);
  assert.match(html, /portal-topbar__domain[\s\S]*Operations &lt;iframe&gt;/);
  assert.match(html, /tag &lt;b&gt;bold&lt;\/b&gt;/);
  assert.match(html, /Operations &lt;iframe&gt;/);
  assert.match(html, /Lin &lt;script&gt;alert\(1\)&lt;\/script&gt;/);
  assert.match(html, /Ops &lt;b&gt;Lead&lt;\/b&gt;/);
  assert.match(html, /Nebula &lt;img src=x onerror=alert\(1\)&gt;/);
  assert.match(html, /租户工作区/);
  assert.match(html, /华东 &lt;svg\/onload=alert\(1\)&gt;/);
  assert.match(html, /Enterprise &lt;script&gt;/);
  assert.match(html, /Platinum &lt;script&gt;/);
  assert.match(html, /12 &lt;mark&gt;/);
  assert.match(html, /84 &lt;mark&gt;/);
  assert.doesNotMatch(html, /tenant &lt;b&gt;alpha&lt;\/b&gt;/);

  assert.doesNotMatch(html, /Realtime <script>alert\(1\)<\/script>/);
  assert.doesNotMatch(html, /tag <b>bold<\/b>/);
  assert.doesNotMatch(html, /Lin <script>alert\(1\)<\/script>/);
  assert.doesNotMatch(html, /Ops <b>Lead<\/b>/);
  assert.doesNotMatch(html, /Nebula <img src=x onerror=alert\(1\)>/);
  assert.doesNotMatch(html, /tenant <b>alpha<\/b>/);
});

test('data table escapes column and cell content before rendering HTML', async () => {
  const domModule = await import(
    moduleUrl('packages/craw-chat-portal-commons/src/framework/dom.js'),
  );

  const html = domModule.renderDataTable({
    columns: ['Topic <script>alert(1)</script>', 'Status & "Owner"'],
    rows: [[
      'VIP <img src=x onerror=alert(1)>',
      'Healthy & "Ready"',
    ]],
  });

  assert.match(html, /Topic &lt;script&gt;alert\(1\)&lt;\/script&gt;/);
  assert.match(html, /Status &amp; &quot;Owner&quot;/);
  assert.match(html, /VIP &lt;img src=x onerror=alert\(1\)&gt;/);
  assert.match(html, /Healthy &amp; &quot;Ready&quot;/);

  assert.doesNotMatch(html, /<script>alert\(1\)<\/script>/);
  assert.doesNotMatch(html, /<img src=x onerror=alert\(1\)>/);
});

test('portal pages escape snapshot copy before rendering hero content', async () => {
  const dataSourceModule = await import(
    moduleUrl('packages/craw-chat-portal-portal-api/src/runtime/activeDataSource.js'),
  );
  const homeModule = await import(
    moduleUrl('packages/craw-chat-portal-home/src/index.js'),
  );
  const dashboardModule = await import(
    moduleUrl('packages/craw-chat-portal-dashboard/src/index.js'),
  );

  const originalDataSource =
    typeof dataSourceModule.getActivePortalDataSource === 'function'
      ? dataSourceModule.getActivePortalDataSource()
      : dataSourceModule.activePortalDataSource;

  dataSourceModule.setActivePortalDataSource({
    ...originalDataSource,
    async getPortalHome() {
      return {
        hero: {
          eyebrow: '入口 <img src=x onerror=alert(1)>',
          title: '门户 <script>alert(1)</script>',
          description: '说明 <svg/onload=alert(1)>',
        },
        pillars: [
          {
            title: '支柱 <b>一</b>',
            description: '能力 <iframe>',
          },
        ],
      };
    },
    async getPortalDashboard() {
      return {
        hero: {
          title: '总览 <script>alert(1)</script>',
          description: '风险 <img src=x onerror=alert(1)>',
          kpis: [],
        },
        pressure: [],
        posture: [],
        priorities: [],
        timeline: [],
      };
    },
  });

  try {
    const homeHtml = await homeModule.renderPortalHomePage();
    const dashboardHtml = await dashboardModule.renderPortalDashboardPage();

    assert.match(homeHtml, /入口 &lt;img src=x onerror=alert\(1\)&gt;/);
    assert.match(homeHtml, /门户 &lt;script&gt;alert\(1\)&lt;\/script&gt;/);
    assert.match(homeHtml, /说明 &lt;svg\/onload=alert\(1\)&gt;/);
    assert.match(homeHtml, /支柱 &lt;b&gt;一&lt;\/b&gt;/);
    assert.match(homeHtml, /能力 &lt;iframe&gt;/);

    assert.match(dashboardHtml, /总览 &lt;script&gt;alert\(1\)&lt;\/script&gt;/);
    assert.match(dashboardHtml, /风险 &lt;img src=x onerror=alert\(1\)&gt;/);

    assert.doesNotMatch(homeHtml, /门户 <script>alert\(1\)<\/script>/);
    assert.doesNotMatch(dashboardHtml, /风险 <img src=x onerror=alert\(1\)>/);
  } finally {
    dataSourceModule.resetActivePortalDataSource();
  }
});

test('portal auth page escapes control-plane copy before rendering HTML', async () => {
  const dataSourceModule = await import(
    moduleUrl('packages/craw-chat-portal-portal-api/src/runtime/activeDataSource.js'),
  );
  const authModule = await import(
    moduleUrl('packages/craw-chat-portal-auth/src/index.js'),
  );

  const originalDataSource =
    typeof dataSourceModule.getActivePortalDataSource === 'function'
      ? dataSourceModule.getActivePortalDataSource()
      : dataSourceModule.activePortalDataSource;

  dataSourceModule.setActivePortalDataSource({
    ...originalDataSource,
    async getPortalAuth() {
      return {
        eyebrow: 'Tenant <img src=x onerror=alert(1)>',
        title: 'Sign in <script>alert(1)</script>',
        description: 'Workspace <svg/onload=alert(1)>',
        details: [
          {
            label: 'Workspace <iframe>',
            value: 'Nebula <b>Commerce</b>',
          },
        ],
        primaryActionLabel: 'Open <b>Tenant</b>',
        secondaryActionLabel: 'Back <script>',
      };
    },
  });

  try {
    const authHtml = await authModule.renderPortalAuthPage();

    assert.match(authHtml, /Tenant &lt;img src=x onerror=alert\(1\)&gt;/);
    assert.match(authHtml, /Sign in &lt;script&gt;alert\(1\)&lt;\/script&gt;/);
    assert.match(authHtml, /Workspace &lt;svg\/onload=alert\(1\)&gt;/);
    assert.match(authHtml, /Workspace &lt;iframe&gt;/);
    assert.match(authHtml, /Nebula &lt;b&gt;Commerce&lt;\/b&gt;/);
    assert.match(authHtml, /Open &lt;b&gt;Tenant&lt;\/b&gt;/);
    assert.match(authHtml, /Back &lt;script&gt;/);

    assert.doesNotMatch(authHtml, /Tenant <img src=x onerror=alert\(1\)>/);
    assert.doesNotMatch(authHtml, /Sign in <script>alert\(1\)<\/script>/);
  } finally {
    dataSourceModule.resetActivePortalDataSource();
  }
});
