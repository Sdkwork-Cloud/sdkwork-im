/**
 * VideoCallQROverlay - QR code overlay for party video call
 */
import React from 'react';
import { useTranslation } from 'react-i18next';
import { motion, AnimatePresence } from 'motion/react';
import { X, Video } from 'lucide-react';
import type { Party } from '@sdkwork/clawchat-pc-types';

function isQrCellFilled(index: number): boolean {
  const row = Math.floor(index / 8);
  const column = index % 8;
  return (row * 3 + column * 5 + row * column) % 7 < 3;
}

export interface VideoCallQROverlayProps {
  /** Whether the overlay is open */
  isOpen: boolean;
  /** The party for whom the QR code is displayed */
  party: Party | null;
  /** Called when the overlay is closed */
  onClose: () => void;
}

export const VideoCallQROverlay: React.FC<VideoCallQROverlayProps> = ({
  isOpen,
  party,
  onClose,
}) => {
  const { t } = useTranslation('notary');

  return (
    <AnimatePresence>
      {isOpen && party && (
        <motion.div
          initial={{ opacity: 0 }}
          animate={{ opacity: 1 }}
          exit={{ opacity: 0 }}
          onClick={onClose}
          className="fixed inset-0 bg-black/60 backdrop-blur-sm z-50 flex items-center justify-center"
        >
          <motion.div
            initial={{ opacity: 0, scale: 0.95 }}
            animate={{ opacity: 1, scale: 1 }}
            exit={{ opacity: 0, scale: 0.95 }}
            onClick={(e) => e.stopPropagation()}
            className="bg-[#1e1e1e] rounded-2xl shadow-2xl border border-white/10 p-8 flex flex-col items-center gap-6 max-w-md w-full"
          >
            {/* Header */}
            <div className="flex items-center justify-between w-full">
              <h3 className="text-lg font-medium text-gray-200 flex items-center gap-2">
                <Video size={20} className="text-green-400" /> {t('createTask.videoCallQR')}
              </h3>
              <button onClick={onClose} className="text-gray-400 hover:text-white p-2 rounded-lg hover:bg-white/10 transition-colors">
                <X size={20} />
              </button>
            </div>

            {/* Party info */}
            <div className="flex items-center gap-3">
              <div className="w-12 h-12 rounded-full bg-indigo-500/10 flex items-center justify-center text-indigo-400 text-xl font-medium">
                {party.name.charAt(0)}
              </div>
              <div>
                <div className="text-gray-200 font-medium">{party.name}</div>
                <div className="text-xs text-gray-500">{party.role}</div>
              </div>
            </div>

            {/* QR Code placeholder */}
            <div className="w-48 h-48 bg-white rounded-xl p-3 flex items-center justify-center">
              <div className="w-full h-full bg-gray-100 rounded-lg flex items-center justify-center text-gray-400">
                <div className="grid grid-cols-8 gap-0.5 w-full h-full p-2">
                  {Array.from({ length: 64 }).map((_, i) => (
                    <div
                      key={i}
                      className={`rounded-sm ${isQrCellFilled(i) ? 'bg-gray-800' : 'bg-transparent'}`}
                    />
                  ))}
                </div>
              </div>
            </div>

            {/* Hint */}
            <p className="text-sm text-gray-500 text-center">
              {t('createTask.scanQRHint')}
            </p>
          </motion.div>
        </motion.div>
      )}
    </AnimatePresence>
  );
};
