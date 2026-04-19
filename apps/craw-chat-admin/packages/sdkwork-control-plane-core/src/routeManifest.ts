import type {
  AdminRouteKey,
  AdminRouteManifestEntry,
  AdminRouteModuleId,
  AdminProductModuleManifest,
} from 'sdkwork-control-plane-types';

import { adminRoutePathByKey } from './routePaths';
import { adminRoutes } from './routes';

export const adminProductModules: AdminProductModuleManifest[] = [
  {
    moduleId: 'sdkwork-control-plane-overview',
    pluginId: 'sdkwork-control-plane-overview',
    pluginKind: 'admin-module',
    packageName: 'sdkwork-control-plane-overview',
    displayName: 'Overview',
    routeKeys: ['overview'],
    capabilityTags: ['operator-overview', 'message-throughput', 'hotspots'],
    requiredPermissions: ['admin.overview.read'],
    navigation: {
      group: 'Operations',
      order: 10,
      sidebar: true,
    },
    loading: {
      strategy: 'lazy',
      prefetch: 'intent',
      chunkGroup: 'overview',
    },
  },
  {
    moduleId: 'sdkwork-control-plane-tenants',
    pluginId: 'sdkwork-control-plane-tenants',
    pluginKind: 'admin-module',
    packageName: 'sdkwork-control-plane-tenants',
    displayName: 'Tenants',
    routeKeys: ['tenants'],
    capabilityTags: ['tenant-governance', 'workspace-governance', 'organization-governance'],
    requiredPermissions: ['admin.tenants.read', 'admin.tenants.write'],
    navigation: {
      group: 'Workspace Governance',
      order: 20,
      sidebar: true,
    },
    loading: {
      strategy: 'lazy',
      prefetch: 'intent',
      chunkGroup: 'tenants',
    },
  },
  {
    moduleId: 'sdkwork-control-plane-users',
    pluginId: 'sdkwork-control-plane-users',
    pluginKind: 'admin-module',
    packageName: 'sdkwork-control-plane-users',
    displayName: 'Identity',
    routeKeys: ['users'],
    capabilityTags: ['portal-identities', 'operator-identities', 'device-posture'],
    requiredPermissions: ['admin.users.read', 'admin.users.write'],
    navigation: {
      group: 'Workspace Governance',
      order: 30,
      sidebar: true,
    },
    loading: {
      strategy: 'lazy',
      prefetch: 'intent',
      chunkGroup: 'users',
    },
  },
  {
    moduleId: 'sdkwork-control-plane-groups',
    pluginId: 'sdkwork-control-plane-groups',
    pluginKind: 'admin-module',
    packageName: 'sdkwork-control-plane-groups',
    displayName: 'Groups',
    routeKeys: ['groups'],
    capabilityTags: ['group-governance', 'channel-governance', 'membership-posture'],
    requiredPermissions: ['admin.groups.read', 'admin.groups.write'],
    navigation: {
      group: 'Workspace Governance',
      order: 40,
      sidebar: true,
    },
    loading: {
      strategy: 'lazy',
      prefetch: 'intent',
      chunkGroup: 'groups',
    },
  },
  {
    moduleId: 'sdkwork-control-plane-announcements',
    pluginId: 'sdkwork-control-plane-announcements',
    pluginKind: 'admin-module',
    packageName: 'sdkwork-control-plane-announcements',
    displayName: 'Announcements',
    routeKeys: ['announcements'],
    capabilityTags: ['broadcast-tasks', 'notice-center', 'delivery-posture'],
    requiredPermissions: ['admin.announcements.read', 'admin.announcements.write'],
    navigation: {
      group: 'Workspace Governance',
      order: 50,
      sidebar: true,
    },
    loading: {
      strategy: 'lazy',
      prefetch: 'intent',
      chunkGroup: 'announcements',
    },
  },
  {
    moduleId: 'sdkwork-control-plane-conversations',
    pluginId: 'sdkwork-control-plane-conversations',
    pluginKind: 'admin-module',
    packageName: 'sdkwork-control-plane-conversations',
    displayName: 'Conversations',
    routeKeys: ['conversations'],
    capabilityTags: ['conversation-lifecycle', 'handoff-posture', 'freeze-governance'],
    requiredPermissions: ['admin.conversations.read', 'admin.conversations.write'],
    navigation: {
      group: 'Conversation Governance',
      order: 60,
      sidebar: true,
    },
    loading: {
      strategy: 'lazy',
      prefetch: 'intent',
      chunkGroup: 'conversations',
    },
  },
  {
    moduleId: 'sdkwork-control-plane-messages',
    pluginId: 'sdkwork-control-plane-messages',
    pluginKind: 'admin-module',
    packageName: 'sdkwork-control-plane-messages',
    displayName: 'Messages',
    routeKeys: ['messages'],
    capabilityTags: ['message-audit', 'transcript-search', 'evidence-export'],
    requiredPermissions: ['admin.messages.read', 'admin.messages.moderate'],
    navigation: {
      group: 'Conversation Governance',
      order: 70,
      sidebar: true,
    },
    loading: {
      strategy: 'lazy',
      prefetch: 'intent',
      chunkGroup: 'messages',
    },
  },
  {
    moduleId: 'sdkwork-control-plane-moderation',
    pluginId: 'sdkwork-control-plane-moderation',
    pluginKind: 'admin-module',
    packageName: 'sdkwork-control-plane-moderation',
    displayName: 'Moderation',
    routeKeys: ['moderation'],
    capabilityTags: ['report-queue', 'keyword-policy', 'risk-escalation'],
    requiredPermissions: ['admin.moderation.read', 'admin.moderation.write'],
    navigation: {
      group: 'Conversation Governance',
      order: 80,
      sidebar: true,
    },
    loading: {
      strategy: 'lazy',
      prefetch: 'intent',
      chunkGroup: 'moderation',
    },
  },
  {
    moduleId: 'sdkwork-control-plane-automation',
    pluginId: 'sdkwork-control-plane-automation',
    pluginKind: 'admin-module',
    packageName: 'sdkwork-control-plane-automation',
    displayName: 'Automation',
    routeKeys: ['automation'],
    capabilityTags: ['bot-registry', 'workflow-automation', 'run-history'],
    requiredPermissions: ['admin.automation.read', 'admin.automation.write'],
    navigation: {
      group: 'Automation',
      order: 90,
      sidebar: true,
    },
    loading: {
      strategy: 'lazy',
      prefetch: 'intent',
      chunkGroup: 'automation',
    },
  },
  {
    moduleId: 'sdkwork-control-plane-realtime',
    pluginId: 'sdkwork-control-plane-realtime',
    pluginKind: 'admin-module',
    packageName: 'sdkwork-control-plane-realtime',
    displayName: 'Realtime',
    routeKeys: ['realtime'],
    capabilityTags: ['realtime-sessions', 'rtc-posture', 'gateway-health'],
    requiredPermissions: ['admin.realtime.read'],
    navigation: {
      group: 'System',
      order: 100,
      sidebar: true,
    },
    loading: {
      strategy: 'lazy',
      prefetch: 'intent',
      chunkGroup: 'realtime',
    },
  },
  {
    moduleId: 'sdkwork-control-plane-system',
    pluginId: 'sdkwork-control-plane-system',
    pluginKind: 'admin-module',
    packageName: 'sdkwork-control-plane-system',
    displayName: 'System',
    routeKeys: ['system'],
    capabilityTags: ['protocol-governance', 'compatibility-matrix', 'runtime-health'],
    requiredPermissions: ['admin.system.read'],
    navigation: {
      group: 'System',
      order: 110,
      sidebar: true,
    },
    loading: {
      strategy: 'lazy',
      prefetch: 'intent',
      chunkGroup: 'system',
    },
  },
  {
    moduleId: 'sdkwork-control-plane-storage',
    pluginId: 'sdkwork-control-plane-storage',
    pluginKind: 'admin-module',
    packageName: 'sdkwork-control-plane-storage',
    displayName: 'Storage',
    routeKeys: ['storage'],
    capabilityTags: ['object-storage', 'tenant-overrides', 'presigned-uploads'],
    requiredPermissions: ['admin.storage.read', 'admin.storage.write'],
    navigation: {
      group: 'System',
      order: 115,
      sidebar: true,
    },
    loading: {
      strategy: 'lazy',
      prefetch: 'intent',
      chunkGroup: 'storage',
    },
  },
  {
    moduleId: 'sdkwork-control-plane-settings',
    pluginId: 'sdkwork-control-plane-settings',
    pluginKind: 'admin-module',
    packageName: 'sdkwork-control-plane-settings',
    displayName: 'Settings',
    routeKeys: ['settings'],
    capabilityTags: ['workspace-preferences', 'shell-settings'],
    requiredPermissions: ['admin.settings.read', 'admin.settings.write'],
    navigation: {
      group: 'System',
      order: 120,
      sidebar: true,
    },
    loading: {
      strategy: 'lazy',
      prefetch: 'intent',
      chunkGroup: 'settings',
    },
  },
];

