import './i18n';
import React, { useEffect, useMemo, useState } from 'react';
import { useTranslation } from 'react-i18next';
import { AnimatePresence } from 'motion/react';
import { PenTool } from 'lucide-react';
import { toast } from '@sdkwork/im-pc-chat';
import { MediaViewer } from '@sdkwork/im-pc-commons';
import type { NotaryDocument, Party } from '@sdkwork/im-pc-types';
import {
  BusinessTypeSelector,
  ConfirmationSummary,
  NotaryPickerDrawer,
  PartyBindingStep,
  TaskDetailsForm,
  VideoCallQROverlay,
  WizardHeader,
  WizardNavigation,
  WizardStepper,
} from './components/create';
import { PartyDriveModal } from './components/shared/PartyDriveModal';
import { useLocalAttachments } from './hooks/useLocalAttachments';
import { PartyDrawer } from './PartyDrawer';
import { SignaturePad } from './SignaturePad';
import { notaryService, type NotaryStaffOption } from './services/NotaryService';
import type { LocalAttachment, MediaPreviewState, NotaryMatterOption } from './types';

const WIZARD_STEPS = 4;

export const CreateNotaryTaskView: React.FC<{ onBack: () => void; onSuccess?: () => void }> = ({
  onBack,
  onSuccess,
}) => {
  const { t } = useTranslation('notary');

  const [step, setStep] = useState(1);
  const [businessType, setBusinessType] = useState('');
  const [selectedSkuId, setSelectedSkuId] = useState<string | undefined>(undefined);
  const [notary, setNotary] = useState('');
  const [notaryStaffMembers, setNotaryStaffMembers] = useState<NotaryStaffOption[]>([]);
  const [selectedNotaryStaff, setSelectedNotaryStaff] = useState<NotaryStaffOption | null>(null);
  const [matters, setMatters] = useState<NotaryMatterOption[]>([]);
  const [wizardLoading, setWizardLoading] = useState(true);
  const [showNotaryDrawer, setShowNotaryDrawer] = useState(false);
  const [parties, setParties] = useState<Party[]>([]);
  const [appInfo, setAppInfo] = useState('');
  const [isSubmitting, setIsSubmitting] = useState(false);
  const [showAddPartyPanel, setShowAddPartyPanel] = useState(false);
  const [isPartyReadOnly, setIsPartyReadOnly] = useState(false);
  const [editingPartyId, setEditingPartyId] = useState<string | null>(null);
  const [activeQrCodeParty, setActiveQrCodeParty] = useState<Party | null>(null);
  const [activeDriveParty, setActiveDriveParty] = useState<Party | null>(null);
  const [activeSignParty, setActiveSignParty] = useState<Party | null>(null);
  const [previewMedia, setPreviewMedia] = useState<MediaPreviewState>({
    isOpen: false,
    type: 'image',
    url: '',
    name: '',
  });

  const { attachments, addFiles, removeAttachment } = useLocalAttachments();

  const stepLabels = useMemo(
    () => [
      t('createTask.stepSelectBusiness'),
      t('createTask.stepParty'),
      t('createTask.stepFillInfo'),
      t('createTask.stepConfirm'),
    ],
    [t],
  );

  useEffect(() => {
    let disposed = false;
    setWizardLoading(true);
    Promise.all([
      notaryService.getStaff({ staffRole: 'notary' }),
      notaryService.getMatters(),
    ])
      .then(([staff, matterItems]) => {
        if (!disposed) {
          setNotaryStaffMembers(staff);
          setMatters(matterItems);
        }
      })
      .catch((error) => {
        console.error('Failed to load notary create wizard data', error);
        toast(t('toast.wizardLoadFailed'), 'error');
      })
      .finally(() => {
        if (!disposed) {
          setWizardLoading(false);
        }
      });
    return () => {
      disposed = true;
    };
  }, [t]);

  const partyDriveDocuments = attachments.filter((attachment) => attachment.partyId === activeDriveParty?.id);

  const handleAddParty = () => {
    setEditingPartyId(null);
    setIsPartyReadOnly(false);
    setShowAddPartyPanel(true);
  };

  const handleEditParty = (party: Party) => {
    setEditingPartyId(party.id);
    setIsPartyReadOnly(false);
    setShowAddPartyPanel(true);
  };

  const handleViewParty = (party: Party) => {
    setEditingPartyId(party.id);
    setIsPartyReadOnly(true);
    setShowAddPartyPanel(true);
  };

  const handleSaveParty = (partyData: Party) => {
    if (editingPartyId) {
      setParties((current) => current.map((party) => (party.id === editingPartyId ? partyData : party)));
    } else {
      setParties((current) => [...current, partyData]);
      setActiveSignParty(partyData);
    }
    setShowAddPartyPanel(false);
    setEditingPartyId(null);
  };

  const handleRemoveParty = (partyId: string) => {
    setParties((current) => current.filter((party) => party.id !== partyId));
  };

  const handleVideoCall = (party: Party) => {
    if (!selectedNotaryStaff?.membershipId) {
      toast(t('createTask.selectNotaryFirst'), 'error');
      return;
    }
    toast(t('createTask.videoRequiresCase'), 'info');
  };

  const handleQrCodeParty = (party: Party) => {
    toast(t('createTask.videoRequiresCase'), 'info');
    setActiveQrCodeParty(party);
  };

  const handlePreviewAttachment = (attachment: LocalAttachment) => {
    if (attachment.type === 'image' || attachment.type === 'video') {
      setPreviewMedia({
        isOpen: true,
        type: attachment.type,
        url: attachment.url,
        name: attachment.name,
      });
      return;
    }
    window.open(attachment.url, '_blank');
  };

  const validateWizardStep = (currentStep: number): boolean => {
    if (currentStep === 1 && !businessType.trim()) {
      toast(t('toast.selectBusinessTypeFirst'), 'error');
      return false;
    }
    if (currentStep === 2 && (!selectedNotaryStaff?.membershipId || parties.length === 0)) {
      toast(t('toast.bindNotaryAndPartyFirst'), 'error');
      return false;
    }
    return true;
  };

  const handleSubmit = async () => {
    if (!businessType.trim()) {
      toast(t('toast.selectBusinessTypeFirst'), 'error');
      return;
    }
    if (!selectedNotaryStaff?.membershipId || parties.length === 0) {
      toast(t('toast.bindNotaryAndPartyFirst'), 'error');
      return;
    }
    setIsSubmitting(true);
    try {
      await notaryService.createTask({
        type: businessType,
        skuId: selectedSkuId,
        notary,
        parties,
        remarks: appInfo,
        title: `${businessType}${t('createTask.businessTypeSuffix')}`,
        primaryNotaryMembershipId: selectedNotaryStaff?.membershipId,
        documents: attachments.map((attachment) => ({
          name: attachment.file.name,
          size: `${Math.max(1, Math.round(attachment.file.size / 1024))} KB`,
          status: 'pending' as const,
          category: 'evidence' as const,
          materialCode: attachment.file.name,
          partyId: attachment.partyId,
          file: attachment.file,
        })),
      });
      if (onSuccess) {
        onSuccess();
      } else {
        onBack();
      }
    } catch (error) {
      console.error('Failed to create task', error);
      toast(t('toast.createTaskFailed'), 'error');
    } finally {
      setIsSubmitting(false);
    }
  };

  if (activeSignParty) {
    return (
      <SignaturePad
        title={t('createTask.partyOnlineSignature', { name: activeSignParty.name })}
        subtitle={(
          <>
            <PenTool size={16} />
            {' '}
            {t('createTask.pleaseWriteName', { name: activeSignParty.name })}
          </>
        )}
        onCancel={() => setActiveSignParty(null)}
        onSave={(imgUrl) => {
          toast(t('signaturePad.signatureSaved'), 'success');
          setParties((current) =>
            current.map((party) => (party.id === activeSignParty.id ? { ...party, signatureUrl: imgUrl } : party)),
          );
          setActiveSignParty(null);
        }}
      />
    );
  }

  return (
    <div className="flex w-full h-full flex-col bg-[#1e1e1e] relative min-h-0 min-w-0">
      <WizardHeader onBack={onBack} title={t('createTask.newNotaryBusiness')} />

      <div className="flex-1 overflow-y-auto custom-scrollbar p-4 sm:p-6 lg:p-8 flex flex-col items-center">
        <div className="w-full h-full flex flex-col max-w-[1600px]">
          <WizardStepper currentStep={step} totalSteps={WIZARD_STEPS} stepLabels={stepLabels} />

          <div className="w-full bg-[#2b2b2d] rounded-xl border border-white/5 p-6 sm:p-8 shadow-sm flex flex-col">
            {step === 1 && (
              <BusinessTypeSelector
                value={selectedSkuId ?? businessType}
                onChange={(selection) => {
                  setBusinessType(selection.title);
                  setSelectedSkuId(selection.skuId);
                }}
                matters={matters}
                loading={wizardLoading}
              />
            )}

            {step === 2 && (
              <PartyBindingStep
                notaryLabel={notary}
                parties={parties}
                onOpenNotaryPicker={() => setShowNotaryDrawer(true)}
                onAddParty={handleAddParty}
                onViewParty={handleViewParty}
                onEditParty={handleEditParty}
                onRemoveParty={handleRemoveParty}
                onSignParty={setActiveSignParty}
                onDriveParty={setActiveDriveParty}
                onQrCodeParty={handleQrCodeParty}
                onVideoCallParty={handleVideoCall}
              />
            )}

            {step === 3 && (
              <TaskDetailsForm
                description={appInfo}
                attachments={attachments}
                isSubmitting={isSubmitting}
                onDescriptionChange={setAppInfo}
                onUpload={(files) => addFiles(files)}
                onRemove={removeAttachment}
                onPreview={handlePreviewAttachment}
              />
            )}

            {step === 4 && (
              <ConfirmationSummary
                businessType={businessType}
                notary={notary}
                parties={parties}
                description={appInfo}
                attachments={attachments}
                onViewParty={handleViewParty}
                onPreviewAttachment={handlePreviewAttachment}
              />
            )}

            <WizardNavigation
              currentStep={step}
              totalSteps={WIZARD_STEPS}
              isSubmitting={isSubmitting}
              onPrev={() => setStep((current) => Math.max(current - 1, 1))}
              onNext={() => {
                if (!validateWizardStep(step)) {
                  return;
                }
                setStep((current) => Math.min(current + 1, WIZARD_STEPS));
              }}
              onSubmit={() => void handleSubmit()}
            />
          </div>
        </div>
      </div>

      <NotaryPickerDrawer
        isOpen={showNotaryDrawer}
        staff={notaryStaffMembers}
        selected={selectedNotaryStaff}
        onClose={() => setShowNotaryDrawer(false)}
        onSelect={(staff) => {
          setSelectedNotaryStaff(staff);
          setNotary(staff.displayName);
          setShowNotaryDrawer(false);
        }}
      />

      <PartyDriveModal
        isOpen={Boolean(activeDriveParty)}
        party={activeDriveParty}
        documents={partyDriveDocuments}
        onClose={() => setActiveDriveParty(null)}
        onUpload={(files) => {
          if (activeDriveParty?.id) {
            addFiles(files, activeDriveParty.id);
          }
        }}
        onPreview={handlePreviewAttachment}
      />

      <VideoCallQROverlay
        isOpen={Boolean(activeQrCodeParty)}
        party={activeQrCodeParty}
        onClose={() => setActiveQrCodeParty(null)}
      />

      <PartyDrawer
        isOpen={showAddPartyPanel}
        onClose={() => setShowAddPartyPanel(false)}
        party={editingPartyId ? parties.find((party) => party.id === editingPartyId) || null : null}
        onSave={handleSaveParty}
        readOnly={isPartyReadOnly}
        onSign={setActiveSignParty}
      />

      <MediaViewer
        isOpen={previewMedia.isOpen}
        type={previewMedia.type}
        src={previewMedia.url}
        fileName={previewMedia.name}
        onClose={() => setPreviewMedia((prev) => ({ ...prev, isOpen: false }))}
      />
    </div>
  );
};
