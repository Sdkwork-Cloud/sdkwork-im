import assert from 'node:assert/strict';

import { ImAdminSdkClient } from '../dist/index.js';

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
  const storage = { marker: 'storage' };

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
    storage,
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
    setTokenManager(manager) {
      calls.push({ method: 'setTokenManager', manager });
      return backendClient;
    },
  };

  return { backendClient, calls };
}

async function testCreateFactoryAndTokenHelpers() {
  const { backendClient, calls } = createBackendClientStub();
  const sdk = await ImAdminSdkClient.create({ backendClient });

  sdk.setAuthToken('auth-token');

  assert.deepEqual(calls.slice(-1), [
    { method: 'setAuthToken', token: 'auth-token' },
  ]);
}

function testConstructorSurface() {
  const { backendClient } = createBackendClientStub();
  const sdk = new ImAdminSdkClient({ backendClient });

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
  assert.equal(sdk.storage, backendClient.storage);
}

await testCreateFactoryAndTokenHelpers();
testConstructorSurface();

console.log('im-admin composed sdk smoke tests passed');
