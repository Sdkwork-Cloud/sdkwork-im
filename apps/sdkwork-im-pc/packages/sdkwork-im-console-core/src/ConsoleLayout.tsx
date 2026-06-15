import { ConsoleDashboard } from '@sdkwork/im-console-dashboard';
import React from 'react';
import { 
  LayoutDashboard, Users, MessageSquare, Shield, 
  Settings, Activity, Search, Bell, LogOut, ChartBar,
  Network, Key, Database, CreditCard, Building, Blocks,
  ShieldCheck, FileText, Store, Package, GraduationCap
} from 'lucide-react';
import { cn } from '@sdkwork/im-pc-commons';
import { ConsoleRoles } from '@sdkwork/im-console-roles';
import { TenantUsers } from '@sdkwork/im-console-users';
import { ConsoleGroups, ConsoleMessages, ConsoleAnnouncements } from '@sdkwork/im-console-communications';
import { ConsoleIntegrations } from '@sdkwork/im-console-integrations';
import { ConsoleSecurity, ConsoleAnalytics } from '@sdkwork/im-console-security';
import { ConsoleSettings } from '@sdkwork/im-console-settings';
import { ConsoleStores } from '@sdkwork/im-console-shop';
import { ConsoleProducts } from '@sdkwork/im-console-product';
import { ConsoleCourse } from './ConsoleCourse';
import { Routes, Route, useNavigate, useLocation, Navigate } from 'react-router-dom';

type ConsolePage = 'dashboard' | 'users' | 'roles' | 'groups' | 'messages' | 'stores' | 'products' | 'announcements' | 'integrations' | 'security' | 'analytics' | 'settings';

