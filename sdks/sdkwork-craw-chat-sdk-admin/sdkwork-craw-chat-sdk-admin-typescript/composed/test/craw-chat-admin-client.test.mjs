import assert from 'node:assert/strict';

import { CrawChatAdminSdkClient } from '../dist/index.js';

function createBackendClientStub() {
  const calls = [];
  const backendClient = {
    meta: {
      async getHealthz() {
        calls.push({ method: 'meta.getHealthz' });
        return { status: 'ok' };
      },
    },
    protocol: {
      async getProtocolGovernance() {
        calls.push({ method: 'protocol.getProtocolGovernance' });
        return { protocolVersion: 'v1' };
      },
      async getProtocolRegistry() {
        calls.push({ method: 'protocol.getProtocolRegistry' });
        return { bindings: ['ws'] };
      },
    },
    providers: {
      async getProviderBindings(params) {
        calls.push({ method: 'providers.getProviderBindings', params });
        return { items: [] };
      },
      async getProviderRegistry() {
        calls.push({ method: 'providers.getProviderRegistry' });
        return { providers: [] };
      },
    },
    social: {
      async getFriendshipSnapshot(friendshipId) {
        calls.push({ method: 'social.getFriendshipSnapshot', friendshipId });
        return { friendshipId };
      },
    },
    socialRuntime: {
      async getPendingSharedChannelSyncInventory() {
        calls.push({ method: 'socialRuntime.getPendingSharedChannelSyncInventory' });
        return { items: [] };
      },
    },
    nodes: {
      async activateNode(nodeId) {
        calls.push({ method: 'nodes.activateNode', nodeId });
        return { nodeId, state: 'active' };
      },
    },
    setAuthToken(token) {
      calls.push({ method: 'setAuthToken', token });
      return backendClient;
    },
  };

  return { backendClient, calls };
}

async function testConstructedClientExposesSemanticModules() {
  const { backendClient, calls } = createBackendClientStub();
  const sdk = new CrawChatAdminSdkClient({ backendClient });

  const registry = await sdk.protocol.getRegistry();
  const activation = await sdk.nodes.activate('node-sh-01');

  assert.deepEqual(registry, { bindings: ['ws'] });
  assert.deepEqual(activation, { nodeId: 'node-sh-01', state: 'active' });
  assert.deepEqual(calls, [
    { method: 'protocol.getProtocolRegistry' },
    { method: 'nodes.activateNode', nodeId: 'node-sh-01' },
  ]);
}

async function testCreateAcceptsInjectedBackendClient() {
  const { backendClient } = createBackendClientStub();
  const sdk = await CrawChatAdminSdkClient.create({ backendClient });

  assert.equal(sdk.backendClient, backendClient);
}

async function testCreateAcceptsFlatRuntimeOptions() {
  const sdk = await CrawChatAdminSdkClient.create({
    baseUrl: 'https://admin.example.test',
    authToken: 'token-1',
  });

  assert.ok(sdk.backendClient);
  assert.ok(sdk.protocol);
  assert.ok(sdk.providers);
  assert.ok(sdk.socialRuntime);
}

async function testSetAuthTokenDelegatesToBackendClient() {
  const { backendClient, calls } = createBackendClientStub();
  const sdk = new CrawChatAdminSdkClient({ backendClient });

  const sameSdk = sdk.setAuthToken('token-2');

  assert.equal(sameSdk, sdk);
  assert.deepEqual(calls, [{ method: 'setAuthToken', token: 'token-2' }]);
}

await testConstructedClientExposesSemanticModules();
await testCreateAcceptsInjectedBackendClient();
await testCreateAcceptsFlatRuntimeOptions();
await testSetAuthTokenDelegatesToBackendClient();
