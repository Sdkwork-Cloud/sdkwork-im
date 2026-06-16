/**
 * PartyDrawerFooter - Action bar at the bottom of PartyDrawer
 */
import React from 'react';
import { useTranslation } from 'react-i18next';
import { PenTool } from 'lucide-react';
import type { Party } from '@sdkwork/im-pc-types';

export interface PartyDrawerFooterProps {
  /** The party being edited (if any) */
  party: Partial<Party> | null;
  /** Whether required fields are filled */
  canSave: boolean;
  /** Whether the form is read-only */
  readOnly: boolean;
  /** Called when online signature is triggered */
  onSign?: (party: Party) => void;
  /** Called when cancel/close is clicked */
  onClose: () => void;
  /** Called when save is clicked */
  onSave: () => void;
}

export const PartyDrawerFooter: React.FC<PartyDrawerFooterProps> = ({
  party,
  canSave,
  readOnly,
  onSign,
  onClose,
  onSave,
}) => {
  const { t } = useTranslation('notary');

  return (
    <div className="p-6 border-t border-white/5 bg-[#2b2b2d] shrink-0 flex justify-between items-center">
      {/* Left: Online Signature */}
      <div>
        {party && onSign && !readOnly && (
          <button
            onClick={() => {
              onClose();
              onSign(party as Party);
            }}
            className="px-4 py-2 rounded-lg text-sm text-yellow-400 bg-yellow-400/10 hover:bg-yellow-400/20 transition-colors border border-yellow-400/20 flex items-center gap-2"
          >
            <PenTool size={16} /> {t('party.onlineSignature')}
          </button>
        )}
      </div>

      {/* Right: Cancel + Save */}
      <div className="flex justify-end gap-3">
        <button
          onClick={onClose}
          className="px-6 py-2 rounded-lg text-sm text-gray-300 border border-white/10 hover:bg-white/5 transition-colors"
        >
          {readOnly ? t('party.close') : t('party.cancelBtn')}
        </button>
        {!readOnly && (
          <button
            onClick={onSave}
            disabled={!canSave}
            className="px-6 py-2 rounded-lg text-sm text-white bg-indigo-600 hover:bg-indigo-700 disabled:opacity-50 transition-colors shadow-md"
          >
            {party ? t('party.saveRecord') : t('party.saveAndAdd')}
          </button>
        )}
      </div>
    </div>
  );
};