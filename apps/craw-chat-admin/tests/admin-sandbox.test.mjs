import assert from 'node:assert/strict';
import test from 'node:test';

import {
  createAdminSandboxState,
  handleAdminSandboxRequest,
  isAdminSandboxEnabled,
} from '../dev/admin-sandbox.mjs';

async function sandboxRequest(state, { method = 'GET', path, token, body }) {
  const response = await handleAdminSandboxRequest({
    state,
    method,
    url: `/api/admin${path}`,
    headers: token ? { authorization: `Bearer ${token}` } : {},
    bodyText: body ? JSON.stringify(body) : '',
  });

  return {
    ...response,
    json: response.body ? JSON.parse(response.body) : null,
  };
}

test('admin sandbox opt-in flag parser only enables explicit truthy env values', () => {
  assert.equal(isAdminSandboxEnabled({ SDKWORK_ADMIN_SANDBOX: '1' }), true);
  assert.equal(isAdminSandboxEnabled({ SDKWORK_ADMIN_SANDBOX: 'true' }), true);
  assert.equal(isAdminSandboxEnabled({ SDKWORK_ADMIN_SANDBOX: 'yes' }), true);
  assert.equal(isAdminSandboxEnabled({ SDKWORK_ADMIN_SANDBOX: '0' }), false);
  assert.equal(isAdminSandboxEnabled({}), false);
});

test('admin sandbox login establishes a session and guards protected routes', async () => {
  const state = createAdminSandboxState();

  const unauthenticated = await sandboxRequest(state, {
    path: '/auth/me',
  });
  assert.equal(unauthenticated.status, 401);

  const login = await sandboxRequest(state, {
    method: 'POST',
    path: '/auth/login',
    body: {
      email: 'admin@sdkwork.local',
      password: 'ChangeMe123!',
    },
  });
  assert.equal(login.status, 200);
  assert.equal(login.json.user.email, 'admin@sdkwork.local');
  assert.match(login.json.token, /sandbox-admin-session/);

  const me = await sandboxRequest(state, {
    path: '/auth/me',
    token: login.json.token,
  });
  assert.equal(me.status, 200);
  assert.equal(me.json.email, 'admin@sdkwork.local');
});

test('admin sandbox persists tenant, project, and api key mutations across refresh reads', async () => {
  const state = createAdminSandboxState();
  const login = await sandboxRequest(state, {
    method: 'POST',
    path: '/auth/login',
    body: {
      email: 'admin@sdkwork.local',
      password: 'ChangeMe123!',
    },
  });
  const token = login.json.token;

  await sandboxRequest(state, {
    method: 'POST',
    path: '/tenants',
    token,
    body: {
      id: 'tenant_demo',
      name: 'Demo Tenant',
    },
  });
  await sandboxRequest(state, {
    method: 'POST',
    path: '/projects',
    token,
    body: {
      tenant_id: 'tenant_demo',
      id: 'project_demo',
      name: 'Demo Project',
    },
  });
  const createdKey = await sandboxRequest(state, {
    method: 'POST',
    path: '/api-keys',
    token,
    body: {
      tenant_id: 'tenant_demo',
      project_id: 'project_demo',
      environment: 'staging',
      label: 'Demo key',
    },
  });

  const tenants = await sandboxRequest(state, {
    path: '/tenants',
    token,
  });
  const projects = await sandboxRequest(state, {
    path: '/projects',
    token,
  });
  const apiKeys = await sandboxRequest(state, {
    path: '/api-keys',
    token,
  });

  assert.equal(tenants.status, 200);
  assert.equal(projects.status, 200);
  assert.equal(apiKeys.status, 200);
  assert.equal(tenants.json.some((tenant) => tenant.id === 'tenant_demo'), true);
  assert.equal(projects.json.some((project) => project.id === 'project_demo'), true);
  assert.equal(
    apiKeys.json.some((apiKey) => apiKey.hashed_key === createdKey.json.hashed),
    true,
  );
});

test('admin sandbox creates a matching window for new rate limit policies', async () => {
  const state = createAdminSandboxState();
  const login = await sandboxRequest(state, {
    method: 'POST',
    path: '/auth/login',
    body: {
      email: 'admin@sdkwork.local',
      password: 'ChangeMe123!',
    },
  });
  const token = login.json.token;

  const createdPolicy = await sandboxRequest(state, {
    method: 'POST',
    path: '/gateway/rate-limit-policies',
    token,
    body: {
      policy_id: 'policy_demo',
      project_id: 'project_support_cn',
      requests_per_window: 300,
      window_seconds: 60,
      burst_requests: 30,
      enabled: true,
    },
  });
  const windows = await sandboxRequest(state, {
    path: '/gateway/rate-limit-windows',
    token,
  });

  assert.equal(createdPolicy.status, 200);
  assert.equal(createdPolicy.json.policy_id, 'policy_demo');
  assert.equal(
    windows.json.some((windowRecord) => windowRecord.policy_id === 'policy_demo'),
    true,
  );
});
