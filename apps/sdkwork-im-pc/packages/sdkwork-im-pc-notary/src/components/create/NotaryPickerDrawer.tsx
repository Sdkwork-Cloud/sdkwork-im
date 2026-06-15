/**
 * NotaryPickerDrawer - Side drawer for selecting a notary staff member
 */
import React from 'react';
import { useTranslation } from 'react-i18next';
import { motion, AnimatePresence } from 'motion/react';
import { X } from 'lucide-react';
import type { NotaryStaffOption } from '../../services/NotaryService';

export interface NotaryPickerDrawerProps {
  /** Whether the drawer is open */
  isOpen: boolean;
  /** List of available notary staff */
  staff: NotaryStaffOption[];
  /** Currently selected staff member */
  selected: NotaryStaffOption | null;
  /** Called when drawer is closed */
  onClose: () => void;
  /** Called when a staff member is selected */
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
            className="fixed right-0 top-0 bottom-0 w-[400px] bg-[#222224] border-l border-white/5 z-50 flex flex-col shadow-2xl"
          >
            {/* Header */}
            <div className="flex justify-between items-center p-6 border-b border-white/5 bg-[#2b2b2d] shrink-0">
              <h3 className="text-lg font-medium text-gray-200">{t('createTask.selectNotaryTitle')}</h3>
              <button onClick={onClose} className="text-gray-400 hover:text-white p-1 rounded-full hover:bg-white/10 transition-colors">
                <span className="sr-only">Close</span>
                <X size={20} />
              </button>
            </div>

            {/* Search */}
            <div className="p-4 border-b border-white/5 shrink-0 bg-[#2b2b2d]">
              <div className="relative">
                <input
                  type="text"
                  placeholder={t('createTask.searchNotaryPlaceholder')}
                  className="w-full bg-[#181818] border border-white/10 rounded-lg pl-9 pr-4 py-2 text-sm text-gray-200 outline-none focus:border-indigo-500"
                />
                <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round" className="absolute left-3 top-1/2 -translate-y-1/2 text-gray-500">
                  <circle cx="11" cy="11" r="8" /><path d="m21 21-4.3-4.3" />
                </svg>
              </div>
            </div>

            {/* Staff list */}
            <div className="flex-1 overflow-y-auto custom-scrollbar">
              <div className="px-4 py-2 flex flex-col gap-1">
                {staff.map((s) => (
                  <div
                    key={s.membershipId}
                    onClick={() => {
                      onSelect(s);
                      onClose();
                    }}
                    className={`flex items-center gap-3 p-3 hover:bg-white/5 rounded-lg cursor-pointer transition-colors group ${
                      selected?.membershipId === s.membershipId ? 'bg-indigo-500/10' : ''
                    }`}
                  >
                    <div className="w-10 h-10 rounded-full bg-indigo-500/20 text-indigo-400 flex items-center justify-center font-medium">
                      {(s.displayName || s.membershipId).slice(0, 1).toUpperCase()}
                    </div>
                    <div>
                      <div className="text-sm font-medium text-gray-200 group-hover:text-indigo-400 transition-colors">{s.displayName}</div>
                      <div className="text-xs text-gray-500">
                        {[s.notaryStaffRole, ...(s.positions ?? []), ...(s.departments ?? [])].filter(Boolean).join(' / ') || s.status}
                      </div>
                    </div>
                  </div>
                ))}
              </div>
            </div>
          </motion.div>
        </>
      )}
    </AnimatePresence>
  );
};