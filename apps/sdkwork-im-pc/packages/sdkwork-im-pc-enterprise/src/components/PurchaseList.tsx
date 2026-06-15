import React from 'react';
import { useTranslation } from 'react-i18next';
import { ShoppingCart, Clock, Building2, MapPin } from 'lucide-react';

const mockPurchases = [
  {
    id: 'p1',
    title: '急购一批高性能服务器及GPU算力设备',
    company: '深圳灵动科技有限公司',
    budget: '￥50万 - 200万',
    deadline: '2026-06-30',
    location: '深圳',
    status: '采购中',
    description: '因业务需求扩张，现需采购一批高性能服务器，包含A100/H800等算力卡。要求供应商具备原厂授权资质，提供完整的售后支持与维护方案。'
  },
  {
    id: 'p2',
    title: '采购工业级传感器及通讯模组',
    company: '杭州字节流物联技术有限公司',
    budget: '按实际报价',
    deadline: '2026-06-15',
    location: '杭州',
    status: '采购中',
    description: '长期采购各类工业级温湿度、压力传感器，以及NB-IoT/4G通讯模组。要求产品一致性好，不良率低，能提供长期的供货保证和技术支持。'
  }
];

export const PurchaseList: React.FC = () => {
  const { t } = useTranslation('enterprise');
  return (
    <div className="max-w-[1000px] mx-auto space-y-4">
      {mockPurchases.map(item => (
        <div key={item.id} className="bg-white dark:bg-[#28282b] rounded-2xl p-6 border border-gray-200 dark:border-white/5 shadow-sm hover:shadow-lg transition-all group flex flex-col md:flex-row md:items-center gap-6 cursor-pointer">
          <div className="w-16 h-16 rounded-2xl bg-orange-50 dark:bg-orange-500/10 flex items-center justify-center shrink-0 text-orange-500">
            <ShoppingCart size={28} />
          </div>
          <div className="flex-1 min-w-0">
            <div className="flex items-center justify-between gap-4 mb-2">
              <h3 className="text-lg font-bold text-gray-900 dark:text-gray-100 truncate group-hover:text-orange-600 dark:group-hover:text-orange-400 transition-colors">{item.title}</h3>
              <span className="shrink-0 px-2.5 py-1 text-xs font-medium text-orange-600 bg-orange-50 dark:bg-orange-500/10 border border-orange-200 dark:border-orange-500/20 rounded-md">
                {t('purchase.status.purchasing')}
              </span>
            </div>
            <p className="text-sm text-gray-500 dark:text-gray-400 line-clamp-2 mb-3 leading-relaxed">
              {item.description}
            </p>
            <div className="flex flex-wrap items-center gap-x-6 gap-y-2 text-sm text-gray-500 dark:text-gray-400">
              <span className="flex items-center gap-1.5"><Building2 size={14} className="text-gray-400" /> {item.company}</span>
              <span className="flex items-center gap-1.5 text-orange-600 dark:text-orange-500 font-medium">{t('purchase.budget')}: {item.budget}</span>
              <span className="flex items-center gap-1.5"><MapPin size={14} className="text-gray-400" /> {item.location}</span>
              <span className="flex items-center gap-1.5"><Clock size={14} className="text-gray-400" /> {t('purchase.deadline')}: {item.deadline}</span>
            </div>
          </div>
          <div className="shrink-0 pt-4 md:pt-0 md:border-l border-gray-100 dark:border-white/5 md:pl-6">
             <button className="w-full md:w-auto px-6 py-2.5 bg-orange-50 hover:bg-orange-100 dark:bg-orange-500/10 dark:hover:bg-orange-500/20 text-orange-600 dark:text-orange-400 rounded-xl font-medium transition-colors border border-orange-100 dark:border-orange-500/20">
               {t('detail.contactForQuote')}
             </button>
          </div>
        </div>
      ))}
    </div>
  );
};
