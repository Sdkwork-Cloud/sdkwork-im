import assert from 'node:assert/strict';
import { existsSync, readFileSync } from 'node:fs';
import path from 'node:path';
import test from 'node:test';

const appRoot = path.resolve(import.meta.dirname, '..');
let operatorErrorStatusRuntimeModulePromise;

function read(relativePath) {
  return readFileSync(path.join(appRoot, relativePath), 'utf8');
}

function assertModuleSurface(relativePath, patterns) {
  assert.equal(existsSync(path.join(appRoot, relativePath)), true, `missing ${relativePath}`);
  const source = read(relativePath);

  for (const pattern of patterns) {
    assert.match(source, pattern, `${relativePath} missing ${String(pattern)}`);
  }
}

function collectStaticTranslationKeys(source) {
  const keys = new Set();
  const patterns = [
    /(?:^|[^\w$])t\(\s*'((?:\\'|[^'])+)'\s*(?:,|\))/g,
    /(?:^|[^\w$])t\(\s*"((?:\\"|[^"])*)"\s*(?:,|\))/g,
    /(?:^|[^\w$])translateAdminText\(\s*'((?:\\'|[^'])+)'\s*(?:,|\))/g,
    /(?:^|[^\w$])translateAdminText\(\s*"((?:\\"|[^"])*)"\s*(?:,|\))/g,
  ];

  for (const pattern of patterns) {
    let match = pattern.exec(source);
    while (match) {
      keys.add(match[1]);
      match = pattern.exec(source);
    }
  }

  return [...keys];
}

function collectCapabilityTagLabels(source) {
  const tags = new Set();
  const capabilityTagLists = source.matchAll(/capabilityTags:\s*\[([^\]]+)\]/g);

  for (const match of capabilityTagLists) {
    for (const tagMatch of match[1].matchAll(/'([^']+)'/g)) {
      tags.add(tagMatch[1].replaceAll('-', ' '));
    }
  }

  return [...tags];
}

function collectRequiredPermissions(source) {
  const permissions = new Set();
  const permissionLists = source.matchAll(/requiredPermissions:\s*\[([^\]]+)\]/g);

  for (const match of permissionLists) {
    for (const permissionMatch of match[1].matchAll(/'([^']+)'/g)) {
      permissions.add(permissionMatch[1]);
    }
  }

  return [...permissions];
}

async function loadOperatorErrorStatusRuntimeModule() {
  if (!operatorErrorStatusRuntimeModulePromise) {
    const source = read('packages/sdkwork-craw-chat-admin-core/src/operatorErrorStatus.ts')
      .replace(
        "import { AdminApiError } from 'sdkwork-craw-chat-admin-admin-api';",
        [
          'export class AdminApiError extends Error {',
          '  constructor(status, message) {',
          '    super(message);',
          '    this.name = "AdminApiError";',
          '    this.status = status;',
          '  }',
          '}',
        ].join('\n'),
      )
      .replace('function normalizeMessage(error: Error) {', 'function normalizeMessage(error) {')
      .replace(
        'function isTechnicalTransportMessage(message: string) {',
        'function isTechnicalTransportMessage(message) {',
      )
      .replace(
        'function resolveOperatorSafeMessage(message: string, fallback: string) {',
        'function resolveOperatorSafeMessage(message, fallback) {',
      )
      .replace(
        'export function resolveAdminOperatorErrorStatus(error: unknown, fallback: string) {',
        'export function resolveAdminOperatorErrorStatus(error, fallback) {',
      )
      .replace(
        'export function resolveAdminOperatorMessage(message: string | null | undefined, fallback: string) {',
        'export function resolveAdminOperatorMessage(message, fallback) {',
      );

    operatorErrorStatusRuntimeModulePromise = import(
      `data:text/javascript;base64,${Buffer.from(source, 'utf8').toString('base64')}`
    );
  }

  return operatorErrorStatusRuntimeModulePromise;
}

test('overview exposes IM operations posture and hotspots', () => {
  assertModuleSurface('packages/sdkwork-craw-chat-admin-overview/src/index.tsx', [
    /Message throughput/,
    /Moderation backlog/,
    /Online users/,
    /Hot conversations|hot conversations/i,
    /Tenant load|Command board/,
    /Incident watch|Shift handoff/,
  ]);
});

test('overview avoids synthetic fallback metrics and seeded placeholder queues', () => {
  const overview = read('packages/sdkwork-craw-chat-admin-overview/src/index.tsx');

  assert.doesNotMatch(overview, /\|\| 12480/);
  assert.doesNotMatch(overview, /\|\| 312/);
  assert.doesNotMatch(overview, /alerts\.length \|\| 4/);
  assert.doesNotMatch(overview, /Escalation desk/);
  assert.doesNotMatch(overview, /VIP support/);
  assert.doesNotMatch(overview, /creator-hub/);
  assert.doesNotMatch(overview, /tenant-1/);
  assert.doesNotMatch(overview, /Northstar support cloud/);
  assert.doesNotMatch(overview, /Creator network cn/);
});

test('overview localizes incident watch details instead of rendering raw alert strings', () => {
  const overview = read('packages/sdkwork-craw-chat-admin-overview/src/index.tsx');
  const overviewModel = read('packages/sdkwork-craw-chat-admin-overview/src/overviewModel.ts');
  const coreIndex = read('packages/sdkwork-craw-chat-admin-core/src/index.tsx');
  const alertCopy = read('packages/sdkwork-craw-chat-admin-core/src/adminAlertCopy.ts');

  assert.doesNotMatch(overview, /\{incident\.detail\}/);
  assert.match(overview, /renderOverviewCopy\(incident\.detail, t, formatNumber\)/);
  assert.match(alertCopy, /export function resolveAdminAlertDetailCopy/);
  assert.match(coreIndex, /resolveAdminAlertDetailCopy/);
  assert.match(overviewModel, /resolveAdminAlertDetailCopy\(alert\.detail\)/);
  assert.doesNotMatch(overviewModel, /function buildAlertDetailCopy/);
});

test('moderation queue and operations pulse localize alert-backed incident copy instead of rendering raw alert strings', () => {
  const coreIndex = read('packages/sdkwork-craw-chat-admin-core/src/index.tsx');
  const alertCopy = read('packages/sdkwork-craw-chat-admin-core/src/adminAlertCopy.ts');
  const moderation = read('packages/sdkwork-craw-chat-admin-moderation/src/index.tsx');
  const operationsPulse = read(
    'packages/sdkwork-craw-chat-admin-shell/src/components/OperationsPulseDrawer.tsx',
  );

  assert.match(alertCopy, /export function translateAdminAlertTitle/);
  assert.match(alertCopy, /export function translateAdminAlertDetail/);
  assert.match(coreIndex, /translateAdminAlertTitle/);
  assert.match(coreIndex, /translateAdminAlertDetail/);
  assert.match(moderation, /translateAdminAlertTitle\(alert\.title, t\)/);
  assert.match(moderation, /translateAdminAlertDetail\(alert\.detail, t, formatNumber\)/);
  assert.doesNotMatch(moderation, /title:\s*alert\.title/);
  assert.doesNotMatch(moderation, /detail:\s*alert\.detail/);
  assert.match(operationsPulse, /translateAdminAlertTitle\(alert\.title, t\)/);
  assert.match(operationsPulse, /translateAdminAlertDetail\(alert\.detail, t, formatNumber\)/);
  assert.doesNotMatch(operationsPulse, /title:\s*alert\.title/);
  assert.doesNotMatch(operationsPulse, /detail:\s*alert\.detail/);
});

test('workspace modules avoid seeded rosters, transcript queues, campaign tasks, and transport metrics', () => {
  const users = read('packages/sdkwork-craw-chat-admin-users/src/index.tsx');
  const messages = read('packages/sdkwork-craw-chat-admin-messages/src/index.tsx');
  const conversations = read('packages/sdkwork-craw-chat-admin-conversations/src/index.tsx');
  const announcements = read('packages/sdkwork-craw-chat-admin-announcements/src/index.tsx');
  const realtime = read('packages/sdkwork-craw-chat-admin-realtime/src/index.tsx');

  assert.doesNotMatch(users, /\|\| 18/);
  assert.doesNotMatch(users, /\|\| 248/);
  assert.doesNotMatch(users, /Nina Xu/);
  assert.doesNotMatch(users, /marcus@example\.com/);

  assert.doesNotMatch(messages, /\|\| 148/);
  assert.doesNotMatch(messages, /vip-support \/ refund-dispute/);
  assert.doesNotMatch(messages, /creator-hub \/ copyright-appeal/);
  assert.doesNotMatch(messages, /Math\.max\(3, snapshot\.alerts\.length\)/);

  assert.doesNotMatch(conversations, /Customer care/);
  assert.doesNotMatch(conversations, /VIP service/);
  assert.doesNotMatch(conversations, /Operations war room/);
  assert.doesNotMatch(conversations, /Math\.max\(2, Math\.floor\(lanes\.length \/ 2\)\)/);
  assert.doesNotMatch(conversations, /routingLogs\.length \|\| 24/);

  assert.doesNotMatch(announcements, /Service upgrade notice/);
  assert.doesNotMatch(announcements, /Regional policy update/);
  assert.doesNotMatch(announcements, /\|\| 1/);

  assert.doesNotMatch(realtime, /runtimeStatuses\.length \|\| 9/);
  assert.doesNotMatch(realtime, /healthyProviders \|\| 4/);
  assert.doesNotMatch(realtime, /providerHealth\.length \|\| 6/);
  assert.doesNotMatch(realtime, /RTC edge cluster/);
  assert.doesNotMatch(realtime, /WebSocket fanout/);
});

test('automation, moderation, groups, system, and shell avoid seeded activity and synthetic minimum counters', () => {
  const automation = read('packages/sdkwork-craw-chat-admin-automation/src/index.tsx');
  const moderation = read('packages/sdkwork-craw-chat-admin-moderation/src/index.tsx');
  const groups = read('packages/sdkwork-craw-chat-admin-groups/src/index.tsx');
  const system = read('packages/sdkwork-craw-chat-admin-system/src/index.tsx');
  const appHeader = read('packages/sdkwork-craw-chat-admin-shell/src/components/AppHeader.tsx');
  const operationsPulse = read(
    'packages/sdkwork-craw-chat-admin-shell/src/components/OperationsPulseDrawer.tsx',
  );

  assert.doesNotMatch(automation, /Queue triage bot/);
  assert.doesNotMatch(automation, /Escalation bot/);
  assert.doesNotMatch(
    automation,
    /Completed queue hygiene sweep for high-priority inboxes without manual overrides\./,
  );
  assert.doesNotMatch(automation, /Paused after confidence dropped below the moderation threshold\./);
  assert.doesNotMatch(automation, /Finished announcement targeting refresh and queued a follow-up diff\./);
  assert.doesNotMatch(automation, /Math\.max\(2, snapshot\.runtimeStatuses\.length\)/);
  assert.doesNotMatch(automation, /Math\.max\(6, snapshot\.providers\.length\)/);

  assert.doesNotMatch(moderation, /Repeat abuse complaints/);
  assert.doesNotMatch(moderation, /Suspected credential sharing/);
  assert.doesNotMatch(moderation, /Math\.max\(6, snapshot\.operatorUsers\.length\)/);
  assert.doesNotMatch(moderation, /Math\.max\(2, reports\.length - 1\)/);

  assert.doesNotMatch(groups, /40 \+ index \* 12/);
  assert.doesNotMatch(groups, /Product champions/);
  assert.doesNotMatch(groups, /Creator moderation/);
  assert.doesNotMatch(groups, /Regional support leads/);
  assert.doesNotMatch(groups, /Math\.max\(3, groups\.length - 1\)/);

  assert.doesNotMatch(system, /snapshot\.runtimeStatuses\.length \|\| 5/);
  assert.doesNotMatch(system, /Math\.max\(3, snapshot\.providers\.length\)/);

  assert.doesNotMatch(appHeader, /snapshot\.alerts\.length \|\| 2/);

  assert.doesNotMatch(
    operationsPulse,
    /Shift handoff is carrying two moderation escalations and one evidence-hold thread into the next operating window\./,
  );
  assert.doesNotMatch(
    operationsPulse,
    /Realtime transport remains healthy, but VIP queues still need manual oversight before broadcast windows open\./,
  );
  assert.doesNotMatch(operationsPulse, /Math\.max\(highSeverityAlerts, 2\)/);
  assert.doesNotMatch(operationsPulse, /Math\.max\(unhealthyRuntimes \+ degradedProviders, 1\)/);
  assert.doesNotMatch(operationsPulse, /Math\.max\(fallbackRoutingLogs, 1\)/);
  assert.doesNotMatch(operationsPulse, /Math\.max\(highSeverityAlerts \+ mediumSeverityAlerts, 2\)/);
  assert.doesNotMatch(operationsPulse, /Math\.max\(unhealthyRuntimes, 1\)/);
  assert.doesNotMatch(operationsPulse, /Math\.max\(snapshot\.alerts\.length, 2\)/);
});

