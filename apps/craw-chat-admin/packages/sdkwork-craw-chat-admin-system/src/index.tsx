import {
  AdminActionChip,
  AdminMetricCard,
  AdminPageFrame,
  AdminSectionCard,
  useAdminI18n,
} from 'sdkwork-craw-chat-admin-core';
import type { AdminPageProps } from 'sdkwork-craw-chat-admin-types';

export function SystemPage({ snapshot }: AdminPageProps) {
  const { formatNumber, t } = useAdminI18n();
  const matrix = [
    { surface: 'iOS', status: t('Supported') },
    { surface: 'Android', status: t('Supported') },
    { surface: 'Web', status: t('Supported') },
    { surface: 'Desktop', status: t('Pilot') },
  ];
  const rolloutRisks = [
    {
      id: 'risk-1',
      title: t('Desktop pilot lag'),
      detail: t('Desktop clients still trail the latest protocol framing and must remain behind the gate.'),
    },
    {
      id: 'risk-2',
      title: t('Cross-region cutover'),
      detail: t('Route ownership migration depends on clean checkpoint replication before transport flags can flip.'),
    },
    {
      id: 'risk-3',
      title: t('Moderation policy sync'),
      detail: t('Keyword and recall semantics must ship alongside protocol changes to avoid operator mismatch.'),
    },
  ];

  return (
    <AdminPageFrame
      actions={
        <>
          <AdminActionChip label={t('Protocol change gate')} />
          <AdminActionChip label={t('Rollout risks')} tone="warning" />
        </>
      }
      description={t(
        'System governance keeps protocol decisions, compatibility posture, and runtime health visible so product and platform teams share one source of operational truth.',
      )}
      eyebrow={t('Platform governance')}
      rail={
        <div className="space-y-6">
          <AdminSectionCard
            description={t('Protocol changes should only ship when runtime health, compatibility, and fallback posture are all explicit.')}
            title={t('Runtime watch')}
          >
            <div className="space-y-3 text-sm text-[var(--admin-text-secondary)]">
              <div>{t('{count} runtime surfaces are currently reported into the admin workspace.', { count: formatNumber(snapshot.runtimeStatuses.length || 5) })}</div>
              <div>{t('Provider health and transport health should remain green before protocol cutovers.')}</div>
            </div>
          </AdminSectionCard>
          <AdminSectionCard
            description={t('Protocol change gate is the release checkpoint that protects clients, transport, and operators from out-of-sync semantics.')}
            title={t('Protocol change gate')}
          >
            <div className="space-y-3 text-sm text-[var(--admin-text-secondary)]">
              <div>{t('Compatibility matrix, runtime health, and fallback readiness must all pass before a new wire contract opens.')}</div>
              <div>{t('Rollouts are staged by client surface so operator tools never outrun end-user transports.')}</div>
              <div>{t('Emergency reversions stay pre-approved with the previous protocol bundle pinned and ready.')}</div>
            </div>
          </AdminSectionCard>
        </div>
      }
      title={t('System')}
    >
      <div className="grid gap-4 md:grid-cols-3">
        <AdminMetricCard
          detail={t('Protocol governance keeps transport, auth, and moderation contracts aligned.')}
          label={t('Protocol governance')}
          value={formatNumber(Math.max(3, snapshot.providers.length))}
        />
        <AdminMetricCard
          detail={t('Compatibility matrix coverage across the major client and operator surfaces.')}
          label={t('Compatibility matrix')}
          value={formatNumber(matrix.length)}
        />
        <AdminMetricCard
          detail={t('Runtime nodes and services currently participating in the admin health model.')}
          label={t('Runtime health')}
          value={formatNumber(snapshot.runtimeStatuses.length || 5)}
        />
      </div>

      <AdminSectionCard
        description={t(
          'The compatibility matrix prevents product changes from outrunning the clients and operator tools that need to absorb them.',
        )}
        title={t('Compatibility matrix')}
      >
        <div className="space-y-3">
          {matrix.map((item) => (
            <div
              className="flex flex-col gap-3 rounded-3xl border border-[var(--admin-border-color)] bg-[var(--admin-content-background)]/60 p-4 md:flex-row md:items-center md:justify-between"
              key={item.surface}
            >
              <div className="text-sm font-semibold text-[var(--admin-text-primary)]">
                {item.surface}
              </div>
              <div className="text-sm text-[var(--admin-text-secondary)]">{item.status}</div>
            </div>
          ))}
        </div>
      </AdminSectionCard>

      <AdminSectionCard
        description={t(
          'Rollout risks stay visible in the same page as the gate so release decisions are grounded in operational reality rather than optimism.',
        )}
        title={t('Rollout risks')}
      >
        <div className="space-y-3">
          {rolloutRisks.map((item) => (
            <div
              className="rounded-3xl border border-[var(--admin-border-color)] bg-[var(--admin-content-background)]/60 p-4"
              key={item.id}
            >
              <div className="text-sm font-semibold text-[var(--admin-text-primary)]">{item.title}</div>
              <div className="mt-2 text-sm leading-6 text-[var(--admin-text-secondary)]">{item.detail}</div>
            </div>
          ))}
        </div>
      </AdminSectionCard>
    </AdminPageFrame>
  );
}
