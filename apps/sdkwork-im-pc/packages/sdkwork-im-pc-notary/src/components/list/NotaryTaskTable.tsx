/**
 * NotaryTaskTable - Main task list table with pagination
 */
import React from 'react';
import { useTranslation } from 'react-i18next';
import { motion, AnimatePresence } from 'motion/react';
import { FileCheck, MoreHorizontal, Download, User as UserIcon, X, ChevronLeft, ChevronRight } from 'lucide-react';
import type { NotaryTask } from '@sdkwork/im-pc-types';

export interface NotaryTaskTableProps {
  /** All tasks (for total count) */
  tasks: NotaryTask[];
  /** Tasks to display on current page */
  paginatedTasks: NotaryTask[];
  /** Currently selected task */
  selectedTask: NotaryTask | null;
  /** Which row's dropdown is open */
  activeDropdown: string | null;
  /** Current page size */
  pageSize: number;
  /** Current page number */
  currentPage: number;
  /** Total number of pages */
  totalPages: number;
  /** Function to render status badge */
  getStatusBadge: (status: NotaryTask['status']) => React.ReactNode;
  /** Called when a task is selected */
  onSelectTask: (task: NotaryTask) => void;
  /** Called when dropdown toggle is clicked */
  onDropdownToggle: (taskId: string | null) => void;
  /** Called when download materials is clicked */
  onDownloadMaterials: (task: NotaryTask) => void;
  /** Called when print party info is clicked */
  onPrintPartyInfo: (task: NotaryTask) => void;
  /** Called when delete/cancel task is clicked */
  onDeleteTask: (task: NotaryTask) => void;
  /** Called when page size changes */
  onPageSizeChange: (size: number) => void;
  /** Called when page number changes */
  onPageChange: (page: number) => void;
}

