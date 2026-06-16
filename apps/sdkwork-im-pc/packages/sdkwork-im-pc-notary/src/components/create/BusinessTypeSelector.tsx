/**
 * BusinessTypeSelector - Business type selection grid (Step 1)
 */
import React from 'react';
import { useTranslation } from 'react-i18next';
import { motion } from 'motion/react';
import { Check } from 'lucide-react';
import { cn } from '@sdkwork/im-pc-commons';

export interface BusinessTypeSelectorProps {
  /** Currently selected business type */
  value: string;
  /** Called when a type is selected */
  onChange: (type: string) => void;
}

export const BusinessTypeSelector: React.FC<BusinessTypeSelectorProps> = ({
  value,
  onChange,
}) => {
  const { t } = useTranslation('notary');

  const businessTypes = [
    t('filter.electronicContract'),
    t('filter.iprConfirmation'),
    t('filter.evidencePreservation'),
    t('createTask.tradeSecret'),
    t('createTask.lottery'),
    t('createTask.will'),
  ];

  return (
    <motion.div initial={{ opacity: 0, y: 10 }} animate={{ opacity: 1, y: 0 }} className="flex flex-col gap-6">
      <h3 className="text-xl font-medium text-gray-200 mb-2">{t('createTask.selectBusinessType')}</h3>
      <div className="grid grid-cols-2 gap-4">
        {businessTypes.map((type) => (
          <div
            key={type}
            onClick={() => onChange(type)}
            className={cn(
              'p-5 rounded-xl border cursor-pointer transition-all flex flex-col gap-2 relative overflow-hidden',
              value === type
                ? 'bg-indigo-500/10 border-indigo-500 text-indigo-400'
                : 'bg-[#181818] border-white/5 text-gray-300 hover:border-white/20 hover:bg-[#202020]',
            )}
          >
            <div className="font-medium text-[16px]">{type}</div>
            <div className="text-xs text-gray-500">{t('createTask.businessTypeHint')}</div>
            {value === type && (
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