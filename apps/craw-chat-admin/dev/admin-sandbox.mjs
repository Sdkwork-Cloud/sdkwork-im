import { randomBytes } from 'node:crypto';
import { readFileSync } from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const moduleDir = fileURLToPath(new URL('.', import.meta.url));
const sandboxSeed = JSON.parse(
  readFileSync(path.join(moduleDir, 'admin-sandbox-seed.json'), 'utf8'),
);
const jsonContentType = 'application/json; charset=utf-8';
const DEFAULT_ADMIN_SANDBOX_EMAIL = 'admin@sdkwork.local';

function clone(value) {
  return JSON.parse(JSON.stringify(value));
}

function parseBooleanEnv(value) {
  return /^(1|true|yes|on)$/i.test(String(value ?? '').trim());
}

export function isAdminSandboxEnabled(env = process.env) {
  return parseBooleanEnv(env.SDKWORK_ADMIN_SANDBOX ?? env.SDKWORK_ADMIN_SANDBOX_MODE);
}

function normalizeCredentialString(value) {
  return typeof value === 'string' ? value.trim() : '';
}

function defaultSandboxEmail(seed = sandboxSeed) {
  return normalizeCredentialString(
    seed?.authSession?.user?.email
      ?? seed?.operatorUsers?.[0]?.email
      ?? DEFAULT_ADMIN_SANDBOX_EMAIL,
  ) || DEFAULT_ADMIN_SANDBOX_EMAIL;
}

function generateSandboxPassword() {
  return randomBytes(18).toString('base64url');
}

function applySandboxCredentials(state, credentials) {
  const primaryOperatorId = state?.authSession?.user?.id ?? state?.operatorUsers?.[0]?.id ?? null;

  state.sandboxPassword = credentials.password;
  state.sandboxCredentials = { ...credentials };

  if (state.authSession?.user && primaryOperatorId && state.authSession.user.id === primaryOperatorId) {
    state.authSession.user.email = credentials.email;
  }

  for (const operatorUser of state.operatorUsers ?? []) {
    if (operatorUser?.id === primaryOperatorId) {
      operatorUser.email = credentials.email;
    }
  }
}

export function resolveAdminSandboxCredentials({
  env = process.env,
  sandboxCredentials = {},
  seed = sandboxSeed,
} = {}) {
  const explicitEmail = normalizeCredentialString(sandboxCredentials.email);
  const explicitPassword = normalizeCredentialString(sandboxCredentials.password);
  const envEmail = normalizeCredentialString(env.SDKWORK_ADMIN_SANDBOX_EMAIL);
  const envPassword = normalizeCredentialString(env.SDKWORK_ADMIN_SANDBOX_PASSWORD);

  const email = explicitEmail || envEmail || defaultSandboxEmail(seed);
  const password = explicitPassword || envPassword || generateSandboxPassword();

  return {
    email,
    password,
    source:
      explicitPassword
        ? 'explicit'
        : envPassword
          ? 'env'
          : 'generated',
  };
}

export function getAdminSandboxCredentials(state) {
  if (state?.sandboxCredentials?.email && state?.sandboxCredentials?.password) {
    return clone(state.sandboxCredentials);
  }

  return {
    email: defaultSandboxEmail(state),
    password: typeof state?.sandboxPassword === 'string' ? state.sandboxPassword : '',
    source: 'state',
  };
}

export function createAdminSandboxState(options = {}) {
  const state = clone(sandboxSeed);
  const credentials = resolveAdminSandboxCredentials({
    env: options.env,
    sandboxCredentials: options.sandboxCredentials,
    seed: state,
  });
  state.meta = {
    clockMs: Number(state.clockMs ?? Date.now()),
    sequence: 0,
  };
  applySandboxCredentials(state, credentials);
  syncProviderCredentialReadiness(state);
  return state;
}

function nextTimestamp(state) {
  state.meta.clockMs += 60_000;
  return state.meta.clockMs;
}

function nextSequence(state) {
  state.meta.sequence += 1;
  return String(state.meta.sequence).padStart(4, '0');
}

function nextId(state, prefix) {
  return `${prefix}_${nextSequence(state)}`;
}

function createTokenForUser(userId) {
  return `sandbox-admin-session-${userId}`;
}

