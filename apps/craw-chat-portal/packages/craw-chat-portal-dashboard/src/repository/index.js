import { getPortalDashboard } from '../../../craw-chat-portal-portal-api/src/index.js';

export function readPortalDashboardSnapshot() {
  return getPortalDashboard();
}
