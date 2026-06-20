/**
 * TaskBaseInfo - Grid display of task summary info in the detail pane
 */
import React from 'react';
import { useTranslation } from 'react-i18next';
import { Hash, User as UserIcon, Activity, Clock, Shield } from 'lucide-react';
import type { NotaryTask } from '@sdkwork/im-pc-types';
import { getNotaryTaskDisplayNo } from '../../utils/notaryTask';

export interface TaskBaseInfoProps {
  task: NotaryTask;
  /** Function to render status badge */
  getStatusBadge: (status: NotaryTask['status']) => React.ReactNode;
  /** Whether the assigned notary can be changed */
  canAssignNotary?: boolean;
  /** Called when the user requests to change the assigned notary */
  onAssignNotary?: () => void;
}

export const TaskBaseInfo: React.FC<TaskBaseInfoProps> = ({
  task,
  getStatusBadge,
  canAssignNotary = false,
  onAssignNotary,
}) => {
  const { t } = useTranslation('notary');

  const infoItems = [
    { label: t('detail.notaryBusiness'), value: task.type, icon: Hash },
    { label: t('detail.notaryNo'), value: getNotaryTaskDisplayNo(task), icon: Hash },
    { label: t('detail.applicant'), value: task.applicant, icon: UserIcon },
    { label: t('detail.currentStatus'), value: getStatusBadge(task.status), icon: Activity, isBadge: true },
    { label: t('detail.notary'), value: task.notary, icon: UserIcon },
    { label: t('detail.processingTime'), value: task.createTime ? new Date(task.createTime).toLocaleDateString() : t('common.notAvailable'), icon: Clock },
  ];

  return (
    <div className="grid grid-cols-2 gap-x-6 gap-y-3 py-2">
      {infoItems.map((item, index) => (
        <div key={index} className="flex items-center gap-2">
          <item.icon size={14} className="text-gray-500 shrink-0" />
          <span className="text-xs text-gray-500 w-20 shrink-0">{item.label}</span>
          {item.isBadge ? (
            item.value
          ) : (
            <span className="text-sm text-gray-200 font-medium truncate">{item.value || t('common.notAvailable')}</span>
          )}
        </div>
      ))}
      {canAssignNotary && onAssignNotary && (
        <div className="col-span-2 flex justify-end">
          <button
            type="button"
            onClick={onAssignNotary}
            className="text-xs text-indigo-400 hover:text-indigo-300 hover:underline transition-colors"
          >
            {t('detail.changeNotary')}
          </button>
        </div>
      )}
      {/* Blockchain hash (full width) */}
      <div className="col-span-2 flex items-center gap-2">
        <Shield size={14} className="text-gray-500 shrink-0" />
        <span className="text-xs text-gray-500 w-20 shrink-0">{t('detail.blockchainHash')}</span>
        <span className="text-sm text-gray-400 font-mono truncate">{task.hash ?? t('common.notAvailable')}</span>
      </div>
    </div>
  );
};
