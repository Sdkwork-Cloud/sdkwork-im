import type {
  AdminAuthSession,
  AdminSessionUser,
  ApiKeyGroupRecord,
  BillingEventRecord,
  BillingEventSummary,
  BillingSummary,
  ChannelRecord,
  ChannelModelRecord,
  CompiledRoutingSnapshotRecord,
  CreatedGatewayApiKey,
  CredentialRecord,
  GatewayApiKeyRecord,
  MarketingCampaignRecord,
  MarketingCampaignStatus,
  ModelCatalogRecord,
  ModelPriceRecord,
  OperatorUserRecord,
  PortalUserRecord,
  ProjectRecord,
  ProviderHealthSnapshot,
  ProviderCatalogRecord,
  ProviderRecordWithIntegration,
  RateLimitPolicyRecord,
  RateLimitWindowRecord,
  RoutingDecisionLogRecord,
  RoutingProfileRecord,
  RuntimeReloadReport,
  RuntimeStatusRecord,
  SaveProviderInput,
  StorageAuditRecord,
  StorageConfigSnapshotRecord,
  StorageConfigUpsertInput,
  StorageEffectiveConfigRecord,
  StorageProviderSchemaRecord,
  StorageValidationRecord,
  TenantRecord,
  UsageRecord,
  UsageSummary,
} from 'sdkwork-control-plane-types';
import {
  deleteEmpty,
  getJson,
  patchJson,
  postJson,
  putJson,
  requiredToken,
} from './admin-app-transport.js';

// Manual-owned browser admin routes live here until the /api/admin surface is promoted into a
// checked-in OpenAPI authority. App consumers should still import from the formal SDK package.

export function loginAdminUser(input: {
  email: string;
  password: string;
}): Promise<AdminAuthSession> {
  return postJson<typeof input, AdminAuthSession>('/auth/login', input);
}

export function getAdminMe(token?: string): Promise<AdminSessionUser> {
  return getJson<AdminSessionUser>('/auth/me', token);
}

export function listOperatorUsers(token?: string): Promise<OperatorUserRecord[]> {
  return getJson<OperatorUserRecord[]>('/users/operators', token);
}

export function saveOperatorUser(input: {
  id?: string;
  email: string;
  display_name: string;
  password?: string;
  active: boolean;
}): Promise<OperatorUserRecord> {
  return postJson<typeof input, OperatorUserRecord>('/users/operators', input, requiredToken());
}

export function updateOperatorUserStatus(
  userId: string,
  active: boolean,
): Promise<OperatorUserRecord> {
  return postJson<{ active: boolean }, OperatorUserRecord>(
    `/users/operators/${userId}/status`,
    { active },
    requiredToken(),
  );
}

export function resetOperatorUserPassword(
  userId: string,
  newPassword: string,
): Promise<OperatorUserRecord> {
  return postJson<{ new_password: string }, OperatorUserRecord>(
    `/users/operators/${userId}/password`,
    { new_password: newPassword },
    requiredToken(),
  );
}

export function deleteOperatorUser(userId: string): Promise<void> {
  return deleteEmpty(`/users/operators/${encodeURIComponent(userId)}`, requiredToken());
}

export function listPortalUsers(token?: string): Promise<PortalUserRecord[]> {
  return getJson<PortalUserRecord[]>('/users/portal', token);
}

export function listMarketingCampaigns(token?: string): Promise<MarketingCampaignRecord[]> {
  return getJson<MarketingCampaignRecord[]>('/marketing/campaigns', requiredToken(token));
}

export function saveMarketingCampaign(
  input: MarketingCampaignRecord,
): Promise<MarketingCampaignRecord> {
  return postJson<MarketingCampaignRecord, MarketingCampaignRecord>(
    '/marketing/campaigns',
    input,
    requiredToken(),
  );
}

export function updateMarketingCampaignStatus(
  marketingCampaignId: string,
  status: MarketingCampaignStatus,
): Promise<MarketingCampaignRecord> {
  return postJson<{ status: MarketingCampaignStatus }, MarketingCampaignRecord>(
    `/marketing/campaigns/${encodeURIComponent(marketingCampaignId)}/status`,
    { status },
    requiredToken(),
  );
}

