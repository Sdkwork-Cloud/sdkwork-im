import assert from 'node:assert/strict';
import path from 'node:path';
import { pathToFileURL } from 'node:url';
import { test } from 'node:test';

const appRoot = path.resolve('apps/craw-chat-portal');

test('route manifest carries operator-facing summaries and command metadata', async () => {
  const manifestModule = await import(
    pathToFileURL(
      path.join(
        appRoot,
        'packages/craw-chat-portal-core/src/application/router/routeManifest.js',
      ),
    ).href,
  );

  for (const entry of manifestModule.portalRouteManifest) {
    assert.equal(typeof entry.productModule.summary, 'string');
    assert.ok(entry.productModule.summary.length > 16);
    assert.ok(Array.isArray(entry.productModule.commandDeck.relatedRoutes));
    assert.ok(entry.productModule.commandDeck.relatedRoutes.length >= 2);
    assert.equal(typeof entry.productModule.commandDeck.primaryActionLabel, 'string');
  }

  const dashboardEntry = manifestModule.portalRouteManifest.find((entry) => entry.key === 'dashboard');
  assert.ok(dashboardEntry);
  assert.match(dashboardEntry.productModule.capabilityTags.join(' / '), /响应时效/);
  assert.doesNotMatch(dashboardEntry.productModule.capabilityTags.join(' / '), /SLA/);
});

test('desktop shell renders a command deck with route summary, related routes, workspace pulse, and accessible settings entry', async () => {
  const manifestModule = await import(
    pathToFileURL(
      path.join(
        appRoot,
        'packages/craw-chat-portal-core/src/application/router/routeManifest.js',
      ),
    ).href,
  );
  const mockDataModule = await import(
    pathToFileURL(
      path.join(
        appRoot,
        'packages/craw-chat-portal-portal-api/src/mockData.js',
      ),
    ).href,
  );
  const shellModule = await import(
    pathToFileURL(
      path.join(
        appRoot,
        'packages/craw-chat-portal-core/src/components/PortalDesktopShell.js',
      ),
    ).href,
  );

  const currentRouteEntry = manifestModule.portalRouteManifest.find((entry) => entry.key === 'realtime');
  const html = shellModule.renderPortalDesktopShell({
    currentPath: currentRouteEntry.path,
    currentRouteEntry,
    currentRouteLabel: currentRouteEntry.productModule.displayName,
    pageHtml: '<section>body</section>',
    routeManifest: manifestModule.portalRouteManifest,
    shellState: {
      lastConsolePath: currentRouteEntry.path,
      settingsOpen: false,
      sidebarCollapsed: false,
      hiddenConsolePaths: [],
      consoleEntryMode: 'resume',
      pinnedConsolePath: '/console/dashboard',
      theme: 'signal',
    },
    user: mockDataModule.portalMockData.session.user,
    workspace: {
      name: 'Nebula Commerce IM',
      region: 'CN-East / Multi-AZ',
      tier: 'Enterprise',
      supportPlan: 'Platinum',
      activeBrands: 12,
      seats: 84,
      uptime: '99.983%',
    },
  });

  assert.match(html, /在线工作区/);
  assert.match(html, /实时链路 运行态势/);
  assert.match(html, /会话恢复、在线状态延迟、事件积压与设备同步/);
  assert.match(html, /操作看板/);
  assert.match(html, /工作区健康度/);
  assert.match(html, /活跃品牌/);
  assert.match(html, /操作席位/);
  assert.match(html, /工作台主题/);
  assert.match(html, /信号台/);
  assert.match(html, /标准主题/);
  assert.match(html, /打开实时演练手册/);
  assert.match(html, /当前值守域/);
  assert.match(html, /当前工作面能力焦点/);
  assert.match(html, /会话恢复/);
  assert.match(html, /在线态势/);
  assert.match(html, /设备同步/);
  assert.match(html, /工作台设置/);
  assert.match(html, /退出登录/);
  assert.match(html, /会话/);
  assert.match(html, /治理/);
  assert.match(html, /运营/);
  assert.match(html, /体验/);
  assert.match(html, /支撑/);
  assert.doesNotMatch(html, /undefined/);
  assert.doesNotMatch(html, /Nebula Commerce IM 鐠?CN-East/);
  assert.match(html, /Nebula Commerce IM · 华东 \/ 多可用区/);
  assert.doesNotMatch(html, /Nebula Commerce IM · CN-East \/ Multi-AZ/);
  assert.match(html, /租户工作区/);
  assert.doesNotMatch(html, /nebula-commerce-im/);
  assert.match(html, /12 个品牌 · 84 个操作席位/);
  assert.match(html, /工作台状态摘要/);
  assert.match(html, /接管方式/);
  assert.match(html, /继续上次模块/);
  assert.match(html, /布局状态/);
  assert.match(html, /标准布局/);
  assert.match(html, /侧栏状态/);
  assert.match(html, /展开/);
  assert.match(html, /林涛/);
  assert.match(html, /租户运营负责人/);
  assert.doesNotMatch(html, /Lin Tao/);
  assert.match(html, /企业版/);
  assert.match(html, /白金护航/);
  assert.doesNotMatch(html, />Enterprise</);
  assert.doesNotMatch(html, />Platinum</);
  assert.match(html, /aria-haspopup="dialog"/);
  assert.match(html, /aria-expanded="false"/);
  assert.match(html, /aria-current="page"/);
});

