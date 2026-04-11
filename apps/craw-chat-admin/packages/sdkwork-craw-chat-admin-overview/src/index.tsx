import {
  AdminEmptyState,
  AdminInsetCard,
  AdminMetricCard,
  AdminPageFrame,
  AdminSectionCard,
  useAdminI18n,
} from 'sdkwork-craw-chat-admin-core';
import type { AdminPageProps } from 'sdkwork-craw-chat-admin-types';
import {
  buildOverviewModel,
  type OverviewCommandBoardCard,
  type OverviewCopy,
  type OverviewIncident,
  type OverviewTranslationValues,
} from './overviewModel';

function formatTranslationValues(
  values: OverviewTranslationValues | undefined,
  formatNumber: (value: number) => string,
) {
  if (!values) {
    return undefined;
  }

  return Object.fromEntries(
    Object.entries(values).map(([key, value]) => [
      key,
      typeof value === 'number' ? formatNumber(value) : value,
    ]),
  );
}

function resolveCommandBoardToneClasses(tone: OverviewCommandBoardCard['tone']) {
  if (tone === 'success') {
    return 'border-emerald-200 bg-emerald-50 text-emerald-800 dark:border-emerald-900/60 dark:bg-emerald-950/50 dark:text-emerald-200';
  }

  if (tone === 'warning') {
    return 'border-amber-200 bg-amber-50 text-amber-800 dark:border-amber-900/60 dark:bg-amber-950/50 dark:text-amber-200';
  }

  return 'border-sky-200 bg-sky-50 text-sky-800 dark:border-sky-900/60 dark:bg-sky-950/50 dark:text-sky-200';
}

function resolveIncidentClasses(severity: OverviewIncident['severity']) {
  if (severity === 'high') {
    return 'border-rose-200 bg-rose-50/80 text-rose-900 dark:border-rose-900/60 dark:bg-rose-950/40 dark:text-rose-100';
  }

  return 'border-amber-200 bg-amber-50/80 text-amber-900 dark:border-amber-900/60 dark:bg-amber-950/40 dark:text-amber-100';
}

function renderOverviewCopy(
  copy: OverviewCopy,
  t: (text: string, values?: Record<string, unknown>) => string,
  formatNumber: (value: number) => string,
) {
  return t(copy.text, formatTranslationValues(copy.values, formatNumber));
}

export function OverviewPage({ snapshot }: AdminPageProps) {
  const { formatNumber, t } = useAdminI18n();
  const overview = buildOverviewModel(snapshot);

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
              {overview.hotConversations.length > 0
                ? overview.hotConversations.map((conversation) => (
                    <AdminInsetCard key={conversation.id}>
                      <div className="text-sm font-semibold text-[var(--admin-text-primary)]">
                        {conversation.name}
                      </div>
                      <div className="mt-1 text-sm text-[var(--admin-text-secondary)]">
                        {t('{count} requests in the current operating window.', {
                          count: formatNumber(conversation.requests),
                        })}
                      </div>
                      <div className="mt-2 text-sm leading-6 text-[var(--admin-text-secondary)]">
                        {renderOverviewCopy(conversation.detail, t, formatNumber)}
                      </div>
                    </AdminInsetCard>
                  ))
                : (
                    <AdminEmptyState
                      detail={t(overview.hotConversationEmptyState.detail)}
                      title={t(overview.hotConversationEmptyState.title)}
                    />
                  )}
            </div>
          </AdminSectionCard>
          <AdminSectionCard
            description={t(
              'Incident watch keeps the next operator shift aligned on open escalations, frozen queues, and transport risks.',
            )}
            title={t('Incident watch')}
          >
            <div className="space-y-3">
              {overview.incidentWatch.length > 0
                ? overview.incidentWatch.map((incident) => (
                    <div
                      className={`rounded-3xl border p-4 ${resolveIncidentClasses(incident.severity)}`}
                      key={incident.id}
                    >
                      <div className="text-sm font-semibold">
                        {t(incident.title)}
                      </div>
                      <div className="mt-2 text-sm leading-6 opacity-90">
                        {renderOverviewCopy(incident.detail, t, formatNumber)}
                      </div>
                    </div>
                  ))
                : (
                    <AdminEmptyState
                      detail={t(overview.incidentWatchEmptyState.detail)}
                      title={t(overview.incidentWatchEmptyState.title)}
                    />
                  )}
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
          value={formatNumber(overview.messageThroughput)}
        />
        <AdminMetricCard
          detail={t('Open reports awaiting moderator assignment, review, or escalation sign-off.')}
          label={t('Moderation backlog')}
          value={formatNumber(overview.moderationBacklog)}
        />
        <AdminMetricCard
          detail={t('Authenticated operator and portal identities currently active across the workspace.')}
          label={t('Online users')}
          value={formatNumber(overview.onlineUsers)}
        />
        <AdminMetricCard
          detail={t('Live conversation lanes that should remain pinned for operator watch.')}
          label={t('Hot conversations')}
          value={formatNumber(overview.hotConversationCount)}
        />
      </div>

      <AdminSectionCard
        description={t(
          'A focused overview helps operators decide whether to triage safety, traffic, or delivery posture first.',
        )}
        title={t('Command board')}
      >
        <div className="grid gap-3 lg:grid-cols-2 xl:grid-cols-4">
          {overview.commandBoard.map((card) => (
            <div
              className={`rounded-3xl border p-4 ${resolveCommandBoardToneClasses(card.tone)}`}
              key={card.id}
            >
              <div className="text-[11px] font-semibold uppercase tracking-[0.24em] opacity-80">
                {t(card.label)}
              </div>
              <div className="mt-3 text-sm leading-6">
                {renderOverviewCopy(card.detail, t, formatNumber)}
              </div>
            </div>
          ))}
        </div>
      </AdminSectionCard>

      <AdminSectionCard
        description={t(
          'Tenant load helps operators spot which organizations are driving the next queue surge before it spills into moderation or realtime.',
        )}
        title={t('Tenant load')}
      >
        <div className="grid gap-3 lg:grid-cols-3">
          {overview.tenantLoad.length > 0
            ? overview.tenantLoad.map((tenant) => (
                <AdminInsetCard key={tenant.id}>
                  <div className="text-sm font-semibold text-[var(--admin-text-primary)]">
                    {tenant.name}
                  </div>
                  <div className="mt-2 text-sm text-[var(--admin-text-secondary)]">
                    {t('{count} active tenant workspaces are contributing to the current operator load.', {
                      count: formatNumber(tenant.projectCount),
                    })}
                  </div>
                </AdminInsetCard>
              ))
            : (
                <AdminEmptyState
                  className="lg:col-span-3"
                  detail={t(overview.tenantLoadEmptyState.detail)}
                  title={t(overview.tenantLoadEmptyState.title)}
                />
              )}
        </div>
      </AdminSectionCard>
    </AdminPageFrame>
  );
}
