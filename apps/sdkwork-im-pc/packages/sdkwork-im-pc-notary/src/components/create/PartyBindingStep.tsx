/**
 * PartyBindingStep - Notary staff picker and party list (wizard step 2)
 */
import React from 'react';
import { useTranslation } from 'react-i18next';
import { motion, AnimatePresence } from 'motion/react';
import { ChevronLeft, Plus } from 'lucide-react';
import type { Party } from '@sdkwork/im-pc-types';
import type { NotaryStaffOption } from '../../services/NotaryService';
import { PartyCard } from '../shared/PartyCard';

export interface PartyBindingStepProps {
  notaryLabel: string;
  parties: Party[];
  onOpenNotaryPicker: () => void;
  onAddParty: () => void;
  onViewParty: (party: Party) => void;
  onEditParty: (party: Party) => void;
  onRemoveParty: (partyId: string) => void;
  onSignParty: (party: Party) => void;
  onDriveParty: (party: Party) => void;
  onQrCodeParty: (party: Party) => void;
  onVideoCallParty: (party: Party) => void;
}

export const PartyBindingStep: React.FC<PartyBindingStepProps> = ({
  notaryLabel,
  parties,
  onOpenNotaryPicker,
  onAddParty,
  onViewParty,
  onEditParty,
  onRemoveParty,
  onSignParty,
  onDriveParty,
  onQrCodeParty,
  onVideoCallParty,
}) => {
  const { t } = useTranslation('notary');

  return (
    <motion.div initial={{ opacity: 0, y: 10 }} animate={{ opacity: 1, y: 0 }} className="flex flex-col gap-6">
      <h3 className="text-xl font-medium text-gray-200 mb-2">{t('createTask.selectNotaryAndParty')}</h3>

      <div className="flex flex-col gap-2 relative">
        <label className="text-sm font-medium text-gray-400">
          {t('createTask.selectNotary')}
          {' '}
          <span className="text-red-500">*</span>
        </label>
        <button
          type="button"
          onClick={onOpenNotaryPicker}
          className="w-full bg-[#181818] border border-white/10 rounded-lg px-4 py-3 text-sm flex justify-between items-center text-gray-200 outline-none hover:border-white/20 transition-colors"
        >
          <span>
            {notaryLabel
              ? t('createTask.selectedNotary', { name: notaryLabel })
              : t('createTask.clickToSelectNotary')}
          </span>
          <ChevronLeft className="rotate-180 text-gray-500" size={16} />
        </button>
      </div>

      <div className="flex flex-col gap-4 mt-4">
        <div className="flex justify-between items-center">
          <label className="text-sm font-medium text-gray-400">{t('createTask.partyList')}</label>
          <button
            type="button"
            onClick={onAddParty}
            className="flex items-center gap-1.5 text-xs text-white bg-indigo-600 hover:bg-indigo-700 px-3 py-1.5 rounded-lg transition-colors"
          >
            <Plus size={14} />
            {t('createTask.addPartyBtn')}
          </button>
        </div>

        {parties.length === 0 ? (
          <div className="py-8 text-center border border-dashed border-white/10 rounded-xl text-sm text-gray-500 bg-[#181818]/50">
            {t('createTask.noPartyAdded')}
          </div>
        ) : (
          <div className="flex flex-col gap-3">
            <AnimatePresence>
              {parties.map((party) => (
                <PartyCard
                  key={party.id}
                  party={party}
                  animate
                  useInitial={false}
                  onClick={onViewParty}
                  onDoubleClick={onEditParty}
                  onSign={() => onSignParty(party)}
                  onDrive={() => onDriveParty(party)}
                  onQrCode={() => onQrCodeParty(party)}
                  onVideoCall={() => onVideoCallParty(party)}
                  onEdit={() => onEditParty(party)}
                  onDelete={() => onRemoveParty(party.id)}
                />
              ))}
            </AnimatePresence>
          </div>
        )}
      </div>
    </motion.div>
  );
};
