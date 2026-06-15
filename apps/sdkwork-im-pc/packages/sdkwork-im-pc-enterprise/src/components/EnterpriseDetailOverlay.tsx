import React from 'react';
import { useTranslation } from 'react-i18next';
import { Package, Briefcase, MessageSquare } from 'lucide-react';
import { EnterpriseData } from './EnterpriseDetail';

interface EnterpriseDetailOverlayProps {
  enterprise: EnterpriseData;
  selectedItem: { type: 'product' | 'job', id: string, title?: string };
  onClose: () => void;
  onStartChat: (enterpriseId: string) => void;
}

export const EnterpriseDetailOverlay: React.FC<EnterpriseDetailOverlayProps> = ({ enterprise, selectedItem, onClose, onStartChat }) => {
  const { t } = useTranslation('enterprise');

  return (
    <div className="absolute inset-0 z-50 bg-[#f8fafc] dark:bg-[#121214] flex flex-col animate-in slide-in-from-right-8 duration-300">
      <div className="h-16 flex items-center px-6 border-b border-gray-200 dark:border-white/5 bg-white dark:bg-[#18181b] shrink-0">
        <button 
          onClick={onClose}
          className="p-2 hover:bg-gray-100 dark:hover:bg-white/5 rounded-xl transition-colors text-gray-500 hover:text-gray-900 dark:text-gray-400 dark:hover:text-gray-200 mr-4"
        >
          <span className="font-bold">{t('detail.back')}</span>
        </button>
        <h2 className="text-lg font-bold text-gray-900 dark:text-white line-clamp-1">{selectedItem.title}</h2>
        <div className="ml-auto flex gap-3">
          <button 
            onClick={() => onStartChat(enterprise.id)}
            className="px-4 py-2 bg-indigo-600 hover:bg-indigo-700 text-white text-sm font-bold rounded-lg shadow-sm transition-colors flex items-center gap-2"
          >
            <MessageSquare size={16} /> {t('detail.exclusiveConsult')}
          </button>
        </div>
      </div>
      <div className="flex-1 overflow-y-auto custom-scrollbar p-8">
        <div className="max-w-4xl mx-auto bg-white dark:bg-[#18181b] rounded-2xl p-8 border border-gray-200 dark:border-white/5 shadow-sm">
          {selectedItem.type === 'product' ? (
            <div className="flex flex-col gap-8">
              <div className="aspect-video bg-gradient-to-br from-indigo-50 dark:from-indigo-900/20 to-blue-50 dark:to-blue-900/20 rounded-xl flex items-center justify-center border border-indigo-100/50 dark:border-indigo-500/10">
                <Package className="text-indigo-400 dark:text-indigo-500" size={80} />
              </div>
              <div>
                <h3 className="text-2xl font-bold text-gray-900 dark:text-white mb-4">{selectedItem.title}</h3>
                <p className="text-gray-600 dark:text-gray-400 leading-relaxed text-[15px]">
                  为中大型企业提供安全、极速、高可用的私有云存储计算解决方案，支持多种混合云架构部署，满足严苛的数据合规要求。提供 99.99% 的SLA保障。
                  全面实现采购、库存、物流全链路的智能监控与调配。支持千万级数据并发，沉淀行业最佳实践，平均为企业降低30%的供应链管理成本。
                </p>
                <div className="mt-8 pt-6 border-t border-gray-100 dark:border-white/5">
                  <h4 className="text-lg font-bold text-gray-900 dark:text-white mb-4">产品优势</h4>
                  <ul className="space-y-3 text-sm text-gray-600 dark:text-gray-400 font-medium">
                    <li className="flex items-center gap-2"><span className="w-1.5 h-1.5 rounded-full bg-indigo-500"></span> 安全可靠：金融级数据加密，多重灾备架构</li>
                    <li className="flex items-center gap-2"><span className="w-1.5 h-1.5 rounded-full bg-indigo-500"></span> 弹性扩展：支持PB级数据平滑扩容，无需停机</li>
                    <li className="flex items-center gap-2"><span className="w-1.5 h-1.5 rounded-full bg-indigo-500"></span> 智能运维：AI赋能自动化监控，故障秒级感知</li>
                  </ul>
                </div>
              </div>
            </div>
          ) : (
            <div className="flex flex-col gap-8">
              <div className="pb-6 border-b border-gray-100 dark:border-white/5">
                <div className="flex items-center gap-3 mb-4">
                  <div className="p-2.5 bg-indigo-50 dark:bg-indigo-500/10 rounded-xl text-indigo-600 dark:text-indigo-400">
                    <Briefcase size={24} />
                  </div>
                  <h3 className="text-2xl font-bold text-gray-900 dark:text-white">{selectedItem.title}</h3>
                </div>
                <div className="text-3xl font-extrabold text-indigo-600 dark:text-indigo-400 tracking-tight mt-2">
                  25k-40k
                </div>
              </div>
              
              <div>
                <h4 className="text-lg font-bold text-gray-900 dark:text-white mb-4">{t('detail.jobDesc')}</h4>
                <div className="space-y-3 text-sm text-gray-600 dark:text-gray-400 leading-relaxed font-medium">
                  <p>1. 负责核心业务模块的前端架构设计与开发，保证高性能和高可用性；</p>
                  <p>2. 参与技术选型，推动前端工程化、自动化构建及性能优化；</p>
                  <p>3. 与产品经理、设计师和后端工程师紧密协作，产出高质量的代码；</p>
                  <p>4. 指导初中级开发工程师，提升团队整体技术水平和代码质量。</p>
                </div>
              </div>
              
              <div className="pt-4">
                <h4 className="text-lg font-bold text-gray-900 dark:text-white mb-4">{t('detail.jobReq')}</h4>
                <div className="space-y-3 text-sm text-gray-600 dark:text-gray-400 leading-relaxed font-medium">
                  <p>1. 计算机相关专业本科及以上学历，3年以上前端开发经验；</p>
                  <p>2. 熟练掌握 JavaScript/TypeScript，深入理解 React 核心原理；</p>
                  <p>3. 熟练使用 Tailwind CSS 等现代样式方案；</p>
                  <p>4. 具备良好的编码习惯、架构设计能力和团队协作精神。</p>
                </div>
              </div>
              <div className="mt-8 pt-6 border-t border-gray-100 dark:border-white/5 flex gap-4">
                <button className="flex-1 py-3 bg-indigo-600 hover:bg-indigo-700 text-white font-bold rounded-xl transition-colors active:scale-95 shadow-sm text-center">
                  {t('detail.applyJob')}
                </button>
              </div>
            </div>
          )}
        </div>
      </div>
    </div>
  );
};
