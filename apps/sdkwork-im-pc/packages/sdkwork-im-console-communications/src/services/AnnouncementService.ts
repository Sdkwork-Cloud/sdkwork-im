export interface Announcement {
  id: number;
  title: string;
  status: 'published' | 'draft';
  date: string;
  views: number;
  sender: string;
}

export interface GetAnnouncementsResponse {
  data: Announcement[];
  total: number;
  publishedCount: number;
  viewsCount: number;
  draftCount: number;
}

class AnnouncementService {
  private mockAnnouncements: Announcement[] = [
    { id: 1, title: '2024年春节假期安排与值班通知', status: 'published', date: '2024-01-20', views: 1205, sender: 'HR 部门' },
    { id: 2, title: '关于系统服务器升级停机维护的公告', status: 'published', date: '2023-11-15', views: 890, sender: 'IT 支持' },
    { id: 3, title: 'Q3 季度全员表彰大会议程', status: 'draft', date: '待发布', views: 0, sender: '总裁办' },
    { id: 4, title: '新版员工手册与合规要求下发', status: 'published', date: '2023-09-01', views: 1420, sender: '法务组' },
  ];

  async getAnnouncements(): Promise<GetAnnouncementsResponse> {
    await new Promise(resolve => setTimeout(resolve, 200));
    
    return {
      data: this.mockAnnouncements,
      total: this.mockAnnouncements.length,
      publishedCount: 142,
      viewsCount: 15420,
      draftCount: 3
    };
  }
}

export const announcementService = new AnnouncementService();
