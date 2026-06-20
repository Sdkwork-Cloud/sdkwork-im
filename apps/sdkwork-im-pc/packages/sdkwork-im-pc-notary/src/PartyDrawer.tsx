import React, { useEffect, useMemo, useState } from 'react';
import { useTranslation } from 'react-i18next';
import type { TFunction } from 'i18next';
import { motion, AnimatePresence } from 'motion/react';
import { X } from 'lucide-react';
import type { Party } from '@sdkwork/im-pc-types';
import { toast } from '@sdkwork/im-pc-chat';
import {
  AuxiliaryMaterials,
  BasicInfoForm,
  FaceCaptureOverlay,
  IdentityVerification,
  PartyDrawerFooter,
} from './components/party';
import { useCameraCapture } from './hooks/useCameraCapture';
import { useLocalAttachments } from './hooks/useLocalAttachments';
import { generateClientId } from './utils/notaryTask';
import type { PartyFormState } from './types';

interface PartyDrawerProps {
  isOpen: boolean;
  onClose: () => void;
  party: Partial<Party> | null;
  onSave: (party: Party) => void;
  readOnly?: boolean;
  onSign?: (party: Party) => void;
}

function createEmptyFormState(t: TFunction): PartyFormState {
  return {
    name: '',
    phone: '',
    identityId: '',
    address: '',
    role: t('party.applicant'),
    gender: t('party.male'),
    birthDate: '',
    remarks: '',
    identityValidDateStart: '',
    identityValidDateEnd: '',
  };
}

function mapPartyToFormState(party: Partial<Party>, t: TFunction): PartyFormState {
  return {
    name: party.name || '',
    phone: party.phone || '',
    identityId: party.identityId || '',
    address: party.address || '',
    role: party.role || t('party.applicant'),
    gender: party.gender || t('party.male'),
    birthDate: party.birthDate || '',
    remarks: party.remarks || '',
    identityValidDateStart: party.identityValidDateStart || '',
    identityValidDateEnd: party.identityValidDateEnd || '',
  };
}

export const PartyDrawer: React.FC<PartyDrawerProps> = ({
  isOpen,
  onClose,
  party,
  onSave,
  readOnly = false,
  onSign,
}) => {
  const { t } = useTranslation('notary');
  const emptyForm = useMemo(() => createEmptyFormState(t), [t]);

  const [formState, setFormState] = useState<PartyFormState>(emptyForm);
  const [idFront, setIdFront] = useState<string | null>(null);
  const [idFrontFile, setIdFrontFile] = useState<File | null>(null);
  const [idBack, setIdBack] = useState<string | null>(null);
  const [idBackFile, setIdBackFile] = useState<File | null>(null);

  const {
    isCameraOpen,
    faceImage,
    videoRef,
    canvasRef,
    startCamera,
    stopCamera,
    captureFace,
    clearFaceImage,
  } = useCameraCapture();

  const { attachments, addFiles, removeAttachment, resetAttachments } = useLocalAttachments();

  useEffect(() => {
    if (!isOpen) {
      return;
    }
    setFormState(party ? mapPartyToFormState(party, t) : emptyForm);
    setIdFront(null);
    setIdFrontFile(null);
    setIdBack(null);
    setIdBackFile(null);
    clearFaceImage();
    resetAttachments();
  }, [clearFaceImage, emptyForm, isOpen, party, resetAttachments, t]);

  const drawerTitle = readOnly
    ? t('party.viewInfo')
    : party
      ? t('party.editParty')
      : t('party.addParty');

  const handleIdFrontUpload = (file: File) => {
    setIdFront(URL.createObjectURL(file));
    setIdFrontFile(file);
  };

  const handleIdBackUpload = (file: File) => {
    setIdBack(URL.createObjectURL(file));
    setIdBackFile(file);
  };

  const handleStartCamera = async () => {
    try {
      await startCamera();
    } catch {
      toast(t('toast.cameraAccessDenied'), 'error');
    }
  };

  const handleCompare = () => {
    toast(t('toast.identityWillBeVerified'), 'info');
  };

  const submitParty = () => {
    if (!formState.name || !formState.identityId) {
      return;
    }
    const submittedParty: Party = {
      ...formState,
      id: party?.id || generateClientId('party'),
      identityFrontFile: idFrontFile ?? undefined,
      identityBackFile: idBackFile ?? undefined,
      faceImageDataUrl: faceImage ?? undefined,
    };
    onSave(submittedParty);
  };

  return (
    <AnimatePresence>
      {isOpen && (
        <>
          <motion.div
            initial={{ opacity: 0 }}
            animate={{ opacity: 1 }}
            exit={{ opacity: 0 }}
            onClick={onClose}
            className="fixed inset-0 bg-black/60 backdrop-blur-sm z-[200]"
          />
          <motion.div
            initial={{ x: '100%' }}
            animate={{ x: 0 }}
            exit={{ x: '100%' }}
            transition={{ type: 'spring', damping: 25, stiffness: 200 }}
            className="fixed right-0 top-0 bottom-0 w-full max-w-[640px] bg-[#222224] border-l border-white/5 z-[210] flex flex-col shadow-2xl"
          >
            <div className="flex justify-between items-center p-6 border-b border-white/5 bg-[#2b2b2d] shrink-0">
              <h3 className="text-lg font-medium text-gray-200">{drawerTitle}</h3>
              <button
                type="button"
                onClick={onClose}
                className="text-gray-400 hover:text-white p-1 rounded-full hover:bg-white/10 transition-colors"
              >
                <span className="sr-only">{t('party.close')}</span>
                <X size={20} />
              </button>
            </div>

            <div className="flex-1 overflow-y-auto custom-scrollbar p-6 flex flex-col gap-8">
              <IdentityVerification
                idFront={idFront}
                idBack={idBack}
                faceImage={faceImage}
                readOnly={readOnly}
                onIdFrontUpload={handleIdFrontUpload}
                onIdFrontRemove={() => {
                  setIdFront(null);
                  setIdFrontFile(null);
                }}
                onIdBackUpload={handleIdBackUpload}
                onIdBackRemove={() => {
                  setIdBack(null);
                  setIdBackFile(null);
                }}
                onFaceCapture={() => void handleStartCamera()}
                onFaceRemove={clearFaceImage}
                onCompare={handleCompare}
              />

              <div className="h-px bg-white/5 w-full" />

              <BasicInfoForm
                formState={formState}
                readOnly={readOnly}
                onChange={setFormState}
              />

              <div className="h-px bg-white/5 w-full" />

              <AuxiliaryMaterials
                remarks={formState.remarks}
                attachments={attachments}
                readOnly={readOnly}
                onRemarksChange={(remarks) => setFormState((current) => ({ ...current, remarks }))}
                onAttachmentsChange={(nextAttachments) => {
                  attachments
                    .filter((attachment) => !nextAttachments.some((next) => next.id === attachment.id))
                    .forEach((attachment) => removeAttachment(attachment.id));
                }}
                onUpload={(files) => addFiles(files)}
              />
            </div>

            <PartyDrawerFooter
              party={party}
              canSave={Boolean(formState.name && formState.identityId)}
              readOnly={readOnly}
              onSign={onSign}
              onClose={onClose}
              onSave={submitParty}
            />
          </motion.div>

          <FaceCaptureOverlay
            isOpen={isCameraOpen}
            videoRef={videoRef}
            canvasRef={canvasRef}
            onCapture={captureFace}
            onClose={stopCamera}
          />
        </>
      )}
    </AnimatePresence>
  );
};
