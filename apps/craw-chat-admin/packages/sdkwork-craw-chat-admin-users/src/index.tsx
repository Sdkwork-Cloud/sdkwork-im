import {
  AdminActionChip,
  AdminMetricCard,
  AdminPageFrame,
  AdminSectionCard,
  useAdminI18n,
} from 'sdkwork-craw-chat-admin-core';
import type { AdminPageProps } from 'sdkwork-craw-chat-admin-types';

export function UsersPage({ snapshot }: AdminPageProps) {
  const { formatNumber, t } = useAdminI18n();
  const users = [...snapshot.operatorUsers, ...snapshot.portalUsers].slice(0, 6).map((user) => ({
    id: user.id,
    name: user.display_name,
    email: user.email,
    active: user.active,
  }));
  const visibleUsers =
    users.length > 0
      ? users
      : [
          {
            id: 'operator-01',
            name: 'Nina Xu',
            email: 'nina@example.com',
            active: true,
          },
          {
            id: 'portal-02',
            name: 'Marcus Reed',
            email: 'marcus@example.com',
            active: false,
          },
        ];
  const recoveryQueue = visibleUsers.slice(0, 3).map((user, index) => ({
    id: `recovery-${user.id}`,
    name: user.name,
    reason:
      index === 0
        ? t('Password reset plus device rebind requested after suspicious logout.')
        : index === 1
          ? t('Operator lockout follows repeated OTP drift across mobile and web.')
          : t('Inactive session needs manual reactivation before the next moderation shift.'),
  }));
  const riskWatchlist = visibleUsers.slice(0, 3).map((user, index) => ({
    id: `risk-${user.id}`,
    name: user.name,
    signal:
      index === 0
        ? t('Shared device fingerprint overlaps with previously frozen identities.')
        : index === 1
          ? t('Night-shift access spike exceeds the workspace baseline.')
          : t('Repeated recovery attempts are landing from three network regions.'),
  }));

  return (
    <AdminPageFrame
      actions={
        <>
          <AdminActionChip label={t('Activation')} tone="success" />
          <AdminActionChip label={t('Ban')} tone="warning" />
          <AdminActionChip label={t('Recovery review')} />
        </>
      }
      description={t(
        'Operator identities, portal members, and device readiness are unified here so trust and lifecycle decisions stay auditable.',
      )}
      eyebrow={t('Identity control')}
      rail={
        <div className="space-y-6">
          <AdminSectionCard
            description={t(
              'Device posture calls out the channels most likely to trigger login friction, recovery, or abuse review.',
            )}
            title={t('Device posture')}
          >
            <div className="space-y-3 text-sm text-[var(--admin-text-secondary)]">
              <div>{t('Web sessions remain the dominant support channel during moderation peaks.')}</div>
              <div>{t('Mobile login recovery is the main source of temporary operator escalations.')}</div>
              <div>{t('Desktop review consoles are reserved for heavy evidence and transcript workflows.')}</div>
            </div>
          </AdminSectionCard>
          <AdminSectionCard
            description={t('Risk watchlist keeps operator trust, device reuse, and abnormal recovery behavior on one rail.')}
            title={t('Risk watchlist')}
          >
            <div className="space-y-3">
              {riskWatchlist.map((entry) => (
                <div
                  className="rounded-3xl border border-[var(--admin-border-color)] bg-[var(--admin-content-background)]/60 p-4"
                  key={entry.id}
                >
                  <div className="text-sm font-semibold text-[var(--admin-text-primary)]">{entry.name}</div>
                  <div className="mt-2 text-sm leading-6 text-[var(--admin-text-secondary)]">{entry.signal}</div>
                </div>
              ))}
            </div>
          </AdminSectionCard>
        </div>
      }
      title={t('Users')}
    >
      <div className="grid gap-4 md:grid-cols-3">
        <AdminMetricCard
          detail={t('Operator identities with direct access to the admin workspace.')}
          label={t('Operators')}
          value={formatNumber(snapshot.operatorUsers.length || 18)}
        />
        <AdminMetricCard
          detail={t('Portal accounts participating in messaging, groups, and announcements.')}
          label={t('Portal users')}
          value={formatNumber(snapshot.portalUsers.length || 248)}
        />
        <AdminMetricCard
          detail={t('Device posture remains visible when support, abuse, and activation workflows converge.')}
          label={t('Device posture')}
          value={formatNumber(3)}
        />
      </div>

      <AdminSectionCard
        description={t(
          'The roster is optimized for quick activation checks, deactivation decisions, and downstream handoff to moderation.',
        )}
        title={t('Identity roster')}
      >
        <div className="space-y-3">
          {visibleUsers.map((user) => (
            <div
              className="flex flex-col gap-3 rounded-3xl border border-[var(--admin-border-color)] bg-[var(--admin-content-background)]/60 p-4 md:flex-row md:items-center md:justify-between"
              key={user.id}
            >
              <div>
                <div className="text-sm font-semibold text-[var(--admin-text-primary)]">
                  {user.name}
                </div>
                <div className="mt-1 text-sm text-[var(--admin-text-secondary)]">{user.email}</div>
              </div>
              <div className="text-sm text-[var(--admin-text-secondary)]">
                {user.active ? t('Activation ready') : t('Ban or recovery review')}
              </div>
            </div>
          ))}
        </div>
      </AdminSectionCard>

      <AdminSectionCard
        description={t(
          'Recovery review makes sure resets, rebinds, and operator reactivation all carry a clear reason before access is restored.',
        )}
        title={t('Recovery review')}
      >
        <div className="space-y-3">
          {recoveryQueue.map((entry) => (
            <div
              className="rounded-3xl border border-[var(--admin-border-color)] bg-[var(--admin-content-background)]/60 p-4"
              key={entry.id}
            >
              <div className="text-sm font-semibold text-[var(--admin-text-primary)]">{entry.name}</div>
              <div className="mt-2 text-sm leading-6 text-[var(--admin-text-secondary)]">{entry.reason}</div>
            </div>
          ))}
        </div>
      </AdminSectionCard>
    </AdminPageFrame>
  );
}
