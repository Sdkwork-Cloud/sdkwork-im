import assert from 'node:assert/strict';

import { CrawChatSdkAdminClient } from '../dist/index.js';

function createBackendClientStub() {
  const calls = [];
  const protocol = {
    async listProtocols() {
      calls.push({ method: 'protocol.listProtocols' });
      return { items: [] };
    },
  };
  const providers = {
    async listProviders() {
      calls.push({ method: 'providers.listProviders' });
      return { items: [] };
    },
  };
  const cluster = {
    async listNodes() {
      calls.push({ method: 'cluster.listNodes' });
      return { items: [] };
    },
  };
  const social = {
    async listFriendLinks() {
      calls.push({ method: 'social.listFriendLinks' });
      return { items: [] };
    },
  };
  const system = {
    async getOverview() {
      calls.push({ method: 'system.getOverview' });
      return { status: 'ok' };
    },
  };

  const backendClient = {
    protocol,
    providers,
    cluster,
    social,
    system,
    http: {
      async request() {
        calls.push({ method: 'http.request' });
        return {};
      },
    },
    setAuthToken(token) {
      calls.push({ method: 'setAuthToken', token });
      return backendClient;
    },
    setAccessToken(token) {
      calls.push({ method: 'setAccessToken', token });
      return backendClient;
    },
    setApiKey(apiKey) {
      calls.push({ method: 'setApiKey', apiKey });
      return backendClient;
    },
    setTokenManager(manager) {
      calls.push({ method: 'setTokenManager', manager });
      return backendClient;
    },
  };

  return { backendClient, calls };
}

function testConstructorSurface() {
  const { backendClient } = createBackendClientStub();
  const sdk = new CrawChatSdkAdminClient({ backendClient });

  assert.equal(sdk.backendClient, backendClient);
  assert.equal(sdk.protocol, backendClient.protocol);
  assert.equal(sdk.providers, backendClient.providers);
  assert.equal(sdk.cluster, backendClient.cluster);
  assert.equal(sdk.social, backendClient.social);
  assert.equal(sdk.system, backendClient.system);
}

async function testCreateFactoryAndTokenHelpers() {
  const { backendClient, calls } = createBackendClientStub();
  const sdk = await CrawChatSdkAdminClient.create({ backendClient });

  sdk.setAuthToken('auth-token');
  sdk.setAccessToken('access-token');
  sdk.setApiKey('api-key');

  assert.deepEqual(calls.slice(-3), [
    { method: 'setAuthToken', token: 'auth-token' },
    { method: 'setAccessToken', token: 'access-token' },
    { method: 'setApiKey', apiKey: 'api-key' },
  ]);
}

await testCreateFactoryAndTokenHelpers();
testConstructorSurface();

console.log('craw-chat admin composed sdk smoke tests passed');
