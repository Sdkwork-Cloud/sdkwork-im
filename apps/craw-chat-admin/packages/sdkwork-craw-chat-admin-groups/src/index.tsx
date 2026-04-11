import {
  AdminEmptyState,
  AdminGuidanceList,
  AdminInsetCard,
  AdminInsetSplitRow,
  AdminMetricCard,
  AdminPageFrame,
  AdminSectionCard,
  useAdminI18n,
} from 'sdkwork-craw-chat-admin-core';
import type { AdminPageProps } from 'sdkwork-craw-chat-admin-types';

export function GroupsPage({ snapshot }: AdminPageProps) {
  const { formatNumber, t } = useAdminI18n();
  const groups = snapshot.channels.slice(0, 6).map((channel) => ({
    id: channel.id,
    name: channel.name,
  }));

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
          <AdminGuidanceList>
            <div>{t('High-density groups should keep at least one primary moderator and one backup owner.')}</div>
            <div>{t('Broadcast-only groups can be separated from conversational groups when risk rises.')}</div>
            <div>{t('Dormant groups should be archived before they become a blind spot for compliance review.')}</div>
          </AdminGuidanceList>
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
          value={formatNumber(0)}
        />
        <AdminMetricCard
          detail={t('Groups that should remain under active governance during campaign or moderation events.')}
          label={t('Managed groups')}
          value={formatNumber(groups.length)}
        />
      </div>

      <AdminSectionCard
        description={t(
          'The group directory favors clarity over density so operators can judge ownership and membership shape at a glance.',
        )}
        title={t('Group directory')}
      >
        <div className="space-y-3">
          {groups.length > 0
            ? groups.map((group) => (
                <AdminInsetSplitRow key={group.id}>
                  <div>
                    <div className="text-sm font-semibold text-[var(--admin-text-primary)]">
                      {group.name}
                    </div>
                    <div className="mt-1 text-sm text-[var(--admin-text-secondary)]">
                      {t('Group ID: {id}', { id: group.id })}
                    </div>
                  </div>
                  <div className="text-sm text-[var(--admin-text-secondary)]">
                    {t('Membership totals are not reported by the current workspace snapshot.')}
                  </div>
                </AdminInsetSplitRow>
              ))
            : (
                <AdminEmptyState
                  detail={t('Managed groups will appear here once the workspace sync returns channel governance data.')}
                  title={t('No group directory is loaded yet.')}
                />
              )}
        </div>
      </AdminSectionCard>
    </AdminPageFrame>
  );
}
