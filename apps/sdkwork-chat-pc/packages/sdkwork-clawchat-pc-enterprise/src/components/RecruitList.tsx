import React from 'react';
import { useTranslation } from 'react-i18next';
import { Briefcase, MapPin, Building2, CircleDollarSign } from 'lucide-react';

const mockRecruits = [
  {
    id: 'r1',
    title: '高级前端开发工程师 (React/TS)',
    company: '深圳灵动科技有限公司',
    salary: '25k-40k',
    location: '深圳·南山区',
    exp: '3-5年',
    edu: '本科及以上',
    tags: ['React', 'TypeScript', '大模型应用', '前端架构'],
    updated: '刚刚活跃'
  },
  {
    id: 'r2',
    title: '嵌入式软件工程师',
    company: '杭州字节流物联技术有限公司',
    salary: '15k-25k',
    location: '杭州·余杭区',
    exp: '1-3年',
    edu: '本科及以上',
    tags: ['C/C++', 'RTOS', 'Linux', 'ARM'],
    updated: '3小时前活跃'
  }
];

export const RecruitList: React.FC = () => {
  const { t } = useTranslation('enterprise');
  return (
    <div className="max-w-[1000px] mx-auto space-y-4">
      {mockRecruits.map(job => (
        <div key={job.id} className="bg-white dark:bg-[#28282b] rounded-2xl p-6 border border-gray-200 dark:border-white/5 shadow-sm hover:shadow-lg transition-all group cursor-pointer">
          <div className="flex justify-between items-start mb-3">
            <div>
              <h3 className="text-lg font-bold text-gray-900 dark:text-gray-100 group-hover:text-indigo-600 dark:group-hover:text-indigo-400 transition-colors">{job.title}</h3>
              <div className="flex items-center gap-4 mt-2 text-sm text-gray-500">
                <span className="flex items-center gap-1.5 font-medium text-orange-500"><CircleDollarSign size={14} /> {job.salary}</span>
                <span className="flex items-center gap-1.5"><MapPin size={14} /> {job.location}</span>
                <span className="flex items-center gap-1.5"><Briefcase size={14} /> {job.exp}</span>
                <span className="bg-gray-100 dark:bg-white/5 px-2 py-0.5 rounded text-xs">{job.edu}</span>
              </div>
            </div>
            <button className="px-5 py-2 bg-indigo-600 hover:bg-indigo-700 text-white rounded-xl text-sm font-medium transition-colors shadow-sm">
              {t('recruit.apply')}
            </button>
          </div>
          <div className="flex flex-wrap gap-2 mb-4">
            {job.tags.map(tag => (
              <span key={tag} className="px-2.5 py-1 text-xs text-gray-600 dark:text-gray-400 bg-gray-50 dark:bg-white/5 border border-gray-100 dark:border-white/5 rounded-lg">
                {tag}
              </span>
            ))}
          </div>
          <div className="flex items-center justify-between pt-4 border-t border-gray-100 dark:border-white/5 text-sm">
            <div className="flex items-center gap-2 text-gray-600 dark:text-gray-400">
              <Building2 size={16} />
              <span>{job.company}</span>
            </div>
            <span className="text-gray-400 text-xs">{job.updated}</span>
          </div>
        </div>
      ))}
    </div>
  );
};
