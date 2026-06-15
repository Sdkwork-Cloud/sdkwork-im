/**
 * DocumentItem - Single document row in the materials tab
 */
import React from 'react';
import { useTranslation } from 'react-i18next';
import { FileText, Download, CheckCircle2, Clock, AlertCircle } from 'lucide-react';
import type { NotaryDocument } from '@sdkwork/clawchat-pc-types';

export interface DocumentItemProps {
  document: NotaryDocument;
  onPreview: (doc: NotaryDocument) => void;
  onDownload: (doc: NotaryDocument) => void;
}

export const DocumentItem: React.FC<DocumentItemProps> = ({
  document,
  onPreview,
  onDownload,
}) => {
  const { t } = useTranslation('notary');

  const getStatusIcon = (status?: string) => {
    switch (status) {
      case 'verified':
        return <CheckCircle2 size={14} className="text-green-400" />;
      case 'pending':
        return <Clock size={14} className="text-yellow-400" />;
      case 'failed':
        return <AlertCircle size={14} className="text-red-400" />;
      default:
        return null;
    }
  };

  return (
    <div className="flex items-center justify-between p-3 bg-[#181818] rounded-lg border border-white/5 group hover:border-white/10 transition-colors">
      <div className="flex items-center gap-3 min-w-0 flex-1">
        <FileText size={18} className="text-gray-400 group-hover:text-cyan-400 transition-colors shrink-0" />
        <div className="min-w-0 flex-1">
          <div
            className="text-sm text-gray-300 group-hover:text-gray-100 transition-colors truncate cursor-pointer hover:underline"
            onClick={() => onPreview(document)}
          >
            {document.name}
          </div>
          <div className="text-xs text-gray-500 flex items-center gap-2">
            {document.size && <span>{document.size}</span>}
            {getStatusIcon(document.status)}
          </div>
        </div>
      </div>
      <button
        onClick={() => onDownload(document)}
        className="p-1.5 text-gray-500 hover:text-cyan-400 hover:bg-cyan-500/10 rounded transition-colors opacity-0 group-hover:opacity-100 shrink-0"
        title={t('detail.downloadAttachment')}
      >
        <Download size={16} />
      </button>
    </div>
  );
};