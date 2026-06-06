export interface MailItem {
  id: string;
  senderName: string;
  senderEmail: string;
  time: string;
  subject: string;
  previewText: string;
  bodyHtml?: string;
  attachments?: { name: string; size: string; type: string }[];
  isRead: boolean;
  isStarred: boolean;
  folder: 'inbox' | 'sent' | 'trash' | 'drafts';
}

export interface MailService {
  getMails(folder: string): Promise<MailItem[]>;
  getMailById(id: string): Promise<MailItem | null>;
  markAsRead(id: string): Promise<void>;
  markAsUnread(id: string): Promise<void>;
  toggleStar(id: string): Promise<void>;
  deleteMail(id: string): Promise<void>;
  sendMail(mail: Partial<MailItem>): Promise<MailItem>;
}

const mockMails: MailItem[] = [
  {
    id: 'm1',
    senderName: '管理员',
    senderEmail: 'admin@sdkwork.com',
    time: '10:42 AM',
    subject: '本周产品研发更新与版本发布',
    previewText: '这里有一份最新的版本规划报告，请查阅附件并对即将上线的模块进行评审...',
    bodyHtml: `
      <p class="mb-4">你好，</p>
      <p class="mb-4">这里有一份最新的版本规划报告，请查阅附件并对即将上线的模块进行评审。</p>
      <p class="mb-4">重点提醒：</p>
      <ul class="list-disc pl-5 mb-4 text-gray-400">
          <li>公证业务模块的流程提交已经整合完毕。</li>
          <li>企业内通信的基础体验优化。</li>
          <li>工作台应用的跳转路由重构。</li>
      </ul>
      <p class="mb-8">我们需要在明天下午准时召开齐步会议，会议链接稍后发送。</p>
      <p class="mb-1 text-gray-400">谢谢，</p>
      <p class="text-gray-400">系统团队</p>
    `,
    attachments: [
      { name: '版本发布总结_v2.pdf', size: '2.4 MB', type: 'pdf' }
    ],
    isRead: false,
    isStarred: false,
    folder: 'inbox'
  },
  {
    id: 'm2',
    senderName: 'HR 团队',
    senderEmail: 'hr@sdkwork.com',
    time: '昨天',
    subject: '关于清明节放假安排的通知',
    previewText: '各位同事，根据国家法定节假日安排，现将清明节放假事项通知如下...',
    bodyHtml: '<p>各位同事，清明节将至，根据国家规定...</p>',
    isRead: true,
    isStarred: true,
    folder: 'inbox'
  },
  {
    id: 'm3',
    senderName: '阿里云',
    senderEmail: 'notification@aliyun.com',
    time: '星期一',
    subject: '您的服务器即将到期',
    previewText: '尊敬的用户，您的云服务器 ECS 将于 7 天后到期，请及时续费。',
    bodyHtml: '<p>尊敬的用户，请及时续费...</p>',
    isRead: true,
    isStarred: false,
    folder: 'inbox'
  }
];

class MockMailService implements MailService {
  async getMails(folder: string): Promise<MailItem[]> {
    return new Promise(resolve => {
      setTimeout(() => {
        resolve(mockMails.filter(m => m.folder === folder || folder === 'all'));
      }, 300);
    });
  }

  async getMailById(id: string): Promise<MailItem | null> {
    return new Promise(resolve => {
      setTimeout(() => {
        resolve(mockMails.find(m => m.id === id) || null);
      }, 200);
    });
  }

  async markAsRead(id: string): Promise<void> {
    return new Promise(resolve => {
      setTimeout(() => {
        const mail = mockMails.find(m => m.id === id);
        if (mail) mail.isRead = true;
        resolve();
      }, 200);
    });
  }

  async markAsUnread(id: string): Promise<void> {
    return new Promise(resolve => {
      setTimeout(() => {
        const mail = mockMails.find(m => m.id === id);
        if (mail) mail.isRead = false;
        resolve();
      }, 200);
    });
  }

  async toggleStar(id: string): Promise<void> {
    return new Promise(resolve => {
      setTimeout(() => {
        const mail = mockMails.find(m => m.id === id);
        if (mail) mail.isStarred = !mail.isStarred;
        resolve();
      }, 200);
    });
  }

  async deleteMail(id: string): Promise<void> {
    return new Promise(resolve => {
      setTimeout(() => {
        const mail = mockMails.find(m => m.id === id);
        if (mail) mail.folder = 'trash';
        resolve();
      }, 200);
    });
  }

  async sendMail(mail: Partial<MailItem>): Promise<MailItem> {
    return new Promise(resolve => {
      setTimeout(() => {
        const newMail: MailItem = {
          id: `m${Date.now()}`,
          senderName: '我',
          senderEmail: 'me@sdkwork.com',
          time: '刚刚',
          subject: mail.subject || '无主题',
          previewText: mail.previewText || mail.bodyHtml?.substring(0, 50) || '',
          bodyHtml: mail.bodyHtml || '',
          isRead: true,
          isStarred: false,
          folder: 'sent',
          ...mail
        };
        mockMails.unshift(newMail);
        resolve(newMail);
      }, 400);
    });
  }
}

export const mailService = new MockMailService();
