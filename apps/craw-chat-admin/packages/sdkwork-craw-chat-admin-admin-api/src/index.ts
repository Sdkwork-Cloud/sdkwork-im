import type { SdkworkBackendClient } from '@sdkwork/craw-chat-management-backend-sdk';
import type { CrawChatSdkManagementClient } from '@sdkwork/craw-chat-sdk-management';
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
  TenantRecord,
  UsageRecord,
  UsageSummary,
} from 'sdkwork-craw-chat-admin-types';

import type { AdminManagementModuleName } from './management-sdk';
import { callAdminManagementMethod } from './management-sdk';

// Keep the application boundary explicit: this workspace package delegates its
// implementation to the generated management backend SDK plus the composed
// @sdkwork/craw-chat-sdk-management facade.
type _AdminManagementSdkBoundary =
  | CrawChatSdkManagementClient
  | SdkworkBackendClient;

export {
  AdminApiError,
  adminBaseUrl,
  clearAdminSessionToken,
  persistAdminSessionToken,
  readAdminSessionToken,
} from './transport';

function callPublicMethod<T>(
  moduleName: AdminManagementModuleName,
  methodName: string,
  ...args: unknown[]
): Promise<T> {
  return callAdminManagementMethod<T>(moduleName, methodName, args, {
    requireAuth: false,
  });
}

function callAuthenticatedMethod<T>(
  moduleName: AdminManagementModuleName,
  methodName: string,
  token: string | undefined,
  ...args: unknown[]
): Promise<T> {
  return callAdminManagementMethod<T>(moduleName, methodName, args, {
    token,
    requireAuth: true,
  });
}

export function loginAdminUser(input: {
  email: string;
  password: string;
}): Promise<AdminAuthSession> {
  return callPublicMethod<AdminAuthSession>('auth', 'loginAdminUser', input);
}

export function getAdminMe(token?: string): Promise<AdminSessionUser> {
  return callAuthenticatedMethod<AdminSessionUser>('auth', 'getAdminMe', token);
}

export function listOperatorUsers(token?: string): Promise<OperatorUserRecord[]> {
  return callAuthenticatedMethod<OperatorUserRecord[]>(
    'users',
    'listOperatorUsers',
    token,
  );
}

export function saveOperatorUser(input: {
  id?: string;
  email: string;
  display_name: string;
  password?: string;
  active: boolean;
}): Promise<OperatorUserRecord> {
  return callAuthenticatedMethod<OperatorUserRecord>(
    'users',
    'saveOperatorUser',
    undefined,
    input,
  );
}

export function updateOperatorUserStatus(
  userId: string,
  active: boolean,
): Promise<OperatorUserRecord> {
  return callAuthenticatedMethod<OperatorUserRecord>(
    'users',
    'updateOperatorUserStatus',
    undefined,
    userId,
    { active },
  );
}

export function resetOperatorUserPassword(
  userId: string,
  newPassword: string,
): Promise<OperatorUserRecord> {
  return callAuthenticatedMethod<OperatorUserRecord>(
    'users',
    'resetOperatorUserPassword',
    undefined,
    userId,
    { new_password: newPassword },
  );
}

export function deleteOperatorUser(userId: string): Promise<void> {
  return callAuthenticatedMethod<void>('users', 'deleteOperatorUser', undefined, userId);
}

export function listPortalUsers(token?: string): Promise<PortalUserRecord[]> {
  return callAuthenticatedMethod<PortalUserRecord[]>(
    'users',
    'listPortalUsers',
    token,
  );
}

export function listMarketingCampaigns(
  token?: string,
): Promise<MarketingCampaignRecord[]> {
  return callAuthenticatedMethod<MarketingCampaignRecord[]>(
    'marketing',
    'listMarketingCampaigns',
    token,
  );
}

export function saveMarketingCampaign(
  input: MarketingCampaignRecord,
): Promise<MarketingCampaignRecord> {
  return callAuthenticatedMethod<MarketingCampaignRecord>(
    'marketing',
    'saveMarketingCampaign',
    undefined,
    input,
  );
}