export function savePortalUser(input: {
  id?: string;
  email: string;
  display_name: string;
  password?: string;
  workspace_tenant_id: string;
  workspace_project_id: string;
  active: boolean;
}): Promise<PortalUserRecord> {
  return postJson<typeof input, PortalUserRecord>('/users/portal', input, requiredToken());
}

export function updatePortalUserStatus(
  userId: string,
  active: boolean,
): Promise<PortalUserRecord> {
  return postJson<{ active: boolean }, PortalUserRecord>(
    `/users/portal/${userId}/status`,
    { active },
    requiredToken(),
  );
}

export function resetPortalUserPassword(
  userId: string,
  newPassword: string,
): Promise<PortalUserRecord> {
  return postJson<{ new_password: string }, PortalUserRecord>(
    `/users/portal/${userId}/password`,
    { new_password: newPassword },
    requiredToken(),
  );
}

export function deletePortalUser(userId: string): Promise<void> {
  return deleteEmpty(`/users/portal/${encodeURIComponent(userId)}`, requiredToken());
}

export function listTenants(token?: string): Promise<TenantRecord[]> {
  return getJson<TenantRecord[]>('/tenants', token);
}

export function saveTenant(input: {
  id: string;
  name: string;
}): Promise<TenantRecord> {
  return postJson<typeof input, TenantRecord>('/tenants', input, requiredToken());
}

export function deleteTenant(tenantId: string): Promise<void> {
  return deleteEmpty(`/tenants/${encodeURIComponent(tenantId)}`, requiredToken());
}

export function listProjects(token?: string): Promise<ProjectRecord[]> {
  return getJson<ProjectRecord[]>('/projects', token);
}

export function saveProject(input: {
  tenant_id: string;
  id: string;
  name: string;
}): Promise<ProjectRecord> {
  return postJson<typeof input, ProjectRecord>('/projects', input, requiredToken());
}

export function deleteProject(projectId: string): Promise<void> {
  return deleteEmpty(`/projects/${encodeURIComponent(projectId)}`, requiredToken());
}

export function listApiKeys(token?: string): Promise<GatewayApiKeyRecord[]> {
  return getJson<GatewayApiKeyRecord[]>('/api-keys', token);
}

export function listApiKeyGroups(token?: string): Promise<ApiKeyGroupRecord[]> {
  return getJson<ApiKeyGroupRecord[]>('/api-key-groups', token);
}

export function createApiKeyGroup(input: {
  tenant_id: string;
  project_id: string;
  environment: string;
  name: string;
  slug?: string | null;
  description?: string | null;
  color?: string | null;
  default_capability_scope?: string | null;
  default_accounting_mode?: string | null;
  default_routing_profile_id?: string | null;
}): Promise<ApiKeyGroupRecord> {
  return postJson<typeof input, ApiKeyGroupRecord>(
    '/api-key-groups',
    input,
    requiredToken(),
  );
}

export function updateApiKeyGroup(
  groupId: string,
  input: {
    tenant_id: string;
    project_id: string;
    environment: string;
    name: string;
    slug?: string | null;
    description?: string | null;
    color?: string | null;
    default_capability_scope?: string | null;
    default_accounting_mode?: string | null;
    default_routing_profile_id?: string | null;
  },
): Promise<ApiKeyGroupRecord> {
  return patchJson<typeof input, ApiKeyGroupRecord>(
    `/api-key-groups/${encodeURIComponent(groupId)}`,
    input,
    requiredToken(),
  );
}

export function updateApiKeyGroupStatus(
  groupId: string,
  active: boolean,
): Promise<ApiKeyGroupRecord> {
  return postJson<{ active: boolean }, ApiKeyGroupRecord>(
    `/api-key-groups/${encodeURIComponent(groupId)}/status`,
    { active },
    requiredToken(),
  );
}