export const NotaryTaskTable: React.FC<NotaryTaskTableProps> = ({
  tasks,
  paginatedTasks,
  selectedTask,
  activeDropdown,
  pageSize,
  currentPage,
  totalPages,
  getStatusBadge,
  onSelectTask,
  onDropdownToggle,
  onDownloadMaterials,
  onPrintPartyInfo,
  onDeleteTask,
  onPageSizeChange,
  onPageChange,
}) => {
  const { t } = useTranslation('notary');

  return (
    <div className="bg-[#2b2b2d] rounded-xl border border-white/5 flex-1 flex flex-col min-h-0 shadow-[0_4px_20px_rgba(0,0,0,0.2)]">
      {/* Table body */}
      <div className="flex-1 overflow-auto custom-scrollbar">
        <table className="w-full text-left border-collapse whitespace-nowrap">
          <thead>
            <tr className="bg-[#1e1e1e]/90 text-gray-400 text-[13px] border-b border-white/5 sticky top-0 z-10 backdrop-blur-md">
              <th className="px-6 py-4 font-medium">{t('table.notaryNo')}</th>
              <th className="px-6 py-4 font-medium">{t('table.title')}</th>
              <th className="px-6 py-4 font-medium">{t('table.business')}</th>
              <th className="px-6 py-4 font-medium">{t('table.party')}</th>
              <th className="px-6 py-4 font-medium">{t('table.notary')}</th>
              <th className="px-6 py-4 font-medium">{t('table.status')}</th>
              <th className="px-6 py-4 font-medium">{t('table.remarks')}</th>
              <th className="px-6 py-4 font-medium">{t('table.actions')}</th>
            </tr>
          </thead>
          <tbody>
            {paginatedTasks.map((task, idx) => (
              <tr
                key={task.id}
                onClick={() => onSelectTask(task)}
                className={`text-sm group transition-colors cursor-pointer
                  ${selectedTask?.id === task.id ? 'bg-indigo-500/10' : 'hover:bg-white/5'}
                  ${idx !== paginatedTasks.length - 1 ? 'border-b border-white/5' : ''}
                `}
              >
                <td className="px-6 py-4">
                  <div className="flex items-center gap-2 text-indigo-400 font-medium">
                    <FileCheck size={16} />
                    {(task as any).caseNo ?? task.id}
                  </div>
                </td>
                <td className="px-6 py-4 text-gray-200 font-medium max-w-[200px] truncate" title={task.title}>
                  {task.title}
                </td>
                <td className="px-6 py-4 text-gray-300">
                  <span className="bg-white/5 px-2 py-1 rounded text-xs border border-white/5">{task.type}</span>
                </td>
                <td className="px-6 py-4 text-gray-400 max-w-[150px] truncate" title={task.parties?.map(p => p.name).join(', ') || '--'}>
                  {task.parties && task.parties.length > 0 ? task.parties.map(p => p.name).join(', ') : '--'}
                </td>
                <td className="px-6 py-4 text-gray-400">{task.notary}</td>
                <td className="px-6 py-4">{getStatusBadge(task.status)}</td>
                <td className="px-6 py-4 text-gray-400 max-w-[150px] truncate" title={task.remarks}>
                  {task.remarks || '--'}
                </td>
                <td className="px-6 py-4 relative">
                  <div className="flex items-center gap-3">
                    <button
                      onClick={(e) => {
                        e.stopPropagation();
                        onSelectTask(task);
                      }}
                      className="text-indigo-400 hover:text-indigo-300 font-medium transition-colors p-1 rounded hover:bg-indigo-500/10"
                    >
                      {t('table.viewDetails')}
                    </button>
                    <div className="relative">
                      <button
                        className="text-gray-400 hover:text-gray-200 transition-colors p-1 rounded hover:bg-white/10"
                        onClick={(e) => {
                          e.stopPropagation();
                          onDropdownToggle(activeDropdown === task.id ? null : task.id);
                        }}
                      >
                        <MoreHorizontal size={16} />
                      </button>
                      <AnimatePresence>
                        {activeDropdown === task.id && (
                          <motion.div
                            initial={{ opacity: 0, y: 5 }}
                            animate={{ opacity: 1, y: 0 }}
                            exit={{ opacity: 0, y: 5 }}
                            transition={{ duration: 0.15 }}
                            className="absolute right-0 top-full mt-1 w-40 bg-[#2b2b2d] border border-white/10 shadow-xl rounded-lg py-1 z-50 overflow-hidden"
                          >
                            <div
                              onClick={(e) => {
                                e.stopPropagation();
                                onDropdownToggle(null);
                                onDownloadMaterials(task);
                              }}
                              className="px-4 py-2 hover:bg-white/10 cursor-pointer text-gray-300 hover:text-white transition-colors flex items-center gap-2"
                            >
                              <Download size={14} /> {t('detail.downloadMaterialPackage')}
                            </div>
                            <div
                              onClick={(e) => {
                                e.stopPropagation();
                                onDropdownToggle(null);
                                onPrintPartyInfo(task);
                              }}
                              className="px-4 py-2 hover:bg-white/10 cursor-pointer text-gray-300 hover:text-white transition-colors flex items-center gap-2"
                            >
                              <UserIcon size={14} /> {t('actions.printPartyInfo')}
                            </div>
                            <div className="h-px bg-white/10 my-1 mx-2" />
                            <div
                              onClick={(e) => {
                                e.stopPropagation();
                                onDropdownToggle(null);
                                onDeleteTask(task);
                              }}
                              className="px-4 py-2 hover:bg-red-500/20 text-red-400 cursor-pointer transition-colors flex items-center gap-2"
                            >
                              <X size={14} /> {t('table.delete')}
                            </div>
                          </motion.div>
                        )}
                      </AnimatePresence>
                    </div>
                  </div>
                </td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>

      {/* Pagination */}
      <div className="p-4 border-t border-white/5 flex items-center justify-between text-sm text-gray-400 bg-[#181818]/60 shrink-0">
        <div className="flex items-center gap-4">
          <span>{t('table.totalRecords', { count: tasks.length })}</span>
          <div className="flex items-center gap-2">
            <span className="text-gray-500">{t('table.perPage')}</span>
            <select
              value={pageSize}
              onChange={(e) => {
                onPageSizeChange(Number(e.target.value));
                onPageChange(1);
              }}
              className="bg-[#181818] border border-white/10 rounded px-2 py-1 text-sm text-gray-200 outline-none focus:border-indigo-500 hover:border-white/20 cursor-pointer"
            >
              <option value={10}>10</option>
              <option value={20}>20</option>
              <option value={50}>50</option>
              <option value={100}>100</option>
            </select>
            <span className="text-gray-500">{t('table.items')}</span>
          </div>
        </div>
        <div className="flex items-center gap-2">
          <button
            onClick={() => onPageChange(Math.max(1, currentPage - 1))}
            disabled={currentPage === 1}
            className="p-1.5 rounded hover:bg-white/10 disabled:opacity-50 transition-colors border border-transparent hover:border-white/10"
          >
            <ChevronLeft size={16} />
          </button>
          <div className="flex items-center gap-1">
            {Array.from({ length: totalPages }).map((_, i) => {
              const page = i + 1;
              if (page === 1 || page === totalPages || (page >= currentPage - 1 && page <= currentPage + 1)) {
                return (
                  <button
                    key={page}
                    onClick={() => onPageChange(page)}
                    className={`w-7 h-7 rounded flex items-center justify-center transition-colors ${
                      currentPage === page
                        ? 'bg-indigo-500 text-white font-medium shadow-sm'
                        : 'hover:bg-white/10 text-gray-300'
                    }`}
                  >
                    {page}
                  </button>
                );
              } else if (page === currentPage - 2 || page === currentPage + 2) {
                return <span key={`ellipsis-${page}`} className="px-1 text-gray-500">...</span>;
              }
              return null;
            })}
          </div>
          <button
            onClick={() => onPageChange(Math.min(totalPages, currentPage + 1))}
            disabled={currentPage === totalPages}
            className="p-1.5 rounded hover:bg-white/10 disabled:opacity-50 transition-colors border border-transparent hover:border-white/10"
          >
            <ChevronRight size={16} />
          </button>
        </div>
      </div>
    </div>
  );
};