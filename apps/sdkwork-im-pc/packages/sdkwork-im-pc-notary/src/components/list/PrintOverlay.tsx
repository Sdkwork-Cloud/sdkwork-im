/**
 * PrintOverlay - Full-screen print preview overlay
 */
import React from 'react';
import { useTranslation } from 'react-i18next';
import { motion, AnimatePresence } from 'motion/react';
import { ChevronLeft, AlertCircle, Printer } from 'lucide-react';
import type { NotaryTask } from '@sdkwork/clawchat-pc-types';
import type { PartyIdentityMediaUrls } from '../../types';
import { PrintPartyPage } from './PrintPartyPage';

export interface PrintOverlayProps {
  /** The task being printed */
  task: NotaryTask;
  /** Media URLs for party identity photos */
  partyMediaUrls: Record<string, PartyIdentityMediaUrls>;
  /** Called when overlay is closed */
  onClose: () => void;
}

export const PrintOverlay: React.FC<PrintOverlayProps> = ({
  task,
  partyMediaUrls,
  onClose,
}) => {
  const { t } = useTranslation('notary');

  const parties = task.parties || [];

  return (
    <AnimatePresence>
      <motion.div
        initial={{ opacity: 0, scale: 0.95 }}
        animate={{ opacity: 1, scale: 1 }}
        exit={{ opacity: 0, scale: 0.95 }}
        transition={{ type: 'spring', damping: 25, stiffness: 200 }}
        className="fixed inset-0 bg-[#222] z-[200] flex flex-col overflow-hidden print:static print:bg-white print:z-auto print:overflow-visible print:block"
      >
        {/* Header (hidden in print) */}
        <div className="h-16 bg-[#181818] border-b border-white/10 flex items-center justify-between px-6 shrink-0 shadow-sm z-10 print:hidden">
          <div className="flex items-center gap-4 text-gray-200">
            <button onClick={onClose} className="p-2 hover:bg-white/10 rounded-lg transition-colors">
              <ChevronLeft size={20} />
            </button>
            <h2 className="font-medium text-lg">{t('detail.printPartyVerification')}</h2>
            <span className="text-gray-500 text-sm">{t('print.totalPages', { count: parties.length })}</span>
          </div>
          <div className="flex items-center gap-4">
            <span className="text-orange-400/90 text-[13px] flex items-center gap-1.5 font-medium bg-orange-500/10 px-3 py-1.5 rounded-md">
              <AlertCircle size={15} /> {t('detail.printPreviewHint')}
            </span>
            <button
              onClick={() => window.print()}
              className="px-6 py-2 bg-indigo-600 hover:bg-indigo-700 text-white rounded-lg shadow-md flex items-center gap-2 text-sm transition-colors"
            >
              <Printer size={16} /> {t('detail.startPrint')}
            </button>
          </div>
        </div>

        {/* Scrollable pages */}
        <div className="flex-1 overflow-y-auto custom-scrollbar p-10 print:p-0 print:overflow-visible print:bg-white" id="print-root-container">
          <div className="flex flex-col items-center gap-8 print:block print:gap-0">
            {parties.length === 0 ? (
              <div className="text-gray-400 mt-20 print:hidden">{t('detail.noPartyInfo')}</div>
            ) : (
              parties.map((party) => (
                <PrintPartyPage
                  key={party.id}
                  party={party}
                  mediaUrls={partyMediaUrls[party.id]}
                />
              ))
            )}
          </div>
        </div>
      </motion.div>
    </AnimatePresence>
  );
};