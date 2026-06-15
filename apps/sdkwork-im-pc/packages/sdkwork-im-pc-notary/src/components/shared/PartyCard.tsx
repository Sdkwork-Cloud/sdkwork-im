/**
 * PartyCard - Shared party card component
 *
 * Renders a party's basic info (avatar, name, role, identity ID)
 * with optional action buttons and expand/collapse.
 * Used in both the detail pane and create wizard.
 */
import React from 'react';
import { useTranslation } from 'react-i18next';
import { motion } from 'motion/react';
import { User as UserIcon } from 'lucide-react';
import type { Party } from '@sdkwork/clawchat-pc-types';
import { PartyActionButtons } from './PartyActionButtons';

export interface PartyCardProps {
  /** The party to display */
  party: Party;
  /** Whether the card is in expanded state */
  expanded?: boolean;
  /** Called when expand/collapse toggle is clicked */
  onExpand?: (partyId: string) => void;
  /** Whether to show phone number */
  showPhone?: boolean;
  /** Whether to use avatar initial instead of UserIcon */
  useInitial?: boolean;
  /** Whether to animate entry/exit */
  animate?: boolean;
  /** Called on double-click */
  onDoubleClick?: (party: Party) => void;
  /** Called on single click */
  onClick?: (party: Party) => void;
  /** Action button callbacks */
  onSign?: () => void;
  onDrive?: () => void;
  onQrCode?: () => void;
  onVideoCall?: () => void;
  onEdit?: () => void;
  onDelete?: () => void;
  /** Whether to show the sign button */
  showSign?: boolean;
  /** Additional content rendered below the card header (e.g., expanded info) */
  children?: React.ReactNode;
}

export const PartyCard: React.FC<PartyCardProps> = ({
  party,
  expanded,
  onExpand,
  showPhone = false,
  useInitial = true,
  animate = false,
  onDoubleClick,
  onClick,
  onSign,
  onDrive,
  onQrCode,
  onVideoCall,
  onEdit,
  onDelete,
  showSign = true,
  children,
}) => {
  const { t } = useTranslation('notary');

  const cardContent = (
    <div
      className="bg-[#181818] border border-white/5 rounded-xl p-4 flex flex-col gap-3 transition-colors hover:border-indigo-500/30 cursor-pointer"
      onDoubleClick={onDoubleClick ? () => onDoubleClick(party) : undefined}
      onClick={onClick ? () => onClick(party) : undefined}
    >
      {/* Header row: avatar + info + actions */}
      <div className="flex items-center justify-between border-b border-white/5 pb-3">
        <div className="flex items-center gap-3">
          {/* Avatar */}
          <div className="w-10 h-10 rounded-full bg-indigo-500/10 flex items-center justify-center text-indigo-400 text-lg font-medium">
            {useInitial ? party.name.charAt(0) : <UserIcon size={20} />}
          </div>

          {/* Name, role, gender, identity ID */}
          <div>
            <div className="text-sm font-medium text-gray-200 flex items-center gap-2">
              {party.name}
              {party.role && (
                <span className="px-1.5 py-0.5 rounded text-[10px] bg-white/10 text-gray-400">
                  {party.role}
                </span>
              )}
              {party.gender && (
                <span className="px-1.5 py-0.5 rounded text-[10px] bg-white/10 text-gray-400">
                  {party.gender}
                </span>
              )}
            </div>
            <div className="text-xs text-gray-500 mt-1 font-mono">
              {party.identityId}
            </div>
          </div>
        </div>

        {/* Action buttons */}
        <div onClick={(e) => e.stopPropagation()}>
          <PartyActionButtons
            party={party}
            showSign={showSign}
            onSign={onSign}
            onDrive={onDrive}
            onQrCode={onQrCode}
            onVideoCall={onVideoCall}
            onEdit={onEdit}
            onDelete={onDelete}
            size="sm"
          />
        </div>
      </div>

      {/* Phone + expand toggle */}
      {(showPhone || onExpand) && (
        <div className="text-xs text-gray-400 flex items-center justify-between">
          {showPhone && (
            <div className="flex items-center gap-2">
              <span className="w-16">{t('party.phone')}:</span>
              <span className="text-gray-200 font-mono">{party.phone}</span>
            </div>
          )}
          {onExpand && (
            <button
              onClick={() => onExpand(party.id)}
              className="text-indigo-400 hover:text-indigo-300 font-medium transition-colors"
            >
              {expanded ? t('detail.collapseInfo') : t('detail.viewInfo')}
            </button>
          )}
        </div>
      )}

      {/* Expanded content */}
      {children}
    </div>
  );

  if (animate) {
    return (
      <motion.div
        key={party.id}
        initial={{ opacity: 0, height: 0 }}
        animate={{ opacity: 1, height: 'auto' }}
        exit={{ opacity: 0, height: 0 }}
      >
        {cardContent}
      </motion.div>
    );
  }

  return <div key={party.id}>{cardContent}</div>;
};