export function deleteApiKeyGroup(groupId: string): Promise<void> {
  return deleteEmpty(`/api-key-groups/${encodeURIComponent(groupId)}`, requiredToken());
}

export function listRoutingProfiles(token?: string): Promise<RoutingProfileRecord[]> {
  return getJson<RoutingProfileRecord[]>('/routing/profiles', token);
}

export function createRoutingProfile(input: {
  profile_id?: string;
  tenant_id: string;
  project_id: string;
  name: string;
  slug?: string | null;
  description?: string | null;
  active?: boolean;
  strategy?: string;
  ordered_provider_ids?: string[];
  default_provider_id?: string | null;
  max_cost?: number | null;
  max_latency_ms?: number | null;
  require_healthy?: boolean;
  preferred_region?: string | null;
}): Promise<RoutingProfileRecord> {
  return postJson<typeof input, RoutingProfileRecord>(
    '/routing/profiles',
    input,
    requiredToken(),
  );
}

export function listCompiledRoutingSnapshots(
  token?: string,
): Promise<CompiledRoutingSnapshotRecord[]> {
  return getJson<CompiledRoutingSnapshotRecord[]>('/routing/snapshots', token);
}

export function createApiKey(input: {
  tenant_id: string;
  project_id: string;
  environment: string;
  label?: string;
  notes?: string;
  expires_at_ms?: number | null;
  plaintext_key?: string;
  api_key_group_id?: string | null;
}): Promise<CreatedGatewayApiKey> {
  return postJson<typeof input, CreatedGatewayApiKey>('/api-keys', input, requiredToken());
}

export function updateApiKey(input: {
  hashed_key: string;
  tenant_id: string;
  project_id: string;
  environment: string;
  label: string;
  notes?: string | null;
  expires_at_ms?: number | null;
  api_key_group_id?: string | null;
}): Promise<GatewayApiKeyRecord> {
  return putJson<
    Omit<typeof input, 'hashed_key'>,
    GatewayApiKeyRecord
  >(
    `/api-keys/${encodeURIComponent(input.hashed_key)}`,
    {
      tenant_id: input.tenant_id,
      project_id: input.project_id,
      environment: input.environment,
      label: input.label,
      notes: input.notes,
      expires_at_ms: input.expires_at_ms,
      api_key_group_id: input.api_key_group_id,
    },
    requiredToken(),
  );
}

export function updateApiKeyStatus(
  hashedKey: string,
  active: boolean,
): Promise<GatewayApiKeyRecord> {
  return postJson<{ active: boolean }, GatewayApiKeyRecord>(
    `/api-keys/${encodeURIComponent(hashedKey)}/status`,
    { active },
    requiredToken(),
  );
}

export function deleteApiKey(hashedKey: string): Promise<void> {
  return deleteEmpty(`/api-keys/${encodeURIComponent(hashedKey)}`, requiredToken());
}

export function listChannels(token?: string): Promise<ChannelRecord[]> {
  return getJson<ChannelRecord[]>('/channels', token);
}

export function saveChannel(input: {
  id: string;
  name: string;
}): Promise<ChannelRecord> {
  return postJson<typeof input, ChannelRecord>('/channels', input, requiredToken());
}

export function deleteChannel(channelId: string): Promise<void> {
  return deleteEmpty(`/channels/${encodeURIComponent(channelId)}`, requiredToken());
}

export function listProviders(token?: string): Promise<ProviderCatalogRecord[]> {
  return getJson<ProviderCatalogRecord[]>('/providers', token);
}

export function saveProvider(input: SaveProviderInput): Promise<ProviderRecordWithIntegration> {
  return postJson<SaveProviderInput, ProviderRecordWithIntegration>(
    '/providers',
    input,
    requiredToken(),
  );
}

export function deleteProvider(providerId: string): Promise<void> {
  return deleteEmpty(`/providers/${encodeURIComponent(providerId)}`, requiredToken());
}

export function listCredentials(token?: string): Promise<CredentialRecord[]> {
  return getJson<CredentialRecord[]>('/credentials', token);
}

