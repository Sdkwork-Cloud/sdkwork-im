export interface AuditMessage {
  id: string;
  time: string;
  sender: string;
  receiver: string;
  snippet: string;
  alert: boolean;
}

export interface GetAuditMessagesResponse {
  data: AuditMessage[];
  total: number;
}

class MessageAuditService {
  private mockMessages: AuditMessage[] = [
    { id: '1', time: '10:45:22', sender: '李四', receiver: 'Q3 项目作战室', snippet: '我们这边的生产环境 token 是 sk_test_...', alert: true },
    { id: '2', time: '10:40:11', sender: '王五', receiver: '张三', snippet: '附件：财务报表-Q3.xlsx', alert: false },
    { id: '3', time: '09:12:05', sender: 'Admin', receiver: '全员群 (System)', snippet: '关于元旦放假安排的通知', alert: false },
  ];

  async getMessages(params: { page: number; pageSize: number; search?: string }): Promise<GetAuditMessagesResponse> {
    await new Promise(resolve => setTimeout(resolve, 200));
    let filtered = this.mockMessages;
    if (params.search) {
      const q = params.search.toLowerCase();
      filtered = filtered.filter(m => 
        m.sender.toLowerCase().includes(q) || 
        m.receiver.toLowerCase().includes(q) ||
        m.snippet.toLowerCase().includes(q)
      );
    }
    const start = (params.page - 1) * params.pageSize;
    const end = start + params.pageSize;

    return {
      data: filtered.slice(start, end),
      total: filtered.length
    };
  }
}

export const messageAuditService = new MessageAuditService();
