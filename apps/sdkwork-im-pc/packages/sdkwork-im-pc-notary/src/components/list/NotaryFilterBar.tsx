/**
 * NotaryFilterBar - Search and filter controls for the task list
 */
import React from 'react';
import { useTranslation } from 'react-i18next';
import { Search, Filter } from 'lucide-react';

export interface NotaryFilterBarProps {
  /** Current search term */
  searchTerm: string;
  /** Current type filter value */
  typeFilter: string;
  /** Called when search term changes */
  onSearchChange: (term: string) => void;
  /** Called when type filter changes */
  onTypeFilterChange: (filter: string) => void;
}

export const NotaryFilterBar: React.FC<NotaryFilterBarProps> = ({
  searchTerm,
  typeFilter,
  onSearchChange,
  onTypeFilterChange,
}) => {
  const { t } = useTranslation('notary');

  return (
    <div className="bg-[#2b2b2d] rounded-xl p-4 border border-white/5 flex flex-wrap items-center gap-4 shrink-0">
      {/* Search input */}
      <div className="relative flex-1 min-w-[240px]">
        <Search className="absolute left-3 top-1/2 -translate-y-1/2 text-gray-500" size={16} />
        <input
          value={searchTerm}
          onChange={(e) => onSearchChange(e.target.value)}
          placeholder={t('filter.searchPlaceholder')}
          className="w-full bg-[#181818] border border-white/10 rounded-lg pl-10 pr-4 py-2 text-sm text-gray-200 outline-none focus:border-indigo-500 placeholder:text-gray-600"
        />
      </div>

      {/* Type filter */}
      <select
        value={typeFilter}
        onChange={(e) => onTypeFilterChange(e.target.value)}
        className="bg-[#181818] border border-white/10 rounded-lg px-4 py-2 text-sm text-gray-200 outline-none focus:border-indigo-500 hover:border-white/20 cursor-pointer min-w-[140px]"
      >
        <option value="ALL">{t('filter.allTypes')}</option>
        <option value="ELECTRONIC">{t('filter.electronicContract')}</option>
        <option value="IPR">{t('filter.ipr')}</option>
        <option value="EVIDENCE">{t('filter.evidence')}</option>
      </select>

      {/* Status filter (placeholder) */}
      <select className="bg-[#181818] border border-white/10 rounded-lg px-4 py-2 text-sm text-gray-200 outline-none focus:border-indigo-500 hover:border-white/20 cursor-pointer min-w-[140px]">
        <option>{t('filter.allStatuses')}</option>
      </select>

      {/* Advanced filter button */}
      <button className="px-4 py-2 bg-[#181818] border border-white/10 text-gray-400 hover:text-gray-200 text-sm rounded-lg flex items-center gap-2 transition-colors hover:border-white/20">
        <Filter size={14} />{t('filter.advancedFilter')}
      </button>
    </div>
  );
};