export function saveCredential(input: {
  tenant_id: string;
  provider_id: string;
  key_reference: string;
  secret_value: string;
}): Promise<CredentialRecord> {
  return postJson<typeof input, CredentialRecord>('/credentials', input, requiredToken());
}

export function deleteCredential(
  tenantId: string,
  providerId: string,
  keyReference: string,
): Promise<void> {
  return deleteEmpty(
    `/credentials/${encodeURIComponent(tenantId)}/providers/${encodeURIComponent(providerId)}/keys/${encodeURIComponent(keyReference)}`,
    requiredToken(),
  );
}

export function listModels(token?: string): Promise<ModelCatalogRecord[]> {
  return getJson<ModelCatalogRecord[]>('/models', token);
}

export function listChannelModels(token?: string): Promise<ChannelModelRecord[]> {
  return getJson<ChannelModelRecord[]>('/channel-models', token);
}

export function saveChannelModel(input: {
  channel_id: string;
  model_id: string;
  model_display_name: string;
  capabilities: string[];
  streaming: boolean;
  context_window?: number | null;
  description?: string;
}): Promise<ChannelModelRecord> {
  return postJson<typeof input, ChannelModelRecord>('/channel-models', input, requiredToken());
}

export function deleteChannelModel(channelId: string, modelId: string): Promise<void> {
  return deleteEmpty(
    `/channel-models/${encodeURIComponent(channelId)}/models/${encodeURIComponent(modelId)}`,
    requiredToken(),
  );
}

export function saveModel(input: {
  external_name: string;
  provider_id: string;
  capabilities: string[];
  streaming: boolean;
  context_window?: number;
}): Promise<ModelCatalogRecord> {
  return postJson<typeof input, ModelCatalogRecord>('/models', input, requiredToken());
}

export function deleteModel(externalName: string, providerId: string): Promise<void> {
  return deleteEmpty(
    `/models/${encodeURIComponent(externalName)}/providers/${encodeURIComponent(providerId)}`,
    requiredToken(),
  );
}

export function listModelPrices(token?: string): Promise<ModelPriceRecord[]> {
  return getJson<ModelPriceRecord[]>('/model-prices', token);
}

export function saveModelPrice(input: {
  channel_id: string;
  model_id: string;
  proxy_provider_id: string;
  currency_code: string;
  price_unit: string;
  input_price: number;
  output_price: number;
  cache_read_price: number;
  cache_write_price: number;
  request_price: number;
  is_active: boolean;
}): Promise<ModelPriceRecord> {
  return postJson<typeof input, ModelPriceRecord>('/model-prices', input, requiredToken());
}

export function deleteModelPrice(
  channelId: string,
  modelId: string,
  proxyProviderId: string,
): Promise<void> {
  return deleteEmpty(
    `/model-prices/${encodeURIComponent(channelId)}/models/${encodeURIComponent(modelId)}/providers/${encodeURIComponent(proxyProviderId)}`,
    requiredToken(),
  );
}

export function listUsageRecords(token?: string): Promise<UsageRecord[]> {
  return getJson<UsageRecord[]>('/usage/records', token);
}

export function getUsageSummary(token?: string): Promise<UsageSummary> {
  return getJson<UsageSummary>('/usage/summary', token);
}

export function getBillingSummary(token?: string): Promise<BillingSummary> {
  return getJson<BillingSummary>('/billing/summary', token);
}

export function listBillingEvents(token?: string): Promise<BillingEventRecord[]> {
  return getJson<BillingEventRecord[]>('/billing/events', token);
}

export function getBillingEventSummary(token?: string): Promise<BillingEventSummary> {
  return getJson<BillingEventSummary>('/billing/events/summary', token);
}

export function listRoutingDecisionLogs(token?: string): Promise<RoutingDecisionLogRecord[]> {
  return getJson<RoutingDecisionLogRecord[]>('/routing/decision-logs', token);
}

export function listRateLimitPolicies(token?: string): Promise<RateLimitPolicyRecord[]> {
  return getJson<RateLimitPolicyRecord[]>('/gateway/rate-limit-policies', token);
}

