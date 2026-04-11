import type { AdminAlert, AdminWorkspaceSnapshot } from 'sdkwork-craw-chat-admin-types';
import { resolveAdminAlertDetailCopy } from 'sdkwork-craw-chat-admin-core';

export type OverviewTranslationValues = Record<string, string | number>;

export interface OverviewCopy {
  text: string;
  values?: OverviewTranslationValues;
}

export interface OverviewConversation {
  id: string;
  name: string;
  requests: number;
  detail: OverviewCopy;
}

export interface OverviewTenantLoad {
  id: string;
  name: string;
  projectCount: number;
}

export interface OverviewIncident {
  id: string;
  title: string;
  detail: OverviewCopy;
  severity: 'high' | 'medium';
}

export interface OverviewCommandBoardCard {
  id: string;
  label: string;
  detail: OverviewCopy;
  tone: 'info' | 'success' | 'warning';
}

export interface OverviewEmptyState {
  title: string;
  detail: string;
}

export interface OverviewModel {
  messageThroughput: number;
  moderationBacklog: number;
  onlineUsers: number;
  hotConversationCount: number;
  hotConversations: OverviewConversation[];
  hotConversationEmptyState: OverviewEmptyState;
  incidentWatch: OverviewIncident[];
  incidentWatchEmptyState: OverviewEmptyState;
  commandBoard: OverviewCommandBoardCard[];
  tenantLoad: OverviewTenantLoad[];
  tenantLoadEmptyState: OverviewEmptyState;
}

function isActionableAlert(alert: AdminAlert): alert is AdminAlert & { severity: 'high' | 'medium' } {
  return alert.severity === 'high' || alert.severity === 'medium';
}

function buildHotConversations(snapshot: AdminWorkspaceSnapshot): OverviewConversation[] {
  const usageRequestsByProject = new Map(
    snapshot.usageSummary.projects.map((project) => [project.project_id, project.request_count]),
  );
  const billingRequestsByProject = new Map(
    snapshot.billingEventSummary.projects.map((project) => [project.project_id, project.request_count]),
  );
  const projectById = new Map(snapshot.projects.map((project) => [project.id, project]));
  const projectIds = new Set<string>([
    ...snapshot.projects.map((project) => project.id),
    ...usageRequestsByProject.keys(),
    ...billingRequestsByProject.keys(),
  ]);

  return Array.from(projectIds)
    .map((projectId) => {
      const requests = Math.max(
        usageRequestsByProject.get(projectId) ?? 0,
        billingRequestsByProject.get(projectId) ?? 0,
      );
      const project = projectById.get(projectId);

      return {
        id: projectId,
        name: project?.name ?? projectId,
        requests,
        detail: {
          text: 'High-volume workspace queue with active moderator attention.',
        },
      };
    })
    .filter((conversation) => conversation.requests > 0)
    .sort(
      (left, right) =>
        right.requests - left.requests || left.name.localeCompare(right.name, 'en'),
    )
    .slice(0, 4);
}

function buildIncidentWatch(snapshot: AdminWorkspaceSnapshot): OverviewIncident[] {
  return snapshot.alerts
    .filter(isActionableAlert)
    .slice(0, 3)
    .map((alert) => ({
      id: alert.id,
      title: alert.title,
      detail: resolveAdminAlertDetailCopy(alert.detail),
      severity: alert.severity,
    }));
}

function buildTenantLoad(snapshot: AdminWorkspaceSnapshot): OverviewTenantLoad[] {
  if (!snapshot.tenants.length && !snapshot.projects.length) {
    return [];
  }

  const tenantById = new Map(snapshot.tenants.map((tenant) => [tenant.id, tenant]));
  const projectCountByTenant = new Map<string, number>();

  for (const project of snapshot.projects) {
    projectCountByTenant.set(
      project.tenant_id,
      (projectCountByTenant.get(project.tenant_id) ?? 0) + 1,
    );
  }

  const tenantIds = new Set<string>([
    ...snapshot.tenants.map((tenant) => tenant.id),
    ...projectCountByTenant.keys(),
  ]);

  return Array.from(tenantIds)
    .map((tenantId) => ({
      id: tenantId,
      name: tenantById.get(tenantId)?.name ?? tenantId,
      projectCount: projectCountByTenant.get(tenantId) ?? 0,
    }))
    .sort(
      (left, right) =>
        right.projectCount - left.projectCount || left.name.localeCompare(right.name, 'en'),
    )
    .slice(0, 4);
}

