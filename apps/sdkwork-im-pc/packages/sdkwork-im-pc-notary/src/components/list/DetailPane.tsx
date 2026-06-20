/**
 * DetailPane - Right-side sliding drawer for task details
 */
import React from 'react';
import { useTranslation } from 'react-i18next';
import { motion, AnimatePresence } from 'motion/react';
import { ShieldCheck, X } from 'lucide-react';
import type { NotaryTask, NotaryDocument, Party } from '@sdkwork/im-pc-types';
import type { PartyIdentityMediaUrls } from '../../types';
import { getNotaryTaskDisplayNo } from '../../utils/notaryTask';
import { TaskBaseInfo } from './TaskBaseInfo';
import { PaneTabSwitcher } from './PaneTabSwitcher';
import { PartyListTab } from './PartyListTab';
import { MaterialsTab } from './MaterialsTab';
import { TimelineTab } from './TimelineTab';
import { DetailPaneFooter } from './DetailPaneFooter';

export type DetailPaneTab = 'parties' | 'materials' | 'timeline';

export interface DetailPaneProps {
  task: NotaryTask;
  activeTab: DetailPaneTab;
  loading?: boolean;
  /** Expanded party ID */
  expandedParty: string | null;
  /** Whether expanded party identity media is loading */
  expandedPartyMediaLoading?: boolean;
  /** Expanded party identity media URLs */
  expandedPartyMediaUrls?: PartyIdentityMediaUrls | null;
  /** Function to render status badge */
  getStatusBadge: (status: NotaryTask['status']) => React.ReactNode;
  /** Called when pane is closed */
  onClose: () => void;
  /** Called when tab changes */
  onTabChange: (tab: DetailPaneTab) => void;
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
  /** Whether the assigned notary can be changed */
  canAssignNotary?: boolean;
  /** Called when the user requests to change the assigned notary */
  onAssignNotary?: () => void;
}

export const DetailPane: React.FC<DetailPaneProps> = ({
  task,
  activeTab,
  loading = false,
  expandedParty,
  expandedPartyMediaLoading = false,
  expandedPartyMediaUrls,
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
  canAssignNotary = false,
  onAssignNotary,
}) => {
  const { t } = useTranslation('notary');

  const tabs = [
    { id: 'parties', label: t('detail.partyList') },
    { id: 'materials', label: t('detail.notaryMaterials') },
    { id: 'timeline', label: t('detail.timeline') },
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
        initial={{ x: '100%', opacity: 0 }}
        animate={{ x: 0, opacity: 1 }}
        exit={{ x: '100%', opacity: 0 }}
        transition={{ type: 'spring', damping: 25, stiffness: 200 }}
        className="fixed top-0 right-0 h-full w-full max-w-[720px] xl:max-w-[860px] bg-[#222224] border-l border-white/5 z-[101] flex flex-col shadow-[-10px_0_30px_rgba(0,0,0,0.5)]"
      >
        {/* Header */}
        <div className="flex items-center justify-between p-6 border-b border-white/5 shrink-0 bg-[#2b2b2d]">
          <div className="flex items-center gap-3">
            <div className="w-10 h-10 rounded-lg bg-indigo-500/20 flex items-center justify-center text-indigo-400">
              <ShieldCheck size={20} />
            </div>
            <div>
              <h2 className="text-lg font-medium text-gray-100 leading-tight">{t('detail.title')}</h2>
              <p className="text-xs text-gray-400 font-mono mt-1">{getNotaryTaskDisplayNo(task)}</p>
            </div>
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
          <TaskBaseInfo
            task={task}
            getStatusBadge={getStatusBadge}
            canAssignNotary={canAssignNotary}
            onAssignNotary={onAssignNotary}
          />
        </div>

        {/* Tabs */}
        <PaneTabSwitcher
          activeTab={activeTab}
          tabs={tabs}
          onTabChange={(id) => onTabChange(id as DetailPaneTab)}
        />

        {/* Content */}
        <div className="flex-1 overflow-y-auto p-6 bg-[#181818]/20 relative">
          {loading && (
            <div className="absolute inset-0 z-10 flex items-center justify-center bg-[#181818]/40 backdrop-blur-[1px]">
              <div className="px-4 py-2 rounded-lg bg-[#2b2b2d] border border-white/10 text-sm text-gray-300">
                {t('stats.loading')}
              </div>
            </div>
          )}
          {activeTab === 'parties' && (
            <PartyListTab
              task={task}
              expandedParty={expandedParty}
              expandedPartyMediaLoading={expandedPartyMediaLoading}
              expandedPartyMediaUrls={expandedPartyMediaUrls}
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
          {activeTab === 'timeline' && (
            <TimelineTab task={task} />
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