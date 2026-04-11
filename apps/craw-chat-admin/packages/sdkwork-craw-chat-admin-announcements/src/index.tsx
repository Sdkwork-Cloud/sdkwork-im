import {
  AdminMetricCard,
  AdminPageFrame,
  AdminSectionCard,
  useAdminI18n,
} from 'sdkwork-craw-chat-admin-core';
import type { AdminPageProps } from 'sdkwork-craw-chat-admin-types';

function resolveAnnouncementStatusLabel(status: string, translate: (value: string) => string) {
  if (status === 'active') {
    return translate('Active delivery');
  }

  if (status === 'scheduled') {
    return translate('Scheduled');
  }

  return status;
}

export function AnnouncementsPage({ snapshot }: AdminPageProps) {
  const { formatNumber, t } = useAdminI18n();
  const tasks =
    snapshot.marketingCampaigns.length > 0
      ? snapshot.marketingCampaigns.slice(0, 5).map((campaign) => ({
          id: campaign.marketing_campaign_id,
          name: campaign.display_name,
          status: campaign.status,
        }))
      : [
          { id: 'broadcast-1', name: 'Service upgrade notice', status: 'active' },
          { id: 'broadcast-2', name: 'Regional policy update', status: 'scheduled' },
        ];

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
          <div className="space-y-3 text-sm text-[var(--admin-text-secondary)]">
            <div>{t('Scheduled tasks are isolated from high-risk queues until policy review is complete.')}</div>
            <div>{t('Delivery failures should promote directly into operator remediation workflows.')}</div>
            <div>{t('Critical notices remain pinned until every priority workspace acknowledges receipt.')}</div>
          </div>
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
          value={formatNumber(tasks.filter((task) => task.status === 'scheduled').length || 1)}
        />
        <AdminMetricCard
          detail={t('Announcements actively moving through live delivery or acknowledgement workflows.')}
          label={t('Active delivery')}
          value={formatNumber(tasks.filter((task) => task.status === 'active').length || 1)}
        />
      </div>

      <AdminSectionCard
        description={t(
          'Operators need visibility into every outbound communication task because announcements shape trust as much as product features do.',
        )}
        title={t('Broadcast tasks')}
      >
        <div className="space-y-3">
          {tasks.map((task) => (
            <div
              className="flex flex-col gap-3 rounded-3xl border border-[var(--admin-border-color)] bg-[var(--admin-content-background)]/60 p-4 md:flex-row md:items-center md:justify-between"
              key={task.id}
            >
              <div className="text-sm font-semibold text-[var(--admin-text-primary)]">
                {task.name}
              </div>
              <div className="text-sm capitalize text-[var(--admin-text-secondary)]">
                {resolveAnnouncementStatusLabel(task.status, t)}
              </div>
            </div>
          ))}
        </div>
      </AdminSectionCard>
    </AdminPageFrame>
  );
}
