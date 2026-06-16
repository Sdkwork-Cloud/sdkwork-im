/**
 * DetailPane - Right-side sliding drawer for task details
 */
import React from 'react';
import { useTranslation } from 'react-i18next';
import { motion, AnimatePresence } from 'motion/react';
import { ShieldCheck, X } from 'lucide-react';
import type { NotaryTask, NotaryDocument, Party } from '@sdkwork/im-pc-types';
import { TaskBaseInfo } from './TaskBaseInfo';
import { PaneTabSwitcher } from './PaneTabSwitcher';
import { PartyListTab } from './PartyListTab';
import { MaterialsTab } from './MaterialsTab';
import { DetailPaneFooter } from './DetailPaneFooter';

export interface DetailPaneProps {
  /** The task to display */
  task: NotaryTask;
  /** Active tab */
  activeTab: 'parties' | 'materials';
  /** Expanded party ID */
  expandedParty: string | null;
  /** Function to render status badge */
  getStatusBadge: (status: NotaryTask['status']) => React.ReactNode;
  /** Called when pane is closed */
  onClose: () => void;
  /** Called when tab changes */
  onTabChange: (tab: 'parties' | 'materials') => void;
  /** Called when party expand toggles */
  onExpandParty: (partyId: string) => void;
  /** Called when edit party is clicked */
  onEditParty: (party: Party) => void;
  /** Called when sign party is clicked */
  onSignParty: (party: Party) => void;
  /** Called when drive party is clicked */
  onDriveParty: (party: Party) => void;
  /** Called when video call is clicked */
  onVideoCall: (party: Party) => void;
  /** Called when print is clicked */
  onPrint: () => void;
  /** Called when status change is requested */
  onStatusChange: (status: NotaryTask['status']) => void;
  /** Called when document preview is requested */
  onPreviewDocument: (doc: NotaryDocument) => void;
  /** Called when document download is requested */
  onDownloadDocument: (doc: NotaryDocument) => void;
  /** Called when download all materials is requested */
  onDownloadAllMaterials: () => void;
}

export const DetailPane: React.FC<DetailPaneProps> = ({
  task,
  activeTab,
  expandedParty,
  getStatusBadge,
  onClose,
  onTabChange,
  onExpandParty,
  onEditParty,
  onSignParty,
  onDriveParty,
  onVideoCall,
  onPrint,
  onStatusChange,
  onPreviewDocument,
  onDownloadDocument,
  onDownloadAllMaterials,
}) => {
  const { t } = useTranslation('notary');

  const tabs = [
    { id: 'parties', label: t('detail.partyList') },
    { id: 'materials', label: t('detail.notaryMaterials') },
  ];

  return (
    <AnimatePresence>
      <motion.div
        initial={{ opacity: 0 }}
        animate={{ opacity: 1 }}
        exit={{ opacity: 0 }}
        onClick={onClose}
        className="fixed inset-0 bg-black/60 backdrop-blur-sm z-[100]"
      />
      <motion.div
        initial={{ x: 480 }}
        animate={{ x: 0 }}
        exit={{ x: 480 }}
        transition={{ type: 'spring', damping: 25, stiffness: 300 }}
        className="fixed top-0 right-0 h-full w-[480px] bg-[#1e1e1e] border-l border-white/5 z-[101] flex flex-col shadow-2xl"
      >
        {/* Header */}
        <div className="flex items-center justify-between p-6 border-b border-white/5 bg-[#181818] shrink-0">
          <div className="flex items-center gap-2">
            <ShieldCheck size={18} className="text-indigo-400" />
            <h3 className="text-lg font-medium text-gray-200">{t('detail.title')}</h3>
            {(task as any).caseNo && (
              <span className="text-xs text-gray-500 ml-2">#{(task as any).caseNo}</span>
            )}
          </div>
          <button
            onClick={onClose}
            className="text-gray-400 hover:text-white p-2 rounded-lg hover:bg-white/10 transition-colors"
          >
            <X size={18} />
          </button>
        </div>

        {/* Base info */}
        <div className="px-6 pt-4 pb-4 border-b border-white/5 bg-[#181818]/30 shrink-0">
          <TaskBaseInfo task={task} getStatusBadge={getStatusBadge} />
        </div>

        {/* Tabs */}
        <PaneTabSwitcher
          activeTab={activeTab}
          tabs={tabs}
          onTabChange={(id) => onTabChange(id as 'parties' | 'materials')}
        />

        {/* Content */}
        <div className="flex-1 overflow-y-auto p-6 bg-[#181818]/20">
          {activeTab === 'parties' && (
            <PartyListTab
              task={task}
              expandedParty={expandedParty}
              onExpand={onExpandParty}
              onEdit={onEditParty}
              onSign={onSignParty}
              onDrive={onDriveParty}
              onVideoCall={onVideoCall}
            />
          )}
          {activeTab === 'materials' && (
            <MaterialsTab
              task={task}
              onPreview={onPreviewDocument}
              onDownload={onDownloadDocument}
              onDownloadAll={onDownloadAllMaterials}
            />
          )}
        </div>

        {/* Footer */}
        <DetailPaneFooter
          task={task}
          onPrint={onPrint}
          onStatusChange={onStatusChange}
        />
      </motion.div>
    </AnimatePresence>
  );
};