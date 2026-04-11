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
import type { AdminPageProps, MarketingCampaignStatus } from 'sdkwork-craw-chat-admin-types';

function resolveAnnouncementStatusLabel(
  status: MarketingCampaignStatus,
  translate: (value: string) => string,
) {
  if (status === 'active') {
    return translate('Active delivery');
  }

  if (status === 'scheduled') {
    return translate('Scheduled');
  }

  if (status === 'draft') {
    return translate('Draft');
  }

  if (status === 'paused') {
    return translate('Paused');
  }

  if (status === 'ended') {
    return translate('Ended');
  }

  return translate('Archived');
}

export function AnnouncementsPage({ snapshot }: AdminPageProps) {
  const { formatNumber, t } = useAdminI18n();
  const tasks = snapshot.marketingCampaigns.slice(0, 5).map((campaign) => ({
    id: campaign.marketing_campaign_id,
    name: campaign.display_name,
    status: campaign.status,
  }));

  return (
    <AdminPageFrame
      description={t(
        'Broadcast tasks centralize service notices, campaign delivery, and delivery-risk posture so operators can coordinate mass communication with confidence.',
      )}
      eyebrow={t('Outbound comms')}
      rail={
        <AdminSectionCard
          description={t('Delivery posture keeps fan-out quality and operator confidence visible before large announcements ship.')}
          title={t('Delivery posture')}
        >
          <AdminGuidanceList>
            <div>{t('Scheduled tasks are isolated from high-risk queues until policy review is complete.')}</div>
            <div>{t('Delivery failures should promote directly into operator remediation workflows.')}</div>
            <div>{t('Critical notices remain pinned until every priority workspace acknowledges receipt.')}</div>
          </AdminGuidanceList>
        </AdminSectionCard>
      }
      title={t('Announcements')}
    >
      <div className="grid gap-4 md:grid-cols-3">
        <AdminMetricCard
          detail={t('Broadcast jobs currently visible to campaign and operator teams.')}
          label={t('Broadcast tasks')}
          value={formatNumber(tasks.length)}
        />
        <AdminMetricCard
          detail={t('Notices already scheduled for future delivery windows.')}
          label={t('Scheduled')}
          value={formatNumber(tasks.filter((task) => task.status === 'scheduled').length)}
        />
        <AdminMetricCard
          detail={t('Announcements actively moving through live delivery or acknowledgement workflows.')}
          label={t('Active delivery')}
          value={formatNumber(tasks.filter((task) => task.status === 'active').length)}
        />
      </div>

      <AdminSectionCard
        description={t(
          'Operators need visibility into every outbound communication task because announcements shape trust as much as product features do.',
        )}
        title={t('Broadcast tasks')}
      >
        <div className="space-y-3">
          {tasks.length > 0
            ? tasks.map((task) => (
                <AdminInsetSplitRow key={task.id}>
                  <div className="text-sm font-semibold text-[var(--admin-text-primary)]">
                    {task.name}
                  </div>
                  <div className="text-sm capitalize text-[var(--admin-text-secondary)]">
                    {resolveAnnouncementStatusLabel(task.status, t)}
                  </div>
                </AdminInsetSplitRow>
              ))
            : (
                <AdminEmptyState
                  detail={t('Campaign and notice delivery tasks will appear here once outbound communication is configured.')}
                  title={t('No broadcast tasks are active or scheduled right now.')}
                />
              )}
        </div>
      </AdminSectionCard>
    </AdminPageFrame>
  );
}