function createClaimsForUser(userId, issuedAtMs) {
  const issuedAtSeconds = Math.floor(issuedAtMs / 1000);
  return {
    sub: userId,
    iss: 'sdkwork-admin-sandbox',
    aud: 'sdkwork-craw-chat-admin',
    exp: issuedAtSeconds + 7 * 24 * 60 * 60,
    iat: issuedAtSeconds,
  };
}

function jsonResponse(status, payload) {
  return {
    status,
    headers: {
      'content-type': jsonContentType,
    },
    body: JSON.stringify(payload),
  };
}

function emptyResponse(status = 204) {
  return {
    status,
    headers: {},
    body: '',
  };
}

function errorResponse(status, message) {
  return jsonResponse(status, {
    error: {
      message,
    },
    status,
  });
}

function parseBody(bodyText) {
  if (!bodyText?.trim()) {
    return {};
  }

  return JSON.parse(bodyText);
}

function requestPathFromUrl(url) {
  const requestUrl = new URL(url, 'http://127.0.0.1');
  const path = requestUrl.pathname.replace(/^\/api\/admin/, '') || '/';
  return path.startsWith('/') ? path : `/${path}`;
}

function requestSegments(url) {
  return requestPathFromUrl(url)
    .split('/')
    .filter(Boolean)
    .map((segment) => {
      try {
        return decodeURIComponent(segment);
      } catch {
        return segment;
      }
    });
}

function readHeader(headers, headerName) {
  const normalizedHeaderName = headerName.toLowerCase();

  if (typeof headers?.get === 'function') {
    return headers.get(headerName) ?? headers.get(normalizedHeaderName) ?? null;
  }

  if (!headers || typeof headers !== 'object') {
    return null;
  }

  for (const [key, value] of Object.entries(headers)) {
    if (key.toLowerCase() === normalizedHeaderName) {
      return Array.isArray(value) ? value[0] : value;
    }
  }

  return null;
}

function requireSessionUser(state, headers) {
  const authorization = readHeader(headers, 'authorization');
  const expectedToken = state.authSession?.token;

  if (!authorization?.startsWith('Bearer ') || !expectedToken) {
    return {
      ok: false,
      response: errorResponse(401, 'Admin sandbox session token not found.'),
    };
  }

  const token = authorization.slice('Bearer '.length).trim();
  if (!token || token !== expectedToken) {
    return {
      ok: false,
      response: errorResponse(401, 'Admin sandbox session token is invalid.'),
    };
  }

  return {
    ok: true,
    user: clone(state.authSession.user),
  };
}

function updateSessionUser(state, user) {
  const issuedAtMs = nextTimestamp(state);
  state.authSession = {
    token: createTokenForUser(user.id),
    claims: createClaimsForUser(user.id, issuedAtMs),
    user: clone(user),
  };
  return clone(state.authSession);
}

function findBy(records, predicate) {
  return records.find(predicate) ?? null;
}

function upsertBy(records, predicate, nextRecord) {
  const index = records.findIndex(predicate);
  if (index >= 0) {
    records[index] = {
      ...records[index],
      ...nextRecord,
    };
    return records[index];
  }

  records.push(nextRecord);
  return nextRecord;
}

function removeBy(records, predicate) {
  const nextRecords = records.filter((record) => !predicate(record));
  records.splice(0, records.length, ...nextRecords);
}

function syncProviderCredentialReadiness(state) {
  const providersWithCredentials = new Set(state.credentials.map((credential) => credential.provider_id));

  state.providers = state.providers.map((provider) => ({
    ...provider,
    credential_readiness: {
      ready: providersWithCredentials.has(provider.id),
      state: providersWithCredentials.has(provider.id) ? 'ready' : 'missing',
    },
  }));
}

function listResponse(state, key) {
  return jsonResponse(200, clone(state[key]));
}

function objectResponse(payload) {
  return jsonResponse(200, clone(payload));
}

function buildProviderCatalogRecord(existingProvider, input) {
  const provider = existingProvider ?? {
    integration: {
      mode: 'standard_passthrough',
      default_plugin_family: input.protocol_kind ?? 'openai',
    },
    execution: {
      binding_kind: 'provider',
      runtime: 'sandbox-runtime',
      runtime_key: input.extension_id ?? input.id,
      passthrough_protocol: input.protocol_kind ?? 'openai',
      supports_provider_adapter: true,
      supports_raw_plugin: true,
      fail_closed: true,
      route_readiness: {
        openai: {
          executable: true,
          supported: true,
        },
        anthropic: {
          executable: false,
          supported: false,
        },
        gemini: {
          executable: false,
          supported: false,
        },
      },
    },
    credential_readiness: {
      ready: false,
      state: 'missing',
    },
  };

  return {
    ...provider,
    id: input.id,
    channel_id: input.channel_id,
    extension_id: input.extension_id ?? null,
    adapter_kind: input.adapter_kind ?? provider.adapter_kind ?? 'openai-compatible',
    protocol_kind: input.protocol_kind ?? provider.protocol_kind ?? 'openai',
    base_url: input.base_url,
    display_name: input.display_name,
    channel_bindings: input.channel_bindings,
  };
}

