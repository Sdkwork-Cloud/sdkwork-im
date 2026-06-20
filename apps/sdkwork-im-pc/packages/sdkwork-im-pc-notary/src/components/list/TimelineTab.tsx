/**
 * TimelineTab - Case event timeline in the detail pane
 */
import React from 'react';
import { useTranslation } from 'react-i18next';
import { Clock } from 'lucide-react';
import type { NotaryTask } from '@sdkwork/im-pc-types';

export interface TimelineTabProps {
  task: NotaryTask;
}

export const TimelineTab: React.FC<TimelineTabProps> = ({ task }) => {
  const { t } = useTranslation('notary');
  const events = task.timeline ?? [];

  if (events.length === 0) {
    return (
      <div className="text-sm text-gray-500 text-center py-8 bg-[#181818]/50 rounded-lg border border-dashed border-white/5">
        {t('detail.noTimelineEvents')}
      </div>
    );
  }

  return (
    <div className="flex flex-col gap-3">
      {events.map((event, index) => (
        <div
          key={`${event.time}-${event.event}-${index}`}
          className="bg-[#181818] border border-white/5 rounded-lg p-4 flex gap-3"
        >
          <div className="w-8 h-8 rounded-full bg-indigo-500/10 flex items-center justify-center text-indigo-400 shrink-0">
            <Clock size={14} />
          </div>
          <div className="min-w-0 flex-1">
            <div className="text-sm text-gray-200 font-medium">{event.event}</div>
            <div className="text-xs text-gray-500 mt-1 flex flex-wrap gap-x-3 gap-y-1">
              <span>{event.actor}</span>
              <span>{event.time ? new Date(event.time).toLocaleString() : t('common.notAvailable')}</span>
            </div>
          </div>
        </div>
      ))}
    </div>
  );
};
