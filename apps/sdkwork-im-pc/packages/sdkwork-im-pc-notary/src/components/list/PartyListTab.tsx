/**
 * PartyListTab - Party list with expand/collapse functionality in the detail pane
 */
import React from 'react';
import { useTranslation } from 'react-i18next';
import { motion, AnimatePresence } from 'motion/react';
import { ShieldCheck, CheckCircle2, PenTool, Folder, Edit, Video } from 'lucide-react';
import type { NotaryTask, Party } from '@sdkwork/im-pc-types';
import { PartyActionButtons } from '../shared/PartyActionButtons';

export interface PartyListTabProps {
  /** The current task */
  task: NotaryTask;
  /** Which party is expanded */
  expandedParty: string | null;
  /** Called when expand/collapse is toggled */
  onExpand: (partyId: string) => void;
  /** Called when edit is clicked */
  onEdit: (party: Party) => void;
  /** Called when sign is clicked */
  onSign: (party: Party) => void;
  /** Called when drive is clicked */
  onDrive: (party: Party) => void;
  /** Called when video call is clicked */
  onVideoCall: (party: Party) => void;
}

export const PartyListTab: React.FC<PartyListTabProps> = ({
  task,
  expandedParty,
  onExpand,
  onEdit,
  onSign,
  onDrive,
  onVideoCall,
}) => {
  const { t } = useTranslation('notary');

  const parties = task.parties || [];

  if (parties.length === 0) {
    return (
      <div className="text-sm text-gray-500 text-center py-8 bg-[#181818]/50 rounded-lg border border-dashed border-white/5">
        {t('detail.noPartyInfo')}
      </div>
    );
  }

  return (
    <div className="flex flex-col gap-4">
      {parties.map((party) => (
        <div
          key={party.id}
          onDoubleClick={() => onEdit(party)}
          className="bg-[#181818] p-4 rounded-xl border border-white/5 flex flex-col gap-3 group/party hover:border-indigo-500/30 transition-colors"
        >
          {/* Header row */}
          <div className="flex items-center justify-between border-b border-white/5 pb-3">
            <div className="flex items-center gap-3">
              <div className="w-10 h-10 rounded-full bg-indigo-500/10 flex items-center justify-center text-indigo-400 text-lg font-medium">
                {party.name.charAt(0)}
              </div>
              <div>
                <div className="text-sm font-medium text-gray-200 flex items-center gap-2">
                  {party.name}
                  <span className="px-1.5 py-0.5 rounded text-[10px] bg-white/10 text-gray-400">{party.role}</span>
                  {party.gender && (
                    <span className="px-1.5 py-0.5 rounded text-[10px] bg-white/10 text-gray-400">{party.gender}</span>
                  )}
                </div>
                <div className="text-xs text-gray-500 mt-1 font-mono">{party.identityId}</div>
              </div>
            </div>

            {/* Action buttons */}
            <div className="flex items-center gap-2">
              {party.signatureUrl && (
                <div className="px-2 py-1 bg-teal-500/10 text-teal-400 rounded-lg text-xs font-medium border border-teal-500/20 flex items-center gap-1">
                  <CheckCircle2 size={14} /> {t('createTask.signed')}
                </div>
              )}
              {task.status !== 'COMPLETED' && task.status !== 'REJECTED' && (
                <button
                  onClick={() => onSign(party)}
                  className="px-2 py-1 bg-orange-500/10 text-orange-400 hover:bg-orange-500/20 rounded-lg transition-colors border border-orange-500/20 shrink-0 text-[11px] font-medium flex items-center gap-1"
                  title={t('createTask.signBtn')}
                >
                  <PenTool size={14} /> {t('createTask.signBtn')}
                </button>
              )}
              <button
                onClick={() => onDrive(party)}
                className="p-2 bg-blue-500/10 text-blue-400 hover:bg-blue-500/20 rounded-lg transition-colors flex items-center gap-1.5 text-xs font-medium border border-blue-500/20"
                title={t('detail.driveDocuments')}
              >
                <Folder size={14} />
              </button>
              <button
                onClick={() => onEdit(party)}
                className="p-2 bg-indigo-500/10 text-indigo-400 hover:bg-indigo-500/20 rounded-lg transition-colors border border-indigo-500/20"
                title={t('createTask.edit')}
              >
                <Edit size={14} />
              </button>
              <button
                onClick={() => onVideoCall(party)}
                className="px-3 py-1.5 bg-green-500/10 text-green-500 hover:bg-green-500/20 rounded-lg text-[11px] font-medium transition-colors flex items-center gap-1.5 shrink-0"
              >
                <Video size={14} /> {t('actions.videoCall')}
              </button>
            </div>
          </div>

          {/* Phone row */}
          <div className="text-xs text-gray-400 flex items-center justify-between">
            <div className="flex items-center gap-2">
              <span className="w-16">{t('party.phone')}:</span>
              <span className="text-gray-200 font-mono">{party.phone}</span>
            </div>
            <button
              onClick={() => onExpand(party.id)}
              className="text-indigo-400 hover:text-indigo-300 font-medium transition-colors"
            >
              {expandedParty === party.id ? t('detail.collapseInfo') : t('detail.viewInfo')}
            </button>
          </div>

          {/* Expanded info */}
          <AnimatePresence>
            {expandedParty === party.id && (
              <motion.div
                initial={{ height: 0, opacity: 0 }}
                animate={{ height: 'auto', opacity: 1 }}
                exit={{ height: 0, opacity: 0 }}
                className="overflow-hidden"
              >
                <div className="mt-3 bg-black/20 p-4 rounded-lg border border-white/5 flex flex-col gap-4 text-xs">
                  {/* Verification status */}
                  <div className="flex justify-between items-start">
                    <div>
                      <div className="text-gray-500 mb-1">{t('detail.identityVerification')}</div>
                      <div className="flex items-center gap-1.5 text-green-500 font-medium">
                        <ShieldCheck size={14} /> {t('detail.publicSecurityDbMatch')}
                      </div>
                    </div>
                    <div className="text-right">
                      <div className="text-gray-500 mb-1">{t('detail.faceCapture')}</div>
                      <div className="text-gray-300">{party.faceCaptureTime || '—'}</div>
                    </div>
                  </div>

                  {/* Photos */}
                  <div className="grid grid-cols-2 gap-4 pt-3 border-t border-white/5">
                    <div>
                      <div className="text-gray-500 mb-1">{t('detail.idPhotos')}</div>
                      <div className="flex gap-2 min-h-[60px]">
                        <div className="w-20 h-12 bg-white/5 rounded border border-white/10 flex items-center justify-center text-gray-500">
                          {t('detail.archived')}
                        </div>
                        <div className="w-20 h-12 bg-white/5 rounded border border-white/10 flex items-center justify-center text-gray-500">
                          {t('detail.archived')}
                        </div>
                      </div>
                    </div>
                    <div>
                      <div className="text-gray-500 mb-1">{t('detail.liveCapture')}</div>
                      <div className="w-16 h-16 bg-white/5 rounded border border-white/10 flex items-center justify-center text-gray-500">
                        {t('detail.liveness')}
                      </div>
                    </div>
                  </div>
                </div>
              </motion.div>
            )}
          </AnimatePresence>
        </div>
      ))}
    </div>
  );
};