function buildCredentialRecord(input) {
  return {
    tenant_id: input.tenant_id,
    provider_id: input.provider_id,
    key_reference: input.key_reference,
    secret_backend: 'sandbox-inmemory',
    secret_local_file: null,
    secret_keyring_service: null,
    secret_master_key_id: null,
  };
}

function buildRateLimitWindow(policy, state) {
  return {
    policy_id: policy.policy_id,
    project_id: policy.project_id,
    api_key_hash: policy.api_key_hash ?? null,
    route_key: policy.route_key ?? null,
    model_name: policy.model_name ?? null,
    requests_per_window: policy.requests_per_window,
    window_seconds: policy.window_seconds,
    burst_requests: policy.burst_requests,
    limit_requests: policy.limit_requests,
    request_count: 0,
    remaining_requests: policy.limit_requests,
    window_start_ms: nextTimestamp(state),
    window_end_ms: state.meta.clockMs + policy.window_seconds * 1000,
    updated_at_ms: state.meta.clockMs,
    enabled: policy.enabled,
    exceeded: false,
  };
}

function cascadeDeleteProject(state, projectId) {
  removeBy(state.projects, (project) => project.id === projectId);
  removeBy(state.portalUsers, (user) => user.workspace_project_id === projectId);
  removeBy(state.apiKeys, (apiKey) => apiKey.project_id === projectId);
  removeBy(state.apiKeyGroups, (group) => group.project_id === projectId);
  removeBy(state.routingProfiles, (profile) => profile.project_id === projectId);
  removeBy(state.compiledRoutingSnapshots, (snapshot) => snapshot.project_id === projectId);
  removeBy(state.rateLimitPolicies, (policy) => policy.project_id === projectId);
  removeBy(state.rateLimitWindows, (windowRecord) => windowRecord.project_id === projectId);
  removeBy(state.usageRecords, (record) => record.project_id === projectId);
  removeBy(state.billingEvents, (event) => event.project_id === projectId);
}

function cascadeDeleteTenant(state, tenantId) {
  const projectIds = state.projects
    .filter((project) => project.tenant_id === tenantId)
    .map((project) => project.id);

  removeBy(state.tenants, (tenant) => tenant.id === tenantId);
  for (const projectId of projectIds) {
    cascadeDeleteProject(state, projectId);
  }
  removeBy(state.credentials, (credential) => credential.tenant_id === tenantId);
  syncProviderCredentialReadiness(state);
}

function cascadeDeleteProvider(state, providerId) {
  removeBy(state.providers, (provider) => provider.id === providerId);
  removeBy(state.credentials, (credential) => credential.provider_id === providerId);
  removeBy(state.models, (model) => model.provider_id === providerId);
  removeBy(state.modelPrices, (price) => price.proxy_provider_id === providerId);
  removeBy(state.providerHealth, (providerHealth) => providerHealth.provider_id === providerId);
}

function cascadeDeleteModel(state, externalName, providerId) {
  removeBy(
    state.models,
    (model) => model.external_name === externalName && model.provider_id === providerId,
  );
  removeBy(state.channelModels, (model) => model.model_id === externalName);
  removeBy(state.modelPrices, (price) => price.model_id === externalName);
}

function resolveLoginUser(state, input) {
  return (
    state.operatorUsers.find((user) => user.email === input.email)
    ?? (state.authSession.user?.email === input.email ? state.authSession.user : null)
  );
}

function saveOperatorUser(state, input) {
  const createdAtMs = nextTimestamp(state);
  const operatorUser = upsertBy(
    state.operatorUsers,
    (user) => user.id === input.id,
    input.id
      ? { ...input }
      : {
          id: nextId(state, 'operator'),
          email: input.email,
          display_name: input.display_name,
          active: input.active,
          created_at_ms: createdAtMs,
        },
  );

  if (state.authSession.user?.id === operatorUser.id) {
    updateSessionUser(state, operatorUser);
  }

  return operatorUser;
}

