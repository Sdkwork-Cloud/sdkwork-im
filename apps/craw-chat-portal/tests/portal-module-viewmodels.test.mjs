import assert from 'node:assert/strict';
import path from 'node:path';
import { pathToFileURL } from 'node:url';
import { test } from 'node:test';

import './helpers/installMockPortalDefaultDataSource.mjs';
import {
  resolvePortalAppRoot,
  resolvePortalPackagesRoot,
} from './helpers/portal-paths.mjs';

const root = resolvePortalPackagesRoot(import.meta.url);
const appRoot = resolvePortalAppRoot(import.meta.url);

async function load(moduleRelativePath) {
  return import(pathToFileURL(path.join(root, moduleRelativePath)).href);
}

test('tenant IM modules expose rich view models for the console', async () => {
  const home = await load('craw-chat-portal-home/src/services/index.js');
  const auth = await load('craw-chat-portal-auth/src/services/index.js');
  const dashboard = await load('craw-chat-portal-dashboard/src/services/index.js');
  const conversations = await load('craw-chat-portal-conversations/src/services/index.js');
  const realtime = await load('craw-chat-portal-realtime/src/services/index.js');
  const media = await load('craw-chat-portal-media/src/services/index.js');
  const automation = await load('craw-chat-portal-automation/src/services/index.js');
  const governance = await load('craw-chat-portal-governance/src/services/index.js');

  const dashboardViewModel = await dashboard.buildPortalDashboardViewModel();
  const conversationsViewModel = await conversations.buildPortalConversationsViewModel();
  const realtimeViewModel = await realtime.buildPortalRealtimeViewModel();
  const mediaViewModel = await media.buildPortalMediaViewModel();
  const automationViewModel = await automation.buildPortalAutomationViewModel();
  const governanceViewModel = await governance.buildPortalGovernanceViewModel();
  const homeViewModel = await home.buildPortalHomeViewModel();
  const authViewModel = await auth.buildPortalAuthViewModel();

  assert.equal(homeViewModel.pillars.length, 3);
  assert.equal(authViewModel.details.length, 3);
  assert.equal(dashboardViewModel.hero.kpis.length, 4);
  assert.ok(conversationsViewModel.pipeline.length >= 3);
  assert.ok(realtimeViewModel.subscriptions.length >= 3);
  assert.ok(mediaViewModel.assets.length >= 3);
  assert.ok(automationViewModel.executions.length >= 3);
  assert.ok(governanceViewModel.auditRecords.length >= 3);
});

test('tenant IM module builders reject malformed portal snapshots instead of forwarding bad payloads into the shell', async () => {
  const dataSourceModule = await import(
    pathToFileURL(
      path.join(
        appRoot,
        'packages/craw-chat-portal-portal-api/src/runtime/activeDataSource.js',
      ),
    ).href
  );

  const home = await load('craw-chat-portal-home/src/services/index.js');
  const auth = await load('craw-chat-portal-auth/src/services/index.js');
  const dashboard = await load('craw-chat-portal-dashboard/src/services/index.js');
  const conversations = await load('craw-chat-portal-conversations/src/services/index.js');
  const realtime = await load('craw-chat-portal-realtime/src/services/index.js');
  const media = await load('craw-chat-portal-media/src/services/index.js');
  const automation = await load('craw-chat-portal-automation/src/services/index.js');
  const governance = await load('craw-chat-portal-governance/src/services/index.js');

  const originalDataSource =
    typeof dataSourceModule.getActivePortalDataSource === 'function'
      ? dataSourceModule.getActivePortalDataSource()
      : dataSourceModule.activePortalDataSource;

  const malformedCases = [
    {
      method: 'getPortalHome',
      builder: () => home.buildPortalHomeViewModel(),
      payload: { hero: { title: 'Portal', description: 'missing eyebrow' }, pillars: [] },
    },
    {
      method: 'getPortalAuth',
      builder: () => auth.buildPortalAuthViewModel(),
      payload: {
        eyebrow: 'Demo Tenant Access',
        description: 'missing title',
        details: [],
        primaryActionLabel: 'Sign in',
        secondaryActionLabel: 'Back',
      },
    },
    {
      method: 'getPortalDashboard',
      builder: () => dashboard.buildPortalDashboardViewModel(),
      payload: { hero: { title: 'Dashboard only' } },
    },
    {
      method: 'getPortalConversationsBoard',
      builder: () => conversations.buildPortalConversationsViewModel(),
      payload: { hero: { title: 'Conversations', description: 'bad' }, pipeline: 'broken' },
    },
    {
      method: 'getPortalRealtimeBoard',
      builder: () => realtime.buildPortalRealtimeViewModel(),
      payload: { hero: { title: 'Realtime', description: 'bad' }, posture: [] },
    },
    {
      method: 'getPortalMediaBoard',
      builder: () => media.buildPortalMediaViewModel(),
      payload: { hero: { title: 'Media', description: 'bad' }, assets: [] },
    },
    {
      method: 'getPortalAutomationBoard',
      builder: () => automation.buildPortalAutomationViewModel(),
      payload: { hero: { title: 'Automation', description: 'bad' }, summary: [] },
    },
    {
      method: 'getPortalGovernanceBoard',
      builder: () => governance.buildPortalGovernanceViewModel(),
      payload: { hero: { title: 'Governance', description: 'bad' }, auditRecords: [] },
    },
  ];

  try {
    for (const malformedCase of malformedCases) {
      dataSourceModule.setActivePortalDataSource({
        ...originalDataSource,
        async [malformedCase.method]() {
          return malformedCase.payload;
        },
      });

      await assert.rejects(
        () => malformedCase.builder(),
        {
          name: 'TypeError',
        },
      );
    }
  } finally {
    dataSourceModule.resetActivePortalDataSource();
  }
});

