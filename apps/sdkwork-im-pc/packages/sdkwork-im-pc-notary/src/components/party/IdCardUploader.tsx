/**
 * IdCardUploader - Reusable ID card image upload component
 *
 * Used for both ID front and ID back uploads in IdentityVerification.
 */
import React, { useRef } from 'react';
import { useTranslation } from 'react-i18next';
import { Plus, Trash2 } from 'lucide-react';

export interface IdCardUploaderProps {
  /** Label displayed on the placeholder */
  label: string;
  /** Image preview URL (object URL or data URL) */
  image: string | null;
  /** Whether the field is read-only */
  readOnly: boolean;
  /** Called when a file is selected */
  onUpload: (file: File) => void;
  /** Called when the image is removed */
  onRemove: () => void;
  /** Accept attribute for file input */
  accept?: string;
}

export const IdCardUploader: React.FC<IdCardUploaderProps> = ({
  label,
  image,
  readOnly,
  onUpload,
  onRemove,
  accept = 'image/*',
}) => {
  const { t } = useTranslation('notary');
  const inputRef = useRef<HTMLInputElement>(null);

  const handleChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    const file = event.target.files?.[0];
    if (file) {
      onUpload(file);
    }
    event.target.value = '';
  };

  const triggerUpload = () => {
    if (!readOnly) {
      inputRef.current?.click();
    }
  };

  return (
    <div className="flex flex-col gap-2 relative">
      <input
        ref={inputRef}
        type="file"
        accept={accept}
        className="hidden"
        disabled={readOnly}
        onChange={handleChange}
      />

      {image ? (
        <div className="border border-white/10 bg-black/50 rounded-xl relative overflow-hidden aspect-[1.58/1] group">
          <img src={image} alt={label} className="w-full h-full object-contain" />
          {!readOnly && (
            <button
              onClick={onRemove}
              className="absolute top-1 right-1 p-1 bg-red-500/80 text-white rounded-full opacity-0 group-hover:opacity-100 transition-opacity"
            >
              <Trash2 size={12} />
            </button>
          )}
        </div>
      ) : (
        <div
          onClick={triggerUpload}
          className={`border border-dashed border-white/10 bg-[#181818]/50 rounded-xl flex flex-col items-center justify-center p-4 text-gray-500 ${
            readOnly
              ? 'cursor-not-allowed opacity-50'
              : 'cursor-pointer hover:border-white/30 hover:bg-white/5'
          } transition-colors aspect-[1.58/1]`}
        >
          <Plus size={20} className="mb-1" />
          <span className="text-[11px] font-medium">{label}</span>
        </div>
      )}
    </div>
  );
};