export function createRateLimitPolicy(input: {
  policy_id: string;
  project_id: string;
  requests_per_window: number;
  window_seconds: number;
  burst_requests: number;
  enabled: boolean;
  route_key?: string | null;
  api_key_hash?: string | null;
  model_name?: string | null;
  notes?: string | null;
}): Promise<RateLimitPolicyRecord> {
  return postJson<typeof input, RateLimitPolicyRecord>(
    '/gateway/rate-limit-policies',
    input,
    requiredToken(),
  );
}

export function listRateLimitWindows(token?: string): Promise<RateLimitWindowRecord[]> {
  return getJson<RateLimitWindowRecord[]>('/gateway/rate-limit-windows', token);
}

export function listProviderHealthSnapshots(
  token?: string,
): Promise<ProviderHealthSnapshot[]> {
  return getJson<ProviderHealthSnapshot[]>('/routing/health-snapshots', token);
}

export function listRuntimeStatuses(token?: string): Promise<RuntimeStatusRecord[]> {
  return getJson<RuntimeStatusRecord[]>('/extensions/runtime-statuses', token);
}

export function reloadExtensionRuntimes(input?: {
  extension_id?: string;
  instance_id?: string;
}): Promise<RuntimeReloadReport> {
  return postJson<typeof input, RuntimeReloadReport>(
    '/extensions/runtime-reloads',
    input ?? {},
    requiredToken(),
  );
}

export function listStorageProviders(token?: string): Promise<StorageProviderSchemaRecord[]> {
  return getJson<StorageProviderSchemaRecord[]>('/storage/providers', requiredToken(token));
}

export function getGlobalStorageConfig(token?: string): Promise<StorageConfigSnapshotRecord> {
  return getJson<StorageConfigSnapshotRecord>('/storage/config', requiredToken(token));
}

export function saveGlobalStorageConfig(
  input: StorageConfigUpsertInput,
): Promise<StorageConfigSnapshotRecord> {
  return postJson<StorageConfigUpsertInput, StorageConfigSnapshotRecord>(
    '/storage/config',
    input,
    requiredToken(),
  );
}

export function getTenantStorageConfig(
  tenantId: string,
  token?: string,
): Promise<StorageConfigSnapshotRecord> {
  return getJson<StorageConfigSnapshotRecord>(
    `/storage/config/tenants/${encodeURIComponent(tenantId)}`,
    requiredToken(token),
  );
}

export function saveTenantStorageConfig(
  tenantId: string,
  input: StorageConfigUpsertInput,
): Promise<StorageConfigSnapshotRecord> {
  return postJson<StorageConfigUpsertInput, StorageConfigSnapshotRecord>(
    `/storage/config/tenants/${encodeURIComponent(tenantId)}`,
    input,
    requiredToken(),
  );
}

export function deleteTenantStorageConfig(tenantId: string): Promise<void> {
  return deleteEmpty(`/storage/config/tenants/${encodeURIComponent(tenantId)}`, requiredToken());
}

export function getTenantEffectiveStorageConfig(
  tenantId: string,
  token?: string,
): Promise<StorageEffectiveConfigRecord> {
  return getJson<StorageEffectiveConfigRecord>(
    `/storage/effective/tenants/${encodeURIComponent(tenantId)}`,
    requiredToken(token),
  );
}

export function validateGlobalStorageConfig(token?: string): Promise<StorageValidationRecord> {
  return postJson<Record<string, never>, StorageValidationRecord>(
    '/storage/validate',
    {},
    requiredToken(token),
  );
}

export function validateTenantStorageConfig(
  tenantId: string,
  token?: string,
): Promise<StorageValidationRecord> {
  return postJson<Record<string, never>, StorageValidationRecord>(
    `/storage/validate/tenants/${encodeURIComponent(tenantId)}`,
    {},
    requiredToken(token),
  );
}

export function listStorageAuditTrail(token?: string): Promise<StorageAuditRecord[]> {
  return getJson<StorageAuditRecord[]>('/storage/audit', requiredToken(token));
}
