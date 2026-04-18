import type { PortalUserView } from './portal-user-view.js';
import type { PortalWorkspaceView } from './portal-workspace-view.js';

export interface PortalLoginResponse {
  accessToken: string;
  refreshToken: string;
  expiresAt: number;
  user: PortalUserView;
  workspace?: PortalWorkspaceView;
}
