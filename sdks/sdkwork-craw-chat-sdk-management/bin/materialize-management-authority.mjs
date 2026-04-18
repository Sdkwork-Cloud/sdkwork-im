import { mkdirSync, writeFileSync } from 'node:fs';
import path from 'node:path';

const workspaceRoot = path.resolve(import.meta.dirname, '..');
const openapiDir = path.join(workspaceRoot, 'openapi');

export const routeGroups = {
  auth: 'Operator console session bootstrap and current-session introspection.',
  users: 'Operator and portal identity administration.',
  marketing: 'Campaign configuration and campaign state changes.',
  tenants: 'Tenant and project governance.',
  access: 'API key issuance and key-group governance.',
  routing: 'Routing profile, snapshot, decision-log, and provider-health reads.',
  catalog: 'Channel, provider, credential, model, and pricing catalog administration.',
  usage: 'Usage record and summary reads.',
  billing: 'Billing event and billing summary reads.',
  operations: 'Rate-limit governance and runtime operation controls.',
};

export const routes = [
  ['post', '/api/admin/auth/login', 'loginAdminUser', 'auth'],
  ['get', '/api/admin/auth/me', 'getAdminMe', 'auth'],
  ['get', '/api/admin/users/operators', 'listOperatorUsers', 'users'],
  ['post', '/api/admin/users/operators', 'saveOperatorUser', 'users'],
  ['delete', '/api/admin/users/operators/{userId}', 'deleteOperatorUser', 'users'],
  ['post', '/api/admin/users/operators/{userId}/status', 'updateOperatorUserStatus', 'users'],
  ['post', '/api/admin/users/operators/{userId}/password', 'resetOperatorUserPassword', 'users'],
  ['get', '/api/admin/users/portal', 'listPortalUsers', 'users'],
  ['post', '/api/admin/users/portal', 'savePortalUser', 'users'],
  ['delete', '/api/admin/users/portal/{userId}', 'deletePortalUser', 'users'],
  ['post', '/api/admin/users/portal/{userId}/status', 'updatePortalUserStatus', 'users'],
  ['post', '/api/admin/users/portal/{userId}/password', 'resetPortalUserPassword', 'users'],
  ['get', '/api/admin/marketing/campaigns', 'listMarketingCampaigns', 'marketing'],
  ['post', '/api/admin/marketing/campaigns', 'saveMarketingCampaign', 'marketing'],
  ['post', '/api/admin/marketing/campaigns/{marketingCampaignId}/status', 'updateMarketingCampaignStatus', 'marketing'],
  ['get', '/api/admin/tenants', 'listTenants', 'tenants'],
  ['post', '/api/admin/tenants', 'saveTenant', 'tenants'],
  ['delete', '/api/admin/tenants/{tenantId}', 'deleteTenant', 'tenants'],
  ['get', '/api/admin/projects', 'listProjects', 'tenants'],
  ['post', '/api/admin/projects', 'saveProject', 'tenants'],
  ['delete', '/api/admin/projects/{projectId}', 'deleteProject', 'tenants'],
  ['get', '/api/admin/api-keys', 'listApiKeys', 'access'],
  ['post', '/api/admin/api-keys', 'createApiKey', 'access'],
  ['put', '/api/admin/api-keys/{hashedKey}', 'updateApiKey', 'access'],
  ['delete', '/api/admin/api-keys/{hashedKey}', 'deleteApiKey', 'access'],
  ['post', '/api/admin/api-keys/{hashedKey}/status', 'updateApiKeyStatus', 'access'],
  ['get', '/api/admin/api-key-groups', 'listApiKeyGroups', 'access'],
  ['post', '/api/admin/api-key-groups', 'createApiKeyGroup', 'access'],
  ['patch', '/api/admin/api-key-groups/{groupId}', 'updateApiKeyGroup', 'access'],
  ['delete', '/api/admin/api-key-groups/{groupId}', 'deleteApiKeyGroup', 'access'],
  ['post', '/api/admin/api-key-groups/{groupId}/status', 'updateApiKeyGroupStatus', 'access'],
  ['get', '/api/admin/routing/profiles', 'listRoutingProfiles', 'routing'],
  ['post', '/api/admin/routing/profiles', 'createRoutingProfile', 'routing'],
  ['get', '/api/admin/routing/snapshots', 'listCompiledRoutingSnapshots', 'routing'],
  ['get', '/api/admin/routing/decision-logs', 'listRoutingDecisionLogs', 'routing'],
  ['get', '/api/admin/routing/health-snapshots', 'listProviderHealthSnapshots', 'routing'],
  ['get', '/api/admin/channels', 'listChannels', 'catalog'],
  ['post', '/api/admin/channels', 'saveChannel', 'catalog'],
  ['delete', '/api/admin/channels/{channelId}', 'deleteChannel', 'catalog'],
  ['get', '/api/admin/providers', 'listProviders', 'catalog'],
  ['post', '/api/admin/providers', 'saveProvider', 'catalog'],
  ['delete', '/api/admin/providers/{providerId}', 'deleteProvider', 'catalog'],
  ['get', '/api/admin/credentials', 'listCredentials', 'catalog'],
  ['post', '/api/admin/credentials', 'saveCredential', 'catalog'],
  ['delete', '/api/admin/credentials/{tenantId}/providers/{providerId}/keys/{keyReference}', 'deleteCredential', 'catalog'],
  ['get', '/api/admin/models', 'listModels', 'catalog'],
  ['post', '/api/admin/models', 'saveModel', 'catalog'],
  ['delete', '/api/admin/models/{externalName}/providers/{providerId}', 'deleteModel', 'catalog'],
  ['get', '/api/admin/channel-models', 'listChannelModels', 'catalog'],
  ['post', '/api/admin/channel-models', 'saveChannelModel', 'catalog'],
  ['delete', '/api/admin/channel-models/{channelId}/models/{modelId}', 'deleteChannelModel', 'catalog'],
  ['get', '/api/admin/model-prices', 'listModelPrices', 'catalog'],
  ['post', '/api/admin/model-prices', 'saveModelPrice', 'catalog'],
  ['delete', '/api/admin/model-prices/{channelId}/models/{modelId}/providers/{proxyProviderId}', 'deleteModelPrice', 'catalog'],
  ['get', '/api/admin/usage/records', 'listUsageRecords', 'usage'],
  ['get', '/api/admin/usage/summary', 'getUsageSummary', 'usage'],
  ['get', '/api/admin/billing/events', 'listBillingEvents', 'billing'],
  ['get', '/api/admin/billing/events/summary', 'getBillingEventSummary', 'billing'],
  ['get', '/api/admin/billing/summary', 'getBillingSummary', 'billing'],
  ['get', '/api/admin/gateway/rate-limit-policies', 'listRateLimitPolicies', 'operations'],
  ['post', '/api/admin/gateway/rate-limit-policies', 'createRateLimitPolicy', 'operations'],
  ['get', '/api/admin/gateway/rate-limit-windows', 'listRateLimitWindows', 'operations'],
  ['get', '/api/admin/extensions/runtime-statuses', 'listRuntimeStatuses', 'operations'],
  ['post', '/api/admin/extensions/runtime-reloads', 'reloadExtensionRuntimes', 'operations'],
];

