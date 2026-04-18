import type { PortalUserView } from './portal-user-view';
import type { PortalWorkspaceView } from './portal-workspace-view';
export interface PortalMeResponse {
    tenantId: string;
    user: PortalUserView;
    workspace?: PortalWorkspaceView;
}
//# sourceMappingURL=portal-me-response.d.ts.map