const adminProductModuleById = Object.fromEntries(
  adminProductModules.map((productModule) => [productModule.moduleId, productModule]),
) as Record<AdminRouteModuleId, AdminProductModuleManifest>;

const adminProductModuleByRouteKey = adminProductModules.reduce(
  (accumulator, productModule) => {
    for (const routeKey of productModule.routeKeys) {
      accumulator[routeKey] = productModule;
    }

    return accumulator;
  },
  {} as Record<AdminRouteKey, AdminProductModuleManifest>,
);

const adminRouteModuleByKey = Object.fromEntries(
  Object.entries(adminProductModuleByRouteKey).map(([routeKey, productModule]) => [
    routeKey,
    productModule.moduleId,
  ]),
) as Record<AdminRouteKey, AdminRouteModuleId>;

export const adminRouteManifest: AdminRouteManifestEntry[] = adminRoutes.map((route) => ({
  ...route,
  path: adminRoutePathByKey[route.key],
  moduleId: adminRouteModuleByKey[route.key],
  prefetchGroup: adminProductModuleByRouteKey[route.key].loading.chunkGroup,
  productModule: adminProductModuleByRouteKey[route.key],
}));

export function resolveAdminPath(routeKey: AdminRouteKey): string {
  return adminRoutePathByKey[routeKey];
}

export function resolveAdminProductModule(
  moduleId: AdminRouteModuleId,
): AdminProductModuleManifest {
  return adminProductModuleById[moduleId];
}
