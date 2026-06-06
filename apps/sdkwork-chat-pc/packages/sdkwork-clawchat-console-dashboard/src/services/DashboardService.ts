export interface DashboardMetrics {
  totalUsers: { value: string; trend: string; isUp: boolean };
  dailyMessages: { value: string; trend: string; isUp: boolean };
  activeGroups: { value: string; trend: string; isUp: boolean };
  storageUsage: { value: string; trend: string; isUp: boolean };
}

export interface ActivityTrend {
  day: string;
  value: number; // percentage height
}

export interface SecurityAlert {
  id: string;
  type: 'high' | 'medium' | 'low' | 'info';
  message: string;
  time: string;
}

class DashboardService {
  async getMetrics(): Promise<DashboardMetrics> {
    await new Promise(resolve => setTimeout(resolve, 200));
    return {
      totalUsers: { value: '12,450', trend: '+5.2%', isUp: true },
      dailyMessages: { value: '1.2M', trend: '+12.4%', isUp: true },
      activeGroups: { value: '3,842', trend: '-2.1%', isUp: false },
      storageUsage: { value: '4.2 TB', trend: '安全', isUp: true }
    };
  }

  async getActivityTrends(period: string): Promise<ActivityTrend[]> {
    await new Promise(resolve => setTimeout(resolve, 200));
    return [
      { day: '一', value: 40 },
      { day: '二', value: 65 },
      { day: '三', value: 45 },
      { day: '四', value: 80 },
      { day: '五', value: 55 },
      { day: '六', value: 90 },
      { day: '日', value: 70 },
    ];
  }

  async getSecurityAlerts(): Promise<SecurityAlert[]> {
    await new Promise(resolve => setTimeout(resolve, 200));
    return [
      { id: '1', type: 'high', message: '检测到异常登录地点 (IP: 182.xx.xx.xx)', time: '10分钟前' },
      { id: '2', type: 'medium', message: '大量文件下载行为触发表', time: '2小时前' },
      { id: '3', type: 'low', message: 'API 速率达到警戒值 (80%)', time: '5小时前' },
      { id: '4', type: 'info', message: '本周安全巡检报告已生成', time: '昨天' }
    ];
  }
}

export const dashboardService = new DashboardService();
