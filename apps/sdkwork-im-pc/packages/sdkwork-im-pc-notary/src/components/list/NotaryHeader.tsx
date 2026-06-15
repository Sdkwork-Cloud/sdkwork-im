/**
 * NotaryHeader - Page header with title, stats cards, and action buttons
 */
import React from 'react';
import { useTranslation } from 'react-i18next';
import { ShieldCheck, Download, FileSignature, Clock, AlertCircle, Layers } from 'lucide-react';
import type { NotaryStats } from '../../types';

export interface NotaryHeaderProps {
  /** Called when "Create Task" button is clicked */
  onCreateTask: () => void;
  /** Statistics to display */
  stats: NotaryStats;
}

export const NotaryHeader: React.FC<NotaryHeaderProps> = ({ onCreateTask, stats }) => {
  const { t } = useTranslation('notary');

  const statCards = [
    {
      icon: Clock,
      iconColor: 'text-orange-400',
      label: t('stats.pendingQueue'),
      value: stats.pendingCount,
      sub: t('stats.estimatedProcessingTime'),
      bgIcon: Clock,
    },
    {
      icon: ShieldCheck,
      iconColor: 'text-green-400',
      label: t('stats.todayCompleted'),
      value: stats.completedCount,
      sub: t('stats.comparedToYesterday'),
      subColor: 'text-green-500',
      bgIcon: ShieldCheck,
    },
    {
      icon: AlertCircle,
      iconColor: 'text-red-400',
      label: t('stats.anomalyIntercepted'),
      value: stats.rejectedCount,
      sub: t('stats.riskControlAutoIntercept'),
      bgIcon: AlertCircle,
    },
    {
      icon: Layers,
      iconColor: 'text-indigo-400',
      label: t('stats.monthlyVolume'),
      value: stats.totalCount.toLocaleString(),
      sub: (
        <div className="flex items-center gap-1">
          <div className="w-1.5 h-1.5 rounded-full bg-green-500"></div> Blockchain Sync OK
        </div>
      ),
      bgIcon: Layers,
    },
  ];

  return (
    <div className="flex flex-col gap-6 shrink-0">
      {/* Title row */}
      <div className="flex items-center justify-between">
        <h1 className="text-2xl font-medium text-gray-100 flex items-center gap-2">
          <ShieldCheck className="text-indigo-500" />
          {t('workspace.title')}
        </h1>
        <div className="flex items-center gap-3">
          <button className="px-4 py-2 bg-[#2b2b2d] hover:bg-white/10 text-gray-200 text-sm rounded flex items-center gap-2 transition-colors border border-white/5">
            <Download size={16} />{t('actions.monthlyReport')}
          </button>
          <button
            onClick={onCreateTask}
            className="px-4 py-2 bg-indigo-600 hover:bg-indigo-700 text-white text-sm rounded flex items-center gap-2 transition-colors shadow-[0_0_15px_rgba(79,70,229,0.3)]"
          >
            <FileSignature size={16} />{t('actions.createTask')}
          </button>
        </div>
      </div>

      {/* Stats cards */}
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
        {statCards.map((card, index) => (
          <div
            key={index}
            className="bg-gradient-to-br from-[#2b2b2d] to-[#252527] rounded-xl p-5 border border-white/5 shadow-sm relative overflow-hidden group"
          >
            <div className="absolute top-0 right-0 p-4 opacity-5 group-hover:opacity-10 transition-opacity">
              <card.bgIcon size={48} />
            </div>
            <div className="flex items-center gap-3 text-gray-400 mb-2">
              <card.icon size={16} className={card.iconColor} />
              <span className="text-sm font-medium">{card.label}</span>
            </div>
            <div className="text-3xl font-semibold text-gray-100">{card.value}</div>
            <div className={`text-xs mt-2 ${card.subColor || 'text-gray-500'}`}>{card.sub}</div>
          </div>
        ))}
      </div>
    </div>
  );
};