test('tenant IM module builders reject malformed snapshot items that would otherwise leak undefined cells into the shell', async () => {
  const dataSourceModule = await import(
    pathToFileURL(
      path.join(
        appRoot,
        'packages/craw-chat-portal-portal-api/src/runtime/activeDataSource.js',
      ),
    ).href
  );

  const auth = await load('craw-chat-portal-auth/src/services/index.js');
  const dashboard = await load('craw-chat-portal-dashboard/src/services/index.js');
  const conversations = await load('craw-chat-portal-conversations/src/services/index.js');
  const realtime = await load('craw-chat-portal-realtime/src/services/index.js');
  const media = await load('craw-chat-portal-media/src/services/index.js');
  const automation = await load('craw-chat-portal-automation/src/services/index.js');
  const governance = await load('craw-chat-portal-governance/src/services/index.js');

  const originalDataSource =
    typeof dataSourceModule.getActivePortalDataSource === 'function'
      ? dataSourceModule.getActivePortalDataSource()
      : dataSourceModule.activePortalDataSource;

  const malformedItemCases = [
    {
      method: 'getPortalAuth',
      builder: () => auth.buildPortalAuthViewModel(),
      mutate(snapshot) {
        return {
          ...snapshot,
          details: [{ label: 'Workspace' }],
        };
      },
    },
    {
      method: 'getPortalDashboard',
      builder: () => dashboard.buildPortalDashboardViewModel(),
      mutate(snapshot) {
        return {
          ...snapshot,
          hero: {
            ...snapshot.hero,
            kpis: [{ label: 'Messages today' }],
          },
        };
      },
    },
    {
      method: 'getPortalConversationsBoard',
      builder: () => conversations.buildPortalConversationsViewModel(),
      mutate(snapshot) {
        return {
          ...snapshot,
          handoffs: [{ conversation: 'Refund / #IM-4821' }],
        };
      },
    },
    {
      method: 'getPortalRealtimeBoard',
      builder: () => realtime.buildPortalRealtimeViewModel(),
      mutate(snapshot) {
        return {
          ...snapshot,
          devices: [{ owner: 'Store Ops 01', device: 'iPhone 15 Pro' }],
        };
      },
    },
    {
      method: 'getPortalMediaBoard',
      builder: () => media.buildPortalMediaViewModel(),
      mutate(snapshot) {
        return {
          ...snapshot,
          assets: [{ asset: 'refund-proof-240409.zip', type: 'Archive' }],
        };
      },
    },
    {
      method: 'getPortalAutomationBoard',
      builder: () => automation.buildPortalAutomationViewModel(),
      mutate(snapshot) {
        return {
          ...snapshot,
          notifications: [{ task: 'VIP fallback SMS', channel: 'SMS' }],
        };
      },
    },
    {
      method: 'getPortalGovernanceBoard',
      builder: () => governance.buildPortalGovernanceViewModel(),
      mutate(snapshot) {
        return {
          ...snapshot,
          auditRecords: [{ action: 'Provider binding preview', actor: 'ops.lead' }],
        };
      },
    },
  ];

  try {
    for (const malformedCase of malformedItemCases) {
      dataSourceModule.setActivePortalDataSource({
        ...originalDataSource,
        async [malformedCase.method]() {
          const snapshot = await originalDataSource[malformedCase.method]();
          return malformedCase.mutate(snapshot);
        },
      });

      await assert.rejects(
        () => malformedCase.builder(),
        {
          name: 'TypeError',
        },
      );
    }
  } finally {
    dataSourceModule.resetActivePortalDataSource();
  }
});
