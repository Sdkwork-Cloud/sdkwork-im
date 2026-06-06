import React from 'react';
import { useTranslation } from 'react-i18next';
import { Package, ChevronRight, ShoppingCart } from 'lucide-react';
import { EnterpriseData } from './EnterpriseDetail';

interface EnterpriseDetailProductsProps {
  enterprise: EnterpriseData;
  onSelectProduct: (product: { type: 'product', id: string, title?: string }) => void;
  onStartChat: (enterpriseId: string) => void;
}

export const EnterpriseDetailProducts: React.FC<EnterpriseDetailProductsProps> = ({ enterprise, onSelectProduct, onStartChat }) => {
  const { t } = useTranslation('enterprise');

  return (
    <div className="flex flex-col gap-8">
      <section className="bg-white dark:bg-[#18181b] rounded-2xl p-8 border border-gray-200 dark:border-white/5 shadow-sm animate-in fade-in slide-in-from-bottom-4 duration-300">
        <h3 className="text-xl font-bold text-gray-900 dark:text-white mb-6 flex items-center gap-2">
          <Package className="text-indigo-500" size={22} />
          {t('detail.coreProducts')}
        </h3>
        <div className="grid grid-cols-1 gap-6">
          <div 
            onClick={() => onSelectProduct({ type: 'product', id: 'p1', title: '企业级私有云解决方案' })}
            className="flex flex-col md:flex-row gap-8 p-6 rounded-2xl bg-gray-50/50 dark:bg-white/[0.02] border border-gray-200/60 dark:border-white/5 hover:border-indigo-200 dark:hover:border-indigo-500/30 transition-all hover:bg-white dark:hover:bg-[#1c1c1e] hover:shadow-xl hover:shadow-indigo-500/5 group cursor-pointer"
          >
            <div className="w-full md:w-56 h-36 bg-gradient-to-br from-indigo-50 dark:from-indigo-900/20 to-blue-50 dark:to-blue-900/20 rounded-xl shrink-0 flex items-center justify-center overflow-hidden border border-indigo-100/50 dark:border-indigo-500/10">
               <Package className="text-indigo-400 dark:text-indigo-500 group-hover:scale-110 transition-transform duration-500" size={56} />
            </div>
            <div className="flex flex-col justify-center flex-1">
              <h4 className="text-xl font-bold text-gray-900 dark:text-gray-100 mb-3 group-hover:text-indigo-600 dark:group-hover:text-indigo-400 transition-colors tracking-tight">企业级私有云解决方案</h4>
              <p className="text-[15px] text-gray-600 dark:text-gray-400 leading-relaxed mb-6">
                为中大型企业提供安全、极速、高可用的私有云存储计算解决方案，支持多种混合云架构部署，满足严苛的数据合规要求。提供 99.99% 的SLA保障。
              </p>
              <div className="flex items-center gap-4 mt-auto">
                <button 
                  onClick={(e) => { e.stopPropagation(); onStartChat(enterprise.id); }}
                  className="px-5 py-2 bg-indigo-600 hover:bg-indigo-700 text-white text-sm font-bold rounded-lg shadow-sm transition-colors shadow-indigo-500/20"
                >
                  {t('detail.contactForQuote')}
                </button>
                <button className="text-sm font-bold text-gray-500 dark:text-gray-400 hover:text-gray-900 dark:hover:text-gray-200 flex items-center gap-1 transition-colors">
                  {t('detail.learnMore')} <ChevronRight size={16} />
                </button>
              </div>
            </div>
          </div>
          
          <div 
            onClick={() => onSelectProduct({ type: 'product', id: 'p2', title: '智能供应链协同平台' })}
            className="flex flex-col md:flex-row gap-8 p-6 rounded-2xl bg-gray-50/50 dark:bg-white/[0.02] border border-gray-200/60 dark:border-white/5 hover:border-indigo-200 dark:hover:border-indigo-500/30 transition-all hover:bg-white dark:hover:bg-[#1c1c1e] hover:shadow-xl hover:shadow-indigo-500/5 group cursor-pointer"
          >
            <div className="w-full md:w-56 h-36 bg-gradient-to-br from-blue-50 dark:from-blue-900/20 to-cyan-50 dark:to-cyan-900/20 rounded-xl shrink-0 flex items-center justify-center overflow-hidden border border-blue-100/50 dark:border-blue-500/10">
               <ShoppingCart className="text-blue-400 dark:text-blue-500 group-hover:scale-110 transition-transform duration-500" size={56} />
            </div>
            <div className="flex flex-col justify-center flex-1">
              <h4 className="text-xl font-bold text-gray-900 dark:text-gray-100 mb-3 group-hover:text-indigo-600 dark:group-hover:text-indigo-400 transition-colors tracking-tight">智能供应链协同平台</h4>
              <p className="text-[15px] text-gray-600 dark:text-gray-400 leading-relaxed mb-6">
                全面实现采购、库存、物流全链路的智能监控与调配。支持千万级数据并发，沉淀行业最佳实践，平均为企业降低30%的供应链管理成本。
              </p>
              <div className="flex items-center gap-4 mt-auto">
                <button 
                  onClick={(e) => { e.stopPropagation(); onStartChat(enterprise.id); }}
                  className="px-5 py-2 bg-indigo-600 hover:bg-indigo-700 text-white text-sm font-bold rounded-lg shadow-sm transition-colors shadow-indigo-500/20"
                >
                  {t('detail.contactForQuote')}
                </button>
                <button className="text-sm font-bold text-gray-500 dark:text-gray-400 hover:text-gray-900 dark:hover:text-gray-200 flex items-center gap-1 transition-colors">
                  {t('detail.learnMore')} <ChevronRight size={16} />
                </button>
              </div>
            </div>
          </div>
        </div>
      </section>
    </div>
  );
};
