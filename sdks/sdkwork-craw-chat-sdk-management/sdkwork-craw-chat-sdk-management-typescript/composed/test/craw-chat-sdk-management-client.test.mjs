import assert from 'node:assert/strict';

import { CrawChatSdkManagementClient } from '../dist/index.js';

function createBackendClientStub() {
  const calls = [];
  const auth = { marker: 'auth' };
  const users = { marker: 'users' };
  const marketing = { marker: 'marketing' };
  const tenants = { marker: 'tenants' };
  const access = { marker: 'access' };
  const routing = { marker: 'routing' };
  const catalog = { marker: 'catalog' };
  const usage = { marker: 'usage' };
  const billing = { marker: 'billing' };
  const operations = { marker: 'operations' };

  const backendClient = {
    auth,
    users,
    marketing,
    tenants,
    access,
    routing,
    catalog,
    usage,
    billing,
    operations,
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

async function testCreateFactoryAndTokenHelpers() {
  const { backendClient, calls } = createBackendClientStub();
  const sdk = await CrawChatSdkManagementClient.create({ backendClient });

  sdk.setAuthToken('auth-token');
  sdk.setAccessToken('access-token');
  sdk.setApiKey('api-key');

  assert.deepEqual(calls.slice(-3), [
    { method: 'setAuthToken', token: 'auth-token' },
    { method: 'setAccessToken', token: 'access-token' },
    { method: 'setApiKey', apiKey: 'api-key' },
  ]);
}

function testConstructorSurface() {
  const { backendClient } = createBackendClientStub();
  const sdk = new CrawChatSdkManagementClient({ backendClient });

  assert.equal(sdk.backendClient, backendClient);
  assert.equal(sdk.auth, backendClient.auth);
  assert.equal(sdk.users, backendClient.users);
  assert.equal(sdk.marketing, backendClient.marketing);
  assert.equal(sdk.tenants, backendClient.tenants);
  assert.equal(sdk.access, backendClient.access);
  assert.equal(sdk.routing, backendClient.routing);
  assert.equal(sdk.catalog, backendClient.catalog);
  assert.equal(sdk.usage, backendClient.usage);
  assert.equal(sdk.billing, backendClient.billing);
  assert.equal(sdk.operations, backendClient.operations);
}

await testCreateFactoryAndTokenHelpers();
testConstructorSurface();

console.log('craw-chat management composed sdk smoke tests passed');