test('workspace operations modules expose tenant, user, and group governance', () => {
  assertModuleSurface('packages/sdkwork-craw-chat-admin-tenants/src/index.tsx', [
    /Tenant posture/,
    /Organizations|Workspace/,
  ]);
  assertModuleSurface('packages/sdkwork-craw-chat-admin-users/src/index.tsx', [
    /Device posture/,
    /Activation|Ban/,
    /Recovery review|Risk watchlist/,
  ]);
  assertModuleSurface('packages/sdkwork-craw-chat-admin-groups/src/index.tsx', [
    /Group directory/,
    /Membership posture/,
  ]);
});

test('workspace modules reuse a shared admin empty-state primitive instead of redefining local cards', () => {
  const coreIndex = read('packages/sdkwork-craw-chat-admin-core/src/index.tsx');
  const moduleSurface = read('packages/sdkwork-craw-chat-admin-core/src/moduleSurface.tsx');
  const modules = [
    'packages/sdkwork-craw-chat-admin-users/src/index.tsx',
    'packages/sdkwork-craw-chat-admin-groups/src/index.tsx',
    'packages/sdkwork-craw-chat-admin-realtime/src/index.tsx',
    'packages/sdkwork-craw-chat-admin-moderation/src/index.tsx',
    'packages/sdkwork-craw-chat-admin-system/src/index.tsx',
    'packages/sdkwork-craw-chat-admin-messages/src/index.tsx',
    'packages/sdkwork-craw-chat-admin-conversations/src/index.tsx',
    'packages/sdkwork-craw-chat-admin-announcements/src/index.tsx',
    'packages/sdkwork-craw-chat-admin-automation/src/index.tsx',
  ];

  assert.match(moduleSurface, /export function AdminEmptyState/);
  assert.match(coreIndex, /AdminEmptyState/);

  for (const relativePath of modules) {
    const source = read(relativePath);
    assert.doesNotMatch(source, /function EmptyState\(/, `${relativePath} should use AdminEmptyState`);
    assert.match(source, /AdminEmptyState/, `${relativePath} should import AdminEmptyState`);
  }
});

test('overview and shell reuse the configurable admin empty-state primitive', () => {
  const moduleSurface = read('packages/sdkwork-craw-chat-admin-core/src/moduleSurface.tsx');
  const overview = read('packages/sdkwork-craw-chat-admin-overview/src/index.tsx');
  const operationsPulse = read(
    'packages/sdkwork-craw-chat-admin-shell/src/components/OperationsPulseDrawer.tsx',
  );

  assert.match(moduleSurface, /className\?: string/);
  assert.match(overview, /AdminEmptyState/);
  assert.doesNotMatch(overview, /border-dashed border-\[var\(--admin-border-color\)\]/);
  assert.match(operationsPulse, /AdminEmptyState/);
  assert.doesNotMatch(operationsPulse, /function PulseEmptyState\(/);
});

test('workspace modules reuse a shared admin inset-card primitive for neutral detail blocks', () => {
  const coreIndex = read('packages/sdkwork-craw-chat-admin-core/src/index.tsx');
  const moduleSurface = read('packages/sdkwork-craw-chat-admin-core/src/moduleSurface.tsx');
  const modules = [
    'packages/sdkwork-craw-chat-admin-overview/src/index.tsx',
    'packages/sdkwork-craw-chat-admin-users/src/index.tsx',
    'packages/sdkwork-craw-chat-admin-messages/src/index.tsx',
    'packages/sdkwork-craw-chat-admin-conversations/src/index.tsx',
    'packages/sdkwork-craw-chat-admin-announcements/src/index.tsx',
    'packages/sdkwork-craw-chat-admin-realtime/src/index.tsx',
    'packages/sdkwork-craw-chat-admin-moderation/src/index.tsx',
    'packages/sdkwork-craw-chat-admin-groups/src/index.tsx',
    'packages/sdkwork-craw-chat-admin-system/src/index.tsx',
    'packages/sdkwork-craw-chat-admin-automation/src/index.tsx',
  ];

  assert.match(moduleSurface, /export function AdminInsetCard/);
  assert.match(coreIndex, /AdminInsetCard/);

  for (const relativePath of modules) {
    const source = read(relativePath);
    assert.match(source, /AdminInsetCard/, `${relativePath} should import AdminInsetCard`);
    assert.doesNotMatch(
      source,
      /rounded-3xl border border-\[var\(--admin-border-color\)\] bg-\[var\(--admin-content-background\)\]\/60 p-4/,
      `${relativePath} should use AdminInsetCard instead of duplicating inset card classes`,
    );
  }
});

test('workspace modules reuse a shared admin guidance-list primitive for operator posture notes', () => {
  const coreIndex = read('packages/sdkwork-craw-chat-admin-core/src/index.tsx');
  const moduleSurface = read('packages/sdkwork-craw-chat-admin-core/src/moduleSurface.tsx');
  const modules = [
    'packages/sdkwork-craw-chat-admin-tenants/src/index.tsx',
    'packages/sdkwork-craw-chat-admin-users/src/index.tsx',
    'packages/sdkwork-craw-chat-admin-groups/src/index.tsx',
    'packages/sdkwork-craw-chat-admin-messages/src/index.tsx',
    'packages/sdkwork-craw-chat-admin-conversations/src/index.tsx',
    'packages/sdkwork-craw-chat-admin-announcements/src/index.tsx',
    'packages/sdkwork-craw-chat-admin-realtime/src/index.tsx',
    'packages/sdkwork-craw-chat-admin-moderation/src/index.tsx',
    'packages/sdkwork-craw-chat-admin-system/src/index.tsx',
    'packages/sdkwork-craw-chat-admin-automation/src/index.tsx',
  ];

  assert.match(moduleSurface, /export function AdminGuidanceList/);
  assert.match(coreIndex, /AdminGuidanceList/);

  for (const relativePath of modules) {
    const source = read(relativePath);
    assert.match(source, /AdminGuidanceList/, `${relativePath} should import AdminGuidanceList`);
    assert.doesNotMatch(
      source,
      /space-y-3 text-sm text-\[var\(--admin-text-secondary\)\]/,
      `${relativePath} should use AdminGuidanceList instead of duplicating guidance list classes`,
    );
  }
});

test('workspace modules reuse a shared admin inset split-row primitive for operator list rows', () => {
  const coreIndex = read('packages/sdkwork-craw-chat-admin-core/src/index.tsx');
  const moduleSurface = read('packages/sdkwork-craw-chat-admin-core/src/moduleSurface.tsx');
  const modules = [
    'packages/sdkwork-craw-chat-admin-users/src/index.tsx',
    'packages/sdkwork-craw-chat-admin-messages/src/index.tsx',
    'packages/sdkwork-craw-chat-admin-announcements/src/index.tsx',
    'packages/sdkwork-craw-chat-admin-realtime/src/index.tsx',
    'packages/sdkwork-craw-chat-admin-groups/src/index.tsx',
    'packages/sdkwork-craw-chat-admin-system/src/index.tsx',
    'packages/sdkwork-craw-chat-admin-automation/src/index.tsx',
  ];

  assert.match(moduleSurface, /export function AdminInsetSplitRow/);
  assert.match(coreIndex, /AdminInsetSplitRow/);

  for (const relativePath of modules) {
    const source = read(relativePath);
    assert.match(source, /AdminInsetSplitRow/, `${relativePath} should import AdminInsetSplitRow`);
    assert.doesNotMatch(
      source,
      /flex flex-col gap-3 md:flex-row md:items-center md:justify-between/,
      `${relativePath} should use AdminInsetSplitRow instead of duplicating split row classes`,
    );
  }
});

test('conversation governance modules expose lifecycle, audit, and moderation', () => {
  assertModuleSurface('packages/sdkwork-craw-chat-admin-conversations/src/index.tsx', [
    /Conversation lifecycle/,
    /Handoff|Archive|Freeze/,
    /Handoff SLA|Freeze candidates/,
  ]);
  assertModuleSurface('packages/sdkwork-craw-chat-admin-messages/src/index.tsx', [
    /Message audit/,
    /Export evidence|Search transcript/,
    /Recall review|Retention guardrails/,
  ]);
  assertModuleSurface('packages/sdkwork-craw-chat-admin-moderation/src/index.tsx', [
    /Report queue/,
    /Keyword policy|Blocklist/,
    /Disposition matrix|First response SLA/,
  ]);
});

test('automation, announcements, realtime, and system stay first-class', () => {
  assertModuleSurface('packages/sdkwork-craw-chat-admin-automation/src/index.tsx', [
    /Bot registry/,
    /Automation runs/,
    /Run history|Retry queue/,
  ]);
  assertModuleSurface('packages/sdkwork-craw-chat-admin-announcements/src/index.tsx', [
    /Broadcast tasks/,
    /Delivery posture/,
  ]);
  assertModuleSurface('packages/sdkwork-craw-chat-admin-realtime/src/index.tsx', [
    /Realtime sessions/,
    /RTC posture|Gateway health/,
    /Reconnect watch|Failover window/,
  ]);
  assertModuleSurface('packages/sdkwork-craw-chat-admin-system/src/index.tsx', [
    /Protocol governance/,
    /Compatibility matrix/,
    /Protocol change gate|Rollout risks/,
  ]);
});

test('overview and realtime keep literal operator copy covered by zh-CN translations', () => {
  const i18n = read('packages/sdkwork-craw-chat-admin-core/src/i18n.tsx');
  const modules = [
    'packages/sdkwork-craw-chat-admin-overview/src/index.tsx',
    'packages/sdkwork-craw-chat-admin-realtime/src/index.tsx',
  ];

  for (const relativePath of modules) {
    const source = read(relativePath);
    const keys = collectStaticTranslationKeys(source);

    for (const key of keys) {
      assert.equal(
        i18n.includes(`'${key}':`),
        true,
        `${relativePath} missing zh-CN translation key: ${key}`,
      );
    }
  }
});

test('tenant governance registry, drawers, and dialogs keep literal operator copy covered by zh-CN translations', () => {
  const i18n = read('packages/sdkwork-craw-chat-admin-core/src/i18n.tsx');
  const modules = [
    'packages/sdkwork-craw-chat-admin-tenants/src/index.tsx',
    'packages/sdkwork-craw-chat-admin-tenants/src/page/shared.tsx',
    'packages/sdkwork-craw-chat-admin-tenants/src/page/TenantsRegistrySection.tsx',
    'packages/sdkwork-craw-chat-admin-tenants/src/page/TenantsDetailPanel.tsx',
    'packages/sdkwork-craw-chat-admin-tenants/src/page/TenantsDetailDrawer.tsx',
    'packages/sdkwork-craw-chat-admin-tenants/src/page/TenantDialog.tsx',
    'packages/sdkwork-craw-chat-admin-tenants/src/page/ProjectDialog.tsx',
    'packages/sdkwork-craw-chat-admin-tenants/src/page/ApiKeyDialog.tsx',
    'packages/sdkwork-craw-chat-admin-tenants/src/page/PlaintextApiKeyDialog.tsx',
  ];

  for (const relativePath of modules) {
    const source = read(relativePath);
    const keys = collectStaticTranslationKeys(source);

    for (const key of keys) {
      assert.equal(
        i18n.includes(`'${key}':`),
        true,
        `${relativePath} missing zh-CN translation key: ${key}`,
      );
    }
  }
});

test('remaining operator module narratives keep literal copy covered by zh-CN translations', () => {
  const i18n = read('packages/sdkwork-craw-chat-admin-core/src/i18n.tsx');
  const modules = [
    'packages/sdkwork-craw-chat-admin-announcements/src/index.tsx',
    'packages/sdkwork-craw-chat-admin-automation/src/index.tsx',
    'packages/sdkwork-craw-chat-admin-conversations/src/index.tsx',
    'packages/sdkwork-craw-chat-admin-groups/src/index.tsx',
    'packages/sdkwork-craw-chat-admin-messages/src/index.tsx',
    'packages/sdkwork-craw-chat-admin-moderation/src/index.tsx',
    'packages/sdkwork-craw-chat-admin-settings/src/WorkspaceSettings.tsx',
    'packages/sdkwork-craw-chat-admin-shell/src/application/router/AppRoutes.tsx',
    'packages/sdkwork-craw-chat-admin-system/src/index.tsx',
    'packages/sdkwork-craw-chat-admin-users/src/index.tsx',
  ];

  for (const relativePath of modules) {
    const source = read(relativePath);
    const keys = collectStaticTranslationKeys(source);

    for (const key of keys) {
      assert.equal(
        i18n.includes(`'${key}':`),
        true,
        `${relativePath} missing zh-CN translation key: ${key}`,
      );
    }
  }
});

test('settings run through an operator-grade settings center without router-admin residue', () => {
  assertModuleSurface('packages/sdkwork-craw-chat-admin-settings/src/index.tsx', [
    /from '\.\/Settings'/,
  ]);
  assertModuleSurface('packages/sdkwork-craw-chat-admin-settings/src/Settings.tsx', [
    /SettingsCenter/,
    /Operator workspace|IM operator workspace/,
    /moderation|transcript|realtime|incident/i,
    /Search shortcuts|Operations directory/,
    /Workspace Governance|Conversation Governance|System/,
  ]);

  const settingsSource = read('packages/sdkwork-craw-chat-admin-settings/src/Settings.tsx');
  const routeSource = read('packages/sdkwork-craw-chat-admin-core/src/routes.ts');
  const routeManifest = read('packages/sdkwork-craw-chat-admin-core/src/routeManifest.ts');
  const i18n = read('packages/sdkwork-craw-chat-admin-core/src/i18n.tsx');

  assert.doesNotMatch(settingsSource, /control plane|router-admin|claw-studio/i);
  assert.doesNotMatch(settingsSource, /Workspace Ops/);
  assert.doesNotMatch(routeSource, /Control Plane/);
  assert.doesNotMatch(routeSource, /Workspace Ops/);
  assert.doesNotMatch(routeManifest, /Workspace Ops/);
  assert.match(routeSource, /Operations|Workspace Governance/);
  assert.match(settingsSource, /Workspace Governance, Conversation Governance, System, Moderation, Realtime, Message Audit, and Incident Response/);
  assert.match(i18n, /'Workspace Governance':/);
  assert.match(i18n, /'Incident Response':/);
  assert.match(i18n, /'Message Audit':/);
});

test('shell command center exposes route launch and operator quick actions', () => {
  assertModuleSurface('packages/sdkwork-craw-chat-admin-shell/src/components/CommandPalette.tsx', [
    /Command center/,
    /Quick actions/,
    /Route launch/,
    /Workspace refresh/,
    /Sign out/,
  ]);
  assertModuleSurface('packages/sdkwork-craw-chat-admin-settings/src/GeneralSettings.tsx', [
    /Command center/,
    /Operations pulse/,
    /Quick actions|Search shortcuts/,
  ]);
});

test('shell operations pulse keeps incident and handoff risk visible outside overview', () => {
  assertModuleSurface('packages/sdkwork-craw-chat-admin-shell/src/components/OperationsPulseDrawer.tsx', [
    /Operations pulse/,
    /Incident watch/,
    /Shift handoff/,
    /First response SLA/,
    /Reconnect watch/,
    /Retry queue/,
    /Rollout risks/,
  ]);
});

test('shell operations pulse localizes incident severity badges instead of rendering raw enum values', () => {
  const pulse = read('packages/sdkwork-craw-chat-admin-shell/src/components/OperationsPulseDrawer.tsx');
  const i18n = read('packages/sdkwork-craw-chat-admin-core/src/i18n.tsx');

  assert.doesNotMatch(pulse, /\{incident\.severity\}/);
  assert.match(pulse, /High severity|Medium severity|Low severity/);
  assert.match(i18n, /'High severity':/);
  assert.match(i18n, /'Medium severity':/);
  assert.match(i18n, /'Low severity':/);
});

test('shell route context strip keeps module governance and continuity cues visible', () => {
  assertModuleSurface('packages/sdkwork-craw-chat-admin-shell/src/components/RouteContextStrip.tsx', [
    /Continuity cue/,
    /Required permissions/,
    /Capability tags/,
    /handoff|retention|retry|rollout|delivery|posture/i,
    /Command center|Open command center/,
    /Operations pulse|Open operations pulse/,
  ]);
  assertModuleSurface('packages/sdkwork-craw-chat-admin-settings/src/GeneralSettings.tsx', [
    /Route context strip/,
    /Continuity cue|Capability tags/,
    /Command center|Operations pulse/,
  ]);
});

test('route context strip renders operator-grade permission labels instead of raw admin permission ids', () => {
  const routeManifest = read('packages/sdkwork-craw-chat-admin-core/src/routeManifest.ts');
  const permissionCopy = read('packages/sdkwork-craw-chat-admin-core/src/adminPermissionCopy.ts');
  const coreIndex = read('packages/sdkwork-craw-chat-admin-core/src/index.tsx');
  const routeContext = read('packages/sdkwork-craw-chat-admin-shell/src/components/RouteContextStrip.tsx');
  const i18n = read('packages/sdkwork-craw-chat-admin-core/src/i18n.tsx');

  assert.match(permissionCopy, /export function resolveAdminPermissionLabel/);
  assert.match(coreIndex, /resolveAdminPermissionLabel/);
  assert.match(routeContext, /resolveAdminPermissionLabel\(permission\)/);
  assert.doesNotMatch(routeContext, /\>\s*\{permission\}\s*\</);

  for (const permission of collectRequiredPermissions(routeManifest)) {
    assert.equal(
      permissionCopy.includes(`'${permission}'`),
      true,
      `missing operator permission label mapping for ${permission}`,
    );
  }

  for (const key of [
    'Overview access',
    'Tenant registry access',
    'Tenant changes',
    'Identity roster access',
    'Identity changes',
    'Group directory access',
    'Group governance changes',
    'Announcement access',
    'Announcement changes',
    'Conversation access',
    'Conversation governance changes',
    'Transcript audit access',
    'Evidence and moderation actions',
    'Moderation queue access',
    'Moderation policy changes',
    'Automation access',
    'Automation changes',
    'Realtime posture access',
    'System governance access',
    'Settings access',
    'Settings changes',
  ]) {
    assert.equal(i18n.includes(`'${key}':`), true, `missing zh-CN permission label translation: ${key}`);
  }
});

test('zh-CN translations cover route context capability tags for every admin product module', () => {
  const routeManifest = read('packages/sdkwork-craw-chat-admin-core/src/routeManifest.ts');
  const i18n = read('packages/sdkwork-craw-chat-admin-core/src/i18n.tsx');

  for (const label of collectCapabilityTagLabels(routeManifest)) {
    assert.equal(i18n.includes(`'${label}':`), true, `missing zh-CN capability tag translation: ${label}`);
  }
});

test('shell status localizes compact runtime labels instead of hardcoding English badge text', () => {
  const shellStatus = read('packages/sdkwork-craw-chat-admin-shell/src/components/ShellStatus.tsx');
  const i18n = read('packages/sdkwork-craw-chat-admin-core/src/i18n.tsx');

  assert.match(shellStatus, /useAdminI18n/);
  assert.doesNotMatch(shellStatus, /label=\{compactStatusLabel\(status\)\}/);
  assert.match(shellStatus, /label=\{t\(compactStatusLabel\(status\)\)\}/);
  assert.match(i18n, /'Live sync':/);
  assert.match(i18n, /'Refreshing':/);
  assert.match(i18n, /'Awaiting sign-in':/);
});

test('shell status compacts operator action lifecycle states into operator-grade badge semantics', () => {
  const shellStatus = read('packages/sdkwork-craw-chat-admin-shell/src/components/ShellStatus.tsx');
  const i18n = read('packages/sdkwork-craw-chat-admin-core/src/i18n.tsx');

  assert.match(shellStatus, /'saving'/);
  assert.match(shellStatus, /'creating'/);
  assert.match(shellStatus, /'deleting'/);
  assert.match(shellStatus, /'failed'/);
  assert.match(shellStatus, /'expired'/);
  assert.match(shellStatus, /'unavailable'/);
  assert.match(shellStatus, /'not permitted'/);
  assert.match(shellStatus, /'rate limited'/);
  assert.match(shellStatus, /'saved'/);
  assert.match(shellStatus, /'created'/);
  assert.match(shellStatus, /'deleted'/);
  assert.match(shellStatus, /return 'Applying change';/);
  assert.match(shellStatus, /return 'Change applied';/);
  assert.match(shellStatus, /return 'Action required';/);
  assert.match(shellStatus, /return 'syncing';/);
  assert.match(shellStatus, /return 'live';/);
  assert.match(shellStatus, /return 'pending';/);
  assert.match(i18n, /'Applying change':/);
  assert.match(i18n, /'Change applied':/);
  assert.match(i18n, /'Action required':/);
});

test('workbench failure handling normalizes technical transport errors before they reach operator-facing shell copy', () => {
  const operatorErrorStatus = read(
    'packages/sdkwork-craw-chat-admin-core/src/operatorErrorStatus.ts',
  );
  const workbench = read('packages/sdkwork-craw-chat-admin-core/src/workbench.tsx');
  const workbenchActions = read(
    'packages/sdkwork-craw-chat-admin-core/src/workbenchActions.ts',
  );
  const i18n = read('packages/sdkwork-craw-chat-admin-core/src/i18n.tsx');

  assert.match(operatorErrorStatus, /export function resolveAdminOperatorErrorStatus/);
  assert.match(operatorErrorStatus, /AdminApiError/);
  assert.match(operatorErrorStatus, /status === 401/);
  assert.match(operatorErrorStatus, /status === 403/);
  assert.match(operatorErrorStatus, /status === 429/);
  assert.match(operatorErrorStatus, /status >= 500/);
  assert.match(operatorErrorStatus, /Admin request failed with status/);
  assert.match(operatorErrorStatus, /Admin session token not found/);
  assert.match(operatorErrorStatus, /Tauri invoke bridge is unavailable\./);
  assert.match(workbench, /resolveAdminOperatorErrorStatus/);
  assert.match(workbenchActions, /resolveAdminOperatorErrorStatus/);
  assert.doesNotMatch(workbench, /error instanceof Error \? error\.message/);
  assert.doesNotMatch(
    workbenchActions,
    /return error instanceof Error \? error\.message : fallback;/,
  );

  for (const key of [
    'Operator session expired. Sign in again.',
    'Operator access is not permitted for this action.',
    'Admin workspace is rate limited. Retry in a moment.',
    'Live admin backend is temporarily unavailable. Retry in a moment.',
    'Desktop runtime bridge is unavailable. Retry from the web workspace or restart the desktop shell.',
  ]) {
    assert.equal(i18n.includes(`'${key}':`), true, `missing zh-CN translation key: ${key}`);
  }
});

test('system rollout risks normalize runtime and provider messages before they reach operator-facing cards', () => {
  const operatorErrorStatus = read(
    'packages/sdkwork-craw-chat-admin-core/src/operatorErrorStatus.ts',
  );
  const system = read('packages/sdkwork-craw-chat-admin-system/src/index.tsx');

  assert.match(operatorErrorStatus, /export function resolveAdminOperatorMessage/);
  assert.match(system, /resolveAdminOperatorMessage\(\s*runtime\.message,/);
  assert.match(system, /resolveAdminOperatorMessage\(\s*provider\.message,/);
  assert.doesNotMatch(system, /runtime\.message\s*\?\?/);
  assert.doesNotMatch(system, /provider\.message\s*\?\?/);
});

test('system rollout risks render provider display names instead of raw provider ids', () => {
  const providerCopy = read('packages/sdkwork-craw-chat-admin-core/src/adminProviderCopy.ts');
  const coreIndex = read('packages/sdkwork-craw-chat-admin-core/src/index.tsx');
  const system = read('packages/sdkwork-craw-chat-admin-system/src/index.tsx');
  const i18n = read('packages/sdkwork-craw-chat-admin-core/src/i18n.tsx');

  assert.match(providerCopy, /export function resolveAdminProviderLabel/);
  assert.match(coreIndex, /resolveAdminProviderLabel/);
  assert.match(
    system,
    /title:\s*t\(resolveAdminProviderLabel\(provider\.provider_id,\s*snapshot\.providers\)\)/,
  );
  assert.doesNotMatch(system, /title:\s*provider\.provider_id/);
  assert.equal(
    i18n.includes("'Provider route under review':"),
    true,
    'missing zh-CN translation key: Provider route under review',
  );
});

test('automation run history renders operator-grade route labels instead of raw route keys or capabilities', () => {
  const routingCopy = read('packages/sdkwork-craw-chat-admin-core/src/adminRoutingCopy.ts');
  const coreIndex = read('packages/sdkwork-craw-chat-admin-core/src/index.tsx');
  const automation = read('packages/sdkwork-craw-chat-admin-automation/src/index.tsx');
  const i18n = read('packages/sdkwork-craw-chat-admin-core/src/i18n.tsx');

  assert.match(routingCopy, /export function resolveAdminRoutingDecisionLabel/);
  assert.match(coreIndex, /resolveAdminRoutingDecisionLabel/);
  assert.match(
    automation,
    /name:\s*t\(resolveAdminRoutingDecisionLabel\(log\.route_key,\s*log\.capability\)\)/,
  );
  assert.doesNotMatch(automation, /name:\s*log\.route_key\s*\|\|\s*log\.capability/);
  assert.equal(
    i18n.includes("'Workflow action under review':"),
    true,
    'missing zh-CN translation key: Workflow action under review',
  );
});

test('api key issuance uses operator-friendly expiry scheduling instead of raw epoch milliseconds', () => {
  const shared = read('packages/sdkwork-craw-chat-admin-tenants/src/page/shared.tsx');
  const dialog = read('packages/sdkwork-craw-chat-admin-tenants/src/page/ApiKeyDialog.tsx');
  const tenants = read('packages/sdkwork-craw-chat-admin-tenants/src/index.tsx');
  const i18n = read('packages/sdkwork-craw-chat-admin-core/src/i18n.tsx');

  assert.match(shared, /export function formatApiKeyExpiryInputValue/);
  assert.match(shared, /export function parseApiKeyExpiryInputValue/);
  assert.match(dialog, /type="datetime-local"/);
  assert.match(dialog, /label=\{t\('Access expires'\)\}/);
  assert.match(
    dialog,
    /description=\{t\('Leave blank to keep this key available until it is rotated or revoked\.'\)\}/,
  );
  assert.doesNotMatch(dialog, /Expires at \(ms\)/);
  assert.doesNotMatch(dialog, /placeholder="4102444800000"/);
  assert.match(tenants, /parseApiKeyExpiryInputValue\(apiKeyDraft\.expires_at_local\)/);
  assert.match(i18n, /'Access expires':/);
  assert.match(
    i18n,
    /'Leave blank to keep this key available until it is rotated or revoked\.':/,
  );
});

test('plaintext key handoff uses project and environment labels instead of raw ids and enum values', () => {
  const shared = read('packages/sdkwork-craw-chat-admin-tenants/src/page/shared.tsx');
  const plaintextDialog = read(
    'packages/sdkwork-craw-chat-admin-tenants/src/page/PlaintextApiKeyDialog.tsx',
  );
  const tenants = read('packages/sdkwork-craw-chat-admin-tenants/src/index.tsx');
  const i18n = read('packages/sdkwork-craw-chat-admin-core/src/i18n.tsx');

  assert.match(shared, /export function resolveRevealedApiKeySummary/);
  assert.match(shared, /export function resolveApiKeyEnvironmentLabel/);
  assert.match(
    plaintextDialog,
    /description=\{resolveRevealedApiKeySummary\(revealedApiKey,\s*projects,\s*t\)\}/,
  );
  assert.doesNotMatch(plaintextDialog, /\$\{revealedApiKey\.project_id\}/);
  assert.doesNotMatch(plaintextDialog, /\$\{revealedApiKey\.environment\}/);
  assert.match(tenants, /projects=\{snapshot\.projects\}/);
  assert.match(i18n, /'Workspace environment under review':/);
  assert.match(i18n, /'Environment under review':/);
});

test('messages and conversations use operator-grade workspace labels instead of raw project ids', () => {
  const projectCopy = read('packages/sdkwork-craw-chat-admin-core/src/adminProjectCopy.ts');
  const coreIndex = read('packages/sdkwork-craw-chat-admin-core/src/index.tsx');
  const messages = read('packages/sdkwork-craw-chat-admin-messages/src/index.tsx');
  const conversations = read('packages/sdkwork-craw-chat-admin-conversations/src/index.tsx');
  const i18n = read('packages/sdkwork-craw-chat-admin-core/src/i18n.tsx');

  assert.match(projectCopy, /export function resolveAdminProjectLabel/);
  assert.match(coreIndex, /resolveAdminProjectLabel/);
  assert.match(messages, /t\(resolveAdminProjectLabel\(record\.project_id,\s*snapshot\.projects\)\)/);
  assert.match(conversations, /t\(resolveAdminProjectLabel\(project\.project_id,\s*snapshot\.projects\)\)/);
  assert.doesNotMatch(messages, /projectById\.get\(record\.project_id\) \?\? record\.project_id/);
  assert.doesNotMatch(conversations, /projectById\.get\(project\.project_id\) \?\? project\.project_id/);
  assert.match(i18n, /'Workspace environment under review':/);
});

test('tenant and group governance surfaces label internal identifiers instead of rendering bare ids', () => {
  const groups = read('packages/sdkwork-craw-chat-admin-groups/src/index.tsx');
  const tenants = read('packages/sdkwork-craw-chat-admin-tenants/src/index.tsx');
  const tenantDetailPanel = read(
    'packages/sdkwork-craw-chat-admin-tenants/src/page/TenantsDetailPanel.tsx',
  );
  const tenantDetailDrawer = read(
    'packages/sdkwork-craw-chat-admin-tenants/src/page/TenantsDetailDrawer.tsx',
  );
  const i18n = read('packages/sdkwork-craw-chat-admin-core/src/i18n.tsx');

  assert.match(groups, /t\('Group ID: \{id\}',\s*\{\s*id:\s*group\.id\s*\}\)/);
  assert.doesNotMatch(groups, />\s*\{group\.id\}\s*<\/div>/);
  assert.match(tenants, /t\('Tenant ID: \{id\}',\s*\{\s*id:\s*tenant\.id\s*\}\)/);
  assert.doesNotMatch(tenants, />\s*\{tenant\.id\}\s*<\/div>/);
  assert.match(
    tenantDetailPanel,
    /t\('Tenant ID: \{id\}',\s*\{\s*id:\s*selectedTenant\.id\s*\}\)/,
  );
  assert.doesNotMatch(tenantDetailPanel, />\s*\{selectedTenant\.id\}\s*<\/div>/);
  assert.match(
    tenantDetailPanel,
    /t\('Workspace ID: \{id\}',\s*\{\s*id:\s*project\.id\s*\}\)/,
  );
  assert.doesNotMatch(tenantDetailPanel, />\s*\{project\.id\}\s*<\/div>/);
  assert.match(
    tenantDetailDrawer,
    /DrawerDescription>\s*\{t\('Tenant ID: \{id\}',\s*\{\s*id:\s*selectedTenant\.id\s*\}\)\}\s*<\/DrawerDescription>/,
  );
  assert.doesNotMatch(
    tenantDetailDrawer,
    /DrawerDescription>\{selectedTenant\.id\}<\/DrawerDescription>/,
  );
  assert.match(i18n, /'Group ID: \{id\}':/);
  assert.match(i18n, /'Tenant ID: \{id\}':/);
  assert.match(i18n, /'Workspace ID: \{id\}':/);
});

test('tenant provisioning dialogs and destructive confirmations label identifiers instead of parenthesized raw ids', () => {
  const shared = read('packages/sdkwork-craw-chat-admin-tenants/src/page/shared.tsx');
  const tenants = read('packages/sdkwork-craw-chat-admin-tenants/src/index.tsx');
  const apiKeyDialog = read('packages/sdkwork-craw-chat-admin-tenants/src/page/ApiKeyDialog.tsx');
  const projectDialog = read('packages/sdkwork-craw-chat-admin-tenants/src/page/ProjectDialog.tsx');

  assert.match(shared, /export function resolveTenantSelectionLabel/);
  assert.match(shared, /export function resolveProjectSelectionLabel/);
  assert.match(apiKeyDialog, /resolveTenantSelectionLabel\(tenant,\s*t\)/);
  assert.match(apiKeyDialog, /resolveProjectSelectionLabel\(project,\s*t\)/);
  assert.match(projectDialog, /resolveTenantSelectionLabel\(tenant,\s*t\)/);
  assert.match(tenants, /label:\s*resolveTenantSelectionLabel\(tenant,\s*t\)/);
  assert.match(tenants, /label:\s*resolveTenantSelectionLabel\(selectedTenant,\s*t\)/);
  assert.doesNotMatch(apiKeyDialog, /label:\s*`\$\{tenant\.name\} \(\$\{tenant\.id\}\)`/);
  assert.doesNotMatch(apiKeyDialog, /label:\s*`\$\{project\.name\} \(\$\{project\.id\}\)`/);
  assert.doesNotMatch(projectDialog, /label:\s*`\$\{tenant\.name\} \(\$\{tenant\.id\}\)`/);
  assert.doesNotMatch(tenants, /label:\s*`\$\{tenant\.name\} \(\$\{tenant\.id\}\)`/);
  assert.doesNotMatch(tenants, /label:\s*`\$\{selectedTenant\.name\} \(\$\{selectedTenant\.id\}\)`/);
});

test('tenant provisioning dialogs use operator-grade ID field labels instead of lowercase technical id copy', () => {
  const tenantDialog = read('packages/sdkwork-craw-chat-admin-tenants/src/page/TenantDialog.tsx');
  const projectDialog = read('packages/sdkwork-craw-chat-admin-tenants/src/page/ProjectDialog.tsx');
  const apiKeyDialog = read('packages/sdkwork-craw-chat-admin-tenants/src/page/ApiKeyDialog.tsx');
  const i18n = read('packages/sdkwork-craw-chat-admin-core/src/i18n.tsx');

  assert.match(tenantDialog, /label=\{t\('Tenant ID'\)\}/);
  assert.doesNotMatch(tenantDialog, /label=\{t\('Tenant id'\)\}/);
  assert.match(projectDialog, /label=\{t\('Tenant ID'\)\}/);
  assert.match(projectDialog, /label=\{t\('Workspace ID'\)\}/);
  assert.doesNotMatch(projectDialog, /label=\{t\('Tenant id'\)\}/);
  assert.doesNotMatch(projectDialog, /label=\{t\('Project id'\)\}/);
  assert.match(apiKeyDialog, /label=\{t\('Workspace'\)\}/);
  assert.doesNotMatch(apiKeyDialog, /label=\{t\('Project'\)\}/);
  assert.match(i18n, /'Tenant ID':/);
  assert.match(i18n, /'Workspace ID':/);
});

test('tenant selection labels and environment summaries avoid mojibake and raw environment enums', () => {
  const shared = read('packages/sdkwork-craw-chat-admin-tenants/src/page/shared.tsx');

  assert.match(shared, /export function resolveTenantSelectionLabel/);
  assert.match(shared, /export function resolveProjectSelectionLabel/);
  assert.match(shared, /return `\$\{primaryLabel\} \/ \$\{identifierLabel\}`;/);
  assert.doesNotMatch(shared, /è·¯/);
  assert.match(shared, /export function resolveApiKeyEnvironmentSummary/);
  assert.match(shared, /translateAdminText\(resolveApiKeyEnvironmentLabel\(key\.environment\)\)/);
  assert.doesNotMatch(shared, /new Set\(tenantApiKeys\.map\(\(key\) => key\.environment\)\)/);
});

test('tenant directory search placeholder uses operator-grade workspace guidance', () => {
  const tenants = read('packages/sdkwork-craw-chat-admin-tenants/src/index.tsx');
  const i18n = read('packages/sdkwork-craw-chat-admin-core/src/i18n.tsx');

  assert.match(
    tenants,
    /placeholder=\{t\('Search tenants, workspaces, environments, or access key labels'\)\}/,
  );
  assert.doesNotMatch(tenants, /placeholder=\{t\('tenant, project, environment, key label'\)\}/);
  assert.match(i18n, /'Search tenants, workspaces, environments, or access key labels':/);
});

test('operator error resolver falls back for technical transport failures while preserving operator-safe messages', async () => {
  const { AdminApiError, resolveAdminOperatorErrorStatus } =
    await loadOperatorErrorStatusRuntimeModule();
  const fallback = 'Failed to refresh the IM operator workspace.';

  assert.equal(
    resolveAdminOperatorErrorStatus(new Error('TypeError: Failed to fetch'), fallback),
    fallback,
  );
  assert.equal(
    resolveAdminOperatorErrorStatus(new Error('fetch failed'), fallback),
    fallback,
  );
  assert.equal(
    resolveAdminOperatorErrorStatus(new AdminApiError(400, 'socket hang up'), fallback),
    fallback,
  );
  assert.equal(
    resolveAdminOperatorErrorStatus(new Error('Tenant already exists.'), fallback),
    'Tenant already exists.',
  );
  assert.equal(
    resolveAdminOperatorErrorStatus(new AdminApiError(400, 'Tenant already exists.'), fallback),
    'Tenant already exists.',
  );
});

test('operator message resolver falls back for technical runtime strings while preserving safe rollout copy', async () => {
  const { resolveAdminOperatorMessage } = await loadOperatorErrorStatusRuntimeModule();
  const fallback = 'Runtime health is degraded and should remain behind the protocol change gate.';

  assert.equal(resolveAdminOperatorMessage('TypeError: Failed to fetch', fallback), fallback);
  assert.equal(resolveAdminOperatorMessage('socket hang up at http://localhost:8080', fallback), fallback);
  assert.equal(resolveAdminOperatorMessage('Admin request failed with status 503', fallback), fallback);
  assert.equal(
    resolveAdminOperatorMessage(
      'Provider delivery posture is degraded and should be reviewed before protocol cutovers.',
      fallback,
    ),
    'Provider delivery posture is degraded and should be reviewed before protocol cutovers.',
  );
  assert.equal(
    resolveAdminOperatorMessage('Runtime paused for planned maintenance window.', fallback),
    'Runtime paused for planned maintenance window.',
  );
});

test('tenant operator handoff localizes embedded live workspace status copy instead of interpolating raw shell status text', () => {
  const tenants = read('packages/sdkwork-craw-chat-admin-tenants/src/index.tsx');
  const i18n = read('packages/sdkwork-craw-chat-admin-core/src/i18n.tsx');

  assert.match(
    tenants,
    /t\('Live workspace status: \{status\}', \{ status: t\(status\) \}\)/,
  );
  assert.match(i18n, /'Live workspace status: \{status\}':/);
});

test('workbench action statuses stay translation-friendly and avoid embedding dynamic identifiers into shell-wide feedback', () => {
  const workbenchActions = read('packages/sdkwork-craw-chat-admin-core/src/workbenchActions.ts');

  assert.doesNotMatch(workbenchActions, /`Saving tenant \$\{input\.id\}\.\.\.`/);
  assert.doesNotMatch(workbenchActions, /`Saving project \$\{input\.id\}\.\.\.`/);
  assert.doesNotMatch(workbenchActions, /`Routing profile \$\{profile\.name\} created\.`/);
  assert.doesNotMatch(workbenchActions, /`Updating API key group \$\{input\.group_id\}\.\.\.`/);
  assert.doesNotMatch(
    workbenchActions,
    /`Gateway key issued for \$\{created\.project_id\} \(\$\{created\.environment\}\)\.`/,
  );
  assert.doesNotMatch(
    workbenchActions,
    /`Rate limit policy saved for \$\{policy\.project_id\}\.`/,
  );
  assert.doesNotMatch(workbenchActions, /`Updating gateway key \$\{input\.hashed_key\}\.\.\.`/);
  assert.doesNotMatch(
    workbenchActions,
    /`Runtime reload finished\. Active runtimes: \$\{report\.active_runtime_count\}\.`/,
  );
  assert.doesNotMatch(workbenchActions, /`Saving channel model \$\{input\.model_id\}\.\.\.`/);
  assert.doesNotMatch(
    workbenchActions,
    /`Saving model pricing for \$\{input\.model_id\}\.\.\.`/,
  );
});

test('zh-CN translations cover operator action statuses emitted by workbench actions', () => {
  const i18n = read('packages/sdkwork-craw-chat-admin-core/src/i18n.tsx');
  const keys = [
    'Failed to save operator user.',
    'Updating operator identity...',
    'Provisioning operator identity...',
    'Operator user saved.',
    'Failed to save portal user.',
    'Updating portal identity...',
    'Provisioning portal identity...',
    'Portal user saved.',
    'Failed to update operator access.',
    'Re-activating operator access...',
    'Disabling operator access...',
    'Operator access updated.',
    'Failed to update portal access.',
    'Re-activating portal access...',
    'Disabling portal access...',
    'Portal access updated.',
    'Failed to delete operator user.',
    'Deleting operator identity...',
    'Operator user deleted.',
    'Failed to delete portal user.',
    'Deleting portal identity...',
    'Portal user deleted.',
    'Failed to save tenant.',
    'Saving tenant...',
    'Tenant saved.',
    'Failed to delete tenant.',
    'Deleting tenant...',
    'Tenant deleted.',
    'Failed to save project.',
    'Saving project...',
    'Project saved.',
    'Failed to create routing profile.',
    'Creating routing profile...',
    'Routing profile created.',
    'Failed to save API key group.',
    'Updating API key group...',
    'Creating API key group...',
    'API key group updated.',
    'API key group created.',
    'Failed to update API key group status.',
    'Re-activating API key group...',
    'Disabling API key group...',
    'API key group restored.',
    'API key group disabled.',
    'Failed to delete API key group.',
    'Deleting API key group...',
    'API key group deleted.',
    'Failed to issue gateway key.',
    'Issuing gateway key...',
    'Gateway key issued.',
    'Failed to save rate limit policy.',
    'Saving rate limit policy...',
    'Rate limit policy saved.',
    'Failed to update gateway key.',
    'Updating gateway key...',
    'Gateway key updated.',
    'Restoring gateway key...',
    'Revoking gateway key...',
    'Gateway key restored.',
    'Gateway key revoked.',
    'Failed to delete gateway key.',
    'Deleting gateway key...',
    'Gateway key deleted.',
    'Failed to reload runtimes.',
    'Reloading extension runtimes...',
    'Runtime reload finished.',
    'Failed to delete project.',
    'Deleting project...',
    'Project deleted.',
    'Failed to delete channel.',
    'Deleting channel...',
    'Channel deleted.',
    'Failed to delete provider.',
    'Deleting provider...',
    'Provider deleted.',
    'Failed to save channel model.',
    'Saving channel model...',
    'Channel model saved.',
    'Failed to delete channel model.',
    'Deleting channel model...',
    'Channel model deleted.',
    'Failed to save model pricing.',
    'Saving model pricing...',
    'Model pricing saved.',
    'Failed to delete model pricing.',
    'Deleting model pricing...',
    'Model pricing deleted.',
    'Failed to save provider credential.',
    'Saving provider credential...',
    'Provider credential saved.',
    'Failed to delete provider credential.',
    'Deleting provider credential...',
    'Provider credential deleted.',
    'Failed to delete model.',
    'Deleting model...',
    'Model deleted.',
  ];

  for (const key of keys) {
    assert.equal(i18n.includes(`'${key}':`), true, `missing zh-CN translation key: ${key}`);
  }
});

test('user-facing IM copy avoids router and commerce fallback residue', () => {
  const tenants = read('packages/sdkwork-craw-chat-admin-tenants/src/index.tsx');
  const automation = read('packages/sdkwork-craw-chat-admin-automation/src/index.tsx');
  const realtime = read('packages/sdkwork-craw-chat-admin-realtime/src/index.tsx');
  const workbenchSnapshot = read('packages/sdkwork-craw-chat-admin-core/src/workbenchSnapshot.ts');

  assert.doesNotMatch(tenants, /US commerce/);
  assert.doesNotMatch(automation, /Routing bot|Completed routing sweep/);
  assert.doesNotMatch(realtime, /provider routes|route drain/i);
  assert.doesNotMatch(
    workbenchSnapshot,
    /operator control plane|messaging control plane|The control plane has no published model bindings|quota posture|providers have no credential coverage|admin control plane instead of local mock state/i,
  );
});

test('zh-CN translations cover shell continuity, settings center, and live workspace status strings', () => {
  const i18n = read('packages/sdkwork-craw-chat-admin-core/src/i18n.tsx');
  const commandPalette = read(
    'packages/sdkwork-craw-chat-admin-shell/src/components/CommandPalette.tsx',
  );

  assert.match(i18n, /'Continuity cue':/);
  assert.match(i18n, /'Capability tags':/);
  assert.match(i18n, /'Required permissions':/);
  assert.match(i18n, /'Open settings center':/);
  assert.match(i18n, /'Settings center':/);
  assert.match(i18n, /'Route context strip':/);
  assert.match(i18n, /'Choose the operator workspace language\. Dates, numbers, and shared shell copy follow this setting immediately\.':/);
  assert.match(i18n, /'Current shell posture for the IM operator workspace\.':/);
  assert.match(i18n, /'Theme mode':/);
  assert.match(i18n, /'Theme color':/);
  assert.match(i18n, /'Sidebar mode':/);
  assert.match(i18n, /'Hidden nav items':/);
  assert.match(i18n, /'Close pulse':/);
  assert.match(i18n, /'Active incidents':/);
  assert.match(i18n, /'Attention lanes':/);
  assert.match(i18n, /'Moderator handoff':/);
  assert.match(i18n, /'Realtime guardrail':/);
  assert.match(i18n, /'Automation retry':/);
  assert.match(i18n, /'Review the live incident stack, shift handoff risk, and escalation routes without leaving the current shell\.':/);
  assert.match(i18n, /'Cross-route continuity for moderation, realtime, automation, and rollout posture\.':/);
  assert.match(i18n, /'Pull open moderation, realtime, automation, and system interventions from one persistent drawer\.':/);
  assert.match(i18n, /'Route directly into the linked module with the current incident context still in view\.':/);
  assert.match(i18n, /'\{count\} active incidents require operator ownership\.':/);
  assert.match(i18n, /'IM operator':/);
  assert.match(i18n, /'No published channel capabilities are available\. Review integrations before opening live message traffic\.':/);
  assert.match(i18n, /'Connector credentials are missing':/);
  assert.match(
    i18n,
    /upstream connectors have no credential coverage\. Rotate or create credentials before opening live message traffic\./,
  );
  assert.match(i18n, /projects have exhausted their traffic budget\./);
  assert.match(i18n, /'Live admin sync is backend-backed':/);
  assert.doesNotMatch(commandPalette, /è·¯/);
});

test('zh-CN translations cover auth, sidebar, and advanced settings panels', () => {
  const i18n = read('packages/sdkwork-craw-chat-admin-core/src/i18n.tsx');

  assert.match(i18n, /'Request operator access':/);
  assert.match(i18n, /'Recover access':/);
  assert.match(i18n, /'Request operator access and enter the IM operator workspace after an existing admin provisions your identity\.':/);
  assert.match(i18n, /'Password reset links are not enabled for this workspace yet\. Continue back to sign in with your operator email\.':/);
  assert.match(i18n, /'Operator session':/);
  assert.match(i18n, /'Protected operator access':/);
  assert.match(i18n, /'This workspace currently uses operator email and password as the only live sign-in path\.':/);
  assert.match(i18n, /'Active sign-in path':/);
  assert.match(i18n, /'Password-first access':/);
  assert.match(i18n, /'Email and password sign-in is active for this workspace\.':/);
  assert.match(i18n, /'Recovery and evidence actions still require step-up review before access expands\.':/);
  assert.match(i18n, /'Mobile handoff will appear here after the workspace publishes a verified sign-in bridge\.':/);
  assert.match(i18n, /'Identity provider availability':/);
  assert.match(i18n, /'External identity providers remain disabled until workspace policy enables a live provider\.':/);
  assert.match(i18n, /'GitHub sign-in is disabled until a live provider policy is configured\.':/);
  assert.match(i18n, /'Google sign-in is disabled until a live provider policy is configured\.':/);
  assert.match(i18n, /'Disabled':/);
  assert.match(i18n, /'Local dev credentials are prefilled: \{email\} \/ \{password\}\.':/);
  assert.match(i18n, /'Already provisioned\?':/);
  assert.match(i18n, /'Need access\?':/);
  assert.match(i18n, /'Back to login':/);
  assert.match(i18n, /'Operations lead':/);
  assert.match(i18n, /'ops@workspace\.example':/);
  assert.match(i18n, /'Create a password':/);
  assert.match(i18n, /'Enter your password':/);
  assert.match(i18n, /'Open user menu':/);
  assert.match(i18n, /'Close user menu':/);
  assert.match(i18n, /'Sign in to manage the IM operator workspace':/);
  assert.match(i18n, /'Expand sidebar':/);
  assert.match(i18n, /'Collapse sidebar':/);
  assert.match(i18n, /'Theme posture':/);
  assert.match(i18n, /'Choose how the shell follows light, dark, or system appearance\.':/);
  assert.match(i18n, /'Bright shell with frosted content panes\.':/);
  assert.match(i18n, /'Low-glare operator shell tuned for long moderation and transcript review sessions\.':/);
  assert.match(i18n, /'Follow the device preference automatically\.':/);
  assert.match(i18n, /'Accent':/);
  assert.match(i18n, /'Accent preset':/);
  assert.match(i18n, /'Sidebar behavior':/);
  assert.match(i18n, /'Compact navigation':/);
  assert.match(i18n, /'Visible routes':/);
  assert.match(i18n, /'shell continuity':/);
  assert.match(i18n, /'workspace persistence':/);
  assert.match(i18n, /'Content region':/);
  assert.match(i18n, /'single workspace surface':/);
  assert.match(i18n, /'Appearance':/);
  assert.match(i18n, /'General':/);
  assert.match(i18n, /'Navigation':/);
  assert.match(i18n, /'Operator workspace':/);
  assert.match(i18n, /'Operator shell continuity':/);
  assert.match(i18n, /'IM operator settings center':/);
  assert.match(i18n, /'Search settings':/);
  assert.match(i18n, /'No settings match your search':/);
  assert.match(i18n, /'Try a different keyword or browse the navigation without a search term\.':/);
  assert.match(i18n, /'Clear filters':/);
  assert.match(i18n, /'Shell':/);
  assert.match(i18n, /'Live':/);
  assert.match(i18n, /'Refreshing live IM admin data\.\.\.':/);
  assert.match(i18n, /'Live IM operator data synchronized\.':/);
  assert.match(i18n, /'Establishing operator session\.\.\.':/);
  assert.match(i18n, /'Operator session established\. Loading the IM operator workspace\.\.\.':/);
  assert.match(i18n, /'Signed out of the IM operator workspace\.':/);
  assert.match(i18n, /'Failed to refresh the IM operator workspace\.':/);
  assert.match(i18n, /'Login failed\.':/);
  assert.match(i18n, /'Realtime posture and tenant coverage remain visible while sign-in stays on the verified password-first path\.':/);
  assert.doesNotMatch(i18n, /'Create operator access':/);
  assert.doesNotMatch(i18n, /'Already have an account\?':/);
  assert.doesNotMatch(i18n, /'No account\?':/);
  assert.doesNotMatch(i18n, /'Create account':/);
  assert.doesNotMatch(i18n, /'Workspace owner':/);
  assert.doesNotMatch(i18n, /'name@example\.com':/);
});

test('auth page renders progress and sign-out statuses as operator guidance instead of error red copy', () => {
  const auth = read('packages/sdkwork-craw-chat-admin-auth/src/index.tsx');

  assert.match(auth, /function resolveLoginFeedbackClassName\(status: string, loading: boolean\)/);
  assert.match(auth, /normalized\.includes\('signed out'\)/);
  assert.match(
    auth,
    /mode === 'login'[\s\S]*resolveLoginFeedbackClassName\(status, loading\)[\s\S]*'text-zinc-500 dark:text-zinc-400'/,
  );
  assert.doesNotMatch(
    auth,
    /mode === 'login' \? 'text-rose-500' : 'text-zinc-500 dark:text-zinc-400'/,
  );
});

test('auth page only presents truthful sign-in capabilities and avoids fake qr or social login entrypoints', () => {
  const auth = read('packages/sdkwork-craw-chat-admin-auth/src/index.tsx');

  assert.match(auth, /Protected operator access/);
  assert.match(auth, /Password-first access/);
  assert.match(auth, /Identity provider availability/);
  assert.doesNotMatch(auth, /QR login/);
  assert.doesNotMatch(auth, /Open the SDKWork app and scan this code to continue without typing credentials while the operator command post stays protected\./);
  assert.doesNotMatch(auth, /Open app to scan/);
  assert.doesNotMatch(auth, /Continue with/);
  assert.doesNotMatch(auth, /const SSO_NOTICE/);
  assert.doesNotMatch(auth, /setFeedback\(t\(SSO_NOTICE\)\)/);
});

test('auth page frames onboarding as operator provisioning instead of self-service account creation', () => {
  const auth = read('packages/sdkwork-craw-chat-admin-auth/src/index.tsx');

  assert.match(auth, /title: 'Request operator access'/);
  assert.match(auth, /Need access\?/);
  assert.match(auth, /Already provisioned\?/);
  assert.match(auth, /placeholder=\{t\('Operations lead'\)\}/);
  assert.match(auth, /placeholder=\{t\('ops@workspace\.example'\)\}/);
  assert.match(auth, /Operator account requests stay inside the IM operator workspace\. Ask an existing admin to review and provision \{name\} access from Identity\./);
  assert.doesNotMatch(auth, /title: 'Create operator access'/);
  assert.doesNotMatch(auth, /No account\?/);
  assert.doesNotMatch(auth, /Already have an account\?/);
  assert.doesNotMatch(auth, /Create account/);
  assert.doesNotMatch(auth, /Workspace owner/);
  assert.doesNotMatch(auth, /name@example\.com/);
});

test('identity governance uses Identity as the operator-facing module label instead of Users', () => {
  const usersPage = read('packages/sdkwork-craw-chat-admin-users/src/index.tsx');
  const routes = read('packages/sdkwork-craw-chat-admin-core/src/routes.ts');
  const routeManifest = read('packages/sdkwork-craw-chat-admin-core/src/routeManifest.ts');
  const i18n = read('packages/sdkwork-craw-chat-admin-core/src/i18n.tsx');
  const auth = read('packages/sdkwork-craw-chat-admin-auth/src/index.tsx');

  assert.match(usersPage, /title=\{t\('Identity'\)\}/);
  assert.doesNotMatch(usersPage, /title=\{t\('Users'\)\}/);
  assert.match(routes, /key: 'users'[\s\S]*label: 'Identity'[\s\S]*eyebrow: 'Identity'/);
  assert.doesNotMatch(routes, /key: 'users'[\s\S]*label: 'Users'/);
  assert.match(routeManifest, /moduleId: 'sdkwork-craw-chat-admin-users'[\s\S]*displayName: 'Identity'/);
  assert.doesNotMatch(routeManifest, /moduleId: 'sdkwork-craw-chat-admin-users'[\s\S]*displayName: 'Users'/);
  assert.match(auth, /review and provision \{name\} access from Identity\./);
  assert.match(i18n, /^\s*'Identity':/m);
  assert.doesNotMatch(i18n, /^\s*'Users':/m);
});

test('tenant governance uses workspace-first operator labels instead of raw project copy', () => {
  const overview = read('packages/sdkwork-craw-chat-admin-overview/src/index.tsx');
  const tenants = read('packages/sdkwork-craw-chat-admin-tenants/src/index.tsx');
  const registrySection = read(
    'packages/sdkwork-craw-chat-admin-tenants/src/page/TenantsRegistrySection.tsx',
  );
  const projectDialog = read('packages/sdkwork-craw-chat-admin-tenants/src/page/ProjectDialog.tsx');
  const detailPanel = read('packages/sdkwork-craw-chat-admin-tenants/src/page/TenantsDetailPanel.tsx');
  const detailDrawer = read('packages/sdkwork-craw-chat-admin-tenants/src/page/TenantsDetailDrawer.tsx');
  const i18n = read('packages/sdkwork-craw-chat-admin-core/src/i18n.tsx');

  assert.match(overview, /\{count\} active tenant workspaces are contributing to the current operator load\./);
  assert.match(tenants, /\{count\} active tenant workspaces are contributing to the current operator load\./);
  assert.match(tenants, /\{t\('New workspace'\)\}/);
  assert.match(registrySection, /\{t\('New workspace'\)\}/);
  assert.match(registrySection, /\{count\} workspaces/);
  assert.doesNotMatch(registrySection, /\{t\('New project'\)\}/);
  assert.doesNotMatch(registrySection, /\{count\} projects/);
  assert.match(projectDialog, /DialogTitle>\{draft\.id \? t\('Edit workspace'\) : t\('New workspace'\)\}/);
  assert.match(projectDialog, /title=\{t\('Workspace profile'\)\}/);
  assert.match(projectDialog, /label=\{t\('Workspace name'\)\}/);
  assert.match(projectDialog, /title=\{t\('Selected workspace posture'\)\}/);
  assert.match(projectDialog, /draft\.id \? t\('Save workspace'\) : t\('Create workspace'\)/);
  assert.doesNotMatch(projectDialog, /t\('Edit project'\)|t\('New project'\)|t\('Project name'\)|t\('Selected project posture'\)|t\('Save project'\)|t\('Create project'\)/);
  assert.match(detailDrawer, /\{t\('New workspace'\)\}/);
  assert.match(detailPanel, /\{t\('Workspaces'\)\}/);
  assert.match(detailPanel, /\{t\('Active workspaces, portal users, and live access key coverage for the selected tenant\.'\)\}/);
  assert.match(detailPanel, /\{t\('Workspace footprint'\)\}/);
  assert.match(detailPanel, /\{t\('\{count\} workspaces attached',/);
  assert.match(detailPanel, /\{t\('Linked workspaces'\)\}/);
  assert.doesNotMatch(detailPanel, /\{t\('Projects'\)\}|\{t\('Project footprint'\)\}|\{t\('\{count\} projects attached',|\{t\('Linked projects'\)\}/);
  assert.match(i18n, /'New workspace':/);
  assert.match(i18n, /'Edit workspace':/);
  assert.match(i18n, /'Workspace profile':/);
  assert.match(i18n, /'Workspace name':/);
  assert.match(i18n, /'Selected workspace posture':/);
  assert.match(i18n, /'Save workspace':/);
  assert.match(i18n, /'Create workspace':/);
  assert.match(i18n, /'Linked workspaces':/);
  assert.match(i18n, /'Workspace footprint':/);
  assert.match(i18n, /'Active workspaces, portal users, and live access key coverage for the selected tenant\.':/);
  assert.match(i18n, /'\{count\} workspaces attached':/);
  assert.match(i18n, /'\{count\} active tenant workspaces are contributing to the current operator load\.':/);
  assert.doesNotMatch(i18n, /^\s*'New project':/m);
  assert.doesNotMatch(i18n, /^\s*'Edit project':/m);
  assert.doesNotMatch(i18n, /^\s*'Project name':/m);
  assert.doesNotMatch(i18n, /^\s*'Selected project posture':/m);
  assert.doesNotMatch(i18n, /^\s*'Save project':/m);
  assert.doesNotMatch(i18n, /^\s*'Create project':/m);
  assert.doesNotMatch(i18n, /^\s*'Linked projects':/m);
  assert.doesNotMatch(i18n, /^\s*'Active projects, portal users, and live access key coverage for the selected tenant\.':/m);
  assert.doesNotMatch(i18n, /^\s*'\{count\} projects attached':/m);
  assert.doesNotMatch(i18n, /^\s*'\{count\} active tenant projects are contributing to the current operator load\.':/m);
});

test('tenant access governance uses access-facing labels instead of gateway jargon', () => {
  const tenants = read('packages/sdkwork-craw-chat-admin-tenants/src/index.tsx');
  const apiKeyDialog = read('packages/sdkwork-craw-chat-admin-tenants/src/page/ApiKeyDialog.tsx');
  const detailDrawer = read('packages/sdkwork-craw-chat-admin-tenants/src/page/TenantsDetailDrawer.tsx');
  const detailPanel = read('packages/sdkwork-craw-chat-admin-tenants/src/page/TenantsDetailPanel.tsx');
  const i18n = read('packages/sdkwork-craw-chat-admin-core/src/i18n.tsx');

  assert.match(tenants, /header:\s*t\('Access posture'\)/);
  assert.doesNotMatch(tenants, /header:\s*t\('Gateway posture'\)/);
  assert.match(apiKeyDialog, /title=\{t\('Access key profile'\)\}/);
  assert.doesNotMatch(apiKeyDialog, /title=\{t\('Gateway key profile'\)\}/);
  assert.match(detailDrawer, /\? t\('Access covered'\)/);
  assert.doesNotMatch(detailDrawer, /\? t\('Gateway covered'\)/);
  assert.match(detailPanel, /\{t\('Access coverage'\)\}/);
  assert.doesNotMatch(detailPanel, /\{t\('Gateway coverage'\)\}/);
  assert.match(i18n, /'Access posture':/);
  assert.match(i18n, /'Access coverage':/);
  assert.match(i18n, /'Access covered':/);
  assert.match(i18n, /'Access key profile':/);
  assert.doesNotMatch(i18n, /^\s*'Gateway posture':/m);
  assert.doesNotMatch(i18n, /^\s*'Gateway coverage':/m);
  assert.doesNotMatch(i18n, /^\s*'Gateway covered':/m);
  assert.doesNotMatch(i18n, /^\s*'Gateway key profile':/m);
});

test('tenant key issuance guidance uses workspace labels instead of workspace-environment jargon', () => {
  const tenants = read('packages/sdkwork-craw-chat-admin-tenants/src/index.tsx');
  const registrySection = read(
    'packages/sdkwork-craw-chat-admin-tenants/src/page/TenantsRegistrySection.tsx',
  );
  const apiKeyDialog = read('packages/sdkwork-craw-chat-admin-tenants/src/page/ApiKeyDialog.tsx');
  const detailPanel = read('packages/sdkwork-craw-chat-admin-tenants/src/page/TenantsDetailPanel.tsx');
  const shared = read('packages/sdkwork-craw-chat-admin-tenants/src/page/shared.tsx');
  const i18n = read('packages/sdkwork-craw-chat-admin-core/src/i18n.tsx');

  assert.match(tenants, /t\('Key issuance remains guarded until a workspace exists and coverage can be reviewed\.'\)/);
  assert.match(tenants, /detail=\{t\('Workspaces connected to the operator shell\.'\)\}/);
  assert.doesNotMatch(tenants, /t\('Key issuance remains guarded until a workspace environment exists and coverage can be reviewed\.'\)/);
  assert.doesNotMatch(tenants, /detail=\{t\('Workspace environments connected to the operator shell\.'\)\}/);
  assert.match(registrySection, /emptyDescription=\{t\([\s\S]*'Create a tenant to start assigning workspaces and issuing live access keys\.'/);
  assert.doesNotMatch(registrySection, /emptyDescription=\{t\([\s\S]*'Create a tenant to start assigning workspace environments and issuing live access keys\.'/);
  assert.match(apiKeyDialog, /description=\{t\([\s\S]*'Create a workspace for this tenant before issuing a live access key\.'/);
  assert.doesNotMatch(apiKeyDialog, /description=\{t\([\s\S]*'Create a workspace environment for this tenant before issuing a live access key\.'/);
  assert.match(detailPanel, /\{t\('The busiest workspaces stay visible here so operators can judge ownership, traffic, and access key coverage before opening another dialog\.'\)\}/);
  assert.match(detailPanel, /\{t\('No workspaces are linked to this tenant yet\.'\)\}/);
  assert.match(detailPanel, /\? t\('At least one workspace exists, so live access key issuance can proceed immediately\.'\)/);
  assert.match(detailPanel, /: t\('Issue live access keys only after at least one workspace exists for the selected tenant\.'\)/);
  assert.doesNotMatch(detailPanel, /workspace environment exists|workspace environments are linked|The busiest workspace environments stay visible/);
  assert.match(shared, /translateAdminText\('No workspaces'\)/);
  assert.doesNotMatch(shared, /translateAdminText\('No workspace environments'\)/);
  assert.match(i18n, /'No workspaces':/);
  assert.match(i18n, /'No workspaces are linked to this tenant yet\.':/);
  assert.match(i18n, /'The busiest workspaces stay visible here so operators can judge ownership, traffic, and access key coverage before opening another dialog\.':/);
  assert.match(i18n, /'Key issuance remains guarded until a workspace exists and coverage can be reviewed\.':/);
  assert.match(i18n, /'At least one workspace exists, so live access key issuance can proceed immediately\.':/);
  assert.match(i18n, /'Issue live access keys only after at least one workspace exists for the selected tenant\.':/);
  assert.match(i18n, /'Create a workspace for this tenant before issuing a live access key\.':/);
  assert.match(i18n, /'Create a tenant to start assigning workspaces and issuing live access keys\.':/);
  assert.match(i18n, /'Workspaces connected to the operator shell\.':/);
  assert.doesNotMatch(i18n, /^\s*'No workspace environments':/m);
  assert.doesNotMatch(i18n, /^\s*'No workspace environments are linked to this tenant yet\.':/m);
  assert.doesNotMatch(i18n, /^\s*'The busiest workspace environments stay visible here so operators can judge ownership, traffic, and access key coverage before opening another dialog\.':/m);
  assert.doesNotMatch(i18n, /^\s*'Key issuance remains guarded until a workspace environment exists and coverage can be reviewed\.':/m);
  assert.doesNotMatch(i18n, /^\s*'At least one workspace environment exists, so live access key issuance can proceed immediately\.':/m);
  assert.doesNotMatch(i18n, /^\s*'Issue live access keys only after at least one workspace environment exists for the selected tenant\.':/m);
  assert.doesNotMatch(i18n, /^\s*'Create a workspace environment for this tenant before issuing a live access key\.':/m);
  assert.doesNotMatch(i18n, /^\s*'Create a tenant to start assigning workspace environments and issuing live access keys\.':/m);
  assert.doesNotMatch(i18n, /^\s*'Workspace environments connected to the operator shell\.':/m);
  assert.doesNotMatch(i18n, /^\s*'\{count\} workspace environments attached\.':/m);
});

test('zh-CN translations cover overview and primary workspace governance modules', () => {
  const i18n = read('packages/sdkwork-craw-chat-admin-core/src/i18n.tsx');
  const keys = [
    'Ops',
    'Identity',
    'Membership',
    'Broadcast',
    'Lifecycle',
    'Audit',
    'Safety',
    'Bots',
    'Governance',
    'Preferences',
    'Message throughput, moderation backlog, and operator shortcuts',
    'Tenant posture, organizations, and workspace governance',
    'Portal identities, operator accounts, and device posture',
    'Group directory, channel governance, and membership posture',
    'Broadcast tasks, system notices, and delivery posture',
    'Conversation lifecycle, handoff, archive, and freeze posture',
    'Message audit, transcript search, export, and evidence review',
    'Report queues, keyword policy, and escalation governance',
    'Bot registry, automation runs, and operator workflows',
    'Realtime sessions, RTC posture, and gateway health',
    'Protocol governance, compatibility matrix, and runtime health',
    'Theme mode, theme color, and sidebar preferences',
    'IM command center',
    'Message throughput',
    'Moderation backlog',
    'Online users',
    'Hot conversations',
    'Incident watch',
    'Command board',
    'Tenant load',
    'Rolling admission volume across core messaging, audit, and automation lanes.',
    'Open reports awaiting moderator assignment, review, or escalation sign-off.',
    'Authenticated operator and portal identities currently active across the workspace.',
    'Live conversation lanes that should remain pinned for operator watch.',
    'High-volume workspace queue with active moderator attention.',
    'Cross-tenant complaints and compliance escalations.',
    'Priority concierge threads with manual handoff SLA.',
    'Creator onboarding, media approvals, and revenue notices.',
    'Abuse escalations are rolling into the next operator shift with two frozen conversations pending owner confirmation.',
    'Session health is stable and the live message plane is accepting sustained traffic.',
    'Broadcast and automation systems are available for intervention playbooks if needed.',
    'No hot conversations are reporting yet.',
    'Live conversation lanes will pin here once tenant traffic produces observable queue pressure.',
    'No incidents need handoff right now.',
    'Medium- and high-severity alerts will surface here when the workspace reports them.',
    'No tenant load is visible yet.',
    'Tenant and project demand will appear here once the workspace starts producing live traffic.',
    'Quota posture',
    'No runtime or provider health surfaces are reporting yet.',
    '{count} runtime or provider health surfaces need operator review.',
    '{count} runtime and provider health surfaces are reporting stable posture.',
    'No upstream connectors are configured yet.',
    'All configured connectors have credential coverage for live message traffic.',
    'No project budget posture is available yet.',
    'Tracked project budgets remain within their current traffic limits.',
    'Campaign posture',
    '{count} campaigns are active or scheduled with operator-visible delivery posture.',
    '{count} campaigns are configured, but none are active or scheduled.',
    'No campaigns are active or scheduled right now.',
    'Tenant governance',
    'Workspace governance',
    'Organizations',
    'Tenant posture',
    'Tenant directory',
    'Organizations currently represented in the IM estate.',
    'Workspaces connected to the operator shell.',
    'Organizations with dedicated moderation coverage: {count}',
    'Workspace handoff templates aligned for every active tenant cluster.',
    'Regional escalation paths can be reviewed before new launches are approved.',
    '{count} active tenant workspaces are contributing to the current operator load.',
    'No workspaces',
    'No workspaces are linked to this tenant yet.',
    'The busiest workspaces stay visible here so operators can judge ownership, traffic, and access key coverage before opening another dialog.',
    'Key issuance remains guarded until a workspace exists and coverage can be reviewed.',
    'At least one workspace exists, so live access key issuance can proceed immediately.',
    'Issue live access keys only after at least one workspace exists for the selected tenant.',
    'Create a workspace for this tenant before issuing a live access key.',
    'Create a tenant to start assigning workspaces and issuing live access keys.',
    'Access posture',
    'Access coverage',
    'Access covered',
    'Access key profile',
    'Identity',
    'Identity control',
    'Activation',
    'Ban',
    'Recovery review',
    'Device posture',
    'Risk watchlist',
    'Operators',
    'Portal users',
    'Identity roster',
    'Activation ready',
    'Ban or recovery review',
    'Risk watchlist keeps operator trust, device reuse, and abnormal recovery behavior on one rail.',
    'No risk signals are reported for identities yet.',
    'Dedicated device, trust, and abnormal recovery signals will appear here once the workspace exposes them.',
    'No identity roster is loaded yet.',
    'Operator and portal identities will appear here once the workspace sync returns managed users.',
    'Inactive identities require operator recovery review before access can be restored.',
    'No recovery reviews are pending.',
    'Recovery requests will appear here when inactive identities need operator approval before access is restored.',
    'Operator identities with direct access to the admin workspace.',
    'Portal accounts participating in messaging, groups, and announcements.',
    'Device posture remains visible when support, abuse, and activation workflows converge.',
    'Password reset plus device rebind requested after suspicious logout.',
    'Operator lockout follows repeated OTP drift across mobile and web.',
    'Inactive session needs manual reactivation before the next moderation shift.',
    'Shared device fingerprint overlaps with previously frozen identities.',
    'Night-shift access spike exceeds the workspace baseline.',
    'Repeated recovery attempts are landing from three network regions.',
    'Web sessions remain the dominant support channel during moderation peaks.',
    'Mobile login recovery is the main source of temporary operator escalations.',
    'Conversations',
    'Lifecycle governance',
    'Handoff',
    'Archive',
    'Freeze',
    'Priority queues',
    'Handoff SLA',
    'Conversation lifecycle',
    'Handoff queue',
    'Archived threads',
    'Lifecycle board',
    'Freeze candidates',
    'Conversations with active routing or assignment changes in progress.',
    'Conversation records already sealed for evidence or long-term archive retention.',
    'Pinned lanes keep active handoff and freeze candidates visible without leaving the primary queue view.',
    'Handoff SLA separates queues that can auto-route from those needing human assignment within the current shift.',
    'VIP and compliance queues target sub-5-minute first ownership changes.',
    'Standard support threads can wait 15 minutes before escalation is required.',
    'Unowned freeze candidates bypass the normal SLA and page trust operators immediately.',
    'Handoff keeps cross-region service, abuse review, and VIP escalations from stalling.',
    'Archive policies reduce operator noise once a conversation reaches a stable terminal state.',
    'Freeze is reserved for legal hold, fraud investigation, or severe trust and safety events.',
    'No priority queues are reporting yet.',
    'Active conversation lanes will appear here once the workspace reports lifecycle traffic.',
    'No freeze candidates are reported right now.',
    'Freeze candidates will appear here when trust, legal hold, or evidence-preservation actions are required.',
    'Transcript contains open abuse signals and should pause member writes.',
    'Cross-tenant escalation requires evidence preservation before rerouting.',
    'Legal hold requested while the handoff owner is still unresolved.',
    '{count} active conversation events.',
    'Messages',
    'Message compliance',
    'Search transcript',
    'Export evidence',
    'Recall review',
    'Export controls',
    'Retention guardrails',
    'Message audit',
    'Indexed transcripts',
    'Evidence bundles',
    'Audit queue',
    'Message audit posture across searchable, reviewable transcript slices.',
    'Transcript segments indexed for quick policy and incident review.',
    'Evidence packages prepared for export or downstream compliance tooling.',
    'Evidence exports should be deliberate, limited, and traceable to a clear operator case.',
    'Search transcript before export so evidence scope is minimized.',
    'Attach retention reason and review ticket before releasing a transcript bundle.',
    'Use evidence export only for moderation, legal hold, or enterprise compliance workflows.',
    'Retention guardrails stop operators from mixing investigation, legal hold, and routine search workflows.',
    'Default retention remains searchable but not exportable without a linked case owner.',
    'Legal hold supersedes recall requests until counsel clears the thread.',
    'Expired evidence bundles are deleted on schedule and recreated only from audited searches.',
    'Operator requested urgent withdrawal after a PII leak was confirmed.',
    'Recall requires moderator approval before the transcript can be re-exported.',
    'Cross-region deletion waits for evidence hold confirmation.',
    'Threads currently visible to operators for lifecycle review and intervention.',
    '{count} indexed message units ready for Search transcript review.',
    'No transcript slices are indexed yet.',
    'Searchable transcript evidence will appear here once message traffic is indexed.',
    'No recall reviews are pending.',
    'Recall requests will appear here when transcript withdrawal or evidence-hold decisions are reported.',
  ];

  for (const key of keys) {
    assert.equal(i18n.includes(`'${key}':`), true, `missing zh-CN translation key: ${key}`);
  }
});

test('zh-CN translations cover secondary governance, safety, and live operations modules', () => {
  const i18n = read('packages/sdkwork-craw-chat-admin-core/src/i18n.tsx');
  const keys = [
    'Community governance',
    'Groups',
    'Membership posture',
    'Group directory',
    'Managed groups',
    'Group directory volume currently visible to the operator shell.',
    'Members participating across the most active managed groups.',
    'Groups that should remain under active governance during campaign or moderation events.',
    'Membership totals are not reported by the current workspace snapshot.',
    'No group directory is loaded yet.',
    'Managed groups will appear here once the workspace sync returns channel governance data.',
    'Membership posture stays healthy when ownership, review cadence, and pinned operators remain explicit.',
    'High-density groups should keep at least one primary moderator and one backup owner.',
    'Broadcast-only groups can be separated from conversational groups when risk rises.',
    'Dormant groups should be archived before they become a blind spot for compliance review.',
    '{count} active members under review.',
    'Moderation',
    'Safety operations',
    'First response SLA',
    'Disposition matrix',
    'Keyword policy',
    'Blocklist',
    'Report queue',
    'Reviewed today',
    'Escalation ready',
    'Open reports waiting for first response, escalation, or policy confirmation.',
    'Queues with enough context to move from intake into definitive action.',
    'First response SLA makes sure every new report lands in front of a human or automated owner within the correct risk window.',
    'Self-harm, child safety, and active fraud queues target under 3 minutes.',
    'Enterprise abuse and impersonation reports target under 10 minutes.',
    'Low-confidence keyword matches can batch until the next queue sweep without blocking higher-risk work.',
    'Keyword policy should be versioned, reviewed, and distributed with clear ownership.',
    'High-risk patterns route directly into human review instead of silent suppression.',
    'Region-specific policies can be layered without breaking baseline safety coverage.',
    'Blocklist posture is reserved for deterministic abuse, fraud, or legal hold scenarios.',
    'Temporary blocks expire with case review by default to prevent silent overreach.',
    'Permanent blocks require an audit trail, owner, and reactivation pathway.',
    'Observe',
    'No reports are waiting in the moderation queue.',
    'Alert-backed moderation reports will appear here once the workspace emits safety events.',
    'Retain visibility, enrich evidence, and keep the user active while confidence is still low.',
    'Stop new writes immediately when fraud, coercion, or compromise becomes probable.',
    'Apply only when policy breach is deterministic and the audit trail is complete.',
    'Signals already triaged by an operator during the current operating window.',
    'Announcements',
    'Outbound comms',
    'Delivery posture',
    'Broadcast tasks',
    'Scheduled',
    'Active delivery',
    'Broadcast jobs currently visible to campaign and operator teams.',
    'Notices already scheduled for future delivery windows.',
    'Announcements actively moving through live delivery or acknowledgement workflows.',
    'Delivery posture keeps fan-out quality and operator confidence visible before large announcements ship.',
    'Scheduled tasks are isolated from high-risk queues until policy review is complete.',
    'Delivery failures should promote directly into operator remediation workflows.',
    'Critical notices remain pinned until every priority workspace acknowledges receipt.',
    'No broadcast tasks are active or scheduled right now.',
    'Campaign and notice delivery tasks will appear here once outbound communication is configured.',
    'Workflow orchestration',
    'Automation',
    'Automation runs',
    'Run history',
    'Retry queue',
    'Bot registry',
    'Workflow actions',
    'Running',
    'Standby',
    'Automation runs remain useful only when outcome, trigger, and owner are explicit.',
    'Policy sweeps are scheduled around broadcast and moderation spikes.',
    'Bot-triggered handoffs are logged before they reach human queues.',
    'Replay and retry controls stay operator-owned even when bots initiate the action.',
    'Retry queue isolates automation that needs human judgment before another run is allowed to touch production queues.',
    'Confidence drops, missing ownership, and stale inputs are the main retry blockers.',
    'Retries inherit the original audit trail so operators can compare pre and post states.',
    'High-risk retries remain disabled during active incidents unless an owner explicitly overrides them.',
    'No automation registry is reporting yet.',
    'Runtime-backed automation surfaces will appear here once the workspace exposes supervised runtimes.',
    'Fallback path engaged: {reason}',
    'Selection rationale: {reason}',
    'Strategy applied: {strategy}',
    'Latest routing decision is available for operator review.',
    'No automation runs are reported right now.',
    'Routing and workflow history will appear here once the workspace emits automation decisions.',
    'Bots and workflow engines currently registered for operator-supervised use.',
    'Live automation runs that are actively shaping queue posture or operator tasks.',
    'Registered capabilities that can be attached to lifecycle, moderation, or announcement flows.',
    'Completed queue hygiene sweep for high-priority inboxes without manual overrides.',
    'Paused after confidence dropped below the moderation threshold.',
    'Finished announcement targeting refresh and queued a follow-up diff.',
    'Realtime',
    'Live transport',
    'Gateway health',
    'Failover window',
    'Realtime sessions',
    'RTC posture',
    'Session monitor',
    'Offline',
    'Reconnect watch',
    '{count} live transport edges are reporting healthy realtime posture.',
    'Gateway health remains pinned because realtime incidents are usually transport incidents first.',
    'Session drains, failovers, and reconnect storms should be reviewed before broadcast launches.',
    'Failover window defines when operators can drain edges without breaking active large-room or RTC traffic.',
    'Planned cutovers are limited to low-broadcast windows with checkpoint sync confirmed.',
    'Emergency failovers require reconnect capacity and moderation transport coverage to stay green.',
    'VIP and enterprise rooms are pinned until a replacement edge reports healthy fan-out.',
    'No realtime edges are reporting degraded or recovery posture.',
    'No automation retries are waiting for human review right now.',
    '{count} gateway providers are reporting healthy delivery posture.',
    '{count} gateway providers still need transport review.',
    'Realtime sessions currently visible to the operator shell.',
    'RTC posture tracks live signaling and media-layer readiness.',
    'Gateway health shows whether live transport remains ready for operator-driven fan-out.',
    'No gateway health signals are reporting yet.',
    'Gateway coverage will appear here once transport providers start reporting live health.',
    'No live runtime sessions are reporting yet.',
    'Runtime edges will appear here once the workspace publishes transport session status.',
    'Reconnect rate is elevated after an edge rebalance; keep new room joins throttled.',
    'Session rebinds are healthy but should stay under observation during broadcast peaks.',
    'Edge posture is stable, but cross-region fallback remains armed.',
    'No reconnect risks are reported right now.',
    'Reconnect risks will appear here when runtime edges expose recovery or failover pressure.',
    'System',
    'Platform governance',
    'Runtime watch',
    'Protocol change gate',
    'Rollout risks',
    'Protocol governance',
    'Compatibility matrix',
    'Runtime health',
    'Supported',
    'Pilot',
    'Desktop pilot lag',
    'Cross-region cutover',
    'Moderation policy sync',
    '{count} runtime surfaces are currently reported into the admin workspace.',
    'Protocol changes should only ship when runtime health, compatibility, and fallback posture are all explicit.',
    'Provider health and transport health should remain green before protocol cutovers.',
    'Protocol change gate is the release checkpoint that protects clients, transport, and operators from out-of-sync semantics.',
    'Compatibility matrix, runtime health, and fallback readiness must all pass before a new wire contract opens.',
    'Rollouts are staged by client surface so operator tools never outrun end-user transports.',
    'Emergency reversions stay pre-approved with the previous protocol bundle pinned and ready.',
    'Protocol governance keeps transport, auth, and moderation contracts aligned.',
    'Compatibility matrix coverage across the major client and operator surfaces.',
    'Runtime nodes and services currently participating in the admin health model.',
    'Runtime and provider health signals will appear here once the workspace starts reporting live posture.',
    'Runtime health is degraded and should remain behind the protocol change gate.',
    'Provider delivery posture is degraded and should be reviewed before protocol cutovers.',
    'No rollout risks are reported right now.',
    'Runtime and provider risks will appear here when the workspace reports degraded protocol posture.',
    'Reconnect watch isolates the edges most likely to amplify reconnect storms so operators can drain or protect them deliberately.',
    'No shift handoff queues are waiting for assignment right now.',
    'No rollout risks currently need protocol or runtime review.',
    'Desktop clients still trail the latest protocol framing and must remain behind the gate.',
    'Route ownership migration depends on clean checkpoint replication before transport flags can flip.',
    'Keyword and recall semantics must ship alongside protocol changes to avoid operator mismatch.',
    'Desktop review consoles are reserved for heavy evidence and transcript workflows.',
  ];

  for (const key of keys) {
    assert.equal(i18n.includes(`'${key}':`), true, `missing zh-CN translation key: ${key}`);
  }
});

test('automation and announcements localize fallback status labels instead of rendering raw English state', () => {
  const automation = read('packages/sdkwork-craw-chat-admin-automation/src/index.tsx');
  const announcements = read('packages/sdkwork-craw-chat-admin-announcements/src/index.tsx');

  assert.doesNotMatch(automation, /status: 'Running'/);
  assert.doesNotMatch(automation, /status: 'Standby'/);
  assert.match(automation, /t\('Running'\)/);
  assert.match(automation, /t\('Standby'\)/);

  assert.doesNotMatch(announcements, /\{task\.status\}/);
  assert.match(announcements, /resolveAnnouncementStatusLabel/);
});
