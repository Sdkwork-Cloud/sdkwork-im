import React, { useState } from 'react';
import { Store, Plus, Search, MapPin, Edit, Trash2, Image as ImageIcon, Map, Activity, CheckCircle, Clock, Package } from 'lucide-react';
import { cn } from '@sdkwork/im-pc-commons';

export const ConsoleStores = () => {
  const [searchTerm, setSearchTerm] = useState('');
  const [activeTab, setActiveTab] = useState<'all' | 'operating' | 'decorating'>('all');

  const storeStats = [
    { label: '总门店数', value: '18', trend: '+2', icon: Store, color: 'text-blue-600', bg: 'bg-blue-500/10' },
    { label: '营业中', value: '15', trend: '稳定', icon: CheckCircle, color: 'text-emerald-500', bg: 'bg-emerald-500/10' },
    { label: '装修与休息', value: '3', trend: '-1', icon: Clock, color: 'text-amber-500', bg: 'bg-amber-500/10' },
    { label: '本月日均客流', value: '8.4k', trend: '+12%', icon: Activity, color: 'text-purple-500', bg: 'bg-purple-500/10' }
  ];

  const mockStores = [
    { id: 'ST-001', name: '北京三里屯太古里旗舰店', address: '北京市朝阳区三里屯路19号太古里南区', status: 'operating', type: '直营店', contact: '010-85942200', views: 3450, online: true },
    { id: 'ST-002', name: '上海新天地概念微型店', address: '上海市黄浦区马当路245号新天地', status: 'operating', type: '直营店', contact: '021-33315588', views: 4210, online: true },
    { id: 'ST-003', name: '广州天环广场沉浸式体验店', address: '广州市天河区天河路218号天环广场一楼', status: 'decorating', type: '加盟店', contact: '020-55556666', views: 0, online: false },
    { id: 'ST-004', name: '深圳万象天地快闪店', address: '深圳市南山区深南大道9668号万象天地', status: 'operating', type: '快闪店', contact: '0755-88889999', views: 5690, online: true },
    { id: 'ST-005', name: '成都远洋太古里精品店', address: '成都市锦江区中纱帽街8号', status: 'operating', type: '直营店', contact: '028-66667777', views: 2180, online: true },
  ];

  const filteredStores = mockStores.filter(store => {
    const matchSearch = store.name.includes(searchTerm) || store.id.includes(searchTerm);
    if (activeTab === 'all') return matchSearch;
    return matchSearch && store.status === activeTab;
  });

  return (
    <div className="bg-console-bg-panel border border-console-border rounded-2xl shadow-sm flex flex-col flex-1 min-h-0 h-full overflow-hidden">
      {/* Header */}
      <div className="flex items-center justify-between p-6 border-b border-console-border shrink-0 bg-console-bg-root/50">
        <div>
          <h2 className="text-xl font-bold text-console-text-main tracking-wide line-clamp-1">全域门店与网点管理</h2>
          <p className="text-sm text-console-text-muted mt-1">管理各区域下线下门店及线上微店的配置与同步</p>
        </div>
        <button className="bg-blue-600 hover:bg-blue-700 text-white px-4 py-2 rounded-lg text-sm transition-colors flex items-center gap-2 font-medium shrink-0 shadow-lg shadow-blue-600/20">
          <Plus size={16} />
          <span>部署新门店</span>
        </button>
      </div>

      {/* Stats Cards */}
      <div className="grid grid-cols-4 gap-4 p-6 shrink-0 bg-console-bg-root/30 border-b border-console-border">
        {storeStats.map((stat, i) => (
          <div key={i} className="bg-console-bg-panel border border-console-border p-4 rounded-xl shadow-sm flex items-start justify-between">
            <div>
              <div className="text-[12px] font-medium text-console-text-muted mb-1">{stat.label}</div>
              <div className="text-2xl font-bold text-console-text-main font-mono">{stat.value}</div>
            </div>
            <div className={cn("w-10 h-10 rounded-lg flex items-center justify-center shrink-0", stat.bg)}>
              <stat.icon size={20} className={stat.color} />
            </div>
          </div>
        ))}
      </div>

      {/* Toolbar & Tabs */}
      <div className="px-6 pt-4 border-b border-console-border flex items-end justify-between bg-console-bg-root/50 shrink-0">
        <div className="flex items-center gap-6">
          <button 
            className={cn("pb-3 text-sm font-medium border-b-2 transition-colors", activeTab === 'all' ? "border-blue-500 text-blue-500" : "border-transparent text-console-text-muted hover:text-console-text-main")}
            onClick={() => setActiveTab('all')}
          >
            全部分店 ({mockStores.length})
          </button>
          <button 
            className={cn("pb-3 text-sm font-medium border-b-2 transition-colors", activeTab === 'operating' ? "border-blue-500 text-blue-500" : "border-transparent text-console-text-muted hover:text-console-text-main")}
            onClick={() => setActiveTab('operating')}
          >
            营业中
          </button>
          <button 
            className={cn("pb-3 text-sm font-medium border-b-2 transition-colors", activeTab === 'decorating' ? "border-blue-500 text-blue-500" : "border-transparent text-console-text-muted hover:text-console-text-main")}
            onClick={() => setActiveTab('decorating')}
          >
            休业/装修中
          </button>
        </div>

        <div className="pb-3 flex items-center gap-3">
          <div className="relative">
            <Search size={16} className="absolute left-3 top-1/2 -translate-y-1/2 text-console-text-muted" />
            <input 
              type="text" 
              placeholder="搜索门店名或 ID..." 
              value={searchTerm}
              onChange={(e) => setSearchTerm(e.target.value)}
              className="w-64 bg-console-input-bg border border-console-border rounded-lg py-1.5 pl-9 pr-4 text-sm text-console-text-main focus:ring-1 focus:ring-blue-500 outline-none transition-all placeholder:text-console-text-muted"
            />
          </div>
        </div>
      </div>

      {/* Main Table Content */}
      <div className="flex-1 overflow-auto custom-scrollbar bg-console-bg-panel">
        <table className="w-full text-left border-collapse min-w-[900px]">
          <thead className="bg-console-bg-root sticky top-0 z-10 shadow-sm shadow-black/5">
            <tr>
              <th className="px-6 py-4 font-medium text-[13px] text-console-text-muted">门店信息</th>
              <th className="px-6 py-4 font-medium text-[13px] text-console-text-muted">类型与营业状态</th>
              <th className="px-6 py-4 font-medium text-[13px] text-console-text-muted">联系方式</th>
              <th className="px-6 py-4 font-medium text-[13px] text-console-text-muted">地理位置</th>
              <th className="px-6 py-4 font-medium text-[13px] text-console-text-muted">微店同步/日客流</th>
              <th className="px-6 py-4 font-medium text-[13px] text-console-text-muted text-right">操作</th>
            </tr>
          </thead>
          <tbody className="divide-y divide-console-border text-sm">
            {filteredStores.map((store) => (
              <tr key={store.id} className="hover:bg-console-bg-hover transition-colors group">
                <td className="px-6 py-4">
                  <div className="flex items-center gap-3">
                    <div className="w-12 h-12 rounded-xl bg-gradient-to-br from-blue-500/10 to-transparent border border-blue-500/20 flex flex-col items-center justify-center shrink-0">
                      <Store size={22} className="text-blue-500" />
                    </div>
                    <div>
                      <div className="font-semibold text-console-text-main tracking-wide">{store.name}</div>
                      <div className="text-[11px] text-console-text-muted mt-1 font-mono tracking-wider">{store.id}</div>
                    </div>
                  </div>
                </td>
                <td className="px-6 py-4">
                  <div className="flex flex-col items-start gap-1.5">
                    <span className="text-[12px] font-medium text-console-text-main bg-console-bg-root border border-console-border px-2 py-0.5 rounded-md">
                      {store.type}
                    </span>
                    <span className={cn("px-2 py-0.5 rounded-full text-[11px] font-bold tracking-wider", 
                      store.status === 'operating' ? 'bg-emerald-500/10 text-emerald-500 border border-emerald-500/20' : 
                      'bg-amber-500/10 text-amber-500 border border-amber-500/20'
                    )}>
                      {store.status === 'operating' ? '● 营业中' : '○ 装修中'}
                    </span>
                  </div>
                </td>
                <td className="px-6 py-4 text-console-text-main text-[13px] font-mono tracking-wide">
                  {store.contact}
                </td>
                <td className="px-6 py-4">
                  <div className="flex items-start gap-1.5 text-console-text-muted hover:text-blue-500 cursor-pointer transition-colors max-w-[220px]" title={store.address}>
                    <MapPin size={15} className="shrink-0 mt-0.5 text-console-text-muted" />
                    <span className="text-[13px] leading-relaxed line-clamp-2">{store.address}</span>
                  </div>
                </td>
                <td className="px-6 py-4">
                  <div className="flex flex-col gap-1.5">
                    <div className="flex items-center gap-1.5">
                      <div className={cn("w-2 h-2 rounded-full", store.online ? "bg-emerald-500 shadow-[0_0_8px_rgba(16,185,129,0.5)]" : "bg-gray-400")}></div>
                      <span className="text-[12px] font-medium text-console-text-muted">{store.online ? '已上线微店' : '未同步'}</span>
                    </div>
                    <div className="text-[11px] text-console-text-muted font-mono">{store.views.toLocaleString()} UV/日</div>
                  </div>
                </td>
                <td className="px-6 py-4 text-right">
                  <div className="flex items-center justify-end gap-1.5">
                    <button className="p-1.5 text-console-text-muted hover:text-blue-500 hover:bg-console-bg-root border border-transparent hover:border-console-border rounded-lg transition-all" title="上架商品">
                      <Package size={16} />
                    </button>
                    <button className="p-1.5 text-console-text-muted hover:text-blue-500 hover:bg-console-bg-root border border-transparent hover:border-console-border rounded-lg transition-all" title="门店相册">
                      <ImageIcon size={16} />
                    </button>
                    <button className="p-1.5 text-console-text-muted hover:text-blue-500 hover:bg-console-bg-root border border-transparent hover:border-console-border rounded-lg transition-all" title="地图定位">
                      <Map size={16} />
                    </button>
                    <button className="p-1.5 text-console-text-muted hover:text-emerald-500 hover:bg-console-bg-root border border-transparent hover:border-console-border rounded-lg transition-all" title="编辑门店">
                      <Edit size={16} />
                    </button>
                    <button className="p-1.5 text-console-text-muted hover:text-rose-500 hover:bg-console-bg-root border border-transparent hover:border-rose-500/30 rounded-lg transition-all" title="移除门店">
                      <Trash2 size={16} />
                    </button>
                  </div>
                </td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>
      
      {/* Footer Nav */}
      <div className="p-4 border-t border-console-border flex items-center justify-between text-xs text-console-text-muted shrink-0 bg-console-bg-root/80">
        <div>展示 {filteredStores.length} 家门店网络</div>
        <div className="flex gap-1.5">
          <button className="px-3 py-1.5 border border-console-border rounded-md text-console-text-muted cursor-not-allowed bg-console-bg-root">Previous</button>
          <button className="px-3 py-1.5 border border-blue-500 rounded-md bg-blue-500/10 text-blue-500 font-medium tracking-wide">1</button>
          <button className="px-3 py-1.5 border border-console-border rounded-md text-console-text-main hover:bg-console-bg-hover">2</button>
          <button className="px-3 py-1.5 border border-console-border rounded-md text-console-text-main hover:bg-console-bg-hover">Next</button>
        </div>
      </div>
    </div>
  );
};
