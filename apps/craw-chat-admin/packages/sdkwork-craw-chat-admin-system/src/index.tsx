import {
  AdminActionChip,
  AdminEmptyState,
  AdminGuidanceList,
  AdminInsetCard,
  AdminInsetSplitRow,
  AdminMetricCard,
  AdminPageFrame,
  AdminSectionCard,
  resolveAdminOperatorMessage,
  resolveAdminProviderLabel,
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
  const healthSurfaces = snapshot.runtimeStatuses.length + snapshot.providerHealth.length;
  const rolloutRisks = [
    ...snapshot.runtimeStatuses
      .filter((runtime) => !runtime.healthy)
      .slice(0, 2)
      .map((runtime) => ({
        id: `runtime-${runtime.extension_id}`,
        title: runtime.display_name,
        detail: t(
          resolveAdminOperatorMessage(
            runtime.message,
            'Runtime health is degraded and should remain behind the protocol change gate.',
          ),
        ),
      })),
    ...snapshot.providerHealth
      .filter((provider) => !provider.healthy)
      .slice(0, 2)
      .map((provider) => ({
        id: `provider-${provider.provider_id}`,
        title: t(resolveAdminProviderLabel(provider.provider_id, snapshot.providers)),
        detail: t(
          resolveAdminOperatorMessage(
            provider.message,
            'Provider delivery posture is degraded and should be reviewed before protocol cutovers.',
          ),
        ),
      })),
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
            {healthSurfaces > 0
              ? (
                  <AdminGuidanceList>
                    <div>{t('{count} runtime surfaces are currently reported into the admin workspace.', { count: formatNumber(healthSurfaces) })}</div>
                    <div>{t('Provider health and transport health should remain green before protocol cutovers.')}</div>
                  </AdminGuidanceList>
                )
              : (
                  <AdminEmptyState
                    detail={t('Runtime and provider health signals will appear here once the workspace starts reporting live posture.')}
                    title={t('No runtime or provider health surfaces are reporting yet.')}
                  />
                )}
          </AdminSectionCard>
          <AdminSectionCard
            description={t('Protocol change gate is the release checkpoint that protects clients, transport, and operators from out-of-sync semantics.')}
            title={t('Protocol change gate')}
          >
            <AdminGuidanceList>
              <div>{t('Compatibility matrix, runtime health, and fallback readiness must all pass before a new wire contract opens.')}</div>
              <div>{t('Rollouts are staged by client surface so operator tools never outrun end-user transports.')}</div>
              <div>{t('Emergency reversions stay pre-approved with the previous protocol bundle pinned and ready.')}</div>
            </AdminGuidanceList>
          </AdminSectionCard>
        </div>
      }
      title={t('System')}
    >
      <div className="grid gap-4 md:grid-cols-3">
        <AdminMetricCard
          detail={t('Protocol governance keeps transport, auth, and moderation contracts aligned.')}
          label={t('Protocol governance')}
          value={formatNumber(snapshot.providers.length)}
        />
        <AdminMetricCard
          detail={t('Compatibility matrix coverage across the major client and operator surfaces.')}
          label={t('Compatibility matrix')}
          value={formatNumber(matrix.length)}
        />
        <AdminMetricCard
          detail={t('Runtime nodes and services currently participating in the admin health model.')}
          label={t('Runtime health')}
          value={formatNumber(healthSurfaces)}
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
            <AdminInsetSplitRow key={item.surface}>
              <div className="text-sm font-semibold text-[var(--admin-text-primary)]">
                {item.surface}
              </div>
              <div className="text-sm text-[var(--admin-text-secondary)]">{item.status}</div>
            </AdminInsetSplitRow>
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
          {rolloutRisks.length > 0
            ? rolloutRisks.map((item) => (
                <AdminInsetCard key={item.id}>
                  <div className="text-sm font-semibold text-[var(--admin-text-primary)]">{item.title}</div>
                  <div className="mt-2 text-sm leading-6 text-[var(--admin-text-secondary)]">{item.detail}</div>
                </AdminInsetCard>
              ))
            : (
                <AdminEmptyState
                  detail={t('Runtime and provider risks will appear here when the workspace reports degraded protocol posture.')}
                  title={t('No rollout risks are reported right now.')}
                />
              )}
        </div>
      </AdminSectionCard>
    </AdminPageFrame>
  );
}
