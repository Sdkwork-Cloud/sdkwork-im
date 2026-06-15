import React, { useState } from 'react';
import { Package, Plus, Search, Filter, Edit, Trash2, ArrowUpCircle, ArrowDownCircle, Store, Tag, EyeOff, CheckCircle2, TrendingUp } from 'lucide-react';
import { cn } from '@sdkwork/im-pc-commons';

export const ConsoleProducts = () => {
  const [searchTerm, setSearchTerm] = useState('');
  const [categoryFilter, setCategoryFilter] = useState('all');

  const categories = [
    { id: 'all', label: '所有品类' },
    { id: 'beverage', label: '饮品' },
    { id: 'bakery', label: '烘焙' },
    { id: 'dessert', label: '甜点' },
    { id: 'merch', label: '周边配置' },
    { id: 'beans', label: '咖啡豆' }
  ];

  const productStats = [
    { label: '在售商品', value: '142', icon: CheckCircle2, color: 'text-emerald-500' },
    { label: '库存告急', value: '12', icon: ArrowDownCircle, color: 'text-rose-500' },
    { label: '本周上新', value: '8', icon: Plus, color: 'text-blue-500' },
    { label: '累计销量', value: '45.2k', icon: TrendingUp, color: 'text-purple-500' },
  ];

  const mockProducts = [
    { id: 'PD-10200', name: '极速冷萃咖啡 (Cold Brew)', category: 'beverage', categoryLabel: '饮品', price: '￥28.00', stock: 154, sales: 12450, status: '上架', store: '全部门店同步' },
    { id: 'PD-10201', name: '经典燕麦拿铁 (Oat Latte)', category: 'beverage', categoryLabel: '饮品', price: '￥32.00', stock: 89, sales: 8840, status: '上架', store: '全部门店同步' },
    { id: 'PD-20101', name: '全麦综合坚果碱水结', category: 'bakery', categoryLabel: '烘焙', price: '￥18.00', stock: 0, sales: 3200, status: '售罄', store: '仅北京旗舰店' },
    { id: 'PD-30045', name: '季节限定草莓雪纺慕斯', category: 'dessert', categoryLabel: '甜点', price: '￥42.00', stock: 24, sales: 890, status: '下架', store: '全部门店同步' },
    { id: 'PD-80010', name: '品牌定制不锈钢随行杯 400ml', category: 'merch', categoryLabel: '周边配置', price: '￥128.00', stock: 50, sales: 450, status: '上架', store: '华东区门店' },
    { id: 'PD-90001', name: '耶加雪菲日晒单品豆 (250g)', category: 'beans', categoryLabel: '咖啡豆', price: '￥88.00', stock: 120, sales: 1220, status: '上架', store: '全部门店同步' },
  ];

  const filteredProducts = mockProducts.filter(p => {
    const matchSearch = p.name.includes(searchTerm) || p.id.includes(searchTerm);
    const matchCat = categoryFilter === 'all' || p.category === categoryFilter;
    return matchSearch && matchCat;
  });

  return (
    <div className="bg-console-bg-panel border border-console-border rounded-2xl shadow-sm flex flex-col flex-1 min-h-0 h-full overflow-hidden">
      {/* Header */}
      <div className="flex items-center justify-between p-6 border-b border-console-border shrink-0 bg-console-bg-root/50">
        <div>
          <h2 className="text-xl font-bold text-console-text-main tracking-wide line-clamp-1">商品与微店中台</h2>
          <p className="text-sm text-console-text-muted mt-1">管理各门店下架商品、库存预警以及 SKU 同步配置</p>
        </div>
        <button className="bg-blue-600 hover:bg-blue-700 text-white px-4 py-2 rounded-lg text-sm transition-colors flex items-center gap-2 font-medium shrink-0 shadow-lg shadow-blue-600/20">
          <Plus size={16} />
          <span>添加新商品</span>
        </button>
      </div>

      {/* Stats Summary */}
      <div className="grid grid-cols-4 gap-4 p-6 shrink-0 border-b border-console-border bg-console-bg-root/30">
        {productStats.map((stat, i) => (
          <div key={i} className="flex items-center gap-4 bg-console-bg-panel border border-console-border p-4 rounded-xl shadow-sm">
            <div className={cn("w-12 h-12 rounded-full flex items-center justify-center shrink-0 bg-console-bg-root border border-console-border", stat.color)}>
              <stat.icon size={22} className="opacity-90" />
            </div>
            <div>
              <div className="text-[12px] font-medium text-console-text-muted mb-1">{stat.label}</div>
              <div className="text-xl font-bold text-console-text-main font-mono">{stat.value}</div>
            </div>
          </div>
        ))}
      </div>

      {/* Toolbar & Filters */}
      <div className="p-4 border-b border-console-border flex flex-wrap items-center justify-between gap-4 bg-console-bg-root/50 shrink-0">
        <div className="flex flex-wrap items-center gap-3">
          <div className="relative">
            <Search size={16} className="absolute left-3 top-1/2 -translate-y-1/2 text-console-text-muted" />
            <input 
              type="text" 
              placeholder="搜索商品名称、SKU..." 
              value={searchTerm}
              onChange={(e) => setSearchTerm(e.target.value)}
              className="w-72 bg-console-input-bg border border-console-border rounded-lg py-2 pl-9 pr-4 text-sm text-console-text-main focus:ring-1 focus:ring-blue-500 outline-none transition-all placeholder:text-console-text-muted"
            />
          </div>
          <div className="flex gap-1.5 p-1 bg-console-bg-panel border border-console-border rounded-lg">
            {categories.map(cat => (
              <button 
                key={cat.id}
                onClick={() => setCategoryFilter(cat.id)}
                className={cn(
                  "px-3 py-1.5 text-[13px] font-medium rounded-md transition-colors",
                  categoryFilter === cat.id ? "bg-blue-500/10 text-blue-500" : "text-console-text-muted hover:text-console-text-main"
                )}
              >
                {cat.label}
              </button>
            ))}
          </div>
        </div>
        <button className="text-console-text-muted hover:text-console-text-main px-3 py-2 border border-console-border rounded-lg text-sm flex items-center gap-2 bg-console-bg-panel hover:bg-console-bg-hover transition-colors font-medium">
          <Filter size={15} />
          <span>高级筛选</span>
        </button>
      </div>

      {/* Main Table Content */}
      <div className="flex-1 overflow-auto custom-scrollbar bg-console-bg-panel">
        <table className="w-full text-left border-collapse min-w-[1000px]">
          <thead className="bg-console-bg-root sticky top-0 z-10 shadow-sm shadow-black/5">
            <tr>
              <th className="px-6 py-4 font-medium text-[13px] text-console-text-muted w-10">
                <input type="checkbox" className="rounded border-console-border bg-transparent text-blue-500 focus:ring-blue-500/50" />
              </th>
              <th className="px-6 py-4 font-medium text-[13px] text-console-text-muted">商品档案</th>
              <th className="px-6 py-4 font-medium text-[13px] text-console-text-muted">分销范围</th>
              <th className="px-6 py-4 font-medium text-[13px] text-console-text-muted">定价策略</th>
              <th className="px-6 py-4 font-medium text-[13px] text-console-text-muted">实时库存/销量</th>
              <th className="px-6 py-4 font-medium text-[13px] text-console-text-muted">运维状态</th>
              <th className="px-6 py-4 font-medium text-[13px] text-console-text-muted text-right">资产管理</th>
            </tr>
          </thead>
          <tbody className="divide-y divide-console-border text-sm">
            {filteredProducts.map((product) => (
              <tr key={product.id} className="hover:bg-console-bg-hover transition-colors group">
                <td className="px-6 py-4 text-center">
                  <input type="checkbox" className="rounded border-console-border bg-transparent text-blue-500 focus:ring-blue-500/50" />
                </td>
                <td className="px-6 py-4">
                  <div className="flex items-center gap-3">
                    <div className="w-12 h-12 rounded-xl bg-console-bg-root border border-console-border flex items-center justify-center shrink-0 text-console-text-muted group-hover:border-blue-500/30 transition-colors">
                      <Package size={22} className="opacity-70 group-hover:text-blue-500 transition-colors" />
                    </div>
                    <div>
                      <div className="font-semibold text-console-text-main flex items-center gap-2 tracking-wide">
                        {product.name}
                      </div>
                      <div className="flex items-center gap-2 mt-1.5">
                        <span className="flex items-center gap-1 text-[10px] text-console-text-muted border border-console-border px-1.5 py-0.5 rounded uppercase font-medium bg-console-bg-root">
                          <Tag size={10} /> {product.categoryLabel}
                        </span>
                        <span className="text-[11px] text-console-text-muted font-mono tracking-wider">SKU: {product.id}</span>
                      </div>
                    </div>
                  </div>
                </td>
                <td className="px-6 py-4">
                  <span className="flex items-center gap-1.5 text-console-text-muted text-[13px] bg-console-bg-root border border-console-border px-2.5 py-1 rounded-md w-fit">
                    {product.store === '全部门店同步' ? <Store size={14} className="text-blue-500" /> : <Store size={14} className="text-amber-500" />}
                    {product.store}
                  </span>
                </td>
                <td className="px-6 py-4 font-bold text-console-text-main font-mono tracking-wide text-[14px]">{product.price}</td>
                <td className="px-6 py-4">
                  <div className="flex flex-col gap-1.5">
                    {product.stock > 0 ? (
                      <span className="flex items-center gap-1.5 text-[13px] font-medium text-console-text-main"><ArrowUpCircle size={15} className="text-emerald-500"/> 库存: {product.stock} 件</span>
                    ) : (
                      <span className="flex items-center gap-1.5 text-[13px] font-medium text-rose-500"><ArrowDownCircle size={15}/> 告急: 0 件</span>
                    )}
                    <span className="text-[11px] text-console-text-muted font-mono tracking-wide">累计销 {product.sales.toLocaleString()}</span>
                  </div>
                </td>
                <td className="px-6 py-4">
                  <div className="flex items-center gap-2">
                    <span className={cn("w-2 h-2 rounded-full",
                      product.status === '上架' ? 'bg-emerald-500 shadow-[0_0_8px_rgba(16,185,129,0.5)]' : 
                      product.status === '售罄' ? 'bg-rose-500' : 'bg-gray-500'
                    )}></span>
                    <span className="text-[13px] font-medium text-console-text-main">{product.status}</span>
                  </div>
                </td>
                <td className="px-6 py-4 text-right">
                  <div className="flex items-center justify-end gap-1.5">
                    <button className="p-1.5 text-console-text-muted hover:text-blue-500 hover:bg-console-bg-root border border-transparent hover:border-console-border rounded-lg transition-all" title="门店价格与库存管理">
                      <Store size={16} />
                    </button>
                    <button className="px-2.5 py-1.5 text-[12px] font-medium border border-console-border rounded-lg bg-console-bg-root hover:bg-console-bg-hover hover:text-blue-500 transition-colors">
                      {product.status === '下架' ? '重新上架' : '暂停售卖'}
                    </button>
                    <button className="p-1.5 text-console-text-muted hover:text-blue-500 hover:bg-console-bg-root border border-transparent hover:border-console-border rounded-lg transition-all" title="编辑档案">
                      <Edit size={16} />
                    </button>
                    <button className="p-1.5 text-console-text-muted hover:text-rose-500 hover:bg-console-bg-root border border-transparent hover:border-rose-500/30 rounded-lg transition-all" title="删除">
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
        <div>为您检索到 {filteredProducts.length} 个 SKU</div>
        <div className="flex gap-1.5">
          <button className="px-3 py-1.5 border border-console-border rounded-md text-console-text-muted cursor-not-allowed bg-console-bg-root">上一页</button>
          <button className="px-3 py-1.5 border border-blue-500 rounded-md bg-blue-500/10 text-blue-500 font-medium">1</button>
          <button className="px-3 py-1.5 border border-console-border rounded-md text-console-text-main hover:bg-console-bg-hover">2</button>
          <button className="px-3 py-1.5 border border-console-border rounded-md text-console-text-main hover:bg-console-bg-hover">下一页</button>
        </div>
      </div>
    </div>
  );
};
