/**
 * TaskDetailsForm - Business description textarea and attachments upload (Step 3)
 */
import React, { useRef } from 'react';
import { useTranslation } from 'react-i18next';
import { motion } from 'motion/react';
import { Plus, Upload, Trash2, Camera, Video, FileText } from 'lucide-react';
import type { LocalAttachment } from '../../types';

export interface TaskDetailsFormProps {
  /** Business description text */
  description: string;
  /** List of uploaded attachments */
  attachments: LocalAttachment[];
  /** Whether submitting is in progress */
  isSubmitting: boolean;
  /** Called when description changes */
  onDescriptionChange: (text: string) => void;
  /** Called when files are uploaded */
  onUpload: (files: File[]) => void;
  /** Called when an attachment is removed */
  onRemove: (id: string) => void;
  /** Called when an attachment is previewed */
  onPreview: (attachment: LocalAttachment) => void;
}

export const TaskDetailsForm: React.FC<TaskDetailsFormProps> = ({
  description,
  attachments,
  isSubmitting,
  onDescriptionChange,
  onUpload,
  onRemove,
  onPreview,
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

  return (
    <motion.div initial={{ opacity: 0, y: 10 }} animate={{ opacity: 1, y: 0 }} className="flex flex-col gap-6">
      <h3 className="text-xl font-medium text-gray-200 mb-2">{t('createTask.fillDetails')}</h3>

      {/* Description textarea */}
      <div className="flex flex-col gap-2">
        <label className="text-sm font-medium text-gray-400">{t('createTask.businessDesc')}</label>
        <textarea
          value={description}
          onChange={(e) => onDescriptionChange(e.target.value)}
          placeholder={t('createTask.businessDescPlaceholder')}
          className="w-full h-32 bg-[#181818] border border-white/10 rounded-lg p-4 text-sm text-gray-200 outline-none focus:border-indigo-500 focus:ring-1 focus:ring-indigo-500 resize-none custom-scrollbar"
        />
      </div>

      {/* Attachments */}
      <div className="flex flex-col gap-2 mt-4">
        <div className="flex justify-between items-center">
          <label className="text-sm font-medium text-gray-400">{t('createTask.uploadMaterials')}</label>
          {attachments.length > 0 && (
            <button onClick={() => fileInputRef.current?.click()} className="flex items-center gap-1.5 text-xs text-indigo-400 bg-indigo-500/10 hover:bg-indigo-500/20 px-3 py-1.5 rounded-lg transition-colors border border-indigo-500/20">
              <Plus size={14} /> {t('createTask.addMoreFiles')}
            </button>
          )}
        </div>
        <input type="file" multiple className="hidden" ref={fileInputRef} onChange={handleFileChange} accept="image/*,video/*,application/pdf" />

        {attachments.length > 0 ? (
          <div className="flex flex-col gap-2 mt-2">
            <div className="border border-white/10 rounded-xl overflow-hidden bg-[#181818]/80">
              {attachments.map((att, idx) => (
                <div key={att.id} className={`flex items-center justify-between p-3 hover:bg-white/5 transition-colors group ${idx !== attachments.length - 1 ? 'border-b border-white/5' : ''}`}>
                  <div
                    className="flex items-center gap-4 flex-1 min-w-0 cursor-pointer"
                    onClick={() => onPreview(att)}
                  >
                    <div className="w-10 h-10 rounded-lg bg-[#2b2b2d] flex items-center justify-center shrink-0 border border-white/5">
                      {att.type === 'image' ? <Camera size={18} className="text-indigo-400" /> : att.type === 'video' ? <Video size={18} className="text-indigo-400" /> : <FileText size={18} className="text-indigo-400" />}
                    </div>
                    <div className="flex flex-col min-w-0">
                      <span className="text-sm font-medium text-gray-200 truncate group-hover:text-indigo-400 transition-colors">{att.name}</span>
                      <span className="text-xs text-gray-500 mt-0.5 uppercase">{att.type} • {t('createTask.pendingUpload')}</span>
                    </div>
                  </div>
                  <button
                    onClick={(e) => { e.stopPropagation(); onRemove(att.id); }}
                    className="p-2 text-gray-400 hover:text-red-400 hover:bg-red-500/10 rounded-lg transition-colors shrink-0"
                    title={t('createTask.deleteAttachment')}
                  >
                    <Trash2 size={16} />
                  </button>
                </div>
              ))}
            </div>
          </div>
        ) : (
          <div
            onClick={() => fileInputRef.current?.click()}
            className="border-2 border-dashed border-white/10 rounded-xl p-8 flex flex-col items-center justify-center text-gray-400 hover:border-indigo-500/50 hover:bg-indigo-500/5 hover:text-indigo-400 transition-colors cursor-pointer bg-[#181818]/50 mt-2"
          >
            <Upload size={24} className="mb-3" />
            <span className="text-sm font-medium">{t('createTask.dragFilesHint')}</span>
            <span className="text-xs mt-2 text-gray-500 text-center max-w-sm">{t('createTask.uploadHint')}</span>
          </div>
        )}
      </div>
    </motion.div>
  );
};