function buildPathItems() {
  const pathItems = new Map();

  for (const [method, routePath, operationId, group] of routes) {
    const existing = pathItems.get(routePath) ?? {};
    const responses = {
      200: {
        description: 'Successful response',
        content: {
          'application/json': {
            schema: {
              $ref: '#/components/schemas/LooseJsonValue',
            },
          },
        },
      },
      401: {
        description: 'Authentication failed or no active operator session',
        content: {
          'application/json': {
            schema: {
              $ref: '#/components/schemas/ErrorEnvelope',
            },
          },
        },
      },
    };

    if (method === 'delete') {
      responses[204] = {
        description: 'Delete succeeded and no payload is returned',
      };
    }

    const operation = {
      operationId,
      tags: [group],
      summary: operationId,
      responses,
    };

    if (method !== 'get' && method !== 'delete') {
      operation.requestBody = {
        required: true,
        content: {
          'application/json': {
            schema: {
              $ref: '#/components/schemas/LooseJsonObject',
            },
          },
        },
      };
    }

    existing[method] = operation;
    pathItems.set(routePath, existing);
  }

  return Object.fromEntries(
    [...pathItems.entries()].sort((left, right) => left[0].localeCompare(right[0])),
  );
}

function buildSurfaceGroups() {
  const counts = new Map();

  for (const [, , , group] of routes) {
    counts.set(group, (counts.get(group) ?? 0) + 1);
  }

  return Object.keys(routeGroups).map((operationGroup) => ({
    serviceId: 'admin-console-api',
    operationGroup,
    protocols: ['http'],
    operationCount: counts.get(operationGroup) ?? 0,
  }));
}

