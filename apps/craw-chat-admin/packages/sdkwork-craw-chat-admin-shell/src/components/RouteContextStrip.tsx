import { Badge, Button } from '@sdkwork/ui-pc-react';
import { Activity, Search, Settings2 } from 'lucide-react';
import { startTransition } from 'react';
import { useLocation, useNavigate } from 'react-router-dom';

import {
  adminRouteKeyFromPathname,
  adminRouteManifest,
  resolveAdminPermissionLabel,
  useAdminAppStore,
  useAdminI18n,
} from 'sdkwork-craw-chat-admin-core';
import type { AdminRouteKey } from 'sdkwork-craw-chat-admin-types';

import { prefetchSidebarRoute } from '../application/router/routePrefetch';
import { ROUTE_PATHS } from '../application/router/routePaths';

const continuityCueByRoute: Record<AdminRouteKey, string> = {
  overview:
    'Keep the command board visible so throughput, moderation backlog, and operator load stay aligned before queues spill into other modules.',
  tenants:
    'Tenant posture changes should be reconciled before they widen queue pressure, permissions drift, or workspace delivery issues.',
  users:
    'Recovery review and risk watchlist decisions should stay visible while operator identities, bans, and device posture are being changed.',
  conversations:
    'Handoff SLA and freeze candidates should remain explicit while operators rebalance queue ownership and archive decisions.',
  messages:
    'Retention guardrails and recall review should stay visible whenever transcript search, evidence export, or audit intervention is in motion.',
  groups:
    'Membership posture should stay in view while operators adjust channel governance, group ownership, and broadcast access.',
  moderation:
    'First response SLA, escalation ownership, and evidence posture should remain visible while reports are triaged or policies are tightened.',
  automation:
    'Retry queue posture should remain explicit before another automation run is allowed to touch production queues or customer-facing flows.',
  announcements:
    'Delivery posture should stay visible while broadcasts are approved, staged, or paused for sensitive tenant cohorts.',
  realtime:
    'Reconnect watch should remain pinned while degraded edges, failover windows, and live session recovery are being coordinated.',
  system:
    'Rollout risks and protocol change gates should remain explicit while runtime health or compatibility posture is being adjusted.',
  settings:
    'Shell posture changes affect the Command center, Operations pulse, and route visibility immediately across the operator workspace.',
};

function formatCapabilityTag(tag: string) {
  return tag.replaceAll('-', ' ');
}

export function RouteContextStrip() {
  const location = useLocation();
  const navigate = useNavigate();
  const openCommandPalette = useAdminAppStore((state) => state.openCommandPalette);
  const openOperationsPulse = useAdminAppStore((state) => state.openOperationsPulse);
  const { t } = useAdminI18n();
  const activeRouteKey = adminRouteKeyFromPathname(location.pathname);

  if (!activeRouteKey) {
    return null;
  }

  const activeRoute = adminRouteManifest.find((route) => route.key === activeRouteKey);
  if (!activeRoute) {
    return null;
  }

  const settingsTarget = `${ROUTE_PATHS.SETTINGS}?tab=navigation&query=${encodeURIComponent(
    activeRoute.label,
  )}`;
  const continuityCue = continuityCueByRoute[activeRoute.key];

  return (
    <section className="border-b border-[var(--admin-border-color)] bg-[var(--admin-content-background)]/70 backdrop-blur-xl">
      <div className="flex flex-col gap-4 px-4 py-4 lg:flex-row lg:items-start lg:justify-between">
        <div className="min-w-0 flex-1 space-y-4">
          <div className="flex flex-wrap items-center gap-2">
            <Badge variant="outline">{t(activeRoute.group ?? 'Operations')}</Badge>
            <Badge variant="secondary">{t(activeRoute.eyebrow)}</Badge>
            <Badge variant="outline">{t('Continuity cue')}</Badge>
          </div>

          <div className="space-y-1">
            <div className="text-base font-semibold text-[var(--admin-text-primary)]">
              {t(activeRoute.label)}
            </div>
            <p className="max-w-4xl text-sm leading-6 text-[var(--admin-text-secondary)]">
              {t(activeRoute.detail)}
            </p>
          </div>

          <div className="grid gap-4 xl:grid-cols-[minmax(0,1.2fr)_minmax(0,1fr)]">
            <div className="rounded-3xl border border-[var(--admin-border-color)] bg-[var(--admin-header-control-background)]/70 p-4">
              <div className="text-[11px] font-semibold uppercase tracking-[0.18em] text-[var(--admin-text-muted)]">
                {t('Continuity cue')}
              </div>
              <p className="mt-3 text-sm leading-6 text-[var(--admin-text-secondary)]">
                {t(continuityCue)}
              </p>
            </div>

            <div className="grid gap-4 md:grid-cols-2">
              <div className="rounded-3xl border border-[var(--admin-border-color)] bg-[var(--admin-header-control-background)]/70 p-4">
                <div className="text-[11px] font-semibold uppercase tracking-[0.18em] text-[var(--admin-text-muted)]">
                  {t('Capability tags')}
                </div>
                <div className="mt-3 flex flex-wrap gap-2">
                  {activeRoute.productModule.capabilityTags.map((tag) => (
                    <Badge key={tag} variant="outline">
                      {t(formatCapabilityTag(tag))}
                    </Badge>
                  ))}
                </div>
              </div>

              <div className="rounded-3xl border border-[var(--admin-border-color)] bg-[var(--admin-header-control-background)]/70 p-4">
                <div className="text-[11px] font-semibold uppercase tracking-[0.18em] text-[var(--admin-text-muted)]">
                  {t('Required permissions')}
                </div>
                <div className="mt-3 flex flex-wrap gap-2">
                  {activeRoute.productModule.requiredPermissions.map((permission) => (
                    <Badge key={permission} title={permission} variant="outline">
                      {t(resolveAdminPermissionLabel(permission))}
                    </Badge>
                  ))}
                </div>
              </div>
            </div>
          </div>
        </div>

        <div className="flex shrink-0 flex-wrap items-center gap-2">
          <Button onClick={() => openCommandPalette(activeRoute.label)} type="button" variant="outline">
            <Search className="mr-2 h-4 w-4" />
            {t('Open command center')}
          </Button>
          <Button onClick={openOperationsPulse} type="button" variant="outline">
            <Activity className="mr-2 h-4 w-4" />
            {t('Open operations pulse')}
          </Button>
          <Button
            onClick={() => {
              prefetchSidebarRoute(ROUTE_PATHS.SETTINGS);
              startTransition(() => {
                navigate(settingsTarget);
              });
            }}
            type="button"
            variant="outline"
          >
            <Settings2 className="mr-2 h-4 w-4" />
            {t('Open settings center')}
          </Button>
        </div>
      </div>
    </section>
  );
}
