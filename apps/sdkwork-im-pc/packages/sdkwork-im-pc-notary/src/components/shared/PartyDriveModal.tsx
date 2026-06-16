/**
 * PartyDriveModal - Shared party drive directory modal
 *
 * Renders a full-screen modal for managing party documents.
 * Works with either API documents (NotaryDocument[]) or local files (LocalAttachment[]).
 * Used in both index.tsx (detail pane) and CreateNotaryTaskView.tsx (wizard step 2).
 */
import React, { useRef } from 'react';
import { useTranslation } from 'react-i18next';
import { motion, AnimatePresence } from 'motion/react';
import { Cloud, Folder, Plus, Upload, X, FileText, Download } from 'lucide-react';
import type { NotaryDocument, Party } from '@sdkwork/im-pc-types';
import type { LocalAttachment } from '../../types';

export interface PartyDriveModalProps {
  /** Whether the modal is open */
  isOpen: boolean;
  /** The party whose documents are being managed */
  party: Party | null;
  /** Documents to display (either API docs or local files) */
  documents: NotaryDocument[] | LocalAttachment[];
  /** Loading state */
  loading?: boolean;
  /** Called when modal is closed */
  onClose: () => void;
  /** Called when files are uploaded */
  onUpload: (files: File[]) => void;
  /** Called when a document is clicked for preview (optional) */
  onPreview?: (doc: NotaryDocument | LocalAttachment) => void;
  /** Called when a document download is requested (optional, for API docs) */
  onDownload?: (doc: NotaryDocument) => void;
}

