import {
  AdminActionChip,
  AdminEmptyState,
  AdminGuidanceList,
  AdminInsetCard,
  AdminMetricCard,
  AdminPageFrame,
  AdminSectionCard,
  resolveAdminProjectLabel,
  useAdminI18n,
} from 'sdkwork-control-plane-core';
import type { AdminPageProps } from 'sdkwork-control-plane-types';

export function ConversationsPage({ snapshot }: AdminPageProps) {
  const { formatNumber, t } = useAdminI18n();
  const lanes = snapshot.usageSummary.projects.slice(0, 5).map((project) => ({
    id: project.project_id,
    label: t(resolveAdminProjectLabel(project.project_id, snapshot.projects)),
    requests: project.request_count,
  }));
  const freezeCandidates: Array<{ id: string; label: string; reason: string }> = [];

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
              {lanes.length > 0
                ? lanes.map((lane) => (
                    <AdminInsetCard key={lane.id}>
                      <div className="text-sm font-semibold text-[var(--admin-text-primary)]">
                        {lane.label}
                      </div>
                      <div className="mt-1 text-sm text-[var(--admin-text-secondary)]">
                        {t('{count} active conversation events.', {
                          count: formatNumber(lane.requests),
                        })}
                      </div>
                    </AdminInsetCard>
                  ))
                : (
                    <AdminEmptyState
                      detail={t('Active conversation lanes will appear here once the workspace reports lifecycle traffic.')}
                      title={t('No priority queues are reporting yet.')}
                    />
                  )}
            </div>
          </AdminSectionCard>
          <AdminSectionCard
            description={t('Handoff SLA separates queues that can auto-route from those needing human assignment within the current shift.')}
            title={t('Handoff SLA')}
          >
            <AdminGuidanceList>
              <div>{t('VIP and compliance queues target sub-5-minute first ownership changes.')}</div>
              <div>{t('Standard support threads can wait 15 minutes before escalation is required.')}</div>
              <div>{t('Unowned freeze candidates bypass the normal SLA and page trust operators immediately.')}</div>
            </AdminGuidanceList>
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
          value={formatNumber(snapshot.routingLogs.length)}
        />
        <AdminMetricCard
          detail={t('Conversation records already sealed for evidence or long-term archive retention.')}
          label={t('Archived threads')}
          value={formatNumber(0)}
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
          {freezeCandidates.length > 0
            ? freezeCandidates.map((candidate) => (
                <AdminInsetCard key={candidate.id}>
                  <div className="text-sm font-semibold text-[var(--admin-text-primary)]">{candidate.label}</div>
                  <div className="mt-2 text-sm leading-6 text-[var(--admin-text-secondary)]">{candidate.reason}</div>
                </AdminInsetCard>
              ))
            : (
                <AdminEmptyState
                  detail={t('Freeze candidates will appear here when trust, legal hold, or evidence-preservation actions are required.')}
                  title={t('No freeze candidates are reported right now.')}
                />
              )}
        </div>
      </AdminSectionCard>
    </AdminPageFrame>
  );
}