test('desktop shell surfaces the current shell posture inside the command deck', async () => {
  const manifestModule = await import(
    pathToFileURL(
      path.join(
        appRoot,
        'packages/craw-chat-portal-core/src/application/router/routeManifest.js',
      ),
    ).href,
  );
  const shellModule = await import(
    pathToFileURL(
      path.join(
        appRoot,
        'packages/craw-chat-portal-core/src/components/PortalDesktopShell.js',
      ),
    ).href,
  );

  const currentRouteEntry = manifestModule.portalRouteManifest.find((entry) => entry.key === 'dashboard');
  const html = shellModule.renderPortalDesktopShell({
    currentPath: currentRouteEntry.path,
    currentRouteEntry,
    currentRouteLabel: currentRouteEntry.productModule.displayName,
    pageHtml: '<section>body</section>',
    routeManifest: manifestModule.portalRouteManifest,
    shellState: {
      lastConsolePath: currentRouteEntry.path,
      settingsOpen: false,
      sidebarCollapsed: true,
      hiddenConsolePaths: ['/console/automation', '/console/governance'],
      consoleEntryMode: 'pinned',
      pinnedConsolePath: '/console/media',
      theme: 'signal',
    },
    user: {
      name: '林涛',
    },
    workspace: {
      name: 'Nebula Commerce IM',
      region: 'CN-East / Multi-AZ',
      tier: 'Enterprise',
      supportPlan: 'Platinum',
      activeBrands: 12,
      seats: 84,
      uptime: '99.983%',
    },
  });

  assert.match(html, /当前值守姿态/);
  assert.match(html, /固定进入模块/);
  assert.match(html, /默认入口/);
  assert.match(html, /媒体与 RTC/);
  assert.match(html, /侧栏聚焦/);
  assert.match(html, /已收起 2 个模块/);
  assert.match(html, /侧栏已收起/);
  assert.match(html, /已个性化布局 · 4 项偏好/);
});

test('desktop shell exposes a quick reset action in the command deck when shell preferences are customized', async () => {
  const manifestModule = await import(
    pathToFileURL(
      path.join(
        appRoot,
        'packages/craw-chat-portal-core/src/application/router/routeManifest.js',
      ),
    ).href,
  );
  const shellModule = await import(
    pathToFileURL(
      path.join(
        appRoot,
        'packages/craw-chat-portal-core/src/components/PortalDesktopShell.js',
      ),
    ).href,
  );

  const currentRouteEntry = manifestModule.portalRouteManifest.find((entry) => entry.key === 'realtime');
  const html = shellModule.renderPortalDesktopShell({
    currentPath: currentRouteEntry.path,
    currentRouteEntry,
    currentRouteLabel: currentRouteEntry.productModule.displayName,
    pageHtml: '<section>body</section>',
    routeManifest: manifestModule.portalRouteManifest,
    shellState: {
      lastConsolePath: currentRouteEntry.path,
      settingsOpen: false,
      sidebarCollapsed: true,
      hiddenConsolePaths: ['/console/automation'],
      consoleEntryMode: 'pinned',
      pinnedConsolePath: '/console/media',
      theme: 'ember',
    },
    user: {
      name: '林涛',
    },
    workspace: {
      name: 'Nebula Commerce IM',
      region: 'CN-East / Multi-AZ',
      tier: 'Enterprise',
      supportPlan: 'Platinum',
      activeBrands: 12,
      seats: 84,
      uptime: '99.983%',
    },
  });

  assert.match(html, /恢复 5 项偏好/);
  assert.match(html, /工作台主题/);
  assert.match(html, /余烬班次/);
  assert.match(html, /已启用个性化主题/);
  assert.match(html, /data-command="reset-shell-preferences"/);
});

