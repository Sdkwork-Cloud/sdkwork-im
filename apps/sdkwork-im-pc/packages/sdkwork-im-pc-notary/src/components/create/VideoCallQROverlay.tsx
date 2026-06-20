/**
 * VideoCallQROverlay - QR code overlay for party video call invite
 */
import React from 'react';
import { useTranslation } from 'react-i18next';
import QRCode from 'react-qr-code';
import { motion, AnimatePresence } from 'motion/react';
import { Share, X, Video } from 'lucide-react';
import type { Party } from '@sdkwork/im-pc-types';

export interface VideoCallQROverlayProps {
  isOpen: boolean;
  party: Party | null;
  inviteUrl?: string;
  onClose: () => void;
}

export const VideoCallQROverlay: React.FC<VideoCallQROverlayProps> = ({
  isOpen,
  party,
  inviteUrl,
  onClose,
}) => {
  const { t } = useTranslation('notary');

  const copyInviteLink = async () => {
    if (!inviteUrl) {
      return;
    }
    await navigator.clipboard.writeText(inviteUrl);
  };

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
            onClick={(event) => event.stopPropagation()}
            className="bg-[#1e1e1e] rounded-2xl shadow-2xl border border-white/10 p-8 flex flex-col items-center gap-6 max-w-md w-full"
          >
            <div className="flex items-center justify-between w-full">
              <h3 className="text-lg font-medium text-gray-200 flex items-center gap-2">
                <Video size={20} className="text-green-400" />
                {' '}
                {t('createTask.videoCallQR')}
              </h3>
              <button
                type="button"
                onClick={onClose}
                className="text-gray-400 hover:text-white p-2 rounded-lg hover:bg-white/10 transition-colors"
              >
                <span className="sr-only">{t('party.close')}</span>
                <X size={20} />
              </button>
            </div>

            <div className="flex items-center gap-3">
              <div className="w-12 h-12 rounded-full bg-indigo-500/10 flex items-center justify-center text-indigo-400 text-xl font-medium">
                {party.name.charAt(0)}
              </div>
              <div>
                <div className="text-gray-200 font-medium">{party.name}</div>
                <div className="text-xs text-gray-500">{party.role}</div>
              </div>
            </div>

            <div className="w-48 h-48 bg-white rounded-xl p-3 flex items-center justify-center">
              {inviteUrl ? (
                <QRCode value={inviteUrl} className="w-full h-full" />
              ) : (
                <p className="text-sm text-gray-500 text-center px-4">{t('createTask.videoInvitePending')}</p>
              )}
            </div>

            <p className="text-sm text-gray-500 text-center">
              {inviteUrl ? t('createTask.scanQRHint') : t('createTask.videoRequiresCase')}
            </p>

            {inviteUrl && (
              <button
                type="button"
                onClick={() => void copyInviteLink()}
                className="w-full flex items-center justify-center gap-2 text-sm text-indigo-400 bg-indigo-500/10 hover:bg-indigo-500/20 py-2 rounded-lg transition-colors border border-indigo-500/20"
              >
                <Share size={16} />
                {t('signaturePad.copyLink')}
              </button>
            )}
          </motion.div>
        </motion.div>
      )}
    </AnimatePresence>
  );
};