function savePortalUser(state, input) {
  return upsertBy(
    state.portalUsers,
    (user) => user.id === input.id,
    input.id
      ? { ...input }
      : {
          id: nextId(state, 'portal'),
          email: input.email,
          display_name: input.display_name,
          workspace_tenant_id: input.workspace_tenant_id,
          workspace_project_id: input.workspace_project_id,
          active: input.active,
          created_at_ms: nextTimestamp(state),
        },
  );
}

function saveTenant(state, input) {
  return upsertBy(
    state.tenants,
    (tenant) => tenant.id === input.id,
    {
      id: input.id,
      name: input.name,
    },
  );
}

function saveProject(state, input) {
  return upsertBy(
    state.projects,
    (project) => project.id === input.id,
    {
      tenant_id: input.tenant_id,
      id: input.id,
      name: input.name,
    },
  );
}

function saveApiKeyGroup(state, input, groupId = input.group_id ?? null) {
  const resolvedGroupId = groupId ?? nextId(state, 'group');
  return upsertBy(
    state.apiKeyGroups,
    (group) => group.group_id === resolvedGroupId,
    {
      group_id: resolvedGroupId,
      tenant_id: input.tenant_id,
      project_id: input.project_id,
      environment: input.environment,
      name: input.name,
      slug: input.slug ?? input.name.toLowerCase().replace(/[^a-z0-9]+/g, '-').replace(/^-+|-+$/g, ''),
      description: input.description ?? null,
      color: input.color ?? null,
      default_capability_scope: input.default_capability_scope ?? null,
      default_accounting_mode: input.default_accounting_mode ?? null,
      default_routing_profile_id: input.default_routing_profile_id ?? null,
      active: true,
      created_at_ms:
        findBy(state.apiKeyGroups, (group) => group.group_id === resolvedGroupId)?.created_at_ms
        ?? nextTimestamp(state),
      updated_at_ms: nextTimestamp(state),
    },
  );
}

function saveRoutingProfile(state, input) {
  const profileId = input.profile_id ?? nextId(state, 'routing');
  return upsertBy(
    state.routingProfiles,
    (profile) => profile.profile_id === profileId,
    {
      profile_id: profileId,
      tenant_id: input.tenant_id,
      project_id: input.project_id,
      name: input.name,
      slug: input.slug ?? input.name.toLowerCase().replace(/[^a-z0-9]+/g, '-').replace(/^-+|-+$/g, ''),
      description: input.description ?? null,
      active: input.active ?? true,
      strategy: input.strategy ?? 'priority',
      ordered_provider_ids: input.ordered_provider_ids ?? [],
      default_provider_id: input.default_provider_id ?? null,
      max_cost: input.max_cost ?? null,
      max_latency_ms: input.max_latency_ms ?? null,
      require_healthy: input.require_healthy ?? true,
      preferred_region: input.preferred_region ?? null,
      created_at_ms:
        findBy(state.routingProfiles, (profile) => profile.profile_id === profileId)?.created_at_ms
        ?? nextTimestamp(state),
      updated_at_ms: nextTimestamp(state),
    },
  );
}

function createApiKeyRecord(state, input) {
  const sequence = nextSequence(state);
  const hashedKey = `sandbox_hash_${sequence}`;
  const plaintext = input.plaintext_key?.trim() || `sandbox_sk_${sequence}`;
  const createdAtMs = nextTimestamp(state);
  const apiKeyRecord = {
    tenant_id: input.tenant_id,
    project_id: input.project_id,
    environment: input.environment,
    hashed_key: hashedKey,
    api_key_group_id: input.api_key_group_id ?? null,
    label: input.label?.trim() || `Sandbox key ${sequence}`,
    notes: input.notes?.trim() || null,
    created_at_ms: createdAtMs,
    last_used_at_ms: null,
    expires_at_ms: input.expires_at_ms ?? null,
    active: true,
  };

  state.apiKeys.push(apiKeyRecord);

  return {
    plaintext,
    hashed: hashedKey,
    tenant_id: input.tenant_id,
    project_id: input.project_id,
    environment: input.environment,
    api_key_group_id: input.api_key_group_id ?? null,
    label: apiKeyRecord.label,
    notes: apiKeyRecord.notes,
    created_at_ms: createdAtMs,
    expires_at_ms: apiKeyRecord.expires_at_ms,
  };
}

