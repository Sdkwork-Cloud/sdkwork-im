import {
  AdminActionChip,
  AdminMetricCard,
  AdminPageFrame,
  AdminSectionCard,
  useAdminI18n,
} from 'sdkwork-craw-chat-admin-core';
import type { AdminPageProps } from 'sdkwork-craw-chat-admin-types';

export function ConversationsPage({ snapshot }: AdminPageProps) {
  const { formatNumber, t } = useAdminI18n();
  const lanes =
    snapshot.usageSummary.projects.length > 0
      ? snapshot.usageSummary.projects.slice(0, 5).map((project) => ({
          id: project.project_id,
          label: project.project_id,
          requests: project.request_count,
        }))
      : [
          { id: 'care', label: 'Customer care', requests: 860 },
          { id: 'vip', label: 'VIP service', requests: 430 },
          { id: 'ops', label: 'Operations war room', requests: 280 },
        ];
  const freezeCandidates = lanes.slice(0, 3).map((lane, index) => ({
    id: `freeze-${lane.id}`,
    label: lane.label,
    reason:
      index === 0
        ? t('Transcript contains open abuse signals and should pause member writes.')
        : index === 1
          ? t('Cross-tenant escalation requires evidence preservation before rerouting.')
          : t('Legal hold requested while the handoff owner is still unresolved.'),
  }));

  return (
    <AdminPageFrame
      actions={
        <>
          <AdminActionChip label={t('Handoff')} />
          <AdminActionChip label={t('Archive')} />
          <AdminActionChip label={t('Freeze')} tone="warning" />
        </>
      }
      description={t(
        'Conversation lifecycle controls help operators move high-risk or high-value threads through assignment, review, archive, and freeze states.',
      )}
      eyebrow={t('Lifecycle governance')}
      rail={
        <div className="space-y-6">
          <AdminSectionCard
            description={t('Pinned lanes keep active handoff and freeze candidates visible without leaving the primary queue view.')}
            title={t('Priority queues')}
          >
            <div className="space-y-3">
              {lanes.map((lane) => (
                <div
                  className="rounded-3xl border border-[var(--admin-border-color)] bg-[var(--admin-content-background)]/60 p-4"
                  key={lane.id}
                >
                  <div className="text-sm font-semibold text-[var(--admin-text-primary)]">
                    {lane.label}
                  </div>
                  <div className="mt-1 text-sm text-[var(--admin-text-secondary)]">
                    {t('{count} active conversation events.', {
                      count: formatNumber(lane.requests),
                    })}
                  </div>
                </div>
              ))}
            </div>
          </AdminSectionCard>
          <AdminSectionCard
            description={t('Handoff SLA separates queues that can auto-route from those needing human assignment within the current shift.')}
            title={t('Handoff SLA')}
          >
            <div className="space-y-3 text-sm text-[var(--admin-text-secondary)]">
              <div>{t('VIP and compliance queues target sub-5-minute first ownership changes.')}</div>
              <div>{t('Standard support threads can wait 15 minutes before escalation is required.')}</div>
              <div>{t('Unowned freeze candidates bypass the normal SLA and page trust operators immediately.')}</div>
            </div>
          </AdminSectionCard>
        </div>
      }
      title={t('Conversations')}
    >
      <div className="grid gap-4 md:grid-cols-3">
        <AdminMetricCard
          detail={t('Threads currently visible to operators for lifecycle review and intervention.')}
          label={t('Conversation lifecycle')}
          value={formatNumber(lanes.length)}
        />
        <AdminMetricCard
          detail={t('Conversations with active routing or assignment changes in progress.')}
          label={t('Handoff queue')}
          value={formatNumber(Math.max(2, Math.floor(lanes.length / 2)))}
        />
        <AdminMetricCard
          detail={t('Conversation records already sealed for evidence or long-term archive retention.')}
          label={t('Archived threads')}
          value={formatNumber(snapshot.routingLogs.length || 24)}
        />
      </div>

      <AdminSectionCard
        description={t(
          'Operators need context-rich lifecycle controls so that archive and freeze actions are deliberate rather than reactive.',
        )}
        title={t('Lifecycle board')}
      >
        <div className="grid gap-3 lg:grid-cols-3">
          <div className="rounded-3xl border border-sky-200 bg-sky-50 p-4 text-sm text-sky-800 dark:border-sky-900/60 dark:bg-sky-950/50 dark:text-sky-200">
            {t('Handoff keeps cross-region service, abuse review, and VIP escalations from stalling.')}
          </div>
          <div className="rounded-3xl border border-zinc-200 bg-zinc-50 p-4 text-sm text-zinc-700 dark:border-zinc-800 dark:bg-zinc-900/70 dark:text-zinc-200">
            {t('Archive policies reduce operator noise once a conversation reaches a stable terminal state.')}
          </div>
          <div className="rounded-3xl border border-amber-200 bg-amber-50 p-4 text-sm text-amber-800 dark:border-amber-900/60 dark:bg-amber-950/50 dark:text-amber-200">
            {t('Freeze is reserved for legal hold, fraud investigation, or severe trust and safety events.')}
          </div>
        </div>
      </AdminSectionCard>

      <AdminSectionCard
        description={t(
          'Freeze candidates are listed separately so operators can preserve evidence and stop new writes before the thread drifts further.',
        )}
        title={t('Freeze candidates')}
      >
        <div className="space-y-3">
          {freezeCandidates.map((candidate) => (
            <div
              className="rounded-3xl border border-[var(--admin-border-color)] bg-[var(--admin-content-background)]/60 p-4"
              key={candidate.id}
            >
              <div className="text-sm font-semibold text-[var(--admin-text-primary)]">{candidate.label}</div>
              <div className="mt-2 text-sm leading-6 text-[var(--admin-text-secondary)]">{candidate.reason}</div>
            </div>
          ))}
        </div>
      </AdminSectionCard>
    </AdminPageFrame>
  );
}
