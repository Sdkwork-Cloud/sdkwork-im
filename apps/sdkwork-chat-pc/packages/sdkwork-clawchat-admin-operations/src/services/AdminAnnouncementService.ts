import { mockAdminFetch } from '@sdkwork/clawchat-pc-commons';

export interface AdminAnnouncement {
  id: string;
  title: string;
  date: string;
  target: string;
  views: string;
  status: 'delivered' | 'scheduled' | 'draft';
  tag: string;
}

class AdminAnnouncementService {
  async getAnnouncements(): Promise<AdminAnnouncement[]> {
    const mockData: AdminAnnouncement[] = [
      { id: '1', title: "Critical Server Maintenance Window: APAC Region", date: "Oct 20, 2023", target: "All paid tenants in APAC-1, APAC-2", views: "1,402", status: "delivered", tag: "Ops" },
      { id: '2', title: "New Feature Release: AI Summarizations & DLP Enhanced", date: "Oct 15, 2023", target: "Enterprise Grid Tenants Only", views: "420", status: "delivered", tag: "Release" },
      { id: '3', title: "Policy Update: API Rate Limit Adjustments for Free Tier", date: "Oct 01, 2023", target: "All Free Tier Tenants", views: "4,105", status: "delivered", tag: "Policy" },
    ];
    return mockAdminFetch('/announcements/list', mockData);
  }
}

export const adminAnnouncementService = new AdminAnnouncementService();