function saveApiKeyUpdate(state, hashedKey, input) {
  return upsertBy(
    state.apiKeys,
    (apiKey) => apiKey.hashed_key === hashedKey,
    {
      ...findBy(state.apiKeys, (apiKey) => apiKey.hashed_key === hashedKey),
      hashed_key: hashedKey,
      tenant_id: input.tenant_id,
      project_id: input.project_id,
      environment: input.environment,
      label: input.label,
      notes: input.notes ?? null,
      expires_at_ms: input.expires_at_ms ?? null,
      api_key_group_id: input.api_key_group_id ?? null,
      active:
        findBy(state.apiKeys, (apiKey) => apiKey.hashed_key === hashedKey)?.active
        ?? true,
      created_at_ms:
        findBy(state.apiKeys, (apiKey) => apiKey.hashed_key === hashedKey)?.created_at_ms
        ?? nextTimestamp(state),
      last_used_at_ms:
        findBy(state.apiKeys, (apiKey) => apiKey.hashed_key === hashedKey)?.last_used_at_ms
        ?? null,
    },
  );
}

function saveChannel(state, input) {
  return upsertBy(
    state.channels,
    (channel) => channel.id === input.id,
    {
      id: input.id,
      name: input.name,
    },
  );
}

function saveProvider(state, input) {
  const provider = buildProviderCatalogRecord(
    findBy(state.providers, (providerRecord) => providerRecord.id === input.id),
    input,
  );
  upsertBy(state.providers, (providerRecord) => providerRecord.id === input.id, provider);
  syncProviderCredentialReadiness(state);
  return provider;
}

function saveCredential(state, input) {
  const credential = buildCredentialRecord(input);
  upsertBy(
    state.credentials,
    (existing) =>
      existing.tenant_id === input.tenant_id
      && existing.provider_id === input.provider_id
      && existing.key_reference === input.key_reference,
    credential,
  );
  syncProviderCredentialReadiness(state);
  return credential;
}

function saveModel(state, input) {
  return upsertBy(
    state.models,
    (model) => model.external_name === input.external_name && model.provider_id === input.provider_id,
    {
      external_name: input.external_name,
      provider_id: input.provider_id,
      capabilities: input.capabilities,
      streaming: input.streaming,
      context_window: input.context_window ?? null,
    },
  );
}

function saveChannelModel(state, input) {
  return upsertBy(
    state.channelModels,
    (model) => model.channel_id === input.channel_id && model.model_id === input.model_id,
    {
      channel_id: input.channel_id,
      model_id: input.model_id,
      model_display_name: input.model_display_name,
      capabilities: input.capabilities,
      streaming: input.streaming,
      context_window: input.context_window ?? null,
      description: input.description ?? null,
    },
  );
}

function saveModelPrice(state, input) {
  return upsertBy(
    state.modelPrices,
    (price) =>
      price.channel_id === input.channel_id
      && price.model_id === input.model_id
      && price.proxy_provider_id === input.proxy_provider_id,
    {
      ...input,
    },
  );
}

function saveMarketingCampaign(state, input) {
  return upsertBy(
    state.marketingCampaigns,
    (campaign) => campaign.marketing_campaign_id === input.marketing_campaign_id,
    {
      ...input,
      created_at_ms:
        findBy(
          state.marketingCampaigns,
          (campaign) => campaign.marketing_campaign_id === input.marketing_campaign_id,
        )?.created_at_ms ?? nextTimestamp(state),
      updated_at_ms: nextTimestamp(state),
    },
  );
}

function saveRateLimitPolicy(state, input) {
  const policy = upsertBy(
    state.rateLimitPolicies,
    (record) => record.policy_id === input.policy_id,
    {
      policy_id: input.policy_id,
      project_id: input.project_id,
      api_key_hash: input.api_key_hash ?? null,
      route_key: input.route_key ?? null,
      model_name: input.model_name ?? null,
      requests_per_window: input.requests_per_window,
      window_seconds: input.window_seconds,
      burst_requests: input.burst_requests,
      limit_requests: input.requests_per_window,
      enabled: input.enabled,
      notes: input.notes ?? null,
      created_at_ms:
        findBy(state.rateLimitPolicies, (record) => record.policy_id === input.policy_id)?.created_at_ms
        ?? nextTimestamp(state),
      updated_at_ms: nextTimestamp(state),
    },
  );

  upsertBy(
    state.rateLimitWindows,
    (windowRecord) => windowRecord.policy_id === policy.policy_id,
    buildRateLimitWindow(policy, state),
  );

  return policy;
}

