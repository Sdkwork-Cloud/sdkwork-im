/**
 * IdentityVerification - ID card upload and face capture section
 *
 * Combines IdCardUploader for front/back ID and a face capture trigger
 * with a comparison action button.
 */
import React, { useRef } from 'react';
import { useTranslation } from 'react-i18next';
import { ShieldCheck, Camera, Plus } from 'lucide-react';
import { IdCardUploader } from './IdCardUploader';

export interface IdentityVerificationProps {
  /** ID front image URL */
  idFront: string | null;
  /** ID back image URL */
  idBack: string | null;
  /** Captured face image URL */
  faceImage: string | null;
  /** Whether fields are read-only */
  readOnly: boolean;
  /** Called when ID front is uploaded */
  onIdFrontUpload: (file: File) => void;
  /** Called when ID front is removed */
  onIdFrontRemove: () => void;
  /** Called when ID back is uploaded */
  onIdBackUpload: (file: File) => void;
  /** Called when ID back is removed */
  onIdBackRemove: () => void;
  /** Called when face capture is triggered */
  onFaceCapture: () => void;
  /** Called when captured face is removed */
  onFaceRemove: () => void;
  /** Called when compare button is clicked */
  onCompare: () => void;
}

export const IdentityVerification: React.FC<IdentityVerificationProps> = ({
  idFront,
  idBack,
  faceImage,
  readOnly,
  onIdFrontUpload,
  onIdFrontRemove,
  onIdBackUpload,
  onIdBackRemove,
  onFaceCapture,
  onFaceRemove,
  onCompare,
}) => {
  const { t } = useTranslation('notary');

  return (
    <div className="flex flex-col gap-4">
      {/* Section header */}
      <h4 className="text-sm font-semibold text-gray-300 flex items-center gap-2">
        <ShieldCheck size={16} className="text-indigo-400" /> {t('party.identityVerification')}
      </h4>

      {/* 3-column grid: ID front, ID back, Face capture */}
      <div className="grid grid-cols-3 gap-4">
        {/* ID Front */}
        <IdCardUploader
          label={t('party.portraitSide')}
          image={idFront}
          readOnly={readOnly}
          onUpload={onIdFrontUpload}
          onRemove={onIdFrontRemove}
        />

        {/* ID Back */}
        <IdCardUploader
          label={t('party.emblemSide')}
          image={idBack}
          readOnly={readOnly}
          onUpload={onIdBackUpload}
          onRemove={onIdBackRemove}
        />

        {/* Face Capture */}
        <div className="flex flex-col gap-2 relative">
          {faceImage ? (
            <div className="border border-indigo-500/30 bg-black/50 rounded-xl relative overflow-hidden aspect-[1.58/1] group">
              <img
                src={faceImage}
                alt={t('party.liveFaceCapture')}
                className="w-full h-full object-cover"
              />
              {!readOnly && (
                <button
                  onClick={onFaceRemove}
                  className="absolute top-1 right-1 p-1 bg-red-500/80 text-white rounded-full opacity-0 group-hover:opacity-100 transition-opacity"
                >
                  <Plus size={12} className="rotate-45" />
                </button>
              )}
            </div>
          ) : (
            <div
              onClick={() => !readOnly && onFaceCapture()}
              className={`${
                readOnly
                  ? 'border-dashed border-white/10 bg-white/5 text-gray-600 cursor-not-allowed'
                  : 'border-dashed border-indigo-500/30 bg-indigo-500/5 cursor-pointer hover:border-indigo-500/50 hover:bg-indigo-500/10 text-indigo-400'
              } rounded-xl flex flex-col items-center justify-center p-4 transition-colors aspect-[1.58/1] relative`}
            >
              <Camera size={20} className="mb-1" />
              <span className="text-[11px] font-medium">{t('party.liveFaceCapture')}</span>
            </div>
          )}
        </div>
      </div>

      {/* Comparison result row */}
      <div className="bg-[#181818] border border-white/5 p-3 rounded-lg flex items-center justify-between">
        <span className="text-xs text-gray-400">{t('party.comparisonResult')}</span>
        <div className="flex items-center gap-2">
          <button
            disabled={!faceImage || !idFront || readOnly}
            onClick={onCompare}
            title={!idFront ? t('party.uploadPortraitHint') : undefined}
            className="px-3 py-1 bg-indigo-500/10 text-indigo-400 hover:bg-indigo-500/20 disabled:opacity-50 disabled:cursor-not-allowed text-xs font-medium rounded transition-colors"
          >
            {t('party.clickToCompare')}
          </button>
        </div>
      </div>
    </div>
  );
};