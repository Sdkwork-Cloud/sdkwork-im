import { mockConsoleFetch } from '@sdkwork/clawchat-pc-commons';

export interface SecurityIntercept {
  id: string;
  title: string;
  count: number;
  level: 'critical' | 'high' | 'warning' | 'info';
}

export interface SecurityAuditLog {
  id: string;
  time: string;
  user: string;
  action: string;
}

export interface SecurityDashboardData {
  healthScore: number;
  intercepts: SecurityIntercept[];
  auditLogs: SecurityAuditLog[];
}

class SecurityService {
  async getDashboardData(): Promise<SecurityDashboardData> {
    const mockData: SecurityDashboardData = {
      healthScore: 92,
      intercepts: [
        { id: '1', title: '敏感内容过滤', count: 324, level: 'warning' as const },
        { id: '2', title: '异常地点登录阻断', count: 12, level: 'high' as const },
        { id: '3', title: '恶意文件拦截', count: 3, level: 'critical' as const },
        { id: '4', title: '高频接口调用限制', count: 142, level: 'info' as const }
      ],
      auditLogs: [
        { id: '1', time: '10:42:15', user: 'Admin User', action: '导出了全员成员列表' },
        { id: '2', time: '09:15:02', user: 'System', action: '根据保留策略清理了 12,400 条超期消息' },
        { id: '3', time: '昨天 18:30', user: 'Security Bot', action: '自动隔离了包含敏感信息的附件 (doc-8812.pdf)' },
        { id: '4', time: '昨天 14:20', user: 'Admin User', action: '修改了全局登录认证策略 (强制 2FA)' },
        { id: '5', time: '昨天 11:05', user: '张三', action: '解散了群组「Q2 渠道沟通」(G-0921)' }
      ]
    };
    return mockConsoleFetch('/security/dashboard', mockData);
  }
}

export const securityService = new SecurityService();
