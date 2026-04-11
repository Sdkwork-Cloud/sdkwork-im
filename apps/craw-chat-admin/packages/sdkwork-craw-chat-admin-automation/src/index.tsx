import {
  AdminActionChip,
  AdminMetricCard,
  AdminPageFrame,
  AdminSectionCard,
  useAdminI18n,
} from 'sdkwork-craw-chat-admin-core';
import type { AdminPageProps } from 'sdkwork-craw-chat-admin-types';

export function AutomationPage({ snapshot }: AdminPageProps) {
  const { formatNumber, t } = useAdminI18n();
  const registry =
    snapshot.runtimeStatuses.length > 0
      ? snapshot.runtimeStatuses.slice(0, 5).map((runtime) => ({
          id: runtime.extension_id,
          name: runtime.display_name,
          status: runtime.running ? t('Running') : t('Standby'),
        }))
      : [
          { id: 'bot-routing', name: 'Queue triage bot', status: t('Running') },
          { id: 'bot-escalation', name: 'Escalation bot', status: t('Standby') },
        ];
  const runHistory = registry.slice(0, 3).map((entry, index) => ({
    id: `run-${entry.id}`,
    name: entry.name,
    detail:
      index === 0
        ? t('Completed queue hygiene sweep for high-priority inboxes without manual overrides.')
        : index === 1
          ? t('Paused after confidence dropped below the moderation threshold.')
          : t('Finished announcement targeting refresh and queued a follow-up diff.'),
  }));

  return (
    <AdminPageFrame
      actions={
        <>
          <AdminActionChip label={t('Run history')} />
          <AdminActionChip label={t('Retry queue')} tone="warning" />
        </>
      }
      description={t(
        'Automation should feel deliberate and inspectable, with bot ownership and execution history available before any operator trusts an action path.',
      )}
      eyebrow={t('Workflow orchestration')}
      rail={
        <div className="space-y-6">
          <AdminSectionCard
            description={t('Automation runs remain useful only when outcome, trigger, and owner are explicit.')}
            title={t('Automation runs')}
          >
            <div className="space-y-3 text-sm text-[var(--admin-text-secondary)]">
              <div>{t('Policy sweeps are scheduled around broadcast and moderation spikes.')}</div>
              <div>{t('Bot-triggered handoffs are logged before they reach human queues.')}</div>
              <div>{t('Replay and retry controls stay operator-owned even when bots initiate the action.')}</div>
            </div>
          </AdminSectionCard>
          <AdminSectionCard
            description={t('Retry queue isolates automation that needs human judgment before another run is allowed to touch production queues.')}
            title={t('Retry queue')}
          >
            <div className="space-y-3 text-sm text-[var(--admin-text-secondary)]">
              <div>{t('Confidence drops, missing ownership, and stale inputs are the main retry blockers.')}</div>
              <div>{t('Retries inherit the original audit trail so operators can compare pre and post states.')}</div>
              <div>{t('High-risk retries remain disabled during active incidents unless an owner explicitly overrides them.')}</div>
            </div>
          </AdminSectionCard>
        </div>
      }
      title={t('Automation')}
    >
      <div className="grid gap-4 md:grid-cols-3">
        <AdminMetricCard
          detail={t('Bots and workflow engines currently registered for operator-supervised use.')}
          label={t('Bot registry')}
          value={formatNumber(registry.length)}
        />
        <AdminMetricCard
          detail={t('Live automation runs that are actively shaping queue posture or operator tasks.')}
          label={t('Automation runs')}
          value={formatNumber(Math.max(2, snapshot.runtimeStatuses.length))}
        />
        <AdminMetricCard
          detail={t('Registered capabilities that can be attached to lifecycle, moderation, or announcement flows.')}
          label={t('Workflow actions')}
          value={formatNumber(Math.max(6, snapshot.providers.length))}
        />
      </div>

      <AdminSectionCard
        description={t(
          'The registry shows who owns each automation surface and whether it should be trusted during incident response.',
        )}
        title={t('Bot registry')}
      >
        <div className="space-y-3">
          {registry.map((entry) => (
            <div
              className="flex flex-col gap-3 rounded-3xl border border-[var(--admin-border-color)] bg-[var(--admin-content-background)]/60 p-4 md:flex-row md:items-center md:justify-between"
              key={entry.id}
            >
              <div className="text-sm font-semibold text-[var(--admin-text-primary)]">
                {entry.name}
              </div>
              <div className="text-sm text-[var(--admin-text-secondary)]">{entry.status}</div>
            </div>
          ))}
        </div>
      </AdminSectionCard>

      <AdminSectionCard
        description={t(
          'Run history gives operators a clean timeline of automation outcomes before they trust the next workflow recommendation.',
        )}
        title={t('Run history')}
      >
        <div className="space-y-3">
          {runHistory.map((entry) => (
            <div
              className="rounded-3xl border border-[var(--admin-border-color)] bg-[var(--admin-content-background)]/60 p-4"
              key={entry.id}
            >
              <div className="text-sm font-semibold text-[var(--admin-text-primary)]">{entry.name}</div>
              <div className="mt-2 text-sm leading-6 text-[var(--admin-text-secondary)]">{entry.detail}</div>
            </div>
          ))}
        </div>
      </AdminSectionCard>
    </AdminPageFrame>
  );
}
