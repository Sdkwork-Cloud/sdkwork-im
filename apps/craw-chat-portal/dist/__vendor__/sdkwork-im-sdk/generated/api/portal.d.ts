import type { HttpClient } from '../http/client.js';
import type { PortalSnapshot, PortalWorkspaceView } from '../types/index.js';
export declare class PortalApi {
    private client;
    constructor(client: HttpClient);
    /** Read the tenant portal home snapshot */
    getHome(): Promise<PortalSnapshot>;
    /** Read the tenant portal sign-in snapshot */
    getAuth(): Promise<PortalSnapshot>;
    /** Read the current tenant workspace snapshot */
    getWorkspace(): Promise<PortalWorkspaceView>;
    /** Read the tenant dashboard snapshot */
    getDashboard(): Promise<PortalSnapshot>;
    /** Read the tenant conversations snapshot */
    getConversations(): Promise<PortalSnapshot>;
    /** Read the tenant realtime snapshot */
    getRealtime(): Promise<PortalSnapshot>;
    /** Read the tenant media snapshot */
    getMedia(): Promise<PortalSnapshot>;
    /** Read the tenant automation snapshot */
    getAutomation(): Promise<PortalSnapshot>;
    /** Read the tenant governance snapshot */
    getGovernance(): Promise<PortalSnapshot>;
}
export declare function createPortalApi(client: HttpClient): PortalApi;
//# sourceMappingURL=portal.d.ts.map