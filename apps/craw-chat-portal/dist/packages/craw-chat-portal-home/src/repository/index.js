import { getPortalHome } from '../../../craw-chat-portal-portal-api/src/index.js';

export function readPortalHomeSnapshot() {
  return getPortalHome();
}
