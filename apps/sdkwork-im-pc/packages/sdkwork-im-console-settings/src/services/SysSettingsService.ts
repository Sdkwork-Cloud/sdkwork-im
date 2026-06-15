import { mockConsoleFetch, mockConsolePost } from '@sdkwork/im-pc-commons';

export interface SysSettings {
  orgName: string;
  envFlag: string;
  customDomain: string;
  logoText: string; // for the mock logo A
  publishableKey: string;
  secretKey: string;
}

class SysSettingsService {
  private mockData: SysSettings = {
    orgName: 'Acme Corporation',
    envFlag: 'acme-corp',
    customDomain: '',
    logoText: 'A',
    publishableKey: 'pk_live_839x...429a',
    secretKey: 'sk_live_********************'
  };

  async getSettings(): Promise<SysSettings> {
    return mockConsoleFetch('/settings/overview', this.mockData);
  }

  async updateSettings(updates: Partial<SysSettings>): Promise<SysSettings> {
    this.mockData = { ...this.mockData, ...updates };
    return mockConsolePost('/settings/update', updates, this.mockData);
  }
}

export const sysSettingsService = new SysSettingsService();
