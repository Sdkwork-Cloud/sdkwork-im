import {
  AdminActionChip,
  AdminEmptyState,
  AdminGuidanceList,
  AdminInsetCard,
  AdminInsetSplitRow,
  AdminMetricCard,
  AdminPageFrame,
  AdminSectionCard,
  resolveAdminProjectLabel,
  useAdminI18n,
} from 'sdkwork-craw-chat-admin-core';
import type { AdminPageProps } from 'sdkwork-craw-chat-admin-types';

export function MessagesPage({ snapshot }: AdminPageProps) {
  const { formatNumber, t } = useAdminI18n();
  const auditRows = snapshot.usageRecords.slice(0, 6).map((record, index) => ({
    id: `${record.project_id}-${record.model}-${index}`,
    transcript: `${t(resolveAdminProjectLabel(record.project_id, snapshot.projects))} / ${record.model}`,
    volume: record.total_tokens,
  }));
  const recallQueue: Array<{ id: string; transcript: string; reason: string }> = [];

  return (
    <AdminPageFrame
      actions={
        <>
          <AdminActionChip label={t('Search transcript')} />
          <AdminActionChip label={t('Export evidence')} tone="success" />
          <AdminActionChip label={t('Recall review')} tone="warning" />
        </>
      }
      description={t(
        'Message audit keeps transcript search, export evidence, and high-signal message review in one defensible operator workflow.',
      )}
      eyebrow={t('Message compliance')}
      rail={
        <div className="space-y-6">
          <AdminSectionCard
            description={t('Evidence exports should be deliberate, limited, and traceable to a clear operator case.')}
            title={t('Export controls')}
          >
            <AdminGuidanceList>
              <div>{t('Search transcript before export so evidence scope is minimized.')}</div>
              <div>{t('Attach retention reason and review ticket before releasing a transcript bundle.')}</div>
              <div>{t('Use evidence export only for moderation, legal hold, or enterprise compliance workflows.')}</div>
            </AdminGuidanceList>
          </AdminSectionCard>
          <AdminSectionCard
            description={t('Retention guardrails stop operators from mixing investigation, legal hold, and routine search workflows.')}
            title={t('Retention guardrails')}
          >
            <AdminGuidanceList>
              <div>{t('Default retention remains searchable but not exportable without a linked case owner.')}</div>
              <div>{t('Legal hold supersedes recall requests until counsel clears the thread.')}</div>
              <div>{t('Expired evidence bundles are deleted on schedule and recreated only from audited searches.')}</div>
            </AdminGuidanceList>
          </AdminSectionCard>
        </div>
      }
      title={t('Messages')}
    >
      <div className="grid gap-4 md:grid-cols-3">
        <AdminMetricCard
          detail={t('Message audit posture across searchable, reviewable transcript slices.')}
          label={t('Message audit')}
          value={formatNumber(auditRows.length)}
        />
        <AdminMetricCard
          detail={t('Transcript segments indexed for quick policy and incident review.')}
          label={t('Indexed transcripts')}
          value={formatNumber(snapshot.usageRecords.length)}
        />
        <AdminMetricCard
          detail={t('Evidence packages prepared for export or downstream compliance tooling.')}
          label={t('Evidence bundles')}
          value={formatNumber(recallQueue.length)}
        />
      </div>

      <AdminSectionCard
        description={t(
          'Search-first review reduces unnecessary exports while still giving operators quick access to high-value evidence.',
        )}
        title={t('Audit queue')}
      >
        <div className="space-y-3">
          {auditRows.length > 0
            ? auditRows.map((row) => (
                <AdminInsetSplitRow key={row.id}>
                  <div className="text-sm font-semibold text-[var(--admin-text-primary)]">
                    {row.transcript}
                  </div>
                  <div className="text-sm text-[var(--admin-text-secondary)]">
                    {t('{count} indexed message units ready for Search transcript review.', {
                      count: formatNumber(row.volume),
                    })}
                  </div>
                </AdminInsetSplitRow>
              ))
            : (
                <AdminEmptyState
                  detail={t('Searchable transcript evidence will appear here once message traffic is indexed.')}
                  title={t('No transcript slices are indexed yet.')}
                />
              )}
        </div>
      </AdminSectionCard>

      <AdminSectionCard
        description={t(
          'Recall review is separate from transcript search so message withdrawal decisions are inspected before they affect evidence quality.',
        )}
        title={t('Recall review')}
      >
        <div className="space-y-3">
          {recallQueue.length > 0
            ? recallQueue.map((entry) => (
                <AdminInsetCard key={entry.id}>
                  <div className="text-sm font-semibold text-[var(--admin-text-primary)]">{entry.transcript}</div>
                  <div className="mt-2 text-sm leading-6 text-[var(--admin-text-secondary)]">{entry.reason}</div>
                </AdminInsetCard>
              ))
            : (
                <AdminEmptyState
                  detail={t('Recall requests will appear here when transcript withdrawal or evidence-hold decisions are reported.')}
                  title={t('No recall reviews are pending.')}
                />
              )}
        </div>
      </AdminSectionCard>
    </AdminPageFrame>
  );
}
