import type { PortalUserView } from './portal-user-view';
import type { PortalWorkspaceView } from './portal-workspace-view';

export interface PortalLoginResponse {
  accessToken: string;
  refreshToken: string;
  expiresAt: number;
  user: PortalUserView;
  workspace?: PortalWorkspaceView;
}
