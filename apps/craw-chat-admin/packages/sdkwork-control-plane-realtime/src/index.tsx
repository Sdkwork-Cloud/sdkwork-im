import {
  AdminActionChip,
  AdminEmptyState,
  AdminGuidanceList,
  AdminInsetCard,
  AdminInsetSplitRow,
  AdminMetricCard,
  AdminPageFrame,
  AdminSectionCard,
  useAdminI18n,
} from 'sdkwork-control-plane-core';
import type { AdminPageProps, RuntimeStatusRecord } from 'sdkwork-control-plane-types';

function resolveReconnectDetail(
  runtime: RuntimeStatusRecord,
  translate: (text: string) => string,
) {
  if (!runtime.running) {
    return translate('Runtime is offline and needs operator review before new session load is routed here.');
  }

  if (!runtime.healthy) {
    return translate('Runtime is running with degraded health and should remain under reconnect watch.');
  }

  return translate('Runtime is healthy and currently not reporting reconnect pressure.');
}

export function RealtimePage({ snapshot }: AdminPageProps) {
  const { formatNumber, t } = useAdminI18n();
  const healthyProviders = snapshot.providerHealth.filter((provider) => provider.healthy).length;
  const healthyRuntimes = snapshot.runtimeStatuses.filter((runtime) => runtime.healthy).length;
  const degradedProviders = snapshot.providerHealth.length - healthyProviders;
  const sessions = snapshot.runtimeStatuses.length;
  const reconnectWatch = snapshot.runtimeStatuses.slice(0, 3);

  return (
    <AdminPageFrame
      actions={
        <>
          <AdminActionChip label={t('Reconnect watch')} />
          <AdminActionChip label={t('Failover window')} tone="warning" />
        </>
      }
      description={t(
        'Realtime posture surfaces live session handling, RTC coverage, and delivery availability so operators can intervene before latency or packet loss compounds.',
      )}
      eyebrow={t('Live transport')}
      rail={
        <div className="space-y-6">
          <AdminSectionCard
            description={t('Gateway health remains pinned because realtime incidents are usually transport incidents first.')}
            title={t('Gateway health')}
          >
            {snapshot.providerHealth.length > 0
              ? (
                  <AdminGuidanceList>
                    <div>
                      {degradedProviders > 0
                        ? t('{count} gateway providers still need transport review.', {
                            count: formatNumber(degradedProviders),
                          })
                        : t('{count} gateway providers are reporting healthy delivery posture.', {
                          count: formatNumber(healthyProviders),
                        })}
                    </div>
                    <div>{t('Session drains, failovers, and reconnect storms should be reviewed before broadcast launches.')}</div>
                  </AdminGuidanceList>
                )
              : (
                  <AdminEmptyState
                    detail={t('Gateway coverage will appear here once transport providers start reporting live health.')}
                    title={t('No gateway health signals are reporting yet.')}
                  />
                )}
          </AdminSectionCard>
          <AdminSectionCard
            description={t('Failover window defines when operators can drain edges without breaking active large-room or RTC traffic.')}
            title={t('Failover window')}
          >
            <AdminGuidanceList>
              <div>{t('Planned cutovers are limited to low-broadcast windows with checkpoint sync confirmed.')}</div>
              <div>{t('Emergency failovers require reconnect capacity and moderation transport coverage to stay green.')}</div>
              <div>{t('VIP and enterprise rooms are pinned until a replacement edge reports healthy fan-out.')}</div>
            </AdminGuidanceList>
          </AdminSectionCard>
        </div>
      }
      title={t('Realtime')}
    >
      <div className="grid gap-4 md:grid-cols-3">
        <AdminMetricCard
          detail={t('Realtime sessions currently visible to the operator shell.')}
          label={t('Realtime sessions')}
          value={formatNumber(sessions)}
        />
        <AdminMetricCard
          detail={t('RTC posture tracks live signaling and media-layer readiness.')}
          label={t('RTC posture')}
          value={formatNumber(healthyRuntimes)}
        />
        <AdminMetricCard
          detail={t('Gateway health shows whether live transport remains ready for operator-driven fan-out.')}
          label={t('Gateway health')}
          value={formatNumber(snapshot.providerHealth.length)}
        />
      </div>

      <AdminSectionCard
        description={t(
          'Realtime operators need a concise view of transport readiness because failures compound rapidly once large rooms or broadcasts spike.',
        )}
        title={t('Session monitor')}
      >
        <div className="space-y-3">
          {reconnectWatch.length > 0
            ? reconnectWatch.map((runtime) => (
                <AdminInsetSplitRow key={runtime.extension_id}>
                  <div className="text-sm font-semibold text-[var(--admin-text-primary)]">
                    {runtime.display_name}
                  </div>
                  <div className="text-sm text-[var(--admin-text-secondary)]">
                    {runtime.running ? t('Running') : t('Offline')}
                  </div>
                </AdminInsetSplitRow>
              ))
            : (
                <AdminEmptyState
                  detail={t('Runtime edges will appear here once the workspace publishes transport session status.')}
                  title={t('No live runtime sessions are reporting yet.')}
                />
              )}
        </div>
      </AdminSectionCard>

      <AdminSectionCard
        description={t(
          'Reconnect watch isolates the edges most likely to amplify reconnect storms so operators can drain or protect them deliberately.',
        )}
        title={t('Reconnect watch')}
      >
        <div className="space-y-3">
          {reconnectWatch.length > 0
            ? reconnectWatch.map((runtime) => (
                <AdminInsetCard key={`watch-${runtime.extension_id}`}>
                  <div className="text-sm font-semibold text-[var(--admin-text-primary)]">{runtime.display_name}</div>
                  <div className="mt-2 text-sm leading-6 text-[var(--admin-text-secondary)]">
                    {resolveReconnectDetail(runtime, t)}
                  </div>
                </AdminInsetCard>
              ))
            : (
                <AdminEmptyState
                  detail={t('Reconnect risks will appear here when runtime edges expose recovery or failover pressure.')}
                  title={t('No reconnect risks are reported right now.')}
                />
              )}
        </div>
      </AdminSectionCard>
    </AdminPageFrame>
  );
}
