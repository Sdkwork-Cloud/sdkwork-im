import {
  Badge,
  Button,
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from '@sdkwork/ui-pc-react';
import {
  Activity,
  ArrowRight,
  Blocks,
  Monitor,
  ServerCog,
  ShieldCheck,
  TimerReset,
} from 'lucide-react';
import { startTransition } from 'react';
import { useNavigate } from 'react-router-dom';

import {
  AdminEmptyState,
  translateAdminAlertDetail,
  translateAdminAlertTitle,
  useAdminAppStore,
  useAdminI18n,
  useAdminWorkbench,
} from 'sdkwork-control-plane-core';

import { ROUTE_PATHS } from '../application/router/routePaths';
import { ShellStatus } from './ShellStatus';

type PulseLane = {
  count: number;
  description: string;
  path: string;
  title: string;
};

function resolveIncidentSeverityLabel(
  severity: 'high' | 'medium' | 'low',
  translate: (text: string) => string,
) {
  if (severity === 'high') {
    return translate('High severity');
  }

  if (severity === 'medium') {
    return translate('Medium severity');
  }

  return translate('Low severity');
}

export function OperationsPulseDrawer() {
  const navigate = useNavigate();
  const { snapshot, status } = useAdminWorkbench();
  const { formatNumber, t } = useAdminI18n();
  const closeOperationsPulse = useAdminAppStore((state) => state.closeOperationsPulse);
  const isOperationsPulseOpen = useAdminAppStore((state) => state.isOperationsPulseOpen);

  if (!isOperationsPulseOpen) {
    return null;
  }

  const highSeverityAlerts = snapshot.alerts.filter((alert) => alert.severity === 'high').length;
  const unhealthyRuntimes = snapshot.runtimeStatuses.filter((runtime) => !runtime.healthy).length;
  const degradedProviders = snapshot.providerHealth.filter((provider) => !provider.healthy).length;
  const fallbackRoutingLogs = snapshot.routingLogs.filter((log) => log.fallback_reason).length;
  const activeIncidentCount = snapshot.alerts.length;
  const reconnectWatchCount = unhealthyRuntimes + degradedProviders;
  const rolloutRiskCount = reconnectWatchCount;
  const incidents = snapshot.alerts.slice(0, 4).map((alert) => ({
    detail: translateAdminAlertDetail(alert.detail, t, formatNumber),
    id: alert.id,
    severity: alert.severity,
    title: translateAdminAlertTitle(alert.title, t),
  }));

  const handoffCards = [
    {
      detail:
        highSeverityAlerts > 0
          ? t('{count} queues need assignment before the next shift window closes.', {
              count: formatNumber(highSeverityAlerts),
            })
          : t('No shift handoff queues are waiting for assignment right now.'),
      label: t('Moderator handoff'),
    },
    {
      detail:
        reconnectWatchCount > 0
          ? t('{count} realtime edges are carrying degraded or recovery posture.', {
              count: formatNumber(reconnectWatchCount),
            })
          : t('No realtime edges are reporting degraded or recovery posture.'),
      label: t('Realtime guardrail'),
    },
    {
      detail:
        fallbackRoutingLogs > 0
          ? t('{count} automation lanes need human review before the next retry.', {
              count: formatNumber(fallbackRoutingLogs),
            })
          : t('No automation retries are waiting for human review right now.'),
      label: t('Automation retry'),
    },
  ];

  const pulseLanes: PulseLane[] = [
    {
      count: activeIncidentCount,
      description: t('Route directly into the linked module with the current incident context still in view.'),
      path: ROUTE_PATHS.MODERATION,
      title: t('First response SLA'),
    },
    {
      count: reconnectWatchCount,
      description: t('Reconnect watch isolates the edges most likely to amplify reconnect storms so operators can drain or protect them deliberately.'),
      path: ROUTE_PATHS.REALTIME,
      title: t('Reconnect watch'),
    },
    {
      count: fallbackRoutingLogs,
      description: t('Retry queue isolates automation that needs human judgment before another run is allowed to touch production queues.'),
      path: ROUTE_PATHS.AUTOMATION,
      title: t('Retry queue'),
    },
    {
      count: rolloutRiskCount,
      description:
        rolloutRiskCount > 0
          ? t('{count} rollout risks still need protocol or runtime review.', {
              count: formatNumber(rolloutRiskCount),
            })
          : t('No rollout risks currently need protocol or runtime review.'),
      path: ROUTE_PATHS.SYSTEM,
      title: t('Rollout risks'),
    },
  ];

  return (
    <>
      <button
        aria-label={t('Close pulse')}
        className="fixed inset-0 z-40 bg-slate-950/40 backdrop-blur-[2px]"
        onClick={closeOperationsPulse}
        type="button"
      />
      <aside className="fixed inset-y-0 right-0 z-50 flex w-full max-w-xl flex-col border-l border-[var(--admin-border-color)] bg-[var(--admin-content-background)] shadow-[0_40px_120px_rgba(15,23,42,0.34)]">
        <div className="flex items-start justify-between gap-4 border-b border-[var(--admin-border-color)] px-5 py-4">
          <div className="min-w-0 space-y-3">
            <div className="flex flex-wrap items-center gap-2">
              <Badge variant="secondary">{t('Operations pulse')}</Badge>
              <Badge variant="outline">
                {t('{count} active incidents require operator ownership.', {
                  count: formatNumber(activeIncidentCount),
                })}
              </Badge>
            </div>
            <div className="space-y-1">
              <h2 className="text-lg font-semibold text-[var(--admin-text-primary)]">
                {t('Operations pulse')}
              </h2>
              <p className="text-sm leading-6 text-[var(--admin-text-secondary)]">
                {t('Review the live incident stack, shift handoff risk, and escalation routes without leaving the current shell.')}
              </p>
            </div>
            <ShellStatus status={status} />
          </div>
          <Button onClick={closeOperationsPulse} type="button" variant="outline">
            {t('Close pulse')}
          </Button>
        </div>

        <div className="flex-1 space-y-5 overflow-auto px-5 py-5">
          <Card>
            <CardHeader className="pb-3">
              <CardTitle>{t('Incident watch')}</CardTitle>
              <CardDescription>
                {t('Pull open moderation, realtime, automation, and system interventions from one persistent drawer.')}
              </CardDescription>
            </CardHeader>
            <CardContent className="space-y-3">
              {incidents.length > 0
                ? incidents.map((incident) => (
                    <div
                      className="rounded-2xl border border-[var(--admin-border-color)] bg-[var(--admin-content-background)]/70 p-4"
                      key={incident.id}
                    >
                      <div className="flex items-center justify-between gap-3">
                        <div className="text-sm font-semibold text-[var(--admin-text-primary)]">
                          {incident.title}
                        </div>
                        <Badge variant={incident.severity === 'high' ? 'warning' : 'outline'}>
                          {resolveIncidentSeverityLabel(incident.severity, t)}
                        </Badge>
                      </div>
                      <p className="mt-2 text-sm leading-6 text-[var(--admin-text-secondary)]">
                        {incident.detail}
                      </p>
                    </div>
                  ))
                : (
                    <AdminEmptyState
                      className="rounded-2xl bg-[var(--admin-content-background)]/70 p-4"
                      detail={t('Medium- and high-severity alerts will surface here when the workspace reports them.')}
                      title={t('No incidents need handoff right now.')}
                    />
                  )}
            </CardContent>
          </Card>

          <Card>
            <CardHeader className="pb-3">
              <CardTitle>{t('Shift handoff')}</CardTitle>
              <CardDescription>
                {t('Cross-route continuity for moderation, realtime, automation, and rollout posture.')}
              </CardDescription>
            </CardHeader>
            <CardContent className="grid gap-3 md:grid-cols-3">
              {handoffCards.map((card) => (
                <div
                  className="rounded-2xl border border-[var(--admin-border-color)] bg-[var(--admin-content-background)]/70 p-4"
                  key={card.label}
                >
                  <div className="text-xs font-semibold uppercase tracking-[0.16em] text-[var(--admin-text-muted)]">
                    {card.label}
                  </div>
                  <p className="mt-3 text-sm leading-6 text-[var(--admin-text-secondary)]">
                    {card.detail}
                  </p>
                </div>
              ))}
            </CardContent>
          </Card>

          <Card>
            <CardHeader className="pb-3">
              <CardTitle>{t('Attention lanes')}</CardTitle>
              <CardDescription>
                {t('Cross-route continuity for moderation, realtime, automation, and rollout posture.')}
              </CardDescription>
            </CardHeader>
            <CardContent className="space-y-3">
              {pulseLanes.map((lane) => (
                <button
                  className="flex w-full items-start justify-between gap-4 rounded-2xl border border-[var(--admin-border-color)] bg-[var(--admin-content-background)]/70 p-4 text-left transition-colors hover:bg-[var(--admin-header-control-background)]"
                  key={lane.title}
                  onClick={() => {
                    closeOperationsPulse();
                    startTransition(() => {
                      navigate(lane.path);
                    });
                  }}
                  type="button"
                >
                  <div className="min-w-0">
                    <div className="flex items-center gap-2 text-sm font-semibold text-[var(--admin-text-primary)]">
                      {lane.title === t('First response SLA') ? <ShieldCheck className="h-4 w-4" /> : null}
                      {lane.title === t('Reconnect watch') ? <Monitor className="h-4 w-4" /> : null}
                      {lane.title === t('Retry queue') ? <Blocks className="h-4 w-4" /> : null}
                      {lane.title === t('Rollout risks') ? <ServerCog className="h-4 w-4" /> : null}
                      <span>{lane.title}</span>
                    </div>
                    <p className="mt-2 text-sm leading-6 text-[var(--admin-text-secondary)]">
                      {lane.description}
                    </p>
                  </div>
                  <div className="flex shrink-0 items-center gap-3">
                    <span className="rounded-full border border-[var(--admin-border-color)] px-2.5 py-1 text-xs font-semibold text-[var(--admin-text-primary)]">
                      {formatNumber(lane.count)}
                    </span>
                    <ArrowRight className="h-4 w-4 text-[var(--admin-text-muted)]" />
                  </div>
                </button>
              ))}
            </CardContent>
          </Card>

          <div className="grid gap-3 md:grid-cols-2">
            <Card>
              <CardHeader className="pb-2">
                <CardTitle>{t('Active incidents')}</CardTitle>
              </CardHeader>
              <CardContent className="flex items-center gap-3">
                <Activity className="h-4 w-4 text-[var(--admin-text-muted)]" />
                <span className="text-sm text-[var(--admin-text-secondary)]">
                  {t('{count} active incidents require operator ownership.', {
                    count: formatNumber(activeIncidentCount),
                  })}
                </span>
              </CardContent>
            </Card>
            <Card>
              <CardHeader className="pb-2">
                <CardTitle>{t('Attention lanes')}</CardTitle>
              </CardHeader>
              <CardContent className="flex items-center gap-3">
                <TimerReset className="h-4 w-4 text-[var(--admin-text-muted)]" />
                <span className="text-sm text-[var(--admin-text-secondary)]">
                  {t('Pull open moderation, realtime, automation, and system interventions from one persistent drawer.')}
                </span>
              </CardContent>
            </Card>
          </div>
        </div>
      </aside>
    </>
  );
}
