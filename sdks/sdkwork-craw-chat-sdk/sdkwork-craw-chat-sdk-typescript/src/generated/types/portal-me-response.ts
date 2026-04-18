import type { PortalUserView } from './portal-user-view.js';
import type { PortalWorkspaceView } from './portal-workspace-view.js';

export interface PortalMeResponse {
  tenantId: string;
  user: PortalUserView;
  workspace?: PortalWorkspaceView;
}
