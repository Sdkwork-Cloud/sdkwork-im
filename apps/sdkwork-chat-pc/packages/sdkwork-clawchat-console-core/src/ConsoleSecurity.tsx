import React from 'react';
import { ShieldAlert, FileText, ScrollText, Key, Lock, Bell, ChevronRight, Activity, Download } from 'lucide-react';
import { cn } from '@sdkwork/clawchat-pc-commons';

export const ConsoleSecurity = () => {
  return (
    <div className="space-y-6">
      {/* Header */}
      <div>
        <h2 className="text-lg font-bold text-console-text-main">安全合规</h2>
        <p className="text-sm text-console-text-muted mt-1">全局安全策略、合规审计、敏感词及防泄漏管控</p>
      </div>

      <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
        {/* Security Score */}
        <div className="bg-console-bg-panel border border-console-border rounded-2xl p-6 shadow-sm flex items-center gap-6">
          <div className="relative w-24 h-24 flex items-center justify-center shrink-0">
            <svg className="w-full h-full transform -rotate-90" viewBox="0 0 36 36">
              <path
                className="text-console-border stroke-current"
                strokeWidth="3"
                fill="none"
                d="M18 2.0845 a 15.9155 15.9155 0 0 1 0 31.831 a 15.9155 15.9155 0 0 1 0 -31.831"
              />
              <path
                className="text-emerald-500 stroke-current"
                strokeWidth="3"
                strokeDasharray="92, 100"
                strokeLinecap="round"
                fill="none"
                d="M18 2.0845 a 15.9155 15.9155 0 0 1 0 31.831 a 15.9155 15.9155 0 0 1 0 -31.831"
              />
            </svg>
            <div className="absolute text-2xl font-bold text-console-text-main">92</div>
          </div>
          <div>
            <h3 className="font-semibold text-console-text-main mb-1">健康得分</h3>
            <p className="text-xs text-console-text-muted mb-3">您的企业即时通信环境安全状况良好。</p>
            <button className="text-xs text-blue-600 font-medium hover:text-blue-700 transition-colors">
              查看优化建议
            </button>
          </div>
        </div>

        {/* Quick Actions */}
        <div className="md:col-span-2 bg-console-bg-panel border border-console-border rounded-2xl p-6 shadow-sm">
          <h3 className="font-semibold text-console-text-main mb-4">快捷管理</h3>
          <div className="grid grid-cols-2 lg:grid-cols-4 gap-4">
            <ActionCard icon={ScrollText} title="审计日志" desc="7天内 1.2M 条记录" />
            <ActionCard icon={Lock} title="敏感词管控" desc="防护触发 452 次" />
            <ActionCard icon={Key} title="登录与认证" desc="SSO 已启用" />
            <ActionCard icon={FileText} title="数据合规导出" desc="无待处理请求" />
          </div>
        </div>
      </div>

      <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
        {/* Compliance Reports */}
        <div className="bg-console-bg-panel border border-console-border rounded-2xl shadow-sm flex flex-col overflow-hidden">
          <div className="p-5 border-b border-console-border flex justify-between items-center bg-console-bg-root/30">
            <h3 className="font-semibold text-console-text-main flex items-center gap-2">
              <ShieldAlert size={18} className="text-indigo-500" />
              威胁与异常拦截
            </h3>
            <button className="text-xs text-blue-600">查看明细</button>
          </div>
          <div className="p-5 flex-1 flex flex-col gap-4">
            <InterceptItem title="敏感内容过滤" count={324} level="warning" />
            <InterceptItem title="异常地点登录阻断" count={12} level="high" />
            <InterceptItem title="恶意文件拦截" count={3} level="critical" />
            <InterceptItem title="高频接口调用限制" count={142} level="info" />
          </div>
        </div>

        {/* Audit Log Preview */}
        <div className="bg-console-bg-panel border border-console-border rounded-2xl shadow-sm flex flex-col overflow-hidden">
          <div className="p-5 border-b border-console-border flex justify-between items-center bg-console-bg-root/30">
            <h3 className="font-semibold text-console-text-main flex items-center gap-2">
              <Activity size={18} className="text-blue-500" />
              关键操作审计日志
            </h3>
            <button className="text-xs text-console-text-muted hover:text-console-text-main flex items-center">
              所有日志 <ChevronRight size={14} />
            </button>
          </div>
          <div className="flex-1 overflow-auto custom-scrollbar">
            <div className="divide-y divide-console-border">
              <LogItem time="10:42:15" user="Admin User" action="导出了全员成员列表" />
              <LogItem time="09:15:02" user="System" action="根据保留策略清理了 12,400 条超期消息" />
              <LogItem time="昨天 18:30" user="Security Bot" action="自动隔离了包含敏感信息的附件 (doc-8812.pdf)" />
              <LogItem time="昨天 14:20" user="Admin User" action="修改了全局登录认证策略 (强制 2FA)" />
              <LogItem time="昨天 11:05" user="张三" action="解散了群组「Q2 渠道沟通」(G-0921)" />
            </div>
          </div>
        </div>
      </div>
    </div>
  );
};

const ActionCard = ({ icon: Icon, title, desc }: any) => (
  <div className="p-4 rounded-xl border border-console-border hover:bg-console-bg-hover transition-colors cursor-pointer group">
    <Icon size={20} className="text-console-text-muted group-hover:text-blue-500 transition-colors mb-3" />
    <h4 className="text-sm font-semibold text-console-text-main">{title}</h4>
    <p className="text-[11px] text-console-text-muted mt-1">{desc}</p>
  </div>
);

const InterceptItem = ({ title, count, level }: any) => {
  const styles = {
    critical: "bg-rose-50 border-rose-200 text-rose-700 dark:bg-rose-500/10 dark:border-rose-500/20 dark:text-rose-400",
    high: "bg-amber-50 border-amber-200 text-amber-700 dark:bg-amber-500/10 dark:border-amber-500/20 dark:text-amber-400",
    warning: "bg-orange-50 border-orange-200 text-orange-700 dark:bg-orange-500/10 dark:border-orange-500/20 dark:text-orange-400",
    info: "bg-blue-50 border-blue-200 text-blue-700 dark:bg-blue-500/10 dark:border-blue-500/20 dark:text-blue-400"
  };
  const currentStyle = styles[level as keyof typeof styles];

  return (
    <div className="flex items-center justify-between">
      <span className="text-sm text-console-text-main">{title}</span>
      <span className={cn("px-2.5 py-0.5 rounded-full text-xs font-medium border", currentStyle)}>
        {count} 次
      </span>
    </div>
  );
};

const LogItem = ({ time, user, action }: any) => (
  <div className="p-4 flex gap-4 hover:bg-console-bg-hover transition-colors">
    <div className="text-xs text-console-text-muted font-mono w-20 shrink-0 mt-0.5">{time}</div>
    <div>
      <p className="text-sm text-console-text-main">
        <span className="font-medium text-console-text-main">{user}</span> {action}
      </p>
    </div>
  </div>
);
