/**
 * PartyActionButtons - Shared action buttons for party cards
 *
 * Used in both the detail pane (index.tsx) and create wizard (CreateNotaryTaskView.tsx)
 * to maintain consistent styling and behavior across views.
 */
import React from 'react';
import { useTranslation } from 'react-i18next';
import {
  PenTool,
  Folder,
  QrCode,
  Video,
  Edit,
  Trash2,
  Check,
  CheckCircle2,
} from 'lucide-react';
import type { Party } from '@sdkwork/im-pc-types';

export interface PartyActionButtonsProps {
  /** The party these actions belong to */
  party: Party;
  /** Whether the party has signed (shows signed badge) */
  hasSigned?: boolean;
  /** Whether to show the sign button */
  showSign?: boolean;
  /** Called when sign button is clicked */
  onSign?: () => void;
  /** Called when drive button is clicked */
  onDrive?: () => void;
  /** Called when QR code button is clicked */
  onQrCode?: () => void;
  /** Called when video call button is clicked */
  onVideoCall?: () => void;
  /** Called when edit button is clicked */
  onEdit?: () => void;
  /** Called when delete/remove button is clicked */
  onDelete?: () => void;
  /** Button size variant */
  size?: 'xs' | 'sm';
  /** Icon component for signed badge */
  SignedIcon?: typeof CheckCircle2;
}

export const PartyActionButtons: React.FC<PartyActionButtonsProps> = ({
  party,
  hasSigned = !!party.signatureUrl,
  showSign = true,
  onSign,
  onDrive,
  onQrCode,
  onVideoCall,
  onEdit,
  onDelete,
  size = 'xs',
  SignedIcon = CheckCircle2,
}) => {
  const { t } = useTranslation('notary');

  const isSmall = size === 'xs';
  const btnBase = isSmall
    ? 'p-2 rounded-lg transition-colors flex items-center gap-1.5 text-xs font-medium'
    : 'px-2 py-1 rounded-lg transition-colors flex items-center gap-1 shrink-0 text-[11px] font-medium';

  return (
    <div className="flex items-center gap-2">
      {/* Signed badge */}
      {hasSigned && (
        <div className="px-2 py-1 bg-teal-500/10 text-teal-400 rounded-lg text-xs font-medium border border-teal-500/20 flex items-center gap-1">
          <SignedIcon size={14} /> {t('createTask.signed')}
        </div>
      )}

      {/* Sign */}
      {showSign && onSign && (
        <button
          onClick={onSign}
          className={`${btnBase} bg-orange-500/10 text-orange-400 hover:bg-orange-500/20 border border-orange-500/20`}
          title={t('createTask.signBtn')}
        >
          <PenTool size={14} /> {isSmall ? '' : t('createTask.signBtn')}
        </button>
      )}

      {/* Drive */}
      {onDrive && (
        <button
          onClick={onDrive}
          className={`${btnBase} bg-blue-500/10 text-blue-400 hover:bg-blue-500/20 border border-blue-500/20`}
          title={t('detail.driveDocuments')}
        >
          <Folder size={14} /> {isSmall ? '' : t('createTask.attachmentUpload')}
        </button>
      )}

      {/* QR Code */}
      {onQrCode && (
        <button
          onClick={onQrCode}
          className={`${btnBase} bg-purple-500/10 text-purple-400 hover:bg-purple-500/20 border border-purple-500/20`}
          title={t('createTask.videoCallQR')}
        >
          <QrCode size={14} /> {isSmall ? '' : t('createTask.videoCallQR')}
        </button>
      )}

      {/* Video Call */}
      {onVideoCall && (
        <button
          onClick={onVideoCall}
          className={`${btnBase} bg-green-500/10 text-green-500 hover:bg-green-500/20 border border-green-500/20`}
        >
          <Video size={14} /> {isSmall ? '' : t('actions.videoCall')}
        </button>
      )}

      {/* Edit */}
      {onEdit && (
        <button
          onClick={onEdit}
          className={`p-2 bg-indigo-500/10 text-indigo-400 hover:bg-indigo-500/20 rounded-lg transition-colors border border-indigo-500/20`}
          title={t('createTask.edit')}
        >
          <Edit size={isSmall ? 16 : 14} />
        </button>
      )}

      {/* Delete/Remove */}
      {onDelete && (
        <button
          onClick={onDelete}
          className={`p-2 bg-red-500/10 text-red-500 hover:bg-red-500/20 rounded-lg transition-colors border border-red-500/20`}
          title={t('createTask.remove')}
        >
          <Trash2 size={isSmall ? 16 : 14} />
        </button>
      )}
    </div>
  );
};