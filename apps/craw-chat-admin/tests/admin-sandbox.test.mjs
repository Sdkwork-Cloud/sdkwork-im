import assert from 'node:assert/strict';
import test from 'node:test';

import {
  createAdminSandboxState,
  getAdminSandboxCredentials,
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

test('admin sandbox accepts explicitly provisioned login credentials instead of relying on a fixed repo password', async () => {
  const state = createAdminSandboxState({
    sandboxCredentials: {
      email: 'ops-sandbox@example.invalid',
      password: 'Sandbox#Custom2026',
    },
  });

  const legacyLogin = await sandboxRequest(state, {
    method: 'POST',
    path: '/auth/login',
    body: {
      email: 'admin@sdkwork.local',
      password: 'ChangeMe123!',
    },
  });
  assert.equal(legacyLogin.status, 401);

  const credentials = getAdminSandboxCredentials(state);
  assert.equal(credentials.email, 'ops-sandbox@example.invalid');
  assert.equal(credentials.password, 'Sandbox#Custom2026');

  const login = await sandboxRequest(state, {
    method: 'POST',
    path: '/auth/login',
    body: {
      email: 'ops-sandbox@example.invalid',
      password: 'Sandbox#Custom2026',
    },
  });
  assert.equal(login.status, 200);
  assert.equal(login.json.user.email, 'ops-sandbox@example.invalid');
});

test('admin sandbox manages storage providers, overrides, effective resolution, and redacted secrets', async () => {
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

  const providers = await sandboxRequest(state, {
    path: '/storage/providers',
    token,
  });
  assert.equal(providers.status, 200);
  assert.equal(
    providers.json.some((schema) => schema.providerPluginId === 'object-storage-aws'),
    true,
  );
  assert.equal(
    providers.json.some((schema) => schema.providerPluginId === 'object-storage-microsoft'),
    true,
  );
  const googleProvider = providers.json.find((schema) => schema.providerPluginId === 'object-storage-google');
  assert.deepEqual(
    googleProvider.credentialFields.find((field) => field.name === 'serviceAccountJson').credentialModes,
    ['service-account-json'],
  );
  assert.deepEqual(
    googleProvider.credentialFields.find((field) => field.name === 'interoperabilitySecretKey').credentialModes,
    ['interoperability-key'],
  );
  const awsProvider = providers.json.find((schema) => schema.providerPluginId === 'object-storage-aws');
  assert.equal(
    awsProvider.credentialFields.some((field) => field.name === 'roleArn'),
    true,
  );

  const savedGlobal = await sandboxRequest(state, {
    method: 'POST',
    path: '/storage/config',
    token,
    body: {
      binding: {
        providerPluginId: 'object-storage-aws',
        enabled: true,
      },
      config: {
        bucketOrContainer: 'global-assets',
        region: 'us-east-1',
        endpoint: 'https://s3.amazonaws.com',
        publicBaseUrl: 'https://cdn.global.example',
      },
      secret: {
        credentialMode: 'access-key-pair',
        encryptedSecretPayload: JSON.stringify({
          accessKeyId: 'global-access-key',
          secretAccessKey: 'global-secret-key',
        }),
        secretFingerprint: 'fp-global-aws',
      },
    },
  });
  assert.equal(savedGlobal.status, 200);
  assert.equal(savedGlobal.json.binding.providerPluginId, 'object-storage-aws');
  assert.equal(
    JSON.stringify(savedGlobal.json).includes('global-secret-key'),
    false,
  );

  const savedTenant = await sandboxRequest(state, {
    method: 'POST',
    path: '/storage/config/tenants/tenant_northstar',
    token,
    body: {
      binding: {
        providerPluginId: 'object-storage-google',
        enabled: true,
      },
      config: {
        bucketOrContainer: 'tenant-northstar-assets',
        region: 'asia-east1',
        publicBaseUrl: 'https://cdn.tenant.example',
      },
      secret: {
        credentialMode: 'service-account-json',
        encryptedSecretPayload: JSON.stringify({
          serviceAccountJson: {
            client_email: 'tenant@sdkwork.local',
          },
        }),
        secretFingerprint: 'fp-tenant-google',
      },
    },
  });
  assert.equal(savedTenant.status, 200);
  assert.equal(savedTenant.json.scope.kind, 'tenant');
  assert.equal(savedTenant.json.scope.scopeId, 'tenant_northstar');

  const effectiveTenant = await sandboxRequest(state, {
    path: '/storage/effective/tenants/tenant_northstar',
    token,
  });
  assert.equal(effectiveTenant.status, 200);
  assert.equal(effectiveTenant.json.resolvedScope.kind, 'tenant');
  assert.equal(effectiveTenant.json.binding.providerPluginId, 'object-storage-google');
  assert.equal(
    effectiveTenant.json.secret.secretFingerprint,
    'fp-tenant-google',
  );

  const audit = await sandboxRequest(state, {
    path: '/storage/audit',
    token,
  });
  assert.equal(audit.status, 200);
  assert.equal(audit.json.length >= 2, true);

  const removedTenant = await sandboxRequest(state, {
    method: 'DELETE',
    path: '/storage/config/tenants/tenant_northstar',
    token,
  });
  assert.equal(removedTenant.status, 204);

  const effectiveFallback = await sandboxRequest(state, {
    path: '/storage/effective/tenants/tenant_northstar',
    token,
  });
  assert.equal(effectiveFallback.status, 200);
  assert.equal(effectiveFallback.json.resolvedScope.kind, 'global');
  assert.equal(effectiveFallback.json.binding.providerPluginId, 'object-storage-aws');
});

test('admin sandbox preserves existing storage secret when the same provider updates config without a replacement secret', async () => {
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

  const initialSave = await sandboxRequest(state, {
    method: 'POST',
    path: '/storage/config',
    token,
    body: {
      binding: {
        providerPluginId: 'object-storage-aws',
        enabled: true,
      },
      config: {
        bucketOrContainer: 'global-assets',
        region: 'us-east-1',
      },
      secret: {
        credentialMode: 'access-key-pair',
        encryptedSecretPayload: JSON.stringify({
          accessKeyId: 'global-access-key',
          secretAccessKey: 'global-secret-key',
        }),
        secretFingerprint: 'fp-global-aws',
      },
    },
  });
  assert.equal(initialSave.status, 200);

  const updatedConfig = await sandboxRequest(state, {
    method: 'POST',
    path: '/storage/config',
    token,
    body: {
      binding: {
        providerPluginId: 'object-storage-aws',
        enabled: true,
      },
      config: {
        bucketOrContainer: 'global-assets-v2',
        region: 'us-west-2',
        publicBaseUrl: 'https://cdn.global.example',
      },
    },
  });
  assert.equal(updatedConfig.status, 200);
  assert.equal(updatedConfig.json.config.bucketOrContainer, 'global-assets-v2');
  assert.equal(updatedConfig.json.secret.secretFingerprint, 'fp-global-aws');

  const validation = await sandboxRequest(state, {
    method: 'POST',
    path: '/storage/validate',
    token,
    body: {},
  });
  assert.equal(validation.status, 200);
  assert.equal(validation.json.status, 'healthy');
  assert.equal(validation.json.stage, 'presign');
});

test('admin sandbox rejects storage credential submissions that omit required fields for the selected mode', async () => {
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

  const invalidSave = await sandboxRequest(state, {
    method: 'POST',
    path: '/storage/config',
    token,
    body: {
      binding: {
        providerPluginId: 'object-storage-google',
        enabled: true,
      },
      config: {
        bucketOrContainer: 'tenant-assets',
      },
      secret: {
        credentialMode: 'interoperability-key',
        encryptedSecretPayload: JSON.stringify({
          interoperabilityAccessKey: 'interop-access-key',
        }),
      },
    },
  });

  assert.equal(invalidSave.status, 400);
  assert.match(invalidSave.json.error.message, /Interoperability Secret Key is required/);
});

test('admin sandbox rejects unsupported credential modes for a provider', async () => {
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

  const invalidSave = await sandboxRequest(state, {
    method: 'POST',
    path: '/storage/config',
    token,
    body: {
      binding: {
        providerPluginId: 'object-storage-google',
        enabled: true,
      },
      config: {
        bucketOrContainer: 'tenant-assets',
      },
      secret: {
        credentialMode: 'role-assumption',
        encryptedSecretPayload: JSON.stringify({
          roleArn: 'arn:aws:iam::123456789012:role/sdkwork',
        }),
      },
    },
  });

  assert.equal(invalidSave.status, 400);
  assert.equal(
    invalidSave.json.error.message,
    'Storage provider object-storage-google does not support credential mode role-assumption.',
  );
});
