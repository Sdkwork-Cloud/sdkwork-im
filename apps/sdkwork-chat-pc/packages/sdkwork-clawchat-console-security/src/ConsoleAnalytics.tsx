import React from 'react';
import { LineChart, BarChart, Download, FileText, Calendar, Filter, Users, MessageSquare, Database, Activity } from 'lucide-react';
import { cn } from '@sdkwork/clawchat-pc-commons';

export const ConsoleAnalytics = () => {
  return (
    <div className="space-y-6">
      {/* Header */}
      <div className="flex flex-col sm:flex-row sm:items-center justify-between gap-4">
        <div>
          <h2 className="text-lg font-bold text-console-text-main">数据分析与报表</h2>
          <p className="text-sm text-console-text-muted mt-1">深度洞察用户活跃度、通信趋势与资源消耗</p>
        </div>
        <div className="flex gap-2">
          <div className="flex items-center gap-2 bg-console-bg-panel border border-console-border rounded-lg px-3 py-1.5 text-sm text-console-text-main">
            <Calendar size={14} className="text-console-text-muted" />
            <select className="bg-transparent outline-none">
              <option>过去 30 天</option>
              <option>过去 7 天</option>
              <option>本季度</option>
              <option>自定义时间范围</option>
            </select>
          </div>
          <button className="bg-console-bg-panel border border-console-border hover:bg-console-bg-hover text-console-text-main px-3 py-1.5 rounded-lg text-sm flex items-center gap-2 transition-colors">
            <Download size={14} />
            导出报表
          </button>
        </div>
      </div>

      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
        <StatCard title="日均活跃用户 (DAU)" value="8,245" change="+12%" icon={Users} color="blue" />
        <StatCard title="月活活跃用户 (MAU)" value="11,040" change="+5%" icon={Activity} color="emerald" />
        <StatCard title="日均消息量" value="45.2K" change="-2%" icon={MessageSquare} color="amber" />
        <StatCard title="日均新增存储空间" value="4.5 GB" change="+18%" icon={Database} color="indigo" />
      </div>

      <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
        {/* Message Volume Trend */}
        <div className="bg-console-bg-panel border border-console-border rounded-2xl shadow-sm p-6 flex flex-col min-h-[350px]">
          <div className="flex justify-between items-center mb-6">
            <h3 className="font-semibold text-console-text-main">消息量趋势</h3>
            <div className="flex gap-4 text-xs font-medium text-console-text-muted">
              <span className="flex items-center gap-1.5"><div className="w-2 h-2 rounded-full bg-blue-500"></div>点对点消息</span>
              <span className="flex items-center gap-1.5"><div className="w-2 h-2 rounded-full bg-indigo-500"></div>群组消息</span>
            </div>
          </div>
          <div className="flex-1 flex items-end gap-2 relative">
            {/* Mock Chart */}
            {[45, 52, 38, 65, 72, 48, 85, 90, 60, 55, 78, 65, 40, 80].map((h, i) => (
              <div key={i} className="flex-1 flex flex-col justify-end gap-1 group relative">
                <div 
                  className="w-full bg-indigo-500/80 rounded-t-sm hover:bg-indigo-400 transition-colors" 
                  style={{ height: `${h}%` }}
                ></div>
                <div 
                  className="w-full bg-blue-500/80 rounded-t-sm hover:bg-blue-400 transition-colors" 
                  style={{ height: `${h * 0.4}%` }}
                ></div>
              </div>
            ))}
          </div>
        </div>

        {/* Resource Usage */}
        <div className="bg-console-bg-panel border border-console-border rounded-2xl shadow-sm p-6 flex flex-col">
          <div className="flex justify-between items-center mb-6">
            <h3 className="font-semibold text-console-text-main">存储与资源消耗占比</h3>
            <button className="text-xs text-blue-600 font-medium">资源包详情</button>
          </div>
          
          <div className="space-y-6">
            <UsageBar label="媒体文件 (图片/视频)" percent={65} color="bg-emerald-500" value="2.8 TB" />
            <UsageBar label="常规文档 (PDF/Word/Excel)" percent={22} color="bg-blue-500" value="950 GB" />
            <UsageBar label="文本消息记录" percent={8} color="bg-indigo-500" value="345 GB" />
            <UsageBar label="其他附件 & 日志" percent={5} color="bg-amber-500" value="215 GB" />
          </div>

          <div className="mt-8 p-4 bg-console-bg-hover rounded-xl border border-console-border flex items-center justify-between">
            <p className="text-sm text-console-text-main">
              当前存储空间已使用 <span className="font-bold text-console-text-main">4.3 TB</span> / 5.0 TB
            </p>
            <button className="text-xs px-3 py-1.5 bg-blue-600 text-white rounded-lg font-medium shadow-sm hover:bg-blue-700 transition">
              升级配额
            </button>
          </div>
        </div>
      </div>
    </div>
  );
};

const StatCard = ({ title, value, change, icon: Icon, color }: any) => {
  const isUp = change.startsWith('+');
  const colorMap: Record<string, string> = {
    blue: "text-blue-600 bg-blue-50 dark:bg-blue-500/10 dark:text-blue-400",
    emerald: "text-emerald-600 bg-emerald-50 dark:bg-emerald-500/10 dark:text-emerald-400",
    amber: "text-amber-600 bg-amber-50 dark:bg-amber-500/10 dark:text-amber-400",
    indigo: "text-indigo-600 bg-indigo-50 dark:bg-indigo-500/10 dark:text-indigo-400",
  };

  return (
    <div className="bg-console-bg-panel border border-console-border p-5 rounded-2xl shadow-sm">
      <div className="flex items-center gap-3 mb-3">
        <div className={cn("p-2 rounded-lg", colorMap[color])}>
          <Icon size={18} />
        </div>
        <span className="text-sm font-medium text-console-text-muted">{title}</span>
      </div>
      <div className="flex items-end justify-between">
        <span className="text-2xl font-bold text-console-text-main">{value}</span>
        <span className={cn(
          "text-xs font-semibold px-2 py-1 rounded-md",
          isUp ? "bg-emerald-50 text-emerald-600 dark:bg-emerald-500/10 dark:text-emerald-400" : "bg-rose-50 text-rose-600 dark:bg-rose-500/10 dark:text-rose-400"
        )}>
          {change}
        </span>
      </div>
    </div>
  );
};

const UsageBar = ({ label, percent, value, color }: any) => (
  <div>
    <div className="flex justify-between text-sm mb-1.5">
      <span className="text-console-text-main font-medium">{label}</span>
      <span className="text-console-text-muted">{value} ({percent}%)</span>
    </div>
    <div className="w-full h-2 bg-console-bg-root rounded-full overflow-hidden border border-console-border-light">
      <div className={cn("h-full rounded-full transition-all", color)} style={{ width: `${percent}%` }}></div>
    </div>
  </div>
);
