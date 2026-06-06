import React from 'react';
import { Users, MessageSquare, Network, Database, Shield, Activity, Key, Bell } from 'lucide-react';
import { cn } from '@sdkwork/clawchat-pc-commons';

export const ConsoleDashboard: React.FC = () => {
  return (
    <>
      {/* Key Metrics */}
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
        <MetricCard title="总活跃用户 (30天)" value="12,450" trend="+5.2%" isUp={true} icon={Users} color="blue" />
        <MetricCard title="今日消息量" value="1.2M" trend="+12.4%" isUp={true} icon={MessageSquare} color="indigo" />
        <MetricCard title="活跃群组" value="3,842" trend="-2.1%" isUp={false} icon={Network} color="emerald" />
        <MetricCard title="存储容量使用" value="4.2 TB" trend="安全" isUp={true} icon={Database} color="amber" />
      </div>

      {/* Charts & Activity */}
      <div className="grid grid-cols-1 lg:grid-cols-3 gap-6">
        <div className="lg:col-span-2 bg-console-bg-panel border border-console-border rounded-2xl shadow-sm p-6 flex flex-col min-h-[360px]">
          <div className="flex justify-between items-center mb-6">
            <h3 className="text-base font-semibold text-console-text-main">通信活跃度趋势</h3>
            <select className="bg-console-input-bg border border-console-border text-sm text-console-text-main rounded-lg px-3 py-1 outline-none">
              <option>过去7天</option>
              <option>过去30天</option>
              <option>今年</option>
            </select>
          </div>
          <div className="flex-1 flex items-end gap-2 pb-4">
            {/* Mock Bar Chart */}
            {[40, 65, 45, 80, 55, 90, 70].map((h, i) => (
              <div key={i} className="flex-1 flex flex-col items-center gap-2 group">
                <div className="w-full bg-blue-100 rounded-t-sm relative flex items-end">
                  <div 
                    className="w-full bg-blue-500 rounded-t-sm transition-all group-hover:bg-blue-600" 
                    style={{ height: `${h}%` }}
                  ></div>
                </div>
                <span className="text-[11px] text-console-text-muted">周{['一','二','三','四','五','六','日'][i]}</span>
              </div>
            ))}
          </div>
        </div>

        <div className="bg-console-bg-panel border border-console-border rounded-2xl shadow-sm p-6 flex flex-col">
          <h3 className="text-base font-semibold text-console-text-main mb-4">系统安全告警</h3>
          <div className="flex-1 flex flex-col gap-4">
            <AlertItem type="high" message="检测到异常登录地点 (IP: 182.xx.xx.xx)" time="10分钟前" />
            <AlertItem type="medium" message="大量文件下载行为触发表" time="2小时前" />
            <AlertItem type="low" message="API 速率达到警戒值 (80%)" time="5小时前" />
            <AlertItem type="info" message="本周安全巡检报告已生成" time="昨天" />
          </div>
          <button className="mt-4 w-full py-2 bg-console-bg-hover hover:bg-console-active-bg hover:text-console-active-text text-sm text-console-text-muted font-medium rounded-lg transition-colors border border-transparent hover:border-console-border">
            查看所有告警
          </button>
        </div>
      </div>
    </>
  );
};

const MetricCard = ({ title, value, trend, isUp, icon: Icon, color }: any) => {
  const colorMap: Record<string, string> = {
    blue: "text-blue-600 bg-blue-50",
    indigo: "text-indigo-600 bg-indigo-50",
    emerald: "text-emerald-600 bg-emerald-50",
    amber: "text-amber-600 bg-amber-50"
  };

  return (
    <div className="bg-console-bg-panel p-5 rounded-2xl border border-console-border shadow-sm flex flex-col">
      <div className="flex justify-between items-start mb-4">
        <div className={cn("p-2.5 rounded-xl border border-console-border-light", colorMap[color])}>
          <Icon size={20} />
        </div>
        <div className={cn(
          "px-2 py-1 rounded-full text-xs font-semibold",
          isUp ? "bg-emerald-50 text-emerald-600" : "bg-rose-50 text-rose-600",
          trend === '安全' && "bg-gray-100 text-gray-600"
        )}>
          {trend}
        </div>
      </div>
      <div className="flex flex-col">
        <span className="text-[28px] font-bold text-console-text-main leading-none mb-1">{value}</span>
        <span className="text-sm text-console-text-muted font-medium">{title}</span>
      </div>
    </div>
  );
};

const AlertItem = ({ type, message, time }: any) => {
  const typeMap: Record<string, { icon: any, color: string, bg: string }> = {
    high: { icon: Shield, color: 'text-rose-600', bg: 'bg-rose-50' },
    medium: { icon: Activity, color: 'text-amber-600', bg: 'bg-amber-50' },
    low: { icon: Key, color: 'text-blue-600', bg: 'bg-blue-50' },
    info: { icon: Bell, color: 'text-gray-500', bg: 'bg-gray-100' }
  };
  const t = typeMap[type];
  const Icon = t.icon;

  return (
    <div className="flex gap-3 items-start group">
      <div className={cn("mt-0.5 p-1.5 rounded-full shrink-0", t.bg, t.color)}>
        <Icon size={14} />
      </div>
      <div>
        <p className="text-sm text-console-text-main leading-tight group-hover:text-console-text-main transition-colors opacity-90 group-hover:opacity-100">{message}</p>
        <p className="text-[11px] text-console-text-muted mt-1">{time}</p>
      </div>
    </div>
  );
};
