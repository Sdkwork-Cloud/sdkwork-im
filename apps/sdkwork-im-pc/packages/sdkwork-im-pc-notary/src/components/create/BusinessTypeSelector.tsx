/**
 * BusinessTypeSelector - Business type selection grid (Step 1)
 */
import React from 'react';
import { useTranslation } from 'react-i18next';
import { motion } from 'motion/react';
import { Check } from 'lucide-react';
import { cn } from '@sdkwork/im-pc-commons';

export interface BusinessTypeSelectorProps {
  value: string;
  onChange: (selection: { title: string; skuId?: string }) => void;
  matters?: Array<{ skuId?: string; title: string; description?: string }>;
  loading?: boolean;
}

export const BusinessTypeSelector: React.FC<BusinessTypeSelectorProps> = ({
  value,
  onChange,
  matters,
  loading = false,
}) => {
  const { t } = useTranslation('notary');

  const businessTypes = matters?.length
    ? matters.map((matter) => ({
      key: matter.skuId ?? matter.title,
      title: matter.title,
      skuId: matter.skuId,
    }))
    : [
      { key: t('filter.electronicContract'), title: t('filter.electronicContract') },
      { key: t('filter.iprConfirmation'), title: t('filter.iprConfirmation') },
      { key: t('filter.evidencePreservation'), title: t('filter.evidencePreservation') },
      { key: t('createTask.tradeSecret'), title: t('createTask.tradeSecret') },
      { key: t('createTask.lottery'), title: t('createTask.lottery') },
      { key: t('createTask.will'), title: t('createTask.will') },
    ];

  const matterDescriptionByTitle = new Map(
    (matters ?? []).map((matter) => [matter.title, matter.description]),
  );

  if (loading) {
    return (
      <div className="flex flex-col gap-6">
        <h3 className="text-xl font-medium text-gray-200 mb-2">{t('createTask.selectBusinessType')}</h3>
        <div className="text-sm text-gray-500">{t('stats.loading')}</div>
      </div>
    );
  }

  return (
    <motion.div initial={{ opacity: 0, y: 10 }} animate={{ opacity: 1, y: 0 }} className="flex flex-col gap-6">
      <h3 className="text-xl font-medium text-gray-200 mb-2">{t('createTask.selectBusinessType')}</h3>
      <div className="grid grid-cols-2 gap-4">
        {businessTypes.map((type) => (
          <div
            key={type.key}
            onClick={() => onChange({ title: type.title, skuId: type.skuId })}
            className={cn(
              'p-5 rounded-xl border cursor-pointer transition-all flex flex-col gap-2 relative overflow-hidden',
              value === type.key
                ? 'bg-indigo-500/10 border-indigo-500 text-indigo-400'
                : 'bg-[#181818] border-white/5 text-gray-300 hover:border-white/20 hover:bg-[#202020]',
            )}
          >
            <div className="font-medium text-[16px]">{type.title}</div>
            <div className="text-xs text-gray-500">
              {matterDescriptionByTitle.get(type.title) ?? t('createTask.businessTypeHint')}
            </div>
            {value === type.key && (
              <div className="absolute top-0 right-0 w-0 h-0 border-t-[24px] border-r-[24px] border-t-indigo-500 border-r-transparent">
                <Check size={12} className="absolute -top-[20px] -right-[5px] text-white" />
              </div>
            )}
          </div>
        ))}
      </div>
    </motion.div>
  );
};