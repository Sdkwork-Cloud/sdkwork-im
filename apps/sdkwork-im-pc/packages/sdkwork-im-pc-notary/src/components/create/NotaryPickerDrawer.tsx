/**
 * NotaryPickerDrawer - Side drawer for selecting a notary staff member
 */
import React, { useMemo, useState } from 'react';
import { useTranslation } from 'react-i18next';
import { motion, AnimatePresence } from 'motion/react';
import { Search, X } from 'lucide-react';
import type { NotaryStaffOption } from '../../services/NotaryService';

export interface NotaryPickerDrawerProps {
  isOpen: boolean;
  staff: NotaryStaffOption[];
  selected: NotaryStaffOption | null;
  onClose: () => void;
  onSelect: (staff: NotaryStaffOption) => void;
}

export const NotaryPickerDrawer: React.FC<NotaryPickerDrawerProps> = ({
  isOpen,
  staff,
  selected,
  onClose,
  onSelect,
}) => {
  const { t } = useTranslation('notary');
  const [searchTerm, setSearchTerm] = useState('');

  const filteredStaff = useMemo(() => {
    const normalized = searchTerm.trim().toLowerCase();
    if (!normalized) {
      return staff;
    }
    return staff.filter((member) => {
      const haystack = [
        member.displayName,
        member.membershipId,
        member.notaryStaffRole,
        ...(member.positions ?? []),
        ...(member.departments ?? []),
      ]
        .filter(Boolean)
        .join(' ')
        .toLowerCase();
      return haystack.includes(normalized);
    });
  }, [searchTerm, staff]);

  return (
    <AnimatePresence>
      {isOpen && (
        <>
          <motion.div
            initial={{ opacity: 0 }}
            animate={{ opacity: 1 }}
            exit={{ opacity: 0 }}
            onClick={onClose}
            className="fixed inset-0 bg-black/40 backdrop-blur-sm z-40"
          />
          <motion.div
            initial={{ x: '100%' }}
            animate={{ x: 0 }}
            exit={{ x: '100%' }}
            transition={{ type: 'spring', damping: 25, stiffness: 200 }}
            className="fixed right-0 top-0 bottom-0 w-full max-w-[400px] bg-[#222224] border-l border-white/5 z-50 flex flex-col shadow-2xl"
          >
            <div className="flex justify-between items-center p-6 border-b border-white/5 bg-[#2b2b2d] shrink-0">
              <h3 className="text-lg font-medium text-gray-200">{t('createTask.selectNotaryTitle')}</h3>
              <button type="button" onClick={onClose} className="text-gray-400 hover:text-white p-1 rounded-full hover:bg-white/10 transition-colors">
                <span className="sr-only">{t('party.close')}</span>
                <X size={20} />
              </button>
            </div>

            <div className="p-4 border-b border-white/5 shrink-0 bg-[#2b2b2d]">
              <div className="relative">
                <Search className="absolute left-3 top-1/2 -translate-y-1/2 text-gray-500" size={16} />
                <input
                  type="text"
                  value={searchTerm}
                  onChange={(event) => setSearchTerm(event.target.value)}
                  placeholder={t('createTask.searchNotaryPlaceholder')}
                  className="w-full bg-[#181818] border border-white/10 rounded-lg pl-9 pr-4 py-2 text-sm text-gray-200 outline-none focus:border-indigo-500"
                />
              </div>
            </div>

            <div className="flex-1 overflow-y-auto custom-scrollbar">
              <div className="px-4 py-2 flex flex-col gap-1">
                {filteredStaff.length === 0 ? (
                  <div className="py-8 text-center text-sm text-gray-500">{t('createTask.searchNoResults')}</div>
                ) : (
                  filteredStaff.map((member) => (
                    <div
                      key={member.membershipId}
                      onClick={() => {
                        onSelect(member);
                        onClose();
                      }}
                      className={`flex items-center gap-3 p-3 hover:bg-white/5 rounded-lg cursor-pointer transition-colors group ${
                        selected?.membershipId === member.membershipId ? 'bg-indigo-500/10' : ''
                      }`}
                    >
                      <div className="w-10 h-10 rounded-full bg-indigo-500/20 text-indigo-400 flex items-center justify-center font-medium">
                        {(member.displayName || member.membershipId).slice(0, 1).toUpperCase()}
                      </div>
                      <div>
                        <div className="text-sm font-medium text-gray-200 group-hover:text-indigo-400 transition-colors">
                          {member.displayName}
                        </div>
                        <div className="text-xs text-gray-500">
                          {[member.notaryStaffRole, ...(member.positions ?? []), ...(member.departments ?? [])]
                            .filter(Boolean)
                            .join(' / ') || member.status}
                        </div>
                      </div>
                    </div>
                  ))
                )}
              </div>
            </div>
          </motion.div>
        </>
      )}
    </AnimatePresence>
  );
};
