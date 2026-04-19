import { deleteEmpty, getJson, patchJson, postJson, putJson, requiredToken, } from './admin-app-transport.js';
// Manual-owned browser admin routes live here until the /api/admin surface is promoted into a
// checked-in OpenAPI authority. App consumers should still import from the formal SDK package.
export function loginAdminUser(input) {
    return postJson('/auth/login', input);
}
export function getAdminMe(token) {
    return getJson('/auth/me', token);
}
export function listOperatorUsers(token) {
    return getJson('/users/operators', token);
}
export function saveOperatorUser(input) {
    return postJson('/users/operators', input, requiredToken());
}
export function updateOperatorUserStatus(userId, active) {
    return postJson(`/users/operators/${userId}/status`, { active }, requiredToken());
}
export function resetOperatorUserPassword(userId, newPassword) {
    return postJson(`/users/operators/${userId}/password`, { new_password: newPassword }, requiredToken());
}
export function deleteOperatorUser(userId) {
    return deleteEmpty(`/users/operators/${encodeURIComponent(userId)}`, requiredToken());
}
export function listPortalUsers(token) {
    return getJson('/users/portal', token);
}
export function listMarketingCampaigns(token) {
    return getJson('/marketing/campaigns', requiredToken(token));
}
export function saveMarketingCampaign(input) {
    return postJson('/marketing/campaigns', input, requiredToken());
}
export function updateMarketingCampaignStatus(marketingCampaignId, status) {
    return postJson(`/marketing/campaigns/${encodeURIComponent(marketingCampaignId)}/status`, { status }, requiredToken());
}
export function savePortalUser(input) {
    return postJson('/users/portal', input, requiredToken());
}
export function updatePortalUserStatus(userId, active) {
    return postJson(`/users/portal/${userId}/status`, { active }, requiredToken());
}
export function resetPortalUserPassword(userId, newPassword) {
    return postJson(`/users/portal/${userId}/password`, { new_password: newPassword }, requiredToken());
}
export function deletePortalUser(userId) {
    return deleteEmpty(`/users/portal/${encodeURIComponent(userId)}`, requiredToken());
}
export function listTenants(token) {
    return getJson('/tenants', token);
}
export function saveTenant(input) {
    return postJson('/tenants', input, requiredToken());
}
export function deleteTenant(tenantId) {
    return deleteEmpty(`/tenants/${encodeURIComponent(tenantId)}`, requiredToken());
}
export function listProjects(token) {
    return getJson('/projects', token);
}
export function saveProject(input) {
    return postJson('/projects', input, requiredToken());
}
export function deleteProject(projectId) {
    return deleteEmpty(`/projects/${encodeURIComponent(projectId)}`, requiredToken());
}
export function listApiKeys(token) {
    return getJson('/api-keys', token);
}
export function listApiKeyGroups(token) {
    return getJson('/api-key-groups', token);
}
export function createApiKeyGroup(input) {
    return postJson('/api-key-groups', input, requiredToken());
}
export function updateApiKeyGroup(groupId, input) {
    return patchJson(`/api-key-groups/${encodeURIComponent(groupId)}`, input, requiredToken());
}
export function updateApiKeyGroupStatus(groupId, active) {
    return postJson(`/api-key-groups/${encodeURIComponent(groupId)}/status`, { active }, requiredToken());
}
export function deleteApiKeyGroup(groupId) {
    return deleteEmpty(`/api-key-groups/${encodeURIComponent(groupId)}`, requiredToken());
}
export function listRoutingProfiles(token) {
    return getJson('/routing/profiles', token);
}
export function createRoutingProfile(input) {
    return postJson('/routing/profiles', input, requiredToken());
}
export function listCompiledRoutingSnapshots(token) {
    return getJson('/routing/snapshots', token);
}
export function createApiKey(input) {
    return postJson('/api-keys', input, requiredToken());
}
export function updateApiKey(input) {
    return putJson(`/api-keys/${encodeURIComponent(input.hashed_key)}`, {
        tenant_id: input.tenant_id,
        project_id: input.project_id,
        environment: input.environment,
        label: input.label,
        notes: input.notes,
        expires_at_ms: input.expires_at_ms,
        api_key_group_id: input.api_key_group_id,
    }, requiredToken());
}
export function updateApiKeyStatus(hashedKey, active) {
    return postJson(`/api-keys/${encodeURIComponent(hashedKey)}/status`, { active }, requiredToken());
}
export function deleteApiKey(hashedKey) {
    return deleteEmpty(`/api-keys/${encodeURIComponent(hashedKey)}`, requiredToken());
}
export function listChannels(token) {
    return getJson('/channels', token);
}
export function saveChannel(input) {
    return postJson('/channels', input, requiredToken());
}
export function deleteChannel(channelId) {
    return deleteEmpty(`/channels/${encodeURIComponent(channelId)}`, requiredToken());
}
export function listProviders(token) {
    return getJson('/providers', token);
}
export function saveProvider(input) {
    return postJson('/providers', input, requiredToken());
}
export function deleteProvider(providerId) {
    return deleteEmpty(`/providers/${encodeURIComponent(providerId)}`, requiredToken());
}
export function listCredentials(token) {
    return getJson('/credentials', token);
}
export function saveCredential(input) {
    return postJson('/credentials', input, requiredToken());
}
export function deleteCredential(tenantId, providerId, keyReference) {
    return deleteEmpty(`/credentials/${encodeURIComponent(tenantId)}/providers/${encodeURIComponent(providerId)}/keys/${encodeURIComponent(keyReference)}`, requiredToken());
}
export function listModels(token) {
    return getJson('/models', token);
}
export function listChannelModels(token) {
    return getJson('/channel-models', token);
}
export function saveChannelModel(input) {
    return postJson('/channel-models', input, requiredToken());
}
export function deleteChannelModel(channelId, modelId) {
    return deleteEmpty(`/channel-models/${encodeURIComponent(channelId)}/models/${encodeURIComponent(modelId)}`, requiredToken());
}
export function saveModel(input) {
    return postJson('/models', input, requiredToken());
}
export function deleteModel(externalName, providerId) {
    return deleteEmpty(`/models/${encodeURIComponent(externalName)}/providers/${encodeURIComponent(providerId)}`, requiredToken());
}
export function listModelPrices(token) {
    return getJson('/model-prices', token);
}
export function saveModelPrice(input) {
    return postJson('/model-prices', input, requiredToken());
}
export function deleteModelPrice(channelId, modelId, proxyProviderId) {
    return deleteEmpty(`/model-prices/${encodeURIComponent(channelId)}/models/${encodeURIComponent(modelId)}/providers/${encodeURIComponent(proxyProviderId)}`, requiredToken());
}
export function listUsageRecords(token) {
    return getJson('/usage/records', token);
}
export function getUsageSummary(token) {
    return getJson('/usage/summary', token);
}
export function getBillingSummary(token) {
    return getJson('/billing/summary', token);
}
export function listBillingEvents(token) {
    return getJson('/billing/events', token);
}
export function getBillingEventSummary(token) {
    return getJson('/billing/events/summary', token);
}
export function listRoutingDecisionLogs(token) {
    return getJson('/routing/decision-logs', token);
}
export function listRateLimitPolicies(token) {
    return getJson('/gateway/rate-limit-policies', token);
}
export function createRateLimitPolicy(input) {
    return postJson('/gateway/rate-limit-policies', input, requiredToken());
}
export function listRateLimitWindows(token) {
    return getJson('/gateway/rate-limit-windows', token);
}
export function listProviderHealthSnapshots(token) {
    return getJson('/routing/health-snapshots', token);
}
export function listRuntimeStatuses(token) {
    return getJson('/extensions/runtime-statuses', token);
}
export function reloadExtensionRuntimes(input) {
    return postJson('/extensions/runtime-reloads', input ?? {}, requiredToken());
}
export function listStorageProviders(token) {
    return getJson('/storage/providers', requiredToken(token));
}
export function getGlobalStorageConfig(token) {
    return getJson('/storage/config', requiredToken(token));
}
export function saveGlobalStorageConfig(input) {
    return postJson('/storage/config', input, requiredToken());
}
export function getTenantStorageConfig(tenantId, token) {
    return getJson(`/storage/config/tenants/${encodeURIComponent(tenantId)}`, requiredToken(token));
}
export function saveTenantStorageConfig(tenantId, input) {
    return postJson(`/storage/config/tenants/${encodeURIComponent(tenantId)}`, input, requiredToken());
}
export function deleteTenantStorageConfig(tenantId) {
    return deleteEmpty(`/storage/config/tenants/${encodeURIComponent(tenantId)}`, requiredToken());
}
export function getTenantEffectiveStorageConfig(tenantId, token) {
    return getJson(`/storage/effective/tenants/${encodeURIComponent(tenantId)}`, requiredToken(token));
}
export function validateGlobalStorageConfig(token) {
    return postJson('/storage/validate', {}, requiredToken(token));
}
export function validateTenantStorageConfig(tenantId, token) {
    return postJson(`/storage/validate/tenants/${encodeURIComponent(tenantId)}`, {}, requiredToken(token));
}
export function listStorageAuditTrail(token) {
    return getJson('/storage/audit', requiredToken(token));
}
