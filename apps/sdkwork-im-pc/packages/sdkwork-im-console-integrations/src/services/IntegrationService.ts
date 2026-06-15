export interface IntegrationApp {
  id: string;
  name: string;
  type: string;
  desc: string;
  color: string;
  iconType: 'Puzzle' | 'Webhook' | 'Bot';
  status: 'active' | 'disabled';
}

export interface GetAppsResponse {
  data: IntegrationApp[];
  total: number;
}

class IntegrationService {
  private mockApps: IntegrationApp[] = [
    { id: '1', name: 'Jira Server', type: '内部应用', desc: 'Jira 任务生命周期追踪机器人，支持在群聊中更新任务。', color: 'bg-blue-500', iconType: 'Puzzle', status: 'active' },
    { id: '2', name: 'GitLab Notify', type: 'Webhook', desc: '监控代码库提交记录并推送到研发协作群组。', color: 'bg-orange-500', iconType: 'Webhook', status: 'active' },
    { id: '3', name: 'HR审批助手', type: '自建机器人', desc: '提供请假单审批以及自动化HR消息推送。', color: 'bg-emerald-500', iconType: 'Bot', status: 'active' },
    { id: '4', name: 'Design Feedback', type: '第三方集成', desc: 'Figma 协作设计反馈同步机器人。', color: 'bg-rose-500', iconType: 'Puzzle', status: 'disabled' },
  ];

  async getApps(params: { search?: string; status?: string }): Promise<GetAppsResponse> {
    await new Promise(resolve => setTimeout(resolve, 200));

    let filtered = this.mockApps;
    if (params.search) {
      const q = params.search.toLowerCase();
      filtered = filtered.filter(a => a.name.toLowerCase().includes(q));
    }
    if (params.status && params.status !== 'all') {
      filtered = filtered.filter(a => a.status === params.status);
    }

    return {
      data: filtered,
      total: filtered.length
    };
  }
}

export const integrationService = new IntegrationService();
