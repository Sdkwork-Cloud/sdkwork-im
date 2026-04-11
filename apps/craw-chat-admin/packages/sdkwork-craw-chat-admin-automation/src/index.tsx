import {
  AdminActionChip,
  AdminEmptyState,
  AdminGuidanceList,
  AdminInsetCard,
  AdminInsetSplitRow,
  AdminMetricCard,
  AdminPageFrame,
  AdminSectionCard,
  resolveAdminRoutingDecisionLabel,
  useAdminI18n,
} from 'sdkwork-craw-chat-admin-core';
import type { AdminPageProps, RoutingDecisionLogRecord } from 'sdkwork-craw-chat-admin-types';

type Translate = (text: string, values?: Record<string, unknown>) => string;

function describeAutomationRun(log: RoutingDecisionLogRecord, translate: Translate) {
  if (log.fallback_reason) {
    return translate('Fallback path engaged: {reason}', { reason: log.fallback_reason });
  }

  if (log.selection_reason) {
    return translate('Selection rationale: {reason}', { reason: log.selection_reason });
  }

  if (log.strategy) {
    return translate('Strategy applied: {strategy}', { strategy: log.strategy });
  }

  return translate('Latest routing decision is available for operator review.');
}

export function AutomationPage({ snapshot }: AdminPageProps) {
  const { formatNumber, t } = useAdminI18n();
  const registry = snapshot.runtimeStatuses.slice(0, 5).map((runtime) => ({
    id: runtime.extension_id,
    name: runtime.display_name,
    status: runtime.running ? t('Running') : t('Standby'),
  }));
  const runHistory = snapshot.routingLogs.slice(0, 3).map((log) => ({
    id: log.decision_id,
    name: t(resolveAdminRoutingDecisionLabel(log.route_key, log.capability)),
    detail: describeAutomationRun(log, t),
  }));
  const workflowActionCount = snapshot.billingEventSummary.capability_count;

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
            <AdminGuidanceList>
              <div>{t('Policy sweeps are scheduled around broadcast and moderation spikes.')}</div>
              <div>{t('Bot-triggered handoffs are logged before they reach human queues.')}</div>
              <div>{t('Replay and retry controls stay operator-owned even when bots initiate the action.')}</div>
            </AdminGuidanceList>
          </AdminSectionCard>
          <AdminSectionCard
            description={t('Retry queue isolates automation that needs human judgment before another run is allowed to touch production queues.')}
            title={t('Retry queue')}
          >
            <AdminGuidanceList>
              <div>{t('Confidence drops, missing ownership, and stale inputs are the main retry blockers.')}</div>
              <div>{t('Retries inherit the original audit trail so operators can compare pre and post states.')}</div>
              <div>{t('High-risk retries remain disabled during active incidents unless an owner explicitly overrides them.')}</div>
            </AdminGuidanceList>
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
          value={formatNumber(snapshot.routingLogs.length)}
        />
        <AdminMetricCard
          detail={t('Registered capabilities that can be attached to lifecycle, moderation, or announcement flows.')}
          label={t('Workflow actions')}
          value={formatNumber(workflowActionCount)}
        />
      </div>

      <AdminSectionCard
        description={t(
          'The registry shows who owns each automation surface and whether it should be trusted during incident response.',
        )}
        title={t('Bot registry')}
      >
        <div className="space-y-3">
          {registry.length > 0
            ? registry.map((entry) => (
                <AdminInsetSplitRow key={entry.id}>
                  <div className="text-sm font-semibold text-[var(--admin-text-primary)]">
                    {entry.name}
                  </div>
                  <div className="text-sm text-[var(--admin-text-secondary)]">{entry.status}</div>
                </AdminInsetSplitRow>
              ))
            : (
                <AdminEmptyState
                  detail={t('Runtime-backed automation surfaces will appear here once the workspace exposes supervised runtimes.')}
                  title={t('No automation registry is reporting yet.')}
                />
              )}
        </div>
      </AdminSectionCard>

      <AdminSectionCard
        description={t(
          'Run history gives operators a clean timeline of automation outcomes before they trust the next workflow recommendation.',
        )}
        title={t('Run history')}
      >
        <div className="space-y-3">
          {runHistory.length > 0
            ? runHistory.map((entry) => (
                <AdminInsetCard key={entry.id}>
                  <div className="text-sm font-semibold text-[var(--admin-text-primary)]">{entry.name}</div>
                  <div className="mt-2 text-sm leading-6 text-[var(--admin-text-secondary)]">{entry.detail}</div>
                </AdminInsetCard>
              ))
            : (
                <AdminEmptyState
                  detail={t('Routing and workflow history will appear here once the workspace emits automation decisions.')}
                  title={t('No automation runs are reported right now.')}
                />
              )}
        </div>
      </AdminSectionCard>
    </AdminPageFrame>
  );
}