function buildCommandBoard(snapshot: AdminWorkspaceSnapshot): OverviewCommandBoardCard[] {
  const healthyRuntimeCount = snapshot.runtimeStatuses.filter((runtime) => runtime.healthy).length;
  const degradedRuntimeCount =
    snapshot.runtimeStatuses.length - healthyRuntimeCount
    + snapshot.providerHealth.filter((provider) => !provider.healthy).length;
  const observedRuntimeSurfaceCount =
    snapshot.runtimeStatuses.length + snapshot.providerHealth.length;

  const coveredProviders = new Set(snapshot.credentials.map((credential) => credential.provider_id));
  const missingCredentialCount = snapshot.providers.filter(
    (provider) => !coveredProviders.has(provider.id),
  ).length;
  const trackedBudgetCount = Math.max(
    snapshot.billingSummary.project_count,
    snapshot.billingSummary.projects.length,
  );
  const exhaustedProjectCount = snapshot.billingSummary.exhausted_project_count;
  const activeOrScheduledCampaignCount = snapshot.marketingCampaigns.filter(
    (campaign) => campaign.status === 'active' || campaign.status === 'scheduled',
  ).length;

  return [
    observedRuntimeSurfaceCount === 0
      ? {
          id: 'runtime-posture',
          label: 'Runtime health',
          tone: 'info',
          detail: {
            text: 'No runtime or provider health surfaces are reporting yet.',
          },
        }
      : degradedRuntimeCount > 0
        ? {
            id: 'runtime-posture',
            label: 'Runtime health',
            tone: 'warning',
            detail: {
              text: '{count} runtime or provider health surfaces need operator review.',
              values: { count: degradedRuntimeCount },
            },
          }
        : {
            id: 'runtime-posture',
            label: 'Runtime health',
            tone: 'success',
            detail: {
              text: '{count} runtime and provider health surfaces are reporting stable posture.',
              values: { count: observedRuntimeSurfaceCount },
            },
          },
    snapshot.providers.length === 0
      ? {
          id: 'connector-coverage',
          label: 'Connector coverage',
          tone: 'info',
          detail: {
            text: 'No upstream connectors are configured yet.',
          },
        }
      : missingCredentialCount > 0
        ? {
            id: 'connector-coverage',
            label: 'Connector coverage',
            tone: 'warning',
            detail: {
              text: '{count} upstream connectors have no credential coverage. Rotate or create credentials before opening live message traffic.',
              values: { count: missingCredentialCount },
            },
          }
        : {
            id: 'connector-coverage',
            label: 'Connector coverage',
            tone: 'success',
            detail: {
              text: 'All configured connectors have credential coverage for live message traffic.',
            },
          },
    trackedBudgetCount === 0
      ? {
          id: 'quota-posture',
          label: 'Quota posture',
          tone: 'info',
          detail: {
            text: 'No project budget posture is available yet.',
          },
        }
      : exhaustedProjectCount > 0
        ? {
            id: 'quota-posture',
            label: 'Quota posture',
            tone: 'warning',
            detail: {
              text: '{count} projects have exhausted their traffic budget.',
              values: { count: exhaustedProjectCount },
            },
          }
        : {
            id: 'quota-posture',
            label: 'Quota posture',
            tone: 'success',
            detail: {
              text: 'Tracked project budgets remain within their current traffic limits.',
            },
          },
    activeOrScheduledCampaignCount > 0
      ? {
          id: 'campaign-posture',
          label: 'Campaign posture',
          tone: 'info',
          detail: {
            text: '{count} campaigns are active or scheduled with operator-visible delivery posture.',
            values: { count: activeOrScheduledCampaignCount },
          },
        }
      : snapshot.marketingCampaigns.length > 0
        ? {
            id: 'campaign-posture',
            label: 'Campaign posture',
            tone: 'info',
            detail: {
              text: '{count} campaigns are configured, but none are active or scheduled.',
              values: { count: snapshot.marketingCampaigns.length },
            },
          }
        : {
            id: 'campaign-posture',
            label: 'Campaign posture',
            tone: 'info',
            detail: {
              text: 'No campaigns are active or scheduled right now.',
            },
          },
  ];
}

export function buildOverviewModel(snapshot: AdminWorkspaceSnapshot): OverviewModel {
  const hotConversations = buildHotConversations(snapshot);
  const incidentWatch = buildIncidentWatch(snapshot);
  const tenantLoad = buildTenantLoad(snapshot);

  return {
    messageThroughput: Math.max(
      snapshot.billingEventSummary.total_request_count,
      snapshot.usageSummary.total_requests,
    ),
    moderationBacklog: incidentWatch.length,
    onlineUsers:
      snapshot.portalUsers.filter((user) => user.active).length
      + snapshot.operatorUsers.filter((user) => user.active).length,
    hotConversationCount: hotConversations.length,
    hotConversations,
    hotConversationEmptyState: {
      title: 'No hot conversations are reporting yet.',
      detail: 'Live conversation lanes will pin here once tenant traffic produces observable queue pressure.',
    },
    incidentWatch,
    incidentWatchEmptyState: {
      title: 'No incidents need handoff right now.',
      detail: 'Medium- and high-severity alerts will surface here when the workspace reports them.',
    },
    commandBoard: buildCommandBoard(snapshot),
    tenantLoad,
    tenantLoadEmptyState: {
      title: 'No tenant load is visible yet.',
      detail: 'Tenant and project demand will appear here once the workspace starts producing live traffic.',
    },
  };
}