export const PartyDriveModal: React.FC<PartyDriveModalProps> = ({
  isOpen,
  party,
  documents,
  loading = false,
  onClose,
  onUpload,
  onPreview,
  onDownload,
}) => {
  const { t } = useTranslation('notary');
  const fileInputRef = useRef<HTMLInputElement>(null);

  const handleFileChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    const files = Array.from<File>(event.target.files || []);
    if (files.length > 0) {
      onUpload(files);
    }
    event.target.value = '';
  };

  const triggerUpload = () => {
    fileInputRef.current?.click();
  };

  // Helper to check if document is API type
  const isApiDocument = (doc: NotaryDocument | LocalAttachment): doc is NotaryDocument => {
    return 'size' in doc && typeof doc.size === 'string';
  };

  // Helper to get display size
  const getDisplaySize = (doc: NotaryDocument | LocalAttachment): string => {
    if (isApiDocument(doc)) {
      return doc.size || '—';
    }
    return `${Math.max(1, Math.round(doc.file.size / 1024))} KB`;
  };

  return (
    <AnimatePresence>
      {isOpen && party && (
        <div className="fixed inset-0 z-50 flex items-center justify-center p-8">
          {/* Backdrop */}
          <motion.div
            initial={{ opacity: 0 }}
            animate={{ opacity: 1 }}
            exit={{ opacity: 0 }}
            onClick={onClose}
            className="absolute inset-0 bg-black/60 backdrop-blur-sm"
          />

          {/* Modal content */}
          <motion.div
            initial={{ opacity: 0, scale: 0.95 }}
            animate={{ opacity: 1, scale: 1 }}
            exit={{ opacity: 0, scale: 0.95 }}
            className="relative w-full max-w-5xl h-full max-h-[800px] bg-[#1e1e1e] rounded-2xl shadow-2xl border border-white/10 flex flex-col overflow-hidden"
          >
            {/* Hidden file input */}
            <input
              ref={fileInputRef}
              type="file"
              multiple
              className="hidden"
              onChange={handleFileChange}
              accept="image/*,video/*,application/pdf"
            />

            {/* Header */}
            <div className="h-16 px-6 border-b border-white/5 bg-[#181818] flex items-center justify-between shrink-0">
              <div className="flex items-center gap-3 text-lg font-medium text-gray-200">
                <Cloud size={24} className="text-cyan-400" />
                {t('createTask.enterpriseDrive')}
                <span className="text-gray-500">/</span>
                <span className="text-gray-400 font-normal">{t('createTask.drivePathHint')}</span>
                <span className="text-gray-500">/</span>
                <span className="text-indigo-400">{party.name}</span>
              </div>
              <div className="flex items-center gap-4">
                <button
                  onClick={triggerUpload}
                  className="bg-cyan-600 hover:bg-cyan-700 text-white font-medium px-4 py-2 flex items-center gap-2 rounded-lg transition-colors text-sm shadow-md"
                >
                  <Upload size={16} /> {t('createTask.uploadInDir')}
                </button>
                <button
                  onClick={onClose}
                  className="text-gray-400 hover:text-white p-2 rounded-lg hover:bg-white/10"
                >
                  <X size={20} />
                </button>
              </div>
            </div>

            {/* Content */}
            <div className="flex-1 p-8 overflow-y-auto bg-[#1e1e1e] flex flex-col items-center justify-center border-t border-black/20">
              {/* Empty state icon */}
              <div className="w-24 h-24 rounded-full bg-[#2b2b2d] flex items-center justify-center mb-6 border border-white/5 shadow-inner">
                <Folder size={48} className="text-cyan-500/50" />
              </div>

              {/* Title */}
              <h3 className="text-xl font-medium text-gray-300 mb-2">
                {t('createTask.partyDriveTitle', { name: party.name })}
              </h3>

              {/* Hint */}
              <p className="text-gray-500 text-sm max-w-md text-center mb-8">
                {t('createTask.partyDriveHint')}
              </p>

              {/* Documents list or empty state */}
              {documents.length === 0 ? (
                <button
                  onClick={triggerUpload}
                  className="px-6 py-3 bg-[#2b2b2d] border border-cyan-500/30 text-cyan-400 rounded-xl hover:bg-cyan-500/10 transition-colors flex items-center gap-2"
                >
                  <Plus size={18} /> {loading ? t('createTask.directoryLoading') : t('createTask.uploadToDir')}
                </button>
              ) : (
                <div className="w-full max-w-3xl flex flex-col gap-2">
                  {documents.map((doc, index) => (
                    <div
                      key={isApiDocument(doc) ? `${doc.name}-${index}` : doc.id}
                      className="bg-[#181818] p-3 rounded-lg border border-white/5 flex items-center justify-between group w-full"
                    >
                      {/* Document info */}
                      <div className="flex items-center gap-3 min-w-0">
                        <FileText
                          size={18}
                          className="text-gray-400 group-hover:text-cyan-400 transition-colors shrink-0"
                        />
                        <div className="min-w-0">
                          <div
                            className="text-sm text-gray-300 group-hover:text-gray-100 transition-colors truncate cursor-pointer hover:underline"
                            onClick={() => onPreview?.(doc)}
                          >
                            {doc.name}
                          </div>
                          <div className="text-xs text-gray-500">{getDisplaySize(doc)}</div>
                        </div>
                      </div>

                      {/* Download button (only for API documents) */}
                      {onDownload && isApiDocument(doc) && (
                        <button
                          onClick={() => onDownload(doc)}
                          className="p-1.5 text-gray-500 hover:text-cyan-400 hover:bg-cyan-500/10 rounded transition-colors opacity-0 group-hover:opacity-100"
                          title={t('detail.downloadAttachment')}
                        >
                          <Download size={16} />
                        </button>
                      )}
                    </div>
                  ))}

                  {/* Upload more button */}
                  <button
                    onClick={triggerUpload}
                    className="mt-4 self-center px-6 py-3 bg-[#2b2b2d] border border-cyan-500/30 text-cyan-400 rounded-xl hover:bg-cyan-500/10 transition-colors flex items-center gap-2"
                  >
                    <Plus size={18} /> {loading ? t('createTask.uploadProcessing') : t('createTask.uploadMoreToDir')}
                  </button>
                </div>
              )}
            </div>
          </motion.div>
        </div>
      )}
    </AnimatePresence>
  );
};