export function updateMarketingCampaignStatus(
  marketingCampaignId: string,
  status: MarketingCampaignStatus,
): Promise<MarketingCampaignRecord> {
  return callAuthenticatedMethod<MarketingCampaignRecord>(
    'marketing',
    'updateMarketingCampaignStatus',
    undefined,
    marketingCampaignId,
    { status },
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
  return callAuthenticatedMethod<PortalUserRecord>(
    'users',
    'savePortalUser',
    undefined,
    input,
  );
}

export function updatePortalUserStatus(
  userId: string,
  active: boolean,
): Promise<PortalUserRecord> {
  return callAuthenticatedMethod<PortalUserRecord>(
    'users',
    'updatePortalUserStatus',
    undefined,
    userId,
    { active },
  );
}

export function resetPortalUserPassword(
  userId: string,
  newPassword: string,
): Promise<PortalUserRecord> {
  return callAuthenticatedMethod<PortalUserRecord>(
    'users',
    'resetPortalUserPassword',
    undefined,
    userId,
    { new_password: newPassword },
  );
}

export function deletePortalUser(userId: string): Promise<void> {
  return callAuthenticatedMethod<void>('users', 'deletePortalUser', undefined, userId);
}

export function listTenants(token?: string): Promise<TenantRecord[]> {
  return callAuthenticatedMethod<TenantRecord[]>('tenants', 'listTenants', token);
}

export function saveTenant(input: {
  id: string;
  name: string;
}): Promise<TenantRecord> {
  return callAuthenticatedMethod<TenantRecord>('tenants', 'saveTenant', undefined, input);
}

export function deleteTenant(tenantId: string): Promise<void> {
  return callAuthenticatedMethod<void>('tenants', 'deleteTenant', undefined, tenantId);
}

export function listProjects(token?: string): Promise<ProjectRecord[]> {
  return callAuthenticatedMethod<ProjectRecord[]>('tenants', 'listProjects', token);
}

export function saveProject(input: {
  tenant_id: string;
  id: string;
  name: string;
}): Promise<ProjectRecord> {
  return callAuthenticatedMethod<ProjectRecord>('tenants', 'saveProject', undefined, input);
}

export function deleteProject(projectId: string): Promise<void> {
  return callAuthenticatedMethod<void>('tenants', 'deleteProject', undefined, projectId);
}

export function listApiKeys(token?: string): Promise<GatewayApiKeyRecord[]> {
  return callAuthenticatedMethod<GatewayApiKeyRecord[]>('access', 'listApiKeys', token);
}

export function listApiKeyGroups(token?: string): Promise<ApiKeyGroupRecord[]> {
  return callAuthenticatedMethod<ApiKeyGroupRecord[]>(
    'access',
    'listApiKeyGroups',
    token,
  );
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
  return callAuthenticatedMethod<ApiKeyGroupRecord>(
    'access',
    'createApiKeyGroup',
    undefined,
    input,
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
  return callAuthenticatedMethod<ApiKeyGroupRecord>(
    'access',
    'updateApiKeyGroup',
    undefined,
    groupId,
    input,
  );
}

export function updateApiKeyGroupStatus(
  groupId: string,
  active: boolean,
): Promise<ApiKeyGroupRecord> {
  return callAuthenticatedMethod<ApiKeyGroupRecord>(
    'access',
    'updateApiKeyGroupStatus',
    undefined,
    groupId,
    { active },
  );
}

export function deleteApiKeyGroup(groupId: string): Promise<void> {
  return callAuthenticatedMethod<void>('access', 'deleteApiKeyGroup', undefined, groupId);
}

export function listRoutingProfiles(token?: string): Promise<RoutingProfileRecord[]> {
  return callAuthenticatedMethod<RoutingProfileRecord[]>(
    'routing',
    'listRoutingProfiles',
    token,
  );
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
  return callAuthenticatedMethod<RoutingProfileRecord>(
    'routing',
    'createRoutingProfile',
    undefined,
    input,
  );
}

export function listCompiledRoutingSnapshots(
  token?: string,
): Promise<CompiledRoutingSnapshotRecord[]> {
  return callAuthenticatedMethod<CompiledRoutingSnapshotRecord[]>(
    'routing',
    'listCompiledRoutingSnapshots',
    token,
  );
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
  return callAuthenticatedMethod<CreatedGatewayApiKey>(
    'access',
    'createApiKey',
    undefined,
    input,
  );
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
  return callAuthenticatedMethod<GatewayApiKeyRecord>(
    'access',
    'updateApiKey',
    undefined,
    input.hashed_key,
    {
      tenant_id: input.tenant_id,
      project_id: input.project_id,
      environment: input.environment,
      label: input.label,
      notes: input.notes,
      expires_at_ms: input.expires_at_ms,
      api_key_group_id: input.api_key_group_id,
    },
  );
}

export function updateApiKeyStatus(
  hashedKey: string,
  active: boolean,
): Promise<GatewayApiKeyRecord> {
  return callAuthenticatedMethod<GatewayApiKeyRecord>(
    'access',
    'updateApiKeyStatus',
    undefined,
    hashedKey,
    { active },
  );
}

export function deleteApiKey(hashedKey: string): Promise<void> {
  return callAuthenticatedMethod<void>('access', 'deleteApiKey', undefined, hashedKey);
}

export function listChannels(token?: string): Promise<ChannelRecord[]> {
  return callAuthenticatedMethod<ChannelRecord[]>('catalog', 'listChannels', token);
}

export function saveChannel(input: {
  id: string;
  name: string;
}): Promise<ChannelRecord> {
  return callAuthenticatedMethod<ChannelRecord>('catalog', 'saveChannel', undefined, input);
}

export function deleteChannel(channelId: string): Promise<void> {
  return callAuthenticatedMethod<void>('catalog', 'deleteChannel', undefined, channelId);
}

export function listProviders(token?: string): Promise<ProviderCatalogRecord[]> {
  return callAuthenticatedMethod<ProviderCatalogRecord[]>(
    'catalog',
    'listProviders',
    token,
  );
}

export function saveProvider(
  input: SaveProviderInput,
): Promise<ProviderRecordWithIntegration> {
  return callAuthenticatedMethod<ProviderRecordWithIntegration>(
    'catalog',
    'saveProvider',
    undefined,
    input,
  );
}

export function deleteProvider(providerId: string): Promise<void> {
  return callAuthenticatedMethod<void>('catalog', 'deleteProvider', undefined, providerId);
}

export function listCredentials(token?: string): Promise<CredentialRecord[]> {
  return callAuthenticatedMethod<CredentialRecord[]>(
    'catalog',
    'listCredentials',
    token,
  );
}

export function saveCredential(input: {
  tenant_id: string;
  provider_id: string;
  key_reference: string;
  secret_value: string;
}): Promise<CredentialRecord> {
  return callAuthenticatedMethod<CredentialRecord>(
    'catalog',
    'saveCredential',
    undefined,
    input,
  );
}

export function deleteCredential(
  tenantId: string,
  providerId: string,
  keyReference: string,
): Promise<void> {
  return callAuthenticatedMethod<void>(
    'catalog',
    'deleteCredential',
    undefined,
    tenantId,
    providerId,
    keyReference,
  );
}

export function listModels(token?: string): Promise<ModelCatalogRecord[]> {
  return callAuthenticatedMethod<ModelCatalogRecord[]>('catalog', 'listModels', token);
}

export function listChannelModels(token?: string): Promise<ChannelModelRecord[]> {
  return callAuthenticatedMethod<ChannelModelRecord[]>(
    'catalog',
    'listChannelModels',
    token,
  );
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
  return callAuthenticatedMethod<ChannelModelRecord>(
    'catalog',
    'saveChannelModel',
    undefined,
    input,
  );
}

export function deleteChannelModel(
  channelId: string,
  modelId: string,
): Promise<void> {
  return callAuthenticatedMethod<void>(
    'catalog',
    'deleteChannelModel',
    undefined,
    channelId,
    modelId,
  );
}

export function saveModel(input: {
  external_name: string;
  provider_id: string;
  capabilities: string[];
  streaming: boolean;
  context_window?: number;
}): Promise<ModelCatalogRecord> {
  return callAuthenticatedMethod<ModelCatalogRecord>(
    'catalog',
    'saveModel',
    undefined,
    input,
  );
}

export function deleteModel(
  externalName: string,
  providerId: string,
): Promise<void> {
  return callAuthenticatedMethod<void>(
    'catalog',
    'deleteModel',
    undefined,
    externalName,
    providerId,
  );
}

export function listModelPrices(token?: string): Promise<ModelPriceRecord[]> {
  return callAuthenticatedMethod<ModelPriceRecord[]>(
    'catalog',
    'listModelPrices',
    token,
  );
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
  return callAuthenticatedMethod<ModelPriceRecord>(
    'catalog',
    'saveModelPrice',
    undefined,
    input,
  );
}

export function deleteModelPrice(
  channelId: string,
  modelId: string,
  proxyProviderId: string,
): Promise<void> {
  return callAuthenticatedMethod<void>(
    'catalog',
    'deleteModelPrice',
    undefined,
    channelId,
    modelId,
    proxyProviderId,
  );
}

export function listUsageRecords(token?: string): Promise<UsageRecord[]> {
  return callAuthenticatedMethod<UsageRecord[]>('usage', 'listUsageRecords', token);
}

export function getUsageSummary(token?: string): Promise<UsageSummary> {
  return callAuthenticatedMethod<UsageSummary>('usage', 'getUsageSummary', token);
}

export function getBillingSummary(token?: string): Promise<BillingSummary> {
  return callAuthenticatedMethod<BillingSummary>('billing', 'getBillingSummary', token);
}

export function listBillingEvents(token?: string): Promise<BillingEventRecord[]> {
  return callAuthenticatedMethod<BillingEventRecord[]>(
    'billing',
    'listBillingEvents',
    token,
  );
}

export function getBillingEventSummary(
  token?: string,
): Promise<BillingEventSummary> {
  return callAuthenticatedMethod<BillingEventSummary>(
    'billing',
    'getBillingEventSummary',
    token,
  );
}

export function listRoutingDecisionLogs(
  token?: string,
): Promise<RoutingDecisionLogRecord[]> {
  return callAuthenticatedMethod<RoutingDecisionLogRecord[]>(
    'routing',
    'listRoutingDecisionLogs',
    token,
  );
}

export function listRateLimitPolicies(token?: string): Promise<RateLimitPolicyRecord[]> {
  return callAuthenticatedMethod<RateLimitPolicyRecord[]>(
    'operations',
    'listRateLimitPolicies',
    token,
  );
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
  return callAuthenticatedMethod<RateLimitPolicyRecord>(
    'operations',
    'createRateLimitPolicy',
    undefined,
    input,
  );
}

export function listRateLimitWindows(token?: string): Promise<RateLimitWindowRecord[]> {
  return callAuthenticatedMethod<RateLimitWindowRecord[]>(
    'operations',
    'listRateLimitWindows',
    token,
  );
}

export function listProviderHealthSnapshots(
  token?: string,
): Promise<ProviderHealthSnapshot[]> {
  return callAuthenticatedMethod<ProviderHealthSnapshot[]>(
    'routing',
    'listProviderHealthSnapshots',
    token,
  );
}

export function listRuntimeStatuses(token?: string): Promise<RuntimeStatusRecord[]> {
  return callAuthenticatedMethod<RuntimeStatusRecord[]>(
    'operations',
    'listRuntimeStatuses',
    token,
  );
}

export function reloadExtensionRuntimes(input?: {
  extension_id?: string;
  instance_id?: string;
}): Promise<RuntimeReloadReport> {
  return callAuthenticatedMethod<RuntimeReloadReport>(
    'operations',
    'reloadExtensionRuntimes',
    undefined,
    input ?? {},
  );
}