export const ConsoleLayout: React.FC<{ onSwitchToClient?: () => void }> = ({ onSwitchToClient }) => {
  const navigate = useNavigate();
  const location = useLocation();

  const navItems = [
    { id: 'dashboard', icon: LayoutDashboard, label: '工作台', path: '/console/dashboard' },
    { id: 'users', icon: Users, label: '组织架构与成员', path: '/console/users' },
    { id: 'roles', icon: ShieldCheck, label: '角色与权限', path: '/console/roles' },
    { id: 'groups', icon: MessageSquare, label: '群组与通信管理', path: '/console/groups' },
    { id: 'messages', icon: FileText, label: '消息与内容管理', path: '/console/messages' },
    { id: 'stores', icon: Store, label: '门店与网点管理', path: '/console/stores' },
    { id: 'products', icon: Package, label: '商品与库房管理', path: '/console/products' },
    { id: 'announcements', icon: Bell, label: '租户通知与公告', path: '/console/announcements' },
    { id: 'integrations', icon: Blocks, label: '应用与机器人', path: '/console/integrations' },
    { id: 'security', icon: Shield, label: '安全合规', path: '/console/security' },
    { id: 'analytics', icon: ChartBar, label: '数据分析', path: '/console/analytics' },
    { id: 'course', icon: GraduationCap, label: '课程管理', path: '/console/course' },
    { id: 'settings', icon: Settings, label: '系统设置', path: '/console/settings' },
  ] as const;

  const currentPath = location.pathname.replace(/\/$/, '');
  const activeNavItem = navItems.find(item => item.path === currentPath) || navItems[0];

  return (
    <div className="flex h-screen w-full bg-console-bg-root text-console-text-main font-sans">
      {/* Sidebar */}
      <div className="w-[260px] bg-console-bg-sidebar border-r border-console-border flex flex-col shadow-sm z-10">
        <div className="h-[64px] flex items-center px-6 border-b border-console-border">
          <div className="w-8 h-8 bg-blue-600 rounded-lg flex items-center justify-center text-white font-bold text-lg mr-3 shadow-sm shadow-blue-600/30">
            C
          </div>
          <div>
            <div className="font-bold text-[15px] text-console-text-main leading-tight">Sdkwork IM Console</div>
            <div className="text-[12px] text-console-text-muted">Acme Corp - 租户管理台</div>
          </div>
        </div>

        <div className="flex-1 overflow-y-auto py-4 px-3 flex flex-col gap-1 custom-scrollbar">
          <div className="text-xs font-bold text-console-text-muted px-3 pb-2 uppercase tracking-wider">核心管理</div>
          {navItems.map((item) => (
            <button
              key={item.id}
              onClick={() => navigate(item.path)}
              className={cn(
                "w-full flex items-center gap-3 px-3 py-2.5 rounded-lg text-sm transition-all focus:outline-none",
                activeNavItem.id === item.id 
                  ? "bg-console-active-bg text-console-active-text font-medium" 
                  : "text-console-text-main hover:bg-console-bg-hover hover:text-console-text-main"
              )}
            >
              <item.icon size={18} className={cn(activeNavItem.id === item.id ? "text-console-active-text" : "text-console-text-muted")} />
              {item.label}
            </button>
          ))}
        </div>

        <div className="p-4 border-t border-console-border">
          <button 
            className="w-full flex items-center gap-3 px-3 py-2.5 rounded-lg text-sm text-console-text-main hover:bg-console-bg-hover transition-all focus:outline-none"
            onClick={onSwitchToClient}
          >
            <LogOut size={18} className="text-console-text-muted" />
            返回客户端
          </button>
        </div>
      </div>

      {/* Main Content Area */}
      <div className="flex-1 flex flex-col min-w-0">
        {/* Top Navbar */}
        <header className="h-[64px] bg-console-bg-sidebar border-b border-console-border flex items-center justify-between px-8 shrink-0">
          <h1 className="text-lg font-semibold text-console-text-main">
            {activeNavItem.label}
          </h1>

          <div className="flex items-center gap-5 -mr-2">
            <div className="relative">
              <Search size={16} className="absolute left-3 top-1/2 -translate-y-1/2 text-console-text-muted" />
              <input 
                type="text" 
                placeholder="搜索成员、群组、配置..." 
                className="w-64 bg-console-input-bg border-none rounded-full py-1.5 pl-9 pr-4 text-sm text-console-text-main focus:ring-2 focus:ring-blue-500/50 outline-none transition-shadow"
              />
            </div>
            
            <button className="relative p-2 text-console-text-muted hover:bg-console-bg-hover rounded-full transition-colors focus:outline-none">
              <Bell size={20} />
              <span className="absolute top-1.5 right-1.5 w-2 h-2 bg-red-500 border-2 border-console-bg-sidebar rounded-full"></span>
            </button>

            <div className="w-px h-6 bg-console-border"></div>

            <div className="flex items-center gap-2 cursor-pointer hover:bg-console-bg-hover p-1.5 rounded-lg transition-colors">
              <div className="w-8 h-8 rounded-full bg-blue-100 text-blue-700 flex items-center justify-center font-bold text-sm">
                A
              </div>
              <div className="hidden md:block">
                <div className="text-sm font-medium text-console-text-main leading-none">Admin User</div>
                <div className="text-[11px] text-console-text-muted mt-0.5">超级管理员</div>
              </div>
            </div>
          </div>
        </header>

        {/* Dynamic Page Content */}
        <main className="flex-1 overflow-x-hidden overflow-y-auto bg-console-bg-root p-4 sm:p-6 custom-scrollbar">
          <div className="w-full max-w-full mx-auto h-full flex flex-col">
            <Routes>
              <Route path="/" element={<Navigate to="/console/dashboard" replace />} />
              <Route path="dashboard" element={<ConsoleDashboard />} />
              <Route path="users" element={<TenantUsers />} />
              <Route path="roles" element={<ConsoleRoles />} />
              <Route path="groups" element={<ConsoleGroups />} />
              <Route path="messages" element={<ConsoleMessages />} />
              <Route path="stores" element={<ConsoleStores />} />
              <Route path="products" element={<ConsoleProducts />} />
              <Route path="announcements" element={<ConsoleAnnouncements />} />
              <Route path="integrations" element={<ConsoleIntegrations />} />
              <Route path="security" element={<ConsoleSecurity />} />
              <Route path="analytics" element={<ConsoleAnalytics />} />
              <Route path="course" element={<ConsoleCourse />} />
              <Route path="settings" element={<ConsoleSettings />} />
              <Route path="*" element={
                <div className="w-full h-64 border-2 border-dashed border-console-border rounded-2xl flex flex-col items-center justify-center text-console-text-muted bg-console-bg-panel/50">
                  <Settings size={32} className="text-console-text-muted mb-3" />
                  <p>模块正在开发中</p>
                </div>
              } />
            </Routes>
          </div>
        </main>
      </div>
    </div>
  );
};
