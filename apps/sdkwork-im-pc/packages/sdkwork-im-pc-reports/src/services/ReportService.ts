export interface ReportItem {
  id: string;
  type: 'daily' | 'weekly' | 'monthly';
  content: string;
  plan: string;
  author: string;
  date: string;
  hasRead: boolean;
}

export const reportService = {
  getReports: async (): Promise<ReportItem[]> => {
    return [
      {
        id: 'RPT-001',
        type: 'daily',
        content: '1. 完成了审批中心前端页面的开发与联调。\n2. 修复了聊天界面的若干UI Bug。',
        plan: '1. 开始开发汇报功能模块。\n2. 参与下午的需求评审会。',
        author: '李四',
        date: '今天 18:30',
        hasRead: false
      },
      {
        id: 'RPT-002',
        type: 'weekly',
        content: '本周核心指标进展顺利，累计完成5个核心模块的代码重构，系统性能提升约15%。',
        plan: '下周计划进行压力测试，并着手准备发布V2.0版本。',
        author: '前端组长',
        date: '昨天 17:45',
        hasRead: true
      }
    ];
  },

  submitReport: async (type: 'daily'|'weekly'|'monthly', content: string, plan: string): Promise<ReportItem> => {
    return {
      id: `RPT-NEW-${Date.now()}`,
      type,
      content,
      plan: plan || '无',
      author: '我',
      date: '刚刚',
      hasRead: true
    };
  }
};
