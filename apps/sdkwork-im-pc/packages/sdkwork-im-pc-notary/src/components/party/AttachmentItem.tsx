/**
 * AttachmentItem - Single attachment display in AuxiliaryMaterials
 */
import React from 'react';
import { useTranslation } from 'react-i18next';
import { Trash2 } from 'lucide-react';
import type { LocalAttachment } from '../../types';

export interface AttachmentItemProps {
  /** The attachment to display */
  attachment: LocalAttachment;
  /** Whether the item is read-only */
  readOnly: boolean;
  /** Called when the remove button is clicked */
  onRemove: (id: string) => void;
}

export const AttachmentItem: React.FC<AttachmentItemProps> = ({
  attachment,
  readOnly,
  onRemove,
}) => {
  const { t } = useTranslation('notary');

  return (
    <div className="flex items-center justify-between p-2.5 bg-[#181818] border border-white/10 rounded-lg group hover:border-white/20 transition-colors">
      <div className="flex items-center gap-3 min-w-0">
        <div className="w-10 h-10 rounded shrink-0 bg-black/50 border border-white/5 flex items-center justify-center overflow-hidden p-0.5">
          <img src={attachment.url} alt={attachment.name} className="w-full h-full object-contain" />
        </div>
        <span className="text-sm text-gray-300 truncate">{attachment.name}</span>
      </div>
      {!readOnly && (
        <button
          onClick={() => onRemove(attachment.id)}
          className="p-2 text-gray-500 hover:text-red-400 hover:bg-red-400/10 rounded-lg transition-colors shrink-0"
          title={t('party.deleteAttachment')}
        >
          <Trash2 size={16} />
        </button>
      )}
    </div>
  );
};