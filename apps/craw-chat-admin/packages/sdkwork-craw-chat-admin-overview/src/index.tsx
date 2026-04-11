import {
  AdminMetricCard,
  AdminPageFrame,
  AdminSectionCard,
  useAdminI18n,
} from 'sdkwork-craw-chat-admin-core';
import type { AdminPageProps } from 'sdkwork-craw-chat-admin-types';

export function OverviewPage({ snapshot }: AdminPageProps) {
  const { formatNumber, t } = useAdminI18n();
  const messageThroughput =
    snapshot.billingEventSummary.total_request_count || snapshot.usageSummary.total_requests;
  const moderationBacklog = snapshot.alerts.length || 4;
  const onlineUsers =
    snapshot.portalUsers.filter((user) => user.active).length
    + snapshot.operatorUsers.filter((user) => user.active).length;
  const hotConversations =
    snapshot.usageSummary.projects.length > 0
      ? snapshot.usageSummary.projects.slice(0, 4).map((project) => ({
          id: project.project_id,
          name: project.project_id,
          requests: project.request_count,
          detail: t('High-volume workspace queue with active moderator attention.'),
        }))
      : [
          {
            id: 'escalation-desk',
            name: 'Escalation desk',
            requests: 1840,
            detail: t('Cross-tenant complaints and compliance escalations.'),
          },
          {
            id: 'vip-support',
            name: 'VIP support',
            requests: 1260,
            detail: t('Priority concierge threads with manual handoff SLA.'),
          },
          {
            id: 'creator-hub',
            name: 'Creator hub',
            requests: 980,
            detail: t('Creator onboarding, media approvals, and revenue notices.'),
          },
        ];
  const tenantLoad =
    snapshot.tenants.length > 0
      ? snapshot.tenants.slice(0, 4).map((tenant) => ({
          id: tenant.id,
          name: tenant.name,
          projectCount: snapshot.projects.filter((project) => project.tenant_id === tenant.id).length,
        }))
      : [
          { id: 'tenant-1', name: 'Northstar support cloud', projectCount: 5 },
          { id: 'tenant-2', name: 'Creator network cn', projectCount: 4 },
          { id: 'tenant-3', name: 'Enterprise secure ops', projectCount: 3 },
        ];
  const incidentWatch =
    snapshot.alerts.length > 0
      ? snapshot.alerts.slice(0, 3).map((alert) => ({
          id: alert.id,
          title: alert.title,
          detail: alert.detail,
        }))
      : [
          {
            id: 'incident-1',
            title: 'Shift handoff',
            detail: t('Abuse escalations are rolling into the next operator shift with two frozen conversations pending owner confirmation.'),
          },
          {
            id: 'incident-2',
            title: 'Incident watch',
            detail: t('Realtime transport remains healthy, but VIP queues still need manual oversight before broadcast windows open.'),
          },
        ];

  return (
    <AdminPageFrame
      description={t(
        'Message throughput, moderation posture, and operator hotspots stay visible in one command surface so the IM team can act without switching modules.',
      )}
      eyebrow={t('IM command center')}
      rail={
        <div className="space-y-6">
          <AdminSectionCard
            description={t(
              'The right rail keeps live conversations that require direct operator attention in view.',
            )}
            title={t('Hot conversations')}
          >
            <div className="space-y-3">
              {hotConversations.map((conversation) => (
                <div
                  className="rounded-3xl border border-[var(--admin-border-color)] bg-[var(--admin-content-background)]/60 p-4"
                  key={conversation.id}
                >
                  <div className="text-sm font-semibold text-[var(--admin-text-primary)]">
                    {conversation.name}
                  </div>
                  <div className="mt-1 text-sm text-[var(--admin-text-secondary)]">
                    {t('{count} requests in the current operating window.', {
                      count: formatNumber(conversation.requests),
                    })}
                  </div>
                  <div className="mt-2 text-sm leading-6 text-[var(--admin-text-secondary)]">
                    {conversation.detail}
                  </div>
                </div>
              ))}
            </div>
          </AdminSectionCard>
          <AdminSectionCard
            description={t(
              'Incident watch keeps the next operator shift aligned on open escalations, frozen queues, and transport risks.',
            )}
            title={t('Incident watch')}
          >
            <div className="space-y-3">
              {incidentWatch.map((incident) => (
                <div
                  className="rounded-3xl border border-[var(--admin-border-color)] bg-[var(--admin-content-background)]/60 p-4"
                  key={incident.id}
                >
                  <div className="text-sm font-semibold text-[var(--admin-text-primary)]">
                    {incident.title}
                  </div>
                  <div className="mt-2 text-sm leading-6 text-[var(--admin-text-secondary)]">
                    {incident.detail}
                  </div>
                </div>
              ))}
            </div>
          </AdminSectionCard>
        </div>
      }
      title={t('Overview')}
    >
      <div className="grid gap-4 md:grid-cols-2 xl:grid-cols-4">
        <AdminMetricCard
          detail={t('Rolling admission volume across core messaging, audit, and automation lanes.')}
          label={t('Message throughput')}
          value={formatNumber(messageThroughput || 12480)}
        />
        <AdminMetricCard
          detail={t('Open reports awaiting moderator assignment, review, or escalation sign-off.')}
          label={t('Moderation backlog')}
          value={formatNumber(moderationBacklog)}
        />
        <AdminMetricCard
          detail={t('Authenticated operator and portal identities currently active across the workspace.')}
          label={t('Online users')}
          value={formatNumber(onlineUsers || 312)}
        />
        <AdminMetricCard
          detail={t('Live conversation lanes that should remain pinned for operator watch.')}
          label={t('Hot conversations')}
          value={formatNumber(hotConversations.length)}
        />
      </div>

      <AdminSectionCard
        description={t(
          'A focused overview helps operators decide whether to triage safety, traffic, or delivery posture first.',
        )}
        title={t('Command board')}
      >
        <div className="grid gap-3 lg:grid-cols-3">
          <div className="rounded-3xl border border-emerald-200 bg-emerald-50 p-4 text-sm text-emerald-800 dark:border-emerald-900/60 dark:bg-emerald-950/50 dark:text-emerald-200">
            {t('Session health is stable and the live message plane is accepting sustained traffic.')}
          </div>
          <div className="rounded-3xl border border-amber-200 bg-amber-50 p-4 text-sm text-amber-800 dark:border-amber-900/60 dark:bg-amber-950/50 dark:text-amber-200">
            {t('Shift handoff is carrying two moderation escalations and one evidence-hold thread into the next operating window.')}
          </div>
          <div className="rounded-3xl border border-sky-200 bg-sky-50 p-4 text-sm text-sky-800 dark:border-sky-900/60 dark:bg-sky-950/50 dark:text-sky-200">
            {t('Broadcast and automation systems are available for intervention playbooks if needed.')}
          </div>
        </div>
      </AdminSectionCard>

      <AdminSectionCard
        description={t(
          'Tenant load helps operators spot which organizations are driving the next queue surge before it spills into moderation or realtime.',
        )}
        title={t('Tenant load')}
      >
        <div className="grid gap-3 lg:grid-cols-3">
          {tenantLoad.map((tenant) => (
            <div
              className="rounded-3xl border border-[var(--admin-border-color)] bg-[var(--admin-content-background)]/60 p-4"
              key={tenant.id}
            >
              <div className="text-sm font-semibold text-[var(--admin-text-primary)]">
                {tenant.name}
              </div>
              <div className="mt-2 text-sm text-[var(--admin-text-secondary)]">
                {t('{count} active tenant projects are contributing to the current operator load.', {
                  count: formatNumber(tenant.projectCount),
                })}
              </div>
            </div>
          ))}
        </div>
      </AdminSectionCard>
    </AdminPageFrame>
  );
}
