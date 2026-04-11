import { portalMockData } from '../../mockData.js';

function clone(value) {
  return typeof structuredClone === 'function'
    ? structuredClone(value)
    : JSON.parse(JSON.stringify(value));
}

function delay(value) {
  return Promise.resolve(clone(value));
}

export const mockPortalDataSource = {
  async loginPortalUser() {
    return delay({
      token: portalMockData.session.token,
      user: portalMockData.session.user,
    });
  },
  async bootstrapPortalSession(token) {
    if (token !== portalMockData.session.token) {
      return delay(null);
    }

    return delay({
      token,
      user: portalMockData.session.user,
    });
  },
  async getPortalWorkspace() {
    return delay(portalMockData.workspace);
  },
  async getPortalHome() {
    return delay(portalMockData.home);
  },
  async getPortalAuth() {
    return delay(portalMockData.auth);
  },
  async getPortalDashboard() {
    return delay(portalMockData.dashboard);
  },
  async getPortalConversationsBoard() {
    return delay(portalMockData.conversations);
  },
  async getPortalRealtimeBoard() {
    return delay(portalMockData.realtime);
  },
  async getPortalMediaBoard() {
    return delay(portalMockData.media);
  },
  async getPortalAutomationBoard() {
    return delay(portalMockData.automation);
  },
  async getPortalGovernanceBoard() {
    return delay(portalMockData.governance);
  },
};
