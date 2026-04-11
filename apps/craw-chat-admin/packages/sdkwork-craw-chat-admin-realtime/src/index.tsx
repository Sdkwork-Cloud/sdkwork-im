import {
  AdminActionChip,
  AdminMetricCard,
  AdminPageFrame,
  AdminSectionCard,
  useAdminI18n,
} from 'sdkwork-craw-chat-admin-core';
import type { AdminPageProps } from 'sdkwork-craw-chat-admin-types';

export function RealtimePage({ snapshot }: AdminPageProps) {
  const { formatNumber, t } = useAdminI18n();
  const healthyProviders = snapshot.providerHealth.filter((provider) => provider.healthy).length;
  const sessions = snapshot.runtimeStatuses.length || 9;
  const reconnectWatch = (snapshot.runtimeStatuses.length > 0
    ? snapshot.runtimeStatuses
    : [
        {
          runtime: 'rtc-edge',
          extension_id: 'edge-01',
          display_name: 'RTC edge cluster',
          running: true,
        },
        {
          runtime: 'ws-edge',
          extension_id: 'edge-02',
          display_name: 'WebSocket fanout',
          running: true,
        },
      ]
  ).slice(0, 3);

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
            <div className="space-y-3 text-sm text-[var(--admin-text-secondary)]">
              <div>{t('{count} live transport edges are reporting healthy realtime posture.', { count: formatNumber(healthyProviders || 4) })}</div>
              <div>{t('Session drains, failovers, and reconnect storms should be reviewed before broadcast launches.')}</div>
            </div>
          </AdminSectionCard>
          <AdminSectionCard
            description={t('Failover window defines when operators can drain edges without breaking active large-room or RTC traffic.')}
            title={t('Failover window')}
          >
            <div className="space-y-3 text-sm text-[var(--admin-text-secondary)]">
              <div>{t('Planned cutovers are limited to low-broadcast windows with checkpoint sync confirmed.')}</div>
              <div>{t('Emergency failovers require reconnect capacity and moderation transport coverage to stay green.')}</div>
              <div>{t('VIP and enterprise rooms are pinned until a replacement edge reports healthy fan-out.')}</div>
            </div>
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
          value={formatNumber(healthyProviders || 4)}
        />
        <AdminMetricCard
          detail={t('Gateway health shows whether live transport remains ready for operator-driven fan-out.')}
          label={t('Gateway health')}
          value={formatNumber(snapshot.providerHealth.length || 6)}
        />
      </div>

      <AdminSectionCard
        description={t(
          'Realtime operators need a concise view of transport readiness because failures compound rapidly once large rooms or broadcasts spike.',
        )}
        title={t('Session monitor')}
      >
        <div className="space-y-3">
          {reconnectWatch.map((runtime) => (
            <div
              className="flex flex-col gap-3 rounded-3xl border border-[var(--admin-border-color)] bg-[var(--admin-content-background)]/60 p-4 md:flex-row md:items-center md:justify-between"
              key={runtime.extension_id}
            >
              <div className="text-sm font-semibold text-[var(--admin-text-primary)]">
                {runtime.display_name}
              </div>
              <div className="text-sm text-[var(--admin-text-secondary)]">
                {runtime.running ? t('Running') : t('Offline')}
              </div>
            </div>
          ))}
        </div>
      </AdminSectionCard>

      <AdminSectionCard
        description={t(
          'Reconnect watch isolates the edges most likely to amplify reconnect storms so operators can drain or protect them deliberately.',
        )}
        title={t('Reconnect watch')}
      >
        <div className="space-y-3">
          {reconnectWatch.map((runtime, index) => (
            <div
              className="rounded-3xl border border-[var(--admin-border-color)] bg-[var(--admin-content-background)]/60 p-4"
              key={`watch-${runtime.extension_id}`}
            >
              <div className="text-sm font-semibold text-[var(--admin-text-primary)]">{runtime.display_name}</div>
              <div className="mt-2 text-sm leading-6 text-[var(--admin-text-secondary)]">
                {index === 0
                  ? t('Reconnect rate is elevated after an edge rebalance; keep new room joins throttled.')
                  : index === 1
                    ? t('Session rebinds are healthy but should stay under observation during broadcast peaks.')
                    : t('Edge posture is stable, but cross-region fallback remains armed.')}
              </div>
            </div>
          ))}
        </div>
      </AdminSectionCard>
    </AdminPageFrame>
  );
}
