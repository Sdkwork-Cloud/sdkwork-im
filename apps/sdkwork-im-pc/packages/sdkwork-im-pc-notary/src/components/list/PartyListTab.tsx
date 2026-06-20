/**
 * PartyListTab - Party list with expand/collapse functionality in the detail pane
 */
import React, { useEffect, useState } from 'react';
import { useTranslation } from 'react-i18next';
import { motion, AnimatePresence } from 'motion/react';
import { ShieldCheck } from 'lucide-react';
import type { NotaryTask, Party } from '@sdkwork/im-pc-types';
import { EMPTY_NOTARY_PRINT_IMAGE_URL } from '../../constants';
import type { PartyIdentityMediaUrls } from '../../types';
import { PartyCard } from '../shared/PartyCard';

export interface PartyListTabProps {
  task: NotaryTask;
  expandedParty: string | null;
  expandedPartyMediaLoading?: boolean;
  expandedPartyMediaUrls?: PartyIdentityMediaUrls | null;
  onExpand: (partyId: string) => void;
  onEdit: (party: Party) => void;
  onSign: (party: Party) => void;
  onDrive: (party: Party) => void;
  onVideoCall: (party: Party) => void;
}

const IdentityThumbnail: React.FC<{ src?: string; alt: string; className?: string; loading?: boolean }> = ({
  src,
  alt,
  className = 'w-20 h-12',
  loading = false,
}) => {
  const { t } = useTranslation('notary');
  const [failed, setFailed] = useState(false);

  useEffect(() => {
    setFailed(false);
  }, [src]);

  const imageSrc = src && !failed ? src : EMPTY_NOTARY_PRINT_IMAGE_URL;
  const showPlaceholder = !src || failed;

  return (
    <div className={`${className} bg-white/5 rounded border border-white/10 overflow-hidden flex items-center justify-center`}>
      {loading && !src ? (
        <span className="text-[10px] text-gray-500 px-1 text-center">{t('stats.loading')}</span>
      ) : showPlaceholder ? (
        <span className="text-[10px] text-gray-500 px-1 text-center">{t('detail.noMedia')}</span>
      ) : (
        <img
          src={imageSrc}
          alt={alt}
          className="w-full h-full object-cover"
          onError={() => setFailed(true)}
        />
      )}
    </div>
  );
};

export const PartyListTab: React.FC<PartyListTabProps> = ({
  task,
  expandedParty,
  expandedPartyMediaLoading = false,
  expandedPartyMediaUrls,
  onExpand,
  onEdit,
  onSign,
  onDrive,
  onVideoCall,
}) => {
  const { t } = useTranslation('notary');
  const parties = task.parties || [];

  if (parties.length === 0) {
    return (
      <div className="text-sm text-gray-500 text-center py-8 bg-[#181818]/50 rounded-lg border border-dashed border-white/5">
        {t('detail.noPartyInfo')}
      </div>
    );
  }

  return (
    <div className="flex flex-col gap-4">
      {parties.map((party) => (
        <PartyCard
          key={party.id}
          party={party}
          expanded={expandedParty === party.id}
          showPhone
          onExpand={onExpand}
          onDoubleClick={onEdit}
          showSign={task.status !== 'COMPLETED' && task.status !== 'REJECTED'}
          onSign={() => onSign(party)}
          onDrive={() => onDrive(party)}
          onVideoCall={() => onVideoCall(party)}
          onEdit={() => onEdit(party)}
        >
          <AnimatePresence>
            {expandedParty === party.id && (
              <motion.div
                initial={{ height: 0, opacity: 0 }}
                animate={{ height: 'auto', opacity: 1 }}
                exit={{ height: 0, opacity: 0 }}
                className="overflow-hidden"
              >
                <div className="mt-3 bg-black/20 p-4 rounded-lg border border-white/5 flex flex-col gap-4 text-xs">
                  <div className="flex justify-between items-start">
                    <div>
                      <div className="text-gray-500 mb-1">{t('detail.identityVerification')}</div>
                      <div className={`flex items-center gap-1.5 font-medium ${
                        party.identityVerificationStatus === 'verified' ? 'text-green-500' : 'text-orange-400'
                      }`}>
                        <ShieldCheck size={14} />
                        {' '}
                        {party.identityVerificationStatus === 'verified'
                          ? t('detail.publicSecurityDbMatch')
                          : t('detail.pendingVerification')}
                      </div>
                    </div>
                    <div className="text-right">
                      <div className="text-gray-500 mb-1">{t('detail.faceCapture')}</div>
                      <div className="text-gray-300">{party.faceCaptureTime || t('common.notAvailable')}</div>
                    </div>
                  </div>
                  <div className="grid grid-cols-2 gap-4 pt-3 border-t border-white/5">
                    <div>
                      <div className="text-gray-500 mb-1">{t('detail.idPhotos')}</div>
                      <div className="flex gap-2 min-h-[60px]">
                        <IdentityThumbnail
                          src={expandedPartyMediaUrls?.identityFrontUrl}
                          alt={t('party.portraitSide')}
                          loading={expandedPartyMediaLoading}
                        />
                        <IdentityThumbnail
                          src={expandedPartyMediaUrls?.identityBackUrl}
                          alt={t('party.emblemSide')}
                          loading={expandedPartyMediaLoading}
                        />
                      </div>
                    </div>
                    <div>
                      <div className="text-gray-500 mb-1">{t('detail.liveCapture')}</div>
                      <IdentityThumbnail
                        src={expandedPartyMediaUrls?.faceImageUrl}
                        alt={t('detail.liveCapture')}
                        className="w-16 h-16"
                        loading={expandedPartyMediaLoading}
                      />
                    </div>
                  </div>
                </div>
              </motion.div>
            )}
          </AnimatePresence>
        </PartyCard>
      ))}
    </div>
  );
};
