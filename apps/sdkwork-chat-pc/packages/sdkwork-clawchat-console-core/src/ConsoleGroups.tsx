import React, { useState } from 'react';
import { Search, Plus, MoreHorizontal, Shield, Users, MessageCircle, Settings, Filter, Lock, Globe } from 'lucide-react';
import { cn } from '@sdkwork/clawchat-pc-commons';

const mockGroups = [
  { id: 'G-1001', name: '全员交流群', type: 'public', members: 1240, owner: 'Admin', status: 'active', messagesToDay: 4210, created: '2023-01-15' },
  { id: 'G-1002', name: '产品前线', type: 'private', members: 45, owner: '张三', status: 'active', messagesToDay: 852, created: '2023-03-22' },
  { id: 'G-1003', name: 'Q3 项目作战室', type: 'private', members: 12, owner: '李四', status: 'active', messagesToDay: 124, created: '2023-06-10' },
  { id: 'G-1004', name: '技术支持中心', type: 'public', members: 320, owner: 'System', status: 'active', messagesToDay: 532, created: '2023-02-05' },
  { id: 'G-1005', name: '已归档-旧项目', type: 'private', members: 8, owner: '王五', status: 'archived', messagesToDay: 0, created: '2022-11-10' },
];

export const ConsoleGroups = () => {
  const [searchTerm, setSearchTerm] = useState('');

  return (
    <div className="bg-console-bg-panel border border-console-border rounded-2xl shadow-sm flex flex-col min-h-[600px] overflow-hidden">
      {/* Header */}
      <div className="flex items-center justify-between p-6 border-b border-console-border">
        <div>
          <h2 className="text-lg font-bold text-console-text-main">群组与通信管理</h2>
          <p className="text-sm text-console-text-muted mt-1">管理企业内的所有聊天群组及全局通信策略</p>
        </div>
        <div className="flex gap-3">
          <button className="bg-console-bg-hover hover:bg-console-border text-console-text-main px-4 py-2 rounded-lg text-sm font-medium transition-colors flex items-center gap-2 border border-console-border">
            <Settings size={16} />
            全局策略设置
          </button>
          <button className="bg-blue-600 hover:bg-blue-700 text-white px-4 py-2 rounded-lg text-sm font-medium transition-colors flex items-center gap-2 shadow-sm">
            <Plus size={16} />
            新建群组
          </button>
        </div>
      </div>

      {/* Metrics Row */}
      <div className="grid grid-cols-4 divide-x divide-console-border border-b border-console-border bg-console-bg-root/50">
        <div className="p-4 flex flex-col">
          <span className="text-xs text-console-text-muted mb-1">群组总数</span>
          <span className="text-xl font-bold text-console-text-main">3,842</span>
        </div>
        <div className="p-4 flex flex-col">
          <span className="text-xs text-console-text-muted mb-1">今日新建</span>
          <span className="text-xl font-bold text-emerald-500">+12</span>
        </div>
        <div className="p-4 flex flex-col">
          <span className="text-xs text-console-text-muted mb-1">今日活跃群组</span>
          <span className="text-xl font-bold text-console-text-main">845 <span className="text-xs font-normal text-console-text-muted ml-1">(22%)</span></span>
        </div>
        <div className="p-4 flex flex-col">
          <span className="text-xs text-console-text-muted mb-1">日均消息量/群</span>
          <span className="text-xl font-bold text-console-text-main">152</span>
        </div>
      </div>

      {/* Toolbar */}
      <div className="p-4 flex items-center justify-between bg-console-bg-root border-b border-console-border">
        <div className="flex items-center gap-3">
          <div className="relative">
            <Search size={16} className="absolute left-3 top-1/2 -translate-y-1/2 text-console-text-muted" />
            <input 
              type="text" 
              placeholder="搜索群ID、群名称、群主..." 
              value={searchTerm}
              onChange={(e) => setSearchTerm(e.target.value)}
              className="w-72 bg-console-input-bg border border-console-border rounded-lg py-1.5 pl-9 pr-4 text-sm text-console-text-main focus:ring-2 focus:ring-blue-500/50 focus:border-blue-500 outline-none transition-all"
            />
          </div>
          <button className="bg-console-bg-panel border border-console-border text-console-text-main px-3 py-1.5 rounded-lg text-sm flex items-center gap-2 hover:bg-console-bg-hover transition-colors">
            <Filter size={14} />
            筛选
          </button>
        </div>
        
        <div className="flex gap-2">
          <select className="bg-console-bg-panel border border-console-border text-sm text-console-text-main rounded-lg px-3 py-1.5 outline-none cursor-pointer hover:bg-console-bg-hover transition-colors">
            <option>批量操作</option>
            <option>解散群组</option>
            <option>转移群主</option>
          </select>
        </div>
      </div>

      {/* Table */}
      <div className="flex-1 overflow-auto">
        <table className="w-full text-left border-collapse">
          <thead>
            <tr className="bg-console-bg-root text-console-text-muted text-xs uppercase tracking-wider border-b border-console-border">
              <th className="px-6 py-4 font-semibold w-12 text-center">
                <input type="checkbox" className="rounded border-console-border text-blue-600 focus:ring-blue-500" />
              </th>
              <th className="px-6 py-4 font-semibold">群组信息</th>
              <th className="px-6 py-4 font-semibold">类型</th>
              <th className="px-6 py-4 font-semibold">群主</th>
              <th className="px-6 py-4 font-semibold">成员数</th>
              <th className="px-6 py-4 font-semibold">今日消息数</th>
              <th className="px-6 py-4 font-semibold">状态</th>
              <th className="px-6 py-4 font-semibold text-right">操作</th>
            </tr>
          </thead>
          <tbody className="divide-y divide-console-border text-sm">
            {mockGroups.map((group) => (
              <tr key={group.id} className="hover:bg-console-bg-hover transition-colors group">
                <td className="px-6 py-4 text-center">
                  <input type="checkbox" className="rounded border-console-border text-blue-600 focus:ring-blue-500" />
                </td>
                <td className="px-6 py-4">
                  <div className="flex items-center gap-3">
                    <div className="w-10 h-10 rounded-xl bg-blue-100/50 text-blue-600 flex items-center justify-center">
                      <Users size={18} />
                    </div>
                    <div>
                      <div className="font-semibold text-console-text-main group-hover:text-blue-600 transition-colors cursor-pointer">{group.name}</div>
                      <div className="text-xs text-console-text-muted mt-0.5 font-mono">{group.id}</div>
                    </div>
                  </div>
                </td>
                <td className="px-6 py-4">
                  {group.type === 'public' ? (
                    <span className="inline-flex items-center gap-1 text-xs text-emerald-600 font-medium">
                      <Globe size={12} /> 公开群
                    </span>
                  ) : (
                    <span className="inline-flex items-center gap-1 text-xs text-amber-600 font-medium">
                      <Lock size={12} /> 私密群
                    </span>
                  )}
                </td>
                <td className="px-6 py-4 text-console-text-main">{group.owner}</td>
                <td className="px-6 py-4 text-console-text-main font-medium">{group.members}</td>
                <td className="px-6 py-4">
                  <div className="flex items-center gap-1.5 text-console-text-muted">
                    <MessageCircle size={14} />
                    <span>{group.messagesToDay}</span>
                  </div>
                </td>
                <td className="px-6 py-4">
                  {group.status === 'active' ? (
                    <span className="px-2.5 py-1 rounded-md text-[11px] font-medium bg-emerald-500/10 text-emerald-600 border border-emerald-500/20">正常</span>
                  ) : (
                    <span className="px-2.5 py-1 rounded-md text-[11px] font-medium bg-console-bg-hover text-console-text-muted border border-console-border">已归档</span>
                  )}
                </td>
                <td className="px-6 py-4 text-right">
                  <button className="p-1.5 text-console-text-muted hover:text-blue-600 hover:bg-console-bg-root rounded-md transition-colors">
                    <MoreHorizontal size={18} />
                  </button>
                </td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>
      
      {/* Pagination */}
      <div className="p-4 border-t border-console-border flex items-center justify-between text-xs text-console-text-muted bg-console-bg-root/50">
        <div>显示 1 到 5 条，共 3,842 条记录</div>
        <div className="flex gap-1">
          <button className="px-3 py-1.5 border border-console-border rounded text-console-text-muted cursor-not-allowed bg-console-bg-root">上一页</button>
          <button className="px-3 py-1.5 border border-blue-600 rounded bg-blue-600 text-white font-medium">1</button>
          <button className="px-3 py-1.5 border border-console-border rounded text-console-text-main hover:bg-console-bg-hover transition-colors">2</button>
          <button className="px-3 py-1.5 border border-console-border rounded text-console-text-main hover:bg-console-bg-hover transition-colors">3</button>
          <span className="px-2 py-1.5">...</span>
          <button className="px-3 py-1.5 border border-console-border rounded text-console-text-main hover:bg-console-bg-hover transition-colors">下一页</button>
        </div>
      </div>
    </div>
  );
};