function saveRuntimeReload(state, input) {
  const reloadedAtMs = nextTimestamp(state);
  state.runtimeStatuses = state.runtimeStatuses.map((runtimeStatus) => ({
    ...runtimeStatus,
    message: runtimeStatus.healthy
      ? `Sandbox reload completed at ${reloadedAtMs}.`
      : runtimeStatus.message,
  }));

  return {
    scope: input.extension_id || input.instance_id ? 'targeted' : 'workspace',
    requested_extension_id: input.extension_id ?? null,
    requested_instance_id: input.instance_id ?? null,
    resolved_extension_id: input.extension_id ?? null,
    discovered_package_count: state.runtimeStatuses.length,
    loadable_package_count: state.runtimeStatuses.length,
    active_runtime_count: state.runtimeStatuses.filter((runtimeStatus) => runtimeStatus.running).length,
    reloaded_at_ms: reloadedAtMs,
    runtime_statuses: clone(state.runtimeStatuses),
  };
}

export async function handleAdminSandboxRequest({
  state,
  method,
  url,
  headers = {},
  bodyText = '',
}) {
  const normalizedMethod = String(method ?? 'GET').toUpperCase();
  const segments = requestSegments(url);

  if (normalizedMethod === 'POST' && segments.length === 2 && segments[0] === 'auth' && segments[1] === 'login') {
    const input = parseBody(bodyText);
    const user = resolveLoginUser(state, input);

    if (!user || input.password !== state.sandboxPassword) {
      return errorResponse(401, 'Operator email or password is incorrect.');
    }

    return objectResponse(updateSessionUser(state, user));
  }

  const session = requireSessionUser(state, headers);
  if (!session.ok) {
    return session.response;
  }

  if (normalizedMethod === 'GET') {
    switch (segments.join('/')) {
      case 'auth/me': return objectResponse(state.authSession.user);
      case 'users/operators': return listResponse(state, 'operatorUsers');
      case 'users/portal': return listResponse(state, 'portalUsers');
      case 'marketing/campaigns': return listResponse(state, 'marketingCampaigns');
      case 'tenants': return listResponse(state, 'tenants');
      case 'projects': return listResponse(state, 'projects');
      case 'api-keys': return listResponse(state, 'apiKeys');
      case 'api-key-groups': return listResponse(state, 'apiKeyGroups');
      case 'routing/profiles': return listResponse(state, 'routingProfiles');
      case 'routing/snapshots': return listResponse(state, 'compiledRoutingSnapshots');
      case 'channels': return listResponse(state, 'channels');
      case 'providers': return listResponse(state, 'providers');
      case 'credentials': return listResponse(state, 'credentials');
      case 'models': return listResponse(state, 'models');
      case 'channel-models': return listResponse(state, 'channelModels');
      case 'model-prices': return listResponse(state, 'modelPrices');
      case 'usage/records': return listResponse(state, 'usageRecords');
      case 'usage/summary': return objectResponse(state.usageSummary);
      case 'billing/summary': return objectResponse(state.billingSummary);
      case 'billing/events': return listResponse(state, 'billingEvents');
      case 'billing/events/summary': return objectResponse(state.billingEventSummary);
      case 'routing/decision-logs': return listResponse(state, 'routingLogs');
      case 'gateway/rate-limit-policies': return listResponse(state, 'rateLimitPolicies');
      case 'gateway/rate-limit-windows': return listResponse(state, 'rateLimitWindows');
      case 'routing/health-snapshots': return listResponse(state, 'providerHealth');
      case 'extensions/runtime-statuses': return listResponse(state, 'runtimeStatuses');
      default: break;
    }
  }

  if (normalizedMethod === 'POST') {
    const input = parseBody(bodyText);

    switch (segments.join('/')) {
      case 'users/operators': return objectResponse(saveOperatorUser(state, input));
      case 'users/portal': return objectResponse(savePortalUser(state, input));
      case 'marketing/campaigns': return objectResponse(saveMarketingCampaign(state, input));
      case 'tenants': return objectResponse(saveTenant(state, input));
      case 'projects': return objectResponse(saveProject(state, input));
      case 'api-key-groups': return objectResponse(saveApiKeyGroup(state, input));
      case 'routing/profiles': return objectResponse(saveRoutingProfile(state, input));
      case 'api-keys': return objectResponse(createApiKeyRecord(state, input));
      case 'channels': return objectResponse(saveChannel(state, input));
      case 'providers': return objectResponse(saveProvider(state, input));
      case 'credentials': return objectResponse(saveCredential(state, input));
      case 'models': return objectResponse(saveModel(state, input));
      case 'channel-models': return objectResponse(saveChannelModel(state, input));
      case 'model-prices': return objectResponse(saveModelPrice(state, input));
      case 'gateway/rate-limit-policies': return objectResponse(saveRateLimitPolicy(state, input));
      case 'extensions/runtime-reloads': return objectResponse(saveRuntimeReload(state, input));
      default: break;
    }

    if (segments[0] === 'users' && segments[1] === 'operators' && segments[3] === 'status') {
      const operatorUser = findBy(state.operatorUsers, (user) => user.id === segments[2]);
      if (!operatorUser) return errorResponse(404, 'Operator user not found.');
      operatorUser.active = Boolean(input.active);
      if (state.authSession.user?.id === operatorUser.id) updateSessionUser(state, operatorUser);
      return objectResponse(operatorUser);
    }

    if (segments[0] === 'users' && segments[1] === 'operators' && segments[3] === 'password') {
      const operatorUser = findBy(state.operatorUsers, (user) => user.id === segments[2]);
      return operatorUser ? objectResponse(operatorUser) : errorResponse(404, 'Operator user not found.');
    }

    if (segments[0] === 'users' && segments[1] === 'portal' && segments[3] === 'status') {
      const portalUser = findBy(state.portalUsers, (user) => user.id === segments[2]);
      if (!portalUser) return errorResponse(404, 'Portal user not found.');
      portalUser.active = Boolean(input.active);
      return objectResponse(portalUser);
    }

    if (segments[0] === 'users' && segments[1] === 'portal' && segments[3] === 'password') {
      const portalUser = findBy(state.portalUsers, (user) => user.id === segments[2]);
      return portalUser ? objectResponse(portalUser) : errorResponse(404, 'Portal user not found.');
    }

    if (segments[0] === 'marketing' && segments[1] === 'campaigns' && segments[3] === 'status') {
      const campaign = findBy(
        state.marketingCampaigns,
        (marketingCampaign) => marketingCampaign.marketing_campaign_id === segments[2],
      );
      if (!campaign) return errorResponse(404, 'Marketing campaign not found.');
      campaign.status = input.status;
      campaign.updated_at_ms = nextTimestamp(state);
      return objectResponse(campaign);
    }

    if (segments[0] === 'api-key-groups' && segments[2] === 'status') {
      const apiKeyGroup = findBy(state.apiKeyGroups, (group) => group.group_id === segments[1]);
      if (!apiKeyGroup) return errorResponse(404, 'API key group not found.');
      apiKeyGroup.active = Boolean(input.active);
      apiKeyGroup.updated_at_ms = nextTimestamp(state);
      return objectResponse(apiKeyGroup);
    }

    if (segments[0] === 'api-keys' && segments[2] === 'status') {
      const apiKey = findBy(state.apiKeys, (record) => record.hashed_key === segments[1]);
      if (!apiKey) return errorResponse(404, 'API key not found.');
      apiKey.active = Boolean(input.active);
      return objectResponse(apiKey);
    }
  }

  if (normalizedMethod === 'PATCH' && segments[0] === 'api-key-groups' && segments.length === 2) {
    const input = parseBody(bodyText);
    const existingApiKeyGroup = findBy(state.apiKeyGroups, (group) => group.group_id === segments[1]);
    if (!existingApiKeyGroup) return errorResponse(404, 'API key group not found.');
    return objectResponse(saveApiKeyGroup(state, { ...existingApiKeyGroup, ...input }, segments[1]));
  }

  if (normalizedMethod === 'PUT' && segments[0] === 'api-keys' && segments.length === 2) {
    return objectResponse(saveApiKeyUpdate(state, segments[1], parseBody(bodyText)));
  }

  if (normalizedMethod === 'DELETE') {
    if (segments[0] === 'users' && segments[1] === 'operators' && segments.length === 3) {
      removeBy(state.operatorUsers, (user) => user.id === segments[2]);
      if (state.authSession.user?.id === segments[2]) updateSessionUser(state, state.operatorUsers[0] ?? sandboxSeed.authSession.user);
      return emptyResponse();
    }
    if (segments[0] === 'users' && segments[1] === 'portal' && segments.length === 3) {
      removeBy(state.portalUsers, (user) => user.id === segments[2]);
      return emptyResponse();
    }
    if (segments[0] === 'tenants' && segments.length === 2) {
      cascadeDeleteTenant(state, segments[1]);
      return emptyResponse();
    }
    if (segments[0] === 'projects' && segments.length === 2) {
      cascadeDeleteProject(state, segments[1]);
      return emptyResponse();
    }
    if (segments[0] === 'api-key-groups' && segments.length === 2) {
      removeBy(state.apiKeyGroups, (group) => group.group_id === segments[1]);
      return emptyResponse();
    }
    if (segments[0] === 'api-keys' && segments.length === 2) {
      removeBy(state.apiKeys, (apiKey) => apiKey.hashed_key === segments[1]);
      return emptyResponse();
    }
    if (segments[0] === 'channels' && segments.length === 2) {
      removeBy(state.channels, (channel) => channel.id === segments[1]);
      removeBy(state.channelModels, (channelModel) => channelModel.channel_id === segments[1]);
      removeBy(state.modelPrices, (modelPrice) => modelPrice.channel_id === segments[1]);
      return emptyResponse();
    }
    if (segments[0] === 'providers' && segments.length === 2) {
      cascadeDeleteProvider(state, segments[1]);
      syncProviderCredentialReadiness(state);
      return emptyResponse();
    }
    if (segments[0] === 'credentials' && segments[2] === 'providers' && segments[4] === 'keys' && segments.length === 6) {
      removeBy(
        state.credentials,
        (credential) =>
          credential.tenant_id === segments[1]
          && credential.provider_id === segments[3]
          && credential.key_reference === segments[5],
      );
      syncProviderCredentialReadiness(state);
      return emptyResponse();
    }
    if (segments[0] === 'models' && segments[2] === 'providers' && segments.length === 4) {
      cascadeDeleteModel(state, segments[1], segments[3]);
      return emptyResponse();
    }
    if (segments[0] === 'channel-models' && segments[2] === 'models' && segments.length === 4) {
      removeBy(state.channelModels, (channelModel) => channelModel.channel_id === segments[1] && channelModel.model_id === segments[3]);
      return emptyResponse();
    }
    if (segments[0] === 'model-prices' && segments[2] === 'models' && segments[4] === 'providers' && segments.length === 6) {
      removeBy(
        state.modelPrices,
        (modelPrice) =>
          modelPrice.channel_id === segments[1]
          && modelPrice.model_id === segments[3]
          && modelPrice.proxy_provider_id === segments[5],
      );
      return emptyResponse();
    }
  }

  return errorResponse(404, `Admin sandbox route not implemented for ${normalizedMethod} ${requestPathFromUrl(url)}.`);
}

