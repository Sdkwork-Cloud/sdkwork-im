import {
  AdminActionChip,
  AdminEmptyState,
  AdminGuidanceList,
  AdminInsetCard,
  AdminMetricCard,
  AdminPageFrame,
  AdminSectionCard,
  translateAdminAlertDetail,
  translateAdminAlertTitle,
  useAdminI18n,
} from 'sdkwork-control-plane-core';
import type { AdminPageProps } from 'sdkwork-control-plane-types';

export function ModerationPage({ snapshot }: AdminPageProps) {
  const { formatNumber, t } = useAdminI18n();
  const reports = snapshot.alerts.slice(0, 5).map((alert) => ({
    id: alert.id,
    title: translateAdminAlertTitle(alert.title, t),
    detail: translateAdminAlertDetail(alert.detail, t, formatNumber),
    severity: alert.severity,
  }));
  const escalationReady = reports.filter((report) => report.severity === 'high').length;
  const dispositions = [
    {
      id: 'observe',
      title: t('Observe'),
      detail: t('Retain visibility, enrich evidence, and keep the user active while confidence is still low.'),
    },
    {
      id: 'freeze',
      title: t('Freeze'),
      detail: t('Stop new writes immediately when fraud, coercion, or compromise becomes probable.'),
    },
    {
      id: 'ban',
      title: t('Ban'),
      detail: t('Apply only when policy breach is deterministic and the audit trail is complete.'),
    },
  ];

  return (
    <AdminPageFrame
      actions={
        <>
          <AdminActionChip label={t('First response SLA')} />
          <AdminActionChip label={t('Disposition matrix')} tone="warning" />
        </>
      }
      description={t(
        'Moderation is organized around queue clarity, policy coverage, and escalation hygiene so no unsafe event disappears into background noise.',
      )}
      eyebrow={t('Safety operations')}
      rail={
        <div className="space-y-6">
          <AdminSectionCard
            description={t('First response SLA makes sure every new report lands in front of a human or automated owner within the correct risk window.')}
            title={t('First response SLA')}
          >
            <AdminGuidanceList>
              <div>{t('Self-harm, child safety, and active fraud queues target under 3 minutes.')}</div>
              <div>{t('Enterprise abuse and impersonation reports target under 10 minutes.')}</div>
              <div>{t('Low-confidence keyword matches can batch until the next queue sweep without blocking higher-risk work.')}</div>
            </AdminGuidanceList>
          </AdminSectionCard>
          <AdminSectionCard
            description={t('Keyword policy should be versioned, reviewed, and distributed with clear ownership.')}
            title={t('Keyword policy')}
          >
            <AdminGuidanceList>
              <div>{t('High-risk patterns route directly into human review instead of silent suppression.')}</div>
              <div>{t('Region-specific policies can be layered without breaking baseline safety coverage.')}</div>
            </AdminGuidanceList>
          </AdminSectionCard>
          <AdminSectionCard
            description={t('Blocklist posture is reserved for deterministic abuse, fraud, or legal hold scenarios.')}
            title={t('Blocklist')}
          >
            <AdminGuidanceList>
              <div>{t('Temporary blocks expire with case review by default to prevent silent overreach.')}</div>
              <div>{t('Permanent blocks require an audit trail, owner, and reactivation pathway.')}</div>
            </AdminGuidanceList>
          </AdminSectionCard>
        </div>
      }
      title={t('Moderation')}
    >
      <div className="grid gap-4 md:grid-cols-3">
        <AdminMetricCard
          detail={t('Open reports waiting for first response, escalation, or policy confirmation.')}
          label={t('Report queue')}
          value={formatNumber(reports.length)}
        />
        <AdminMetricCard
          detail={t('Signals already triaged by an operator during the current operating window.')}
          label={t('Reviewed today')}
          value={formatNumber(0)}
        />
        <AdminMetricCard
          detail={t('Queues with enough context to move from intake into definitive action.')}
          label={t('Escalation ready')}
          value={formatNumber(escalationReady)}
        />
      </div>

      <AdminSectionCard
        description={t(
          'A disciplined report queue keeps every intake item attributable, reviewable, and easy to escalate.',
        )}
        title={t('Report queue')}
      >
        <div className="space-y-3">
          {reports.length > 0
            ? reports.map((report) => (
                <AdminInsetCard key={report.id}>
                  <div className="text-sm font-semibold text-[var(--admin-text-primary)]">
                    {report.title}
                  </div>
                  <div className="mt-2 text-sm leading-6 text-[var(--admin-text-secondary)]">
                    {report.detail}
                  </div>
                </AdminInsetCard>
              ))
            : (
                <AdminEmptyState
                  detail={t('Alert-backed moderation reports will appear here once the workspace emits safety events.')}
                  title={t('No reports are waiting in the moderation queue.')}
                />
              )}
        </div>
      </AdminSectionCard>

      <AdminSectionCard
        description={t(
          'Disposition matrix keeps the action model consistent so similar reports land in the same operational outcome across shifts.',
        )}
        title={t('Disposition matrix')}
      >
        <div className="grid gap-3 lg:grid-cols-3">
          {dispositions.map((entry) => (
            <AdminInsetCard key={entry.id}>
              <div className="text-sm font-semibold text-[var(--admin-text-primary)]">{entry.title}</div>
              <div className="mt-2 text-sm leading-6 text-[var(--admin-text-secondary)]">{entry.detail}</div>
            </AdminInsetCard>
          ))}
        </div>
      </AdminSectionCard>
    </AdminPageFrame>
  );
}
