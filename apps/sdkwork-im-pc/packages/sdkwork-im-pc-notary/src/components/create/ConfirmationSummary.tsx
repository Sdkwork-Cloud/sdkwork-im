/**
 * ConfirmationSummary - Review summary for all wizard data (Step 4)
 */
import React from 'react';
import { useTranslation } from 'react-i18next';
import { motion } from 'motion/react';
import { ChevronRight, Camera, Video, FileText } from 'lucide-react';
import type { Party } from '@sdkwork/clawchat-pc-types';
import type { LocalAttachment } from '../../types';

export interface ConfirmationSummaryProps {
  /** Selected business type */
  businessType: string;
  /** Selected notary display name */
  notary: string;
  /** List of parties */
  parties: Party[];
  /** Business description */
  description: string;
  /** List of attachments */
  attachments: LocalAttachment[];
  /** Called when a party is clicked for viewing */
  onViewParty: (party: Party) => void;
  /** Called when an attachment is previewed */
  onPreviewAttachment: (attachment: LocalAttachment) => void;
}

export const ConfirmationSummary: React.FC<ConfirmationSummaryProps> = ({
  businessType,
  notary,
  parties,
  description,
  attachments,
  onViewParty,
  onPreviewAttachment,
}) => {
  const { t } = useTranslation('notary');

  return (
    <motion.div initial={{ opacity: 0, y: 10 }} animate={{ opacity: 1, y: 0 }} className="flex flex-col gap-8">
      {/* Header */}
      <div className="text-center">
        <h3 className="text-xl font-medium text-gray-200 mb-2">{t('createTask.confirmInfo')}</h3>
        <p className="text-sm text-gray-500">{t('createTask.confirmHint')}</p>
      </div>

      {/* Summary card */}
      <div className="bg-[#181818] border border-white/5 rounded-xl p-6 flex flex-col gap-4 text-sm">
        {/* Business type */}
        <div className="flex">
          <span className="w-24 text-gray-500">{t('createTask.notaryType')}</span>
          <span className="text-gray-200 font-medium">{businessType || '--'}</span>
        </div>

        {/* Notary */}
        <div className="flex">
          <span className="w-24 text-gray-500">{t('createTask.handleNotary')}</span>
          <span className="text-gray-200">{notary || '--'}</span>
        </div>

        {/* Parties */}
        <div className="flex">
          <span className="w-24 text-gray-500 mt-2">{t('createTask.partyLabel')}</span>
          <div className="flex flex-col gap-2 flex-1">
            {parties.length === 0 ? (
              <span className="text-gray-500 mt-2">--</span>
            ) : (
              parties.map((p) => (
                <div
                  key={p.id}
                  onClick={() => onViewParty(p)}
                  className="bg-white/5 border border-white/10 px-4 py-3 rounded-xl flex items-center justify-between cursor-pointer hover:bg-white/10 hover:border-indigo-500/30 transition-colors group"
                >
                  <div className="flex flex-col gap-1.5">
                    <div className="flex items-center gap-2">
                      <span className="font-medium text-gray-200 group-hover:text-indigo-400 transition-colors">{p.name}</span>
                      <span className="text-xs text-indigo-400 bg-indigo-500/10 px-2 py-0.5 rounded border border-indigo-500/20">{p.role}</span>
                      {p.signatureUrl && (
                        <img src={p.signatureUrl} alt={`Signature for ${p.name}`} className="h-6 object-contain bg-white/90 rounded border border-white/20 ml-2" />
                      )}
                    </div>
                    <div className="text-xs text-gray-500 flex items-center gap-3">
                      <span>{t('createTask.idNumberLabel')}: {p.identityId}</span>
                      <span>{t('createTask.phoneLabel')}: {p.phone}</span>
                    </div>
                  </div>
                  <ChevronRight size={18} className="text-gray-500 group-hover:text-indigo-400 transition-colors" />
                </div>
              ))
            )}
          </div>
        </div>

        {/* Description */}
        <div className="flex">
          <span className="w-24 text-gray-500 shrink-0">{t('createTask.applicationReason')}</span>
          <span className="text-gray-300 bg-white/5 p-3 rounded-lg flex-1 min-h-[60px] line-clamp-3">{description || '--'}</span>
        </div>

        {/* Attachments */}
        {attachments.length > 0 && (
          <div className="flex">
            <span className="w-24 text-gray-500 shrink-0 mt-3">{t('createTask.relatedAttachments')}</span>
            <div className="flex flex-col gap-3 flex-1 mt-1">
              {attachments.map((att) => (
                <div
                  key={att.id}
                  className="bg-white/5 border border-white/10 p-3 rounded-xl flex items-center justify-between cursor-pointer hover:bg-white/10 hover:border-indigo-500/30 transition-colors group"
                  onClick={() => onPreviewAttachment(att)}
                >
                  <div className="flex items-center gap-4 min-w-0">
                    <div className="w-10 h-10 rounded-lg bg-[#2b2b2d] flex items-center justify-center shrink-0 border border-white/5 group-hover:border-indigo-500/30 transition-colors">
                      {att.type === 'image' ? <Camera size={16} className="text-indigo-400" /> : att.type === 'video' ? <Video size={16} className="text-indigo-400" /> : <FileText size={16} className="text-indigo-400" />}
                    </div>
                    <span className="text-sm text-gray-300 font-medium truncate group-hover:text-indigo-400 transition-colors">{att.name}</span>
                  </div>
                  <span className="text-xs text-gray-500 uppercase">{att.type}</span>
                </div>
              ))}
            </div>
          </div>
        )}
      </div>
    </motion.div>
  );
};