test('collapsed shell shrinks the rail footprint while keeping route targets discoverable', async () => {
  const manifestModule = await import(
    pathToFileURL(
      path.join(
        appRoot,
        'packages/craw-chat-portal-core/src/application/router/routeManifest.js',
      ),
    ).href,
  );
  const shellModule = await import(
    pathToFileURL(
      path.join(
        appRoot,
        'packages/craw-chat-portal-core/src/components/PortalDesktopShell.js',
      ),
    ).href,
  );

  const currentRouteEntry = manifestModule.portalRouteManifest.find((entry) => entry.key === 'governance');
  const html = shellModule.renderPortalDesktopShell({
    currentPath: currentRouteEntry.path,
    currentRouteEntry,
    currentRouteLabel: currentRouteEntry.productModule.displayName,
    pageHtml: '<section>body</section>',
    routeManifest: manifestModule.portalRouteManifest,
    shellState: {
      lastConsolePath: currentRouteEntry.path,
      settingsOpen: false,
      sidebarCollapsed: true,
      hiddenConsolePaths: [],
      consoleEntryMode: 'resume',
      pinnedConsolePath: '/console/dashboard',
      theme: 'signal',
    },
    user: {
      name: 'Lin Tao',
    },
    workspace: {
      name: 'Nebula Commerce IM',
      slug: 'nebula-commerce-im',
      region: 'CN-East / Multi-AZ',
      tier: 'Enterprise',
      supportPlan: 'Platinum',
      activeBrands: 12,
      seats: 84,
      uptime: '99.983%',
    },
  });

  assert.match(html, /portal-shell is-rail-collapsed/);
  assert.match(html, /title="治理"/);
  assert.match(html, /title="会话"/);
  assert.match(html, /展开侧栏/);
  assert.match(html, /portal-rail__glyph">治</);
  assert.match(html, /data-route="\/console\/media"[\s\S]*portal-rail__glyph">媒</);
  assert.match(html, /租户工作区/);
  assert.doesNotMatch(html, /nebula-commerce-im/);
  assert.match(html, /portal-rail__workspace-mark/);
});

test('navigation rail hides muted modules while keeping the current and pinned routes discoverable', async () => {
  const manifestModule = await import(
    pathToFileURL(
      path.join(
        appRoot,
        'packages/craw-chat-portal-core/src/application/router/routeManifest.js',
      ),
    ).href,
  );
  const shellModule = await import(
    pathToFileURL(
      path.join(
        appRoot,
        'packages/craw-chat-portal-core/src/components/PortalDesktopShell.js',
      ),
    ).href,
  );

  const currentRouteEntry = manifestModule.portalRouteManifest.find((entry) => entry.key === 'realtime');
  const html = shellModule.renderPortalDesktopShell({
    currentPath: currentRouteEntry.path,
    currentRouteEntry,
    currentRouteLabel: currentRouteEntry.productModule.displayName,
    pageHtml: '<section>body</section>',
    routeManifest: manifestModule.portalRouteManifest,
    shellState: {
      lastConsolePath: currentRouteEntry.path,
      settingsOpen: false,
      sidebarCollapsed: false,
      hiddenConsolePaths: ['/console/media', '/console/automation'],
      consoleEntryMode: 'pinned',
      pinnedConsolePath: '/console/media',
      theme: 'signal',
    },
    user: {
      name: '林涛',
    },
    workspace: {
      name: 'Nebula Commerce IM',
      slug: 'nebula-commerce-im',
      region: 'CN-East / Multi-AZ',
      tier: 'Enterprise',
      supportPlan: 'Platinum',
      activeBrands: 12,
      seats: 84,
      uptime: '99.983%',
    },
  });

  assert.match(html, /data-route="\/console\/realtime"/);
  assert.match(html, /data-route="\/console\/media"/);
  assert.doesNotMatch(html, /data-route="\/console\/automation"/);
  assert.match(html, /媒体与 RTC/);
  assert.match(html, /实时链路/);
  assert.match(html, /治理/);
});

test('settings panel exposes dialog semantics, active theme state, and console entry preferences for operators', async () => {
  const manifestModule = await import(
    pathToFileURL(
      path.join(
        appRoot,
        'packages/craw-chat-portal-core/src/application/router/routeManifest.js',
      ),
    ).href,
  );
  const settingsModule = await import(
    pathToFileURL(
      path.join(
        appRoot,
        'packages/craw-chat-portal-core/src/components/PortalSettingsPanel.js',
      ),
    ).href,
  );

  const html = settingsModule.renderPortalSettingsPanel({
    routeManifest: manifestModule.portalRouteManifest,
    shellState: {
      settingsOpen: true,
      theme: 'atlas',
      sidebarCollapsed: false,
      hiddenConsolePaths: ['/console/automation'],
      consoleEntryMode: 'pinned',
      pinnedConsolePath: '/console/media',
    },
  });

  assert.match(html, /id="portal-settings-panel"/);
  assert.match(html, /role="dialog"/);
  assert.match(html, /aria-modal="true"/);
  assert.match(html, /aria-labelledby="portal-settings-title"/);
  assert.match(html, /工作区偏好/);
  assert.match(html, /<h2 id="portal-settings-title">控制台外观<\/h2>/);
  assert.match(html, /主题风格/);
  assert.match(html, /控制台进入策略/);
  assert.match(html, /继续上次模块/);
  assert.match(html, /固定进入模块/);
  assert.match(html, /默认进入模块/);
  assert.match(html, /侧栏显示模块/);
  assert.match(html, /侧栏行为/);
  assert.match(html, /媒体与 RTC/);
  assert.match(html, /自动化/);
  assert.match(html, /收起侧栏/);
  assert.match(html, /恢复 4 项偏好/);
  assert.match(html, />关闭</);
  assert.match(html, /信号台/);
  assert.match(html, /纵横网格/);
  assert.match(html, /余烬班次/);
  assert.match(html, /<button[\s\S]*aria-pressed="true"[\s\S]*data-theme="atlas"[\s\S]*>/);
  assert.match(html, /<button[\s\S]*aria-pressed="false"[\s\S]*data-theme="signal"[\s\S]*>/);
  assert.match(html, /<button[\s\S]*aria-pressed="false"[\s\S]*data-theme="ember"[\s\S]*>/);
  assert.match(html, /<button[\s\S]*aria-pressed="true"[\s\S]*data-console-entry-mode="pinned"[\s\S]*>/);
  assert.match(html, /<button[\s\S]*aria-pressed="false"[\s\S]*data-console-entry-mode="resume"[\s\S]*>/);
  assert.match(html, /<button[\s\S]*aria-pressed="true"[\s\S]*data-default-console-path="\/console\/media"[\s\S]*>/);
  assert.match(html, /当前固定入口/);
  assert.match(html, /<button[\s\S]*aria-pressed="false"[\s\S]*data-sidebar-console-path="\/console\/automation"[\s\S]*>/);
  assert.match(html, /<button[\s\S]*aria-pressed="true"[\s\S]*data-sidebar-console-path="\/console\/dashboard"[\s\S]*>/);
  assert.match(html, /data-command="toggle-sidebar"/);
  assert.match(html, /data-command="reset-shell-preferences"/);
  assert.doesNotMatch(
    html,
    /<button[^>]*data-command="reset-shell-preferences"[^>]*disabled[^>]*>/,
  );
});

test('settings panel clarifies pinned module standby semantics while resume mode remains active', async () => {
  const manifestModule = await import(
    pathToFileURL(
      path.join(
        appRoot,
        'packages/craw-chat-portal-core/src/application/router/routeManifest.js',
      ),
    ).href,
  );
  const settingsModule = await import(
    pathToFileURL(
      path.join(
        appRoot,
        'packages/craw-chat-portal-core/src/components/PortalSettingsPanel.js',
      ),
    ).href,
  );

  const html = settingsModule.renderPortalSettingsPanel({
    routeManifest: manifestModule.portalRouteManifest,
    shellState: {
      settingsOpen: true,
      theme: 'signal',
      sidebarCollapsed: false,
      hiddenConsolePaths: [],
      consoleEntryMode: 'resume',
      pinnedConsolePath: '/console/media',
    },
  });

  assert.match(html, /这里保留待命入口，切换为固定进入模块后生效/);
  assert.match(html, /<button[\s\S]*aria-pressed="true"[\s\S]*data-default-console-path="\/console\/media"[\s\S]*>/);
  assert.match(html, /待命入口/);
  assert.doesNotMatch(html, /当前固定入口/);
});

test('settings panel disables shell reset when the operator is already on the standard layout', async () => {
  const manifestModule = await import(
    pathToFileURL(
      path.join(
        appRoot,
        'packages/craw-chat-portal-core/src/application/router/routeManifest.js',
      ),
    ).href,
  );
  const settingsModule = await import(
    pathToFileURL(
      path.join(
        appRoot,
        'packages/craw-chat-portal-core/src/components/PortalSettingsPanel.js',
      ),
    ).href,
  );

  const html = settingsModule.renderPortalSettingsPanel({
    routeManifest: manifestModule.portalRouteManifest,
    currentPath: '/console/dashboard',
    shellState: {
      settingsOpen: true,
      theme: 'signal',
      sidebarCollapsed: false,
      hiddenConsolePaths: [],
      consoleEntryMode: 'resume',
      pinnedConsolePath: '/console/dashboard',
      lastConsolePath: '/console/dashboard',
    },
  });

  assert.match(html, /标准值守布局/);
  assert.match(
    html,
    /<button[^>]*data-command="reset-shell-preferences"[^>]*disabled[^>]*>[\s\S]*当前已是标准布局[\s\S]*<\/button>/,
  );
});

test('settings panel keeps the current and pinned modules locked visible in sidebar preferences', async () => {
  const manifestModule = await import(
    pathToFileURL(
      path.join(
        appRoot,
        'packages/craw-chat-portal-core/src/application/router/routeManifest.js',
      ),
    ).href,
  );
  const settingsModule = await import(
    pathToFileURL(
      path.join(
        appRoot,
        'packages/craw-chat-portal-core/src/components/PortalSettingsPanel.js',
      ),
    ).href,
  );

  const html = settingsModule.renderPortalSettingsPanel({
    routeManifest: manifestModule.portalRouteManifest,
    currentPath: '/console/realtime',
    shellState: {
      settingsOpen: true,
      theme: 'signal',
      sidebarCollapsed: false,
      hiddenConsolePaths: ['/console/realtime', '/console/media', '/console/automation'],
      consoleEntryMode: 'pinned',
      pinnedConsolePath: '/console/media',
    },
  });

  assert.match(
    html,
    /<button[^>]*aria-pressed="true"[^>]*data-sidebar-console-path="\/console\/realtime"[^>]*disabled[^>]*>/,
  );
  assert.match(
    html,
    /<button[^>]*aria-pressed="true"[^>]*data-sidebar-console-path="\/console\/media"[^>]*disabled[^>]*>/,
  );
  assert.match(html, /当前模块始终可见/);
  assert.match(html, /固定入口始终可见/);
  assert.match(
    html,
    /<button[^>]*aria-pressed="false"[^>]*data-sidebar-console-path="\/console\/automation"[^>]*>/,
  );
  assert.doesNotMatch(
    html,
    /<button[^>]*data-sidebar-console-path="\/console\/automation"[^>]*disabled[^>]*>/,
  );
});

test('settings panel infers the active module from the last console path when currentPath is omitted', async () => {
  const manifestModule = await import(
    pathToFileURL(
      path.join(
        appRoot,
        'packages/craw-chat-portal-core/src/application/router/routeManifest.js',
      ),
    ).href,
  );
  const settingsModule = await import(
    pathToFileURL(
      path.join(
        appRoot,
        'packages/craw-chat-portal-core/src/components/PortalSettingsPanel.js',
      ),
    ).href,
  );

  const html = settingsModule.renderPortalSettingsPanel({
    routeManifest: manifestModule.portalRouteManifest,
    shellState: {
      settingsOpen: true,
      theme: 'atlas',
      sidebarCollapsed: false,
      hiddenConsolePaths: ['/console/realtime', '/console/automation'],
      consoleEntryMode: 'pinned',
      pinnedConsolePath: '/console/media',
      lastConsolePath: '/console/realtime',
    },
  });

  assert.match(html, /当前值守模块/);
  assert.match(html, /实时链路/);
  assert.match(html, /运营 · 会话恢复/);
  assert.doesNotMatch(html, /当前工作面/);
  assert.match(
    html,
    /<button[^>]*aria-pressed="true"[^>]*data-sidebar-console-path="\/console\/realtime"[^>]*disabled[^>]*>/,
  );
  assert.match(html, /当前模块始终可见/);
});

test('settings panel surfaces a current shell summary for operators before they change preferences', async () => {
  const manifestModule = await import(
    pathToFileURL(
      path.join(
        appRoot,
        'packages/craw-chat-portal-core/src/application/router/routeManifest.js',
      ),
    ).href,
  );
  const settingsModule = await import(
    pathToFileURL(
      path.join(
        appRoot,
        'packages/craw-chat-portal-core/src/components/PortalSettingsPanel.js',
      ),
    ).href,
  );

  const html = settingsModule.renderPortalSettingsPanel({
    routeManifest: manifestModule.portalRouteManifest,
    currentPath: '/console/realtime',
    shellState: {
      settingsOpen: true,
      theme: 'atlas',
      sidebarCollapsed: false,
      hiddenConsolePaths: ['/console/automation'],
      consoleEntryMode: 'pinned',
      pinnedConsolePath: '/console/media',
    },
  });

  assert.match(html, /当前壳层摘要/);
  assert.match(html, /已同步 4 项值守偏好/);
  assert.match(html, /实时链路/);
  assert.match(html, /运营 · 会话恢复/);
  assert.match(html, /固定进入模块/);
  assert.match(html, /固定入口：媒体与 RTC/);
  assert.match(html, /当前布局已同步 4 项值守偏好，进入策略、主题与侧栏焦点都会按当前值守方式恢复。/);
});

test('settings panel escapes theme metadata before rendering operator controls', async () => {
  const manifestModule = await import(
    pathToFileURL(
      path.join(
        appRoot,
        'packages/craw-chat-portal-core/src/application/router/routeManifest.js',
      ),
    ).href,
  );
  const typesModule = await import(
    pathToFileURL(
      path.join(
        appRoot,
        'packages/craw-chat-portal-types/src/index.js',
      ),
    ).href,
  );
  const settingsModule = await import(
    pathToFileURL(
      path.join(
        appRoot,
        'packages/craw-chat-portal-core/src/components/PortalSettingsPanel.js',
      ),
    ).href,
  );

  const originalTheme = { ...typesModule.PORTAL_THEME_OPTIONS[0] };
  typesModule.PORTAL_THEME_OPTIONS[0].id = 'signal" onclick="alert(1)';
  typesModule.PORTAL_THEME_OPTIONS[0].label = 'Signal <script>alert(1)</script>';
  typesModule.PORTAL_THEME_OPTIONS[0].description = 'Steel <img src=x onerror=alert(1)>';

  try {
    const html = settingsModule.renderPortalSettingsPanel({
      routeManifest: manifestModule.portalRouteManifest,
      shellState: {
        settingsOpen: true,
        theme: 'signal" onclick="alert(1)',
        sidebarCollapsed: false,
        hiddenConsolePaths: [],
        consoleEntryMode: 'resume',
        pinnedConsolePath: '/console/dashboard',
      },
    });

    assert.match(html, /data-theme="signal&quot; onclick=&quot;alert\(1\)"/);
    assert.match(html, /Signal &lt;script&gt;alert\(1\)&lt;\/script&gt;/);
    assert.match(html, /Steel &lt;img src=x onerror=alert\(1\)&gt;/);

    assert.doesNotMatch(html, /data-theme="signal" onclick="alert\(1\)"/);
    assert.doesNotMatch(html, /Signal <script>alert\(1\)<\/script>/);
    assert.doesNotMatch(html, /Steel <img src=x onerror=alert\(1\)>/);
  } finally {
    Object.assign(typesModule.PORTAL_THEME_OPTIONS[0], originalTheme);
  }
});
