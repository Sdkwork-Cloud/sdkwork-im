/**
 * AuxiliaryMaterials - Remarks and attachments section in PartyDrawer
 */
import React, { useRef } from 'react';
import { useTranslation } from 'react-i18next';
import { FileText, Plus } from 'lucide-react';
import type { LocalAttachment } from '../../types';
import { AttachmentItem } from './AttachmentItem';

export interface AuxiliaryMaterialsProps {
  /** Remarks text */
  remarks: string;
  /** List of attachments */
  attachments: LocalAttachment[];
  /** Whether fields are read-only */
  readOnly: boolean;
  /** Called when remarks changes */
  onRemarksChange: (remarks: string) => void;
  /** Called when attachments change */
  onAttachmentsChange: (attachments: LocalAttachment[]) => void;
  /** Called when files are uploaded */
  onUpload: (files: File[]) => void;
}

export const AuxiliaryMaterials: React.FC<AuxiliaryMaterialsProps> = ({
  remarks,
  attachments,
  readOnly,
  onRemarksChange,
  onAttachmentsChange,
  onUpload,
}) => {
  const { t } = useTranslation('notary');
  const fileInputRef = useRef<HTMLInputElement>(null);

  const handleFileChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    const files = Array.from<File>(event.target.files || []);
    if (files.length > 0) {
      onUpload(files);
    }
    event.target.value = '';
  };

  const triggerUpload = () => {
    if (!readOnly) {
      fileInputRef.current?.click();
    }
  };

  const handleRemoveAttachment = (id: string) => {
    onAttachmentsChange(attachments.filter((a) => a.id !== id));
  };

  return (
    <div className="flex flex-col gap-4">
      {/* Section header */}
      <h4 className="text-sm font-semibold text-gray-300 flex items-center gap-2">
        <FileText size={16} className="text-indigo-400" /> {t('party.auxiliaryMaterials')}
      </h4>

      {/* Remarks textarea */}
      <div className="flex flex-col gap-2">
        <label className="text-xs font-medium text-gray-400">{t('party.remarks')}</label>
        <textarea
          readOnly={readOnly}
          value={remarks}
          onChange={(e) => onRemarksChange(e.target.value)}
          placeholder={t('party.remarksPlaceholder')}
          className="w-full h-20 bg-[#181818] border border-white/10 rounded-lg px-3 py-2 text-sm text-gray-200 outline-none focus:border-indigo-500 resize-none custom-scrollbar read-only:bg-white/5 read-only:border-transparent read-only:text-gray-400"
        />
      </div>

      {/* Attachments */}
      <div className="flex flex-col gap-2 mt-2">
        <label className="text-xs font-medium text-gray-400">{t('party.otherAttachments')}</label>
        <input
          disabled={readOnly}
          type="file"
          multiple
          className="hidden"
          ref={fileInputRef}
          onChange={handleFileChange}
          accept="image/*,application/pdf"
        />

        {/* Attachment list */}
        {attachments.length > 0 && (
          <div className="flex flex-col gap-2 mb-2">
            {attachments.map((attachment) => (
              <AttachmentItem
                key={attachment.id}
                attachment={attachment}
                readOnly={readOnly}
                onRemove={handleRemoveAttachment}
              />
            ))}
          </div>
        )}

        {/* Upload trigger */}
        {!readOnly && (
          <div
            onClick={triggerUpload}
            className="border border-dashed border-white/10 bg-[#181818]/50 rounded-xl flex flex-col items-center justify-center p-6 text-gray-500 cursor-pointer hover:border-white/30 hover:bg-white/5 transition-colors"
          >
            <Plus size={20} className="mb-2" />
            <span className="text-[12px]">{t('party.dragOrClick')}</span>
          </div>
        )}
      </div>
    </div>
  );
};