import React from 'react';
import { useTranslation } from 'react-i18next';
import { Briefcase, MapPin, Send } from 'lucide-react';
import { EnterpriseData } from './EnterpriseDetail';

interface EnterpriseDetailRecruitsProps {
  enterprise: EnterpriseData;
  onSelectJob: (job: { type: 'job', id: string, title?: string }) => void;
  onStartChat: (enterpriseId: string) => void;
}

export const EnterpriseDetailRecruits: React.FC<EnterpriseDetailRecruitsProps> = ({ enterprise, onSelectJob, onStartChat }) => {
  const { t } = useTranslation('enterprise');

  return (
    <div className="flex flex-col gap-8">
      <section className="bg-white dark:bg-[#18181b] rounded-2xl p-8 border border-gray-200 dark:border-white/5 shadow-sm animate-in fade-in slide-in-from-bottom-4 duration-300">
        <div className="flex items-center justify-between mb-6">
          <h3 className="text-xl font-bold text-gray-900 dark:text-white flex items-center gap-2">
            <Briefcase className="text-indigo-500" size={22} />
            {t('detail.allJobs')}
            <span className="ml-2 text-xs font-bold text-indigo-600 dark:text-indigo-400 bg-indigo-50 dark:bg-indigo-500/10 px-2.5 py-1 rounded-full">{t('detail.hotJobs', { count: 3 })}</span>
          </h3>
        </div>
        <div className="flex flex-col gap-6 mt-6">
          <div 
            onClick={() => onSelectJob({ type: 'job', id: 'j1', title: '高级前端开发工程师' })}
            className="flex flex-col sm:flex-row items-start sm:items-center justify-between p-6 rounded-2xl bg-white dark:bg-[#1c1c1e] border border-gray-200/60 dark:border-white/5 hover:border-indigo-300 dark:hover:border-indigo-500/40 transition-all cursor-pointer group hover:shadow-lg hover:shadow-indigo-500/5"
          >
            <div className="flex flex-col gap-3">
              <h4 className="text-xl font-bold text-gray-900 dark:text-gray-100 group-hover:text-indigo-600 dark:group-hover:text-indigo-400 transition-colors">高级前端开发工程师</h4>
              <div className="flex flex-wrap items-center gap-4 text-sm font-medium text-gray-600 dark:text-gray-400">
                <span className="flex items-center gap-1 bg-gray-50 dark:bg-white/5 px-2 py-1 rounded"><MapPin size={16}/> {enterprise.location}</span>
                <span className="flex items-center gap-1 bg-gray-50 dark:bg-white/5 px-2 py-1 rounded"><Briefcase size={16} /> 经验 3-5年</span>
                <span className="flex items-center gap-1 bg-gray-50 dark:bg-white/5 px-2 py-1 rounded">统招本科</span>
              </div>
              <div className="flex flex-wrap gap-2 mt-1">
                <span className="px-2.5 py-1 text-xs font-semibold bg-blue-50 dark:bg-blue-500/10 text-blue-600 dark:text-blue-400 border border-blue-100 dark:border-blue-500/20 rounded">六险一金</span>
                <span className="px-2.5 py-1 text-xs font-semibold bg-green-50 dark:bg-green-500/10 text-green-600 dark:text-green-400 border border-green-100 dark:border-green-500/20 rounded">周末双休</span>
                <span className="px-2.5 py-1 text-xs font-semibold bg-purple-50 dark:bg-purple-500/10 text-purple-600 dark:text-purple-400 border border-purple-100 dark:border-purple-500/20 rounded">年底双薪</span>
              </div>
            </div>
            <div className="flex flex-col items-end mt-5 sm:mt-0">
              <span className="text-2xl font-extrabold text-indigo-600 dark:text-indigo-400 mb-4 tracking-tight">25k-40k</span>
              <button 
                onClick={(e) => { e.stopPropagation(); onStartChat(enterprise.id); }}
                className="flex items-center gap-1.5 px-6 py-2.5 bg-indigo-50 dark:bg-indigo-500/10 hover:bg-indigo-600 hover:text-white text-indigo-600 dark:text-indigo-400 font-bold rounded-xl transition-colors active:scale-95 shadow-sm"
              >
                <Send size={16} />
                {t('detail.applyJob')}
              </button>
            </div>
          </div>
          
          <div 
            onClick={() => onSelectJob({ type: 'job', id: 'j2', title: '产品总监 (B端方向)' })}
            className="flex flex-col sm:flex-row items-start sm:items-center justify-between p-6 rounded-2xl bg-white dark:bg-[#1c1c1e] border border-gray-200/60 dark:border-white/5 hover:border-indigo-300 dark:hover:border-indigo-500/40 transition-all cursor-pointer group hover:shadow-lg hover:shadow-indigo-500/5"
          >
            <div className="flex flex-col gap-3">
              <h4 className="text-xl font-bold text-gray-900 dark:text-gray-100 group-hover:text-indigo-600 dark:group-hover:text-indigo-400 transition-colors">产品总监 (B端方向)</h4>
              <div className="flex flex-wrap items-center gap-4 text-sm font-medium text-gray-600 dark:text-gray-400">
                <span className="flex items-center gap-1 bg-gray-50 dark:bg-white/5 px-2 py-1 rounded"><MapPin size={16}/> {enterprise.location}</span>
                <span className="flex items-center gap-1 bg-gray-50 dark:bg-white/5 px-2 py-1 rounded"><Briefcase size={16} /> 经验 5-10年</span>
                <span className="flex items-center gap-1 bg-gray-50 dark:bg-white/5 px-2 py-1 rounded">统招本科</span>
              </div>
              <div className="flex flex-wrap gap-2 mt-1">
                <span className="px-2.5 py-1 text-xs font-semibold bg-gray-100 dark:bg-white/10 text-gray-600 dark:text-gray-300 rounded border border-gray-200 dark:border-white/10">SaaS产品</span>
                <span className="px-2.5 py-1 text-xs font-semibold bg-gray-100 dark:bg-white/10 text-gray-600 dark:text-gray-300 rounded border border-gray-200 dark:border-white/10">团队管理</span>
              </div>
            </div>
            <div className="flex flex-col items-end mt-5 sm:mt-0">
              <span className="text-2xl font-extrabold text-indigo-600 dark:text-indigo-400 mb-4 tracking-tight">40k-60k</span>
              <button 
                onClick={(e) => { e.stopPropagation(); onStartChat(enterprise.id); }}
                className="flex items-center gap-1.5 px-6 py-2.5 bg-indigo-50 dark:bg-indigo-500/10 hover:bg-indigo-600 hover:text-white text-indigo-600 dark:text-indigo-400 font-bold rounded-xl transition-colors active:scale-95 shadow-sm"
              >
                <Send size={16} />
                {t('detail.applyJob')}
              </button>
            </div>
          </div>

          <div 
            onClick={() => onSelectJob({ type: 'job', id: 'j3', title: '资深UI设计师' })}
            className="flex flex-col sm:flex-row items-start sm:items-center justify-between p-6 rounded-2xl bg-white dark:bg-[#1c1c1e] border border-gray-200/60 dark:border-white/5 hover:border-indigo-300 dark:hover:border-indigo-500/40 transition-all cursor-pointer group hover:shadow-lg hover:shadow-indigo-500/5"
          >
            <div className="flex flex-col gap-3">
              <h4 className="text-xl font-bold text-gray-900 dark:text-gray-100 group-hover:text-indigo-600 dark:group-hover:text-indigo-400 transition-colors">资深UI设计师</h4>
              <div className="flex flex-wrap items-center gap-4 text-sm font-medium text-gray-600 dark:text-gray-400">
                <span className="flex items-center gap-1 bg-gray-50 dark:bg-white/5 px-2 py-1 rounded"><MapPin size={16}/> {enterprise.location}</span>
                <span className="flex items-center gap-1 bg-gray-50 dark:bg-white/5 px-2 py-1 rounded"><Briefcase size={16} /> 经验 3-5年</span>
                <span className="flex items-center gap-1 bg-gray-50 dark:bg-white/5 px-2 py-1 rounded">统招本科</span>
              </div>
              <div className="flex flex-wrap gap-2 mt-1">
                <span className="px-2.5 py-1 text-xs font-semibold bg-gray-100 dark:bg-white/10 text-gray-600 dark:text-gray-300 rounded border border-gray-200 dark:border-white/10">Figma</span>
                <span className="px-2.5 py-1 text-xs font-semibold bg-gray-100 dark:bg-white/10 text-gray-600 dark:text-gray-300 rounded border border-gray-200 dark:border-white/10">交互设计</span>
                <span className="px-2.5 py-1 text-xs font-semibold bg-gray-100 dark:bg-white/10 text-gray-600 dark:text-gray-300 rounded border border-gray-200 dark:border-white/10">组件库</span>
              </div>
            </div>
            <div className="flex flex-col items-end mt-5 sm:mt-0">
              <span className="text-2xl font-extrabold text-indigo-600 dark:text-indigo-400 mb-4 tracking-tight">20k-35k</span>
              <button 
                onClick={(e) => { e.stopPropagation(); onStartChat(enterprise.id); }}
                className="flex items-center gap-1.5 px-6 py-2.5 bg-indigo-50 dark:bg-indigo-500/10 hover:bg-indigo-600 hover:text-white text-indigo-600 dark:text-indigo-400 font-bold rounded-xl transition-colors active:scale-95 shadow-sm"
              >
                <Send size={16} />
                {t('detail.applyJob')}
              </button>
            </div>
          </div>
        </div>
      </section>
    </div>
  );
};
