/**
 * DetailPaneFooter - Action buttons at the bottom of the detail pane
 */
import React from 'react';
import { useTranslation } from 'react-i18next';
import { Printer } from 'lucide-react';
import type { NotaryTask } from '@sdkwork/clawchat-pc-types';

export interface DetailPaneFooterProps {
  /** The current task */
  task: NotaryTask;
  /** Called when print button is clicked */
  onPrint: () => void;
  /** Called when status change is requested */
  onStatusChange: (status: NotaryTask['status']) => void;
}

export const DetailPaneFooter: React.FC<DetailPaneFooterProps> = ({
  task,
  onPrint,
  onStatusChange,
}) => {
  const { t } = useTranslation('notary');

  return (
    <div className="p-6 border-t border-white/5 shrink-0 bg-[#2b2b2d] flex justify-between items-center gap-3">
      {/* Left: Print button */}
      <button
        onClick={onPrint}
        className="px-4 py-2 text-sm text-gray-300 hover:text-white hover:bg-white/5 rounded border border-white/10 transition-colors flex items-center gap-2"
      >
        <Printer size={16} /> {t('actions.printPartyInfo')}
      </button>

      {/* Right: Status actions */}
      <div className="flex gap-3">
        {task.status === 'PENDING_REVIEW' && (
          <>
            <button
              onClick={() => onStatusChange('REJECTED')}
              className="px-4 py-2 text-sm text-red-400 hover:text-red-300 hover:bg-red-500/10 rounded border border-red-500/20 transition-colors"
            >
              {t('detail.returnForSupplement')}
            </button>
            <button
              onClick={() => onStatusChange('PROCESSING')}
              className="px-6 py-2 text-sm text-white bg-indigo-600 hover:bg-indigo-700 rounded shadow-md transition-colors"
            >
              {t('detail.transferToProcessing')}
            </button>
          </>
        )}
        {task.status === 'PROCESSING' && (
          <button
            onClick={() => onStatusChange('COMPLETED')}
            className="px-6 py-2 text-sm text-white bg-green-600 hover:bg-green-700 rounded shadow-md transition-colors"
          >
            {t('detail.manualVerification')}
          </button>
        )}
      </div>
    </div>
  );
};