import { mockAdminFetch, mockAdminPost } from '@sdkwork/clawchat-pc-commons';

export interface AdminSettingsData {
  platformName: string;
  supportContact: string;
  allowSelfService: boolean;
}

class AdminSettingsService {
  private mockData: AdminSettingsData = {
    platformName: "ClawChat Cloud",
    supportContact: "noc@sdkwork.com",
    allowSelfService: true,
  };

  async getSettings(): Promise<AdminSettingsData> {
    return mockAdminFetch('/settings/overview', this.mockData);
  }

  async updateSettings(updates: Partial<AdminSettingsData>): Promise<void> {
    this.mockData = { ...this.mockData, ...updates };
    return mockAdminPost('/settings/update', updates, undefined);
  }
}

export const adminSettingsService = new AdminSettingsService();
