/**
 * MaterialsTab - Documents list grouped by category in the detail pane
 */
import React, { useMemo } from 'react';
import { useTranslation } from 'react-i18next';
import { Download, User as UserIcon, Layers, FileSignature, FileText, CheckCircle2, Clock, AlertCircle } from 'lucide-react';
import type { NotaryDocument, NotaryTask } from '@sdkwork/im-pc-types';

const KNOWN_CATEGORIES = ['identity', 'evidence', 'notary'] as const;

export interface MaterialsTabProps {
  task: NotaryTask;
  onPreview: (doc: NotaryDocument) => void;
  onDownload: (doc: NotaryDocument) => void;
  onDownloadAll: () => void;
}

export const MaterialsTab: React.FC<MaterialsTabProps> = ({
  task,
  onPreview,
  onDownload,
  onDownloadAll,
}) => {
  const { t } = useTranslation('notary');

  const documents = task.documents ?? [];

  const categorizedDocuments = useMemo(() => {
    const grouped = Object.fromEntries(KNOWN_CATEGORIES.map((key) => [key, [] as NotaryDocument[]])) as Record<
      typeof KNOWN_CATEGORIES[number],
      NotaryDocument[]
    >;
    const uncategorized: NotaryDocument[] = [];

    for (const document of documents) {
      if (KNOWN_CATEGORIES.includes(document.category as typeof KNOWN_CATEGORIES[number])) {
        grouped[document.category as typeof KNOWN_CATEGORIES[number]].push(document);
      } else {
        uncategorized.push(document);
      }
    }

    return { grouped, uncategorized };
  }, [documents]);

  const categories = [
    { key: 'identity' as const, title: t('detail.identityProofMaterials'), icon: <UserIcon size={16} className="text-indigo-400" /> },
    { key: 'evidence' as const, title: t('detail.businessEvidenceMaterials'), icon: <Layers size={16} className="text-orange-400" /> },
    { key: 'notary' as const, title: t('detail.notaryDeliveryDocuments'), icon: <FileSignature size={16} className="text-indigo-400" /> },
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

  const renderDocumentList = (docs: NotaryDocument[]) => (
    <div className="flex flex-col gap-2">
      {docs.map((doc, index) => (
        <div
          key={`${doc.name}-${index}`}
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
              type="button"
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
  );

  const hasRenderableDocuments = documents.length > 0;

  return (
    <div className="flex flex-col gap-6">
      <div className="flex justify-end">
        <button
          type="button"
          onClick={onDownloadAll}
          className="px-3 py-1.5 bg-indigo-500/10 hover:bg-indigo-500/20 text-indigo-400 rounded flex items-center gap-1.5 text-xs font-medium cursor-pointer transition-colors border border-indigo-500/20"
        >
          <Download size={14} />
          {' '}
          {t('detail.downloadAllAttachments')}
        </button>
      </div>

      {!hasRenderableDocuments ? (
        <div className="text-sm text-gray-500 text-center py-8 bg-[#181818]/50 rounded-lg border border-dashed border-white/5">
          {t('detail.noMaterials')}
        </div>
      ) : (
        <>
          {categories.map(({ key, title, icon }) => {
            const docs = categorizedDocuments.grouped[key];
            if (docs.length === 0) {
              return null;
            }
            return (
              <div key={key}>
                <h4 className="text-sm font-medium text-gray-300 mb-3 flex items-center gap-2">
                  {icon}
                  {' '}
                  {title}
                </h4>
                {renderDocumentList(docs)}
              </div>
            );
          })}
          {categorizedDocuments.uncategorized.length > 0 && (
            <div>
              <h4 className="text-sm font-medium text-gray-300 mb-3 flex items-center gap-2">
                <FileText size={16} className="text-gray-400" />
                {' '}
                {t('detail.otherMaterials')}
              </h4>
              {renderDocumentList(categorizedDocuments.uncategorized)}
            </div>
          )}
        </>
      )}
    </div>
  );
};
