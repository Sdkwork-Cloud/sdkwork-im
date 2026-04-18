import assert from 'node:assert/strict';
import path from 'node:path';
import { pathToFileURL } from 'node:url';
import { test } from 'node:test';

import { resolvePortalAppRoot } from './helpers/portal-paths.mjs';

const root = resolvePortalAppRoot(import.meta.url);

test('portal scaffold and route manifest align to the reference architecture', async () => {
  const routeManifestModulePath = path.join(
    root,
    'packages/craw-chat-portal-core/src/application/router/routeManifest.js',
  );
  const portalApiModulePath = path.join(
    root,
    'packages/craw-chat-portal-portal-api/src/index.js',
  );

  const requiredFiles = [
    path.join(root, 'package.json'),
    path.join(root, 'index.html'),
    path.join(root, 'vite.config.js'),
    path.join(root, 'src/main.js'),
    path.join(root, 'src/App.js'),
    routeManifestModulePath,
    portalApiModulePath,
    path.join(root, 'packages/craw-chat-portal-home/src/repository/index.js'),
    path.join(root, 'packages/craw-chat-portal-home/src/services/index.js'),
    path.join(root, 'packages/craw-chat-portal-auth/src/repository/index.js'),
    path.join(root, 'packages/craw-chat-portal-auth/src/services/index.js'),
  ];

  for (const filePath of requiredFiles) {
    assert.ok(await import('node:fs/promises').then((fs) => fs.access(filePath).then(() => true, () => false)), `${filePath} should exist`);
  }

  const routeManifestModule = await import(pathToFileURL(routeManifestModulePath).href);
  assert.equal(routeManifestModule.portalProductModules.length, 6);

  const routeKeys = routeManifestModule.portalRouteManifest.map((entry) => entry.key);
  assert.deepEqual(routeKeys, [
    'dashboard',
    'conversations',
    'realtime',
    'media',
    'automation',
    'governance',
  ]);

  const groupsByRoute = Object.fromEntries(
    routeManifestModule.portalRouteManifest.map((entry) => [entry.key, entry.productModule.navigation.group]),
  );

  assert.deepEqual(groupsByRoute, {
    dashboard: 'operations',
    conversations: 'operations',
    realtime: 'operations',
    media: 'experience',
    automation: 'enablement',
    governance: 'governance',
  });
});
