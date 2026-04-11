import {
  AdminMetricCard,
  AdminPageFrame,
  AdminSectionCard,
  useAdminI18n,
} from 'sdkwork-craw-chat-admin-core';
import type { AdminPageProps } from 'sdkwork-craw-chat-admin-types';

export function GroupsPage({ snapshot }: AdminPageProps) {
  const { formatNumber, t } = useAdminI18n();
  const groups =
    snapshot.channels.length > 0
      ? snapshot.channels.slice(0, 6).map((channel, index) => ({
          id: channel.id,
          name: channel.name,
          members: 40 + index * 12,
        }))
      : [
          { id: 'grp-001', name: 'Product champions', members: 128 },
          { id: 'grp-002', name: 'Creator moderation', members: 92 },
          { id: 'grp-003', name: 'Regional support leads', members: 56 },
        ];

  return (
    <AdminPageFrame
      description={t(
        'Groups are treated as governed spaces with explicit ownership, member density, and moderation visibility rather than unmanaged channel clutter.',
      )}
      eyebrow={t('Community governance')}
      rail={
        <AdminSectionCard
          description={t('Membership posture stays healthy when ownership, review cadence, and pinned operators remain explicit.')}
          title={t('Membership posture')}
        >
          <div className="space-y-3 text-sm text-[var(--admin-text-secondary)]">
            <div>{t('High-density groups should keep at least one primary moderator and one backup owner.')}</div>
            <div>{t('Broadcast-only groups can be separated from conversational groups when risk rises.')}</div>
            <div>{t('Dormant groups should be archived before they become a blind spot for compliance review.')}</div>
          </div>
        </AdminSectionCard>
      }
      title={t('Groups')}
    >
      <div className="grid gap-4 md:grid-cols-3">
        <AdminMetricCard
          detail={t('Group directory volume currently visible to the operator shell.')}
          label={t('Group directory')}
          value={formatNumber(groups.length)}
        />
        <AdminMetricCard
          detail={t('Members participating across the most active managed groups.')}
          label={t('Membership posture')}
          value={formatNumber(groups.reduce((sum, group) => sum + group.members, 0))}
        />
        <AdminMetricCard
          detail={t('Groups that should remain under active governance during campaign or moderation events.')}
          label={t('Managed groups')}
          value={formatNumber(Math.max(3, groups.length - 1))}
        />
      </div>

      <AdminSectionCard
        description={t(
          'The group directory favors clarity over density so operators can judge ownership and membership shape at a glance.',
        )}
        title={t('Group directory')}
      >
        <div className="space-y-3">
          {groups.map((group) => (
            <div
              className="flex flex-col gap-3 rounded-3xl border border-[var(--admin-border-color)] bg-[var(--admin-content-background)]/60 p-4 md:flex-row md:items-center md:justify-between"
              key={group.id}
            >
              <div>
                <div className="text-sm font-semibold text-[var(--admin-text-primary)]">
                  {group.name}
                </div>
                <div className="mt-1 text-sm text-[var(--admin-text-secondary)]">{group.id}</div>
              </div>
              <div className="text-sm text-[var(--admin-text-secondary)]">
                {t('{count} active members under review.', {
                  count: formatNumber(group.members),
                })}
              </div>
            </div>
          ))}
        </div>
      </AdminSectionCard>
    </AdminPageFrame>
  );
}
