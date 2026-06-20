/**
 * NotaryFilterBar - Search and filter controls for the task list
 */
import React from 'react';
import { useTranslation } from 'react-i18next';
import { Search } from 'lucide-react';
import type { NotaryMatterOption } from '../../types';

export interface NotaryFilterBarProps {
  searchTerm: string;
  typeFilter: string;
  statusFilter: string;
  matters?: NotaryMatterOption[];
  mattersLoading?: boolean;
  onSearchChange: (term: string) => void;
  onTypeFilterChange: (filter: string) => void;
  onStatusFilterChange: (filter: string) => void;
}

const LEGACY_TYPE_FILTERS = [
  { value: 'ELECTRONIC', labelKey: 'filter.electronicContract' as const },
  { value: 'IPR', labelKey: 'filter.ipr' as const },
  { value: 'EVIDENCE', labelKey: 'filter.evidence' as const },
];

export const NotaryFilterBar: React.FC<NotaryFilterBarProps> = ({
  searchTerm,
  typeFilter,
  statusFilter,
  matters = [],
  mattersLoading = false,
  onSearchChange,
  onTypeFilterChange,
  onStatusFilterChange,
}) => {
  const { t } = useTranslation('notary');

  return (
    <div className="bg-[#2b2b2d] rounded-xl p-4 border border-white/5 flex flex-wrap items-center gap-4 shrink-0">
      <div className="relative flex-1 min-w-[240px]">
        <Search className="absolute left-3 top-1/2 -translate-y-1/2 text-gray-500" size={16} />
        <input
          value={searchTerm}
          onChange={(event) => onSearchChange(event.target.value)}
          placeholder={t('filter.searchPlaceholder')}
          className="w-full bg-[#181818] border border-white/10 rounded-lg pl-10 pr-4 py-2 text-sm text-gray-200 outline-none focus:border-indigo-500 placeholder:text-gray-600"
        />
      </div>

      <select
        value={typeFilter}
        onChange={(event) => onTypeFilterChange(event.target.value)}
        disabled={mattersLoading}
        className="bg-[#181818] border border-white/10 rounded-lg px-4 py-2 text-sm text-gray-200 outline-none focus:border-indigo-500 hover:border-white/20 cursor-pointer min-w-[160px] disabled:opacity-60"
      >
        <option value="ALL">{t('filter.allTypes')}</option>
        {matters.length > 0
          ? matters.map((matter) => (
            <option key={matter.skuId} value={matter.skuId}>
              {matter.title}
            </option>
          ))
          : LEGACY_TYPE_FILTERS.map((item) => (
            <option key={item.value} value={item.value}>
              {t(item.labelKey)}
            </option>
          ))}
      </select>

      <select
        value={statusFilter}
        onChange={(event) => onStatusFilterChange(event.target.value)}
        className="bg-[#181818] border border-white/10 rounded-lg px-4 py-2 text-sm text-gray-200 outline-none focus:border-indigo-500 hover:border-white/20 cursor-pointer min-w-[140px]"
      >
        <option value="ALL">{t('filter.allStatuses')}</option>
        <option value="PENDING_REVIEW">{t('filter.pendingReview')}</option>
        <option value="PROCESSING">{t('filter.processing')}</option>
        <option value="COMPLETED">{t('filter.completed')}</option>
        <option value="REJECTED">{t('status.rejected')}</option>
      </select>
    </div>
  );
};
