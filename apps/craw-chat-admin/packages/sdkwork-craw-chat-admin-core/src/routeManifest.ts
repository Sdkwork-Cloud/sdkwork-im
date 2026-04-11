import type {
  AdminRouteKey,
  AdminRouteManifestEntry,
  AdminRouteModuleId,
  AdminProductModuleManifest,
} from 'sdkwork-craw-chat-admin-types';

import { adminRoutePathByKey } from './routePaths';
import { adminRoutes } from './routes';

export const adminProductModules: AdminProductModuleManifest[] = [
  {
    moduleId: 'sdkwork-craw-chat-admin-overview',
    pluginId: 'sdkwork-craw-chat-admin-overview',
    pluginKind: 'admin-module',
    packageName: 'sdkwork-craw-chat-admin-overview',
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
    moduleId: 'sdkwork-craw-chat-admin-tenants',
    pluginId: 'sdkwork-craw-chat-admin-tenants',
    pluginKind: 'admin-module',
    packageName: 'sdkwork-craw-chat-admin-tenants',
    displayName: 'Tenants',
    routeKeys: ['tenants'],
    capabilityTags: ['tenant-governance', 'workspace-governance', 'organization-governance'],
    requiredPermissions: ['admin.tenants.read', 'admin.tenants.write'],
    navigation: {
      group: 'Workspace Ops',
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
    moduleId: 'sdkwork-craw-chat-admin-users',
    pluginId: 'sdkwork-craw-chat-admin-users',
    pluginKind: 'admin-module',
    packageName: 'sdkwork-craw-chat-admin-users',
    displayName: 'Users',
    routeKeys: ['users'],
    capabilityTags: ['portal-identities', 'operator-identities', 'device-posture'],
    requiredPermissions: ['admin.users.read', 'admin.users.write'],
    navigation: {
      group: 'Workspace Ops',
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
    moduleId: 'sdkwork-craw-chat-admin-groups',
    pluginId: 'sdkwork-craw-chat-admin-groups',
    pluginKind: 'admin-module',
    packageName: 'sdkwork-craw-chat-admin-groups',
    displayName: 'Groups',
    routeKeys: ['groups'],
    capabilityTags: ['group-governance', 'channel-governance', 'membership-posture'],
    requiredPermissions: ['admin.groups.read', 'admin.groups.write'],
    navigation: {
      group: 'Workspace Ops',
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
    moduleId: 'sdkwork-craw-chat-admin-announcements',
    pluginId: 'sdkwork-craw-chat-admin-announcements',
    pluginKind: 'admin-module',
    packageName: 'sdkwork-craw-chat-admin-announcements',
    displayName: 'Announcements',
    routeKeys: ['announcements'],
    capabilityTags: ['broadcast-tasks', 'notice-center', 'delivery-posture'],
    requiredPermissions: ['admin.announcements.read', 'admin.announcements.write'],
    navigation: {
      group: 'Workspace Ops',
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
    moduleId: 'sdkwork-craw-chat-admin-conversations',
    pluginId: 'sdkwork-craw-chat-admin-conversations',
    pluginKind: 'admin-module',
    packageName: 'sdkwork-craw-chat-admin-conversations',
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
    moduleId: 'sdkwork-craw-chat-admin-messages',
    pluginId: 'sdkwork-craw-chat-admin-messages',
    pluginKind: 'admin-module',
    packageName: 'sdkwork-craw-chat-admin-messages',
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
    moduleId: 'sdkwork-craw-chat-admin-moderation',
    pluginId: 'sdkwork-craw-chat-admin-moderation',
    pluginKind: 'admin-module',
    packageName: 'sdkwork-craw-chat-admin-moderation',
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
    moduleId: 'sdkwork-craw-chat-admin-automation',
    pluginId: 'sdkwork-craw-chat-admin-automation',
    pluginKind: 'admin-module',
    packageName: 'sdkwork-craw-chat-admin-automation',
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
    moduleId: 'sdkwork-craw-chat-admin-realtime',
    pluginId: 'sdkwork-craw-chat-admin-realtime',
    pluginKind: 'admin-module',
    packageName: 'sdkwork-craw-chat-admin-realtime',
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
    moduleId: 'sdkwork-craw-chat-admin-system',
    pluginId: 'sdkwork-craw-chat-admin-system',
    pluginKind: 'admin-module',
    packageName: 'sdkwork-craw-chat-admin-system',
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
    moduleId: 'sdkwork-craw-chat-admin-settings',
    pluginId: 'sdkwork-craw-chat-admin-settings',
    pluginKind: 'admin-module',
    packageName: 'sdkwork-craw-chat-admin-settings',
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
