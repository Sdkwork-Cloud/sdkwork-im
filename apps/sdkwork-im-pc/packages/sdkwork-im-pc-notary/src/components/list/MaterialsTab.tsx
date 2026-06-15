/**
 * MaterialsTab - Documents list grouped by category in the detail pane
 */
import React from 'react';
import { useTranslation } from 'react-i18next';
import { Download, User as UserIcon, Layers, FileSignature, FileText, CheckCircle2, Clock, AlertCircle } from 'lucide-react';
import type { NotaryDocument, NotaryTask } from '@sdkwork/clawchat-pc-types';

export interface MaterialsTabProps {
  /** The current task */
  task: NotaryTask;
  /** Called when a document is clicked for preview */
  onPreview: (doc: NotaryDocument) => void;
  /** Called when a document download is requested */
  onDownload: (doc: NotaryDocument) => void;
  /** Called when "Download All" is clicked */
  onDownloadAll: () => void;
}

export const MaterialsTab: React.FC<MaterialsTabProps> = ({
  task,
  onPreview,
  onDownload,
  onDownloadAll,
}) => {
  const { t } = useTranslation('notary');

  const categories = [
    { key: 'identity', title: t('detail.identityProofMaterials'), icon: <UserIcon size={16} className="text-indigo-400" /> },
    { key: 'evidence', title: t('detail.businessEvidenceMaterials'), icon: <Layers size={16} className="text-orange-400" /> },
    { key: 'notary', title: t('detail.notaryDeliveryDocuments'), icon: <FileSignature size={16} className="text-indigo-400" /> },
  ];

  const getStatusLabel = (status?: string) => {
    switch (status) {
      case 'verified':
        return <span className="text-green-500 flex items-center gap-1"><CheckCircle2 size={12} /> {t('detail.verified')}</span>;
      case 'pending':
        return <span className="text-orange-500 flex items-center gap-1"><Clock size={12} /> {t('detail.verifying')}</span>;
      case 'error':
        return <span className="text-red-500 flex items-center gap-1"><AlertCircle size={12} /> {t('detail.anomaly')}</span>;
      default:
        return null;
    }
  };

  return (
    <div className="flex flex-col gap-6">
      {/* Download all button */}
      <div className="flex justify-end">
        <button
          onClick={onDownloadAll}
          className="px-3 py-1.5 bg-indigo-500/10 hover:bg-indigo-500/20 text-indigo-400 rounded flex items-center gap-1.5 text-xs font-medium cursor-pointer transition-colors border border-indigo-500/20"
        >
          <Download size={14} /> {t('detail.downloadAllAttachments')}
        </button>
      </div>

      {/* Category groups */}
      {categories.map(({ key, title, icon }) => {
        const docs = (task.documents || []).filter((d) => d.category === key);
        if (docs.length === 0) return null;

        return (
          <div key={key}>
            <h4 className="text-sm font-medium text-gray-300 mb-3 flex items-center gap-2">
              {icon} {title}
            </h4>
            <div className="flex flex-col gap-2">
              {docs.map((doc, i) => (
                <div
                  key={`${doc.name}-${i}`}
                  className="bg-[#181818] p-3 rounded-lg border border-white/5 flex items-center justify-between group"
                >
                  <div className="flex items-center gap-3 min-w-0">
                    <FileText size={18} className="text-gray-400 group-hover:text-indigo-400 transition-colors shrink-0" />
                    <div className="min-w-0">
                      <div
                        className="text-sm text-gray-300 group-hover:text-gray-100 transition-colors truncate cursor-pointer hover:underline"
                        onClick={() => onPreview(doc)}
                      >
                        {doc.name}
                      </div>
                      <div className="text-xs text-gray-500">{doc.size}</div>
                    </div>
                  </div>
                  <div className="flex items-center gap-4 shrink-0 pl-2">
                    <div className="text-xs">{getStatusLabel(doc.status)}</div>
                    <button
                      onClick={() => onDownload(doc)}
                      className="p-1.5 text-gray-500 hover:text-indigo-400 hover:bg-indigo-500/10 rounded transition-colors opacity-0 group-hover:opacity-100"
                      title={t('detail.downloadAttachment')}
                    >
                      <Download size={16} />
                    </button>
                  </div>
                </div>
              ))}
            </div>
          </div>
        );
      })}
    </div>
  );
};