export function buildAuthorityDocument() {
  return {
    openapi: '3.1.0',
    info: {
      title: 'Craw Chat Management API',
      version: '0.1.0',
      description:
        'Authority snapshot for the operator-console management backend served behind /api/admin/*.',
    },
    servers: [
      {
        url: '/api/admin',
        description: 'Unified gateway management proxy root',
      },
    ],
    tags: Object.entries(routeGroups).map(([name, description]) => ({
      name,
      description,
    })),
    paths: buildPathItems(),
    components: {
      securitySchemes: {
        bearerAuth: {
          type: 'http',
          scheme: 'bearer',
          bearerFormat: 'JWT',
        },
      },
      schemas: {
        LooseJsonValue: {},
        LooseJsonObject: {
          type: 'object',
          additionalProperties: true,
        },
        ErrorEnvelope: {
          type: 'object',
          additionalProperties: true,
          properties: {
            error: {
              type: 'object',
              additionalProperties: true,
              properties: {
                code: { type: 'string' },
                message: { type: 'string' },
              },
            },
          },
        },
      },
    },
    security: [{ bearerAuth: [] }],
    'x-sdkwork-sdk-surface': {
      sdkTarget: 'crawChatManagementSdk',
      visibility: 'management',
      generatedProtocols: ['http'],
      manualTransports: [],
      services: [
        {
          serviceId: 'admin-console-api',
          operationGroups: Object.keys(routeGroups),
          protocols: ['http'],
          operationCount: routes.length,
        },
      ],
      surfaceGroups: buildSurfaceGroups(),
      operationBindings: routes.map(([method, routePath, operationId, operationGroup]) => ({
        operationId,
        method,
        path: routePath,
        serviceId: 'admin-console-api',
        operationGroup,
        sdkTarget: 'crawChatManagementSdk',
        visibility: 'management',
        protocol: 'http',
      })),
    },
  };
}

function writeJson(targetPath, payload) {
  writeFileSync(targetPath, `${JSON.stringify(payload, null, 2)}\n`, 'utf8');
}

export function materializeManagementAuthority(options = {}) {
  const targetWorkspaceRoot = options.workspaceRoot || workspaceRoot;
  const targetOpenapiDir = path.join(targetWorkspaceRoot, 'openapi');

  mkdirSync(targetOpenapiDir, { recursive: true });

  const authorityDocument = buildAuthorityDocument();

  writeJson(
    path.join(targetOpenapiDir, 'craw-chat-management.openapi.json'),
    authorityDocument,
  );
  writeJson(
    path.join(targetOpenapiDir, 'craw-chat-management.sdkgen.json'),
    authorityDocument,
  );

  return authorityDocument;
}

const isCli = process.argv[1]
  && path.resolve(process.argv[1]) === import.meta.filename;

if (isCli) {
  materializeManagementAuthority();
  console.log(
    `Materialized management SDK authority with ${routes.length} operations at ${workspaceRoot}`,
  );
}