export function createAdminSandboxMiddleware({
  state = createAdminSandboxState(),
  onSandboxCredentialsResolved,
} = {}) {
  if (typeof onSandboxCredentialsResolved === 'function') {
    onSandboxCredentialsResolved(getAdminSandboxCredentials(state));
  }

  return async function adminSandboxMiddleware(req, res, next) {
    if (!req.url?.startsWith('/api/admin')) {
      next();
      return;
    }

    try {
      const chunks = [];
      for await (const chunk of req) {
        chunks.push(Buffer.isBuffer(chunk) ? chunk : Buffer.from(chunk));
      }

      const response = await handleAdminSandboxRequest({
        state,
        method: req.method,
        url: req.url,
        headers: req.headers,
        bodyText: Buffer.concat(chunks).toString('utf8'),
      });

      res.statusCode = response.status;
      for (const [headerName, headerValue] of Object.entries(response.headers)) {
        res.setHeader(headerName, headerValue);
      }
      res.end(response.body);
    } catch (error) {
      const response = errorResponse(
        500,
        error instanceof Error ? error.message : 'Admin sandbox failed to process the request.',
      );
      res.statusCode = response.status;
      for (const [headerName, headerValue] of Object.entries(response.headers)) {
        res.setHeader(headerName, headerValue);
      }
      res.end(response.body);
    }
  };
}
