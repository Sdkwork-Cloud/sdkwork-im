import React from 'react';
import { useTranslation } from 'react-i18next';
import { Package, TrendingUp, Clock, Building2 } from 'lucide-react';

const mockSupplies = [
  {
    id: 's1',
    title: '提供高性能工业级物联网网关',
    company: '杭州字节流物联技术有限公司',
    category: '物联网/硬件',
    price: '￥1,200',
    unit: '台',
    minOrder: '10台起订',
    date: '2026-06-01',
    description: '采用工业级芯片设计，支持4G/5G全网通，支持多种主流工业协议(Modbus/PLC/OPC)，提供二次开发接口。稳定可靠，防浪涌防静电，适用于各种恶劣工业现场。',
    status: '供应中'
  },
  {
    id: 's2',
    title: '大模型私有化部署解决方案',
    company: '深圳灵动科技有限公司',
    category: '软件及服务',
    price: '面议',
    unit: '-',
    minOrder: '一套起订',
    date: '2026-05-30',
    description: '提供7B-72B级别开源大模型或商业大模型的私有化本地化部署方案，支持企业内部数据微调，构建企业内部知识中枢，保障数据绝对安全。提供软硬件一体机方案。',
    status: '供应中'
  }
];

export const SupplyList: React.FC = () => {
  const { t } = useTranslation('enterprise');
  return (
    <div className="max-w-[1000px] mx-auto space-y-4">
      {mockSupplies.map(supply => (
        <div key={supply.id} className="bg-white dark:bg-[#28282b] rounded-2xl p-6 border border-gray-200 dark:border-white/5 shadow-sm hover:shadow-lg transition-all group flex flex-col md:flex-row md:items-center gap-6 cursor-pointer">
          <div className="w-16 h-16 rounded-2xl bg-indigo-50 dark:bg-indigo-500/10 flex items-center justify-center shrink-0 text-indigo-500">
            <Package size={28} />
          </div>
          <div className="flex-1 min-w-0">
            <div className="flex items-center justify-between gap-4 mb-2">
              <h3 className="text-lg font-bold text-gray-900 dark:text-gray-100 truncate group-hover:text-indigo-600 dark:group-hover:text-indigo-400 transition-colors">{supply.title}</h3>
              <span className="shrink-0 px-2.5 py-1 text-xs font-medium text-green-600 bg-green-50 dark:bg-green-500/10 border border-green-200 dark:border-green-500/20 rounded-md">
                {t('supply.status.supplying')}
              </span>
            </div>
            <p className="text-sm text-gray-500 dark:text-gray-400 line-clamp-2 mb-3 leading-relaxed">
              {supply.description}
            </p>
            <div className="flex flex-wrap items-center gap-x-6 gap-y-2 text-sm text-gray-500 dark:text-gray-400">
              <span className="flex items-center gap-1.5"><Building2 size={14} className="text-gray-400" /> {supply.company}</span>
              <span className="flex items-center gap-1.5"><TrendingUp size={14} className="text-gray-400" /> {supply.price}/{supply.unit} ({supply.minOrder})</span>
              <span className="flex items-center gap-1.5"><Clock size={14} className="text-gray-400" /> {t('supply.publishedAt')} {supply.date}</span>
            </div>
          </div>
          <div className="shrink-0 pt-4 md:pt-0 md:border-l border-gray-100 dark:border-white/5 md:pl-6">
             <button className="w-full md:w-auto px-6 py-2.5 bg-indigo-50 hover:bg-indigo-100 dark:bg-indigo-500/10 dark:hover:bg-indigo-500/20 text-indigo-600 dark:text-indigo-400 rounded-xl font-medium transition-colors border border-indigo-100 dark:border-indigo-500/20">
               {t('supply.contactSupplier')}
             </button>
          </div>
        </div>
      ))}
    </div>
  );
};
