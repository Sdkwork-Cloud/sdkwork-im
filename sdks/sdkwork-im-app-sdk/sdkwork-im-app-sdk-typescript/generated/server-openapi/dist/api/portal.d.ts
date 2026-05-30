import type { HttpClient } from '../http/client';
import type { PortalSnapshot, PortalWorkspaceView } from '../types';
export declare class PortalWorkspaceApi {
    private client;
    constructor(client: HttpClient);
    /** Read the current tenant workspace snapshot */
    retrieve(): Promise<PortalWorkspaceView>;
}
export declare class PortalRealtimeApi {
    private client;
    constructor(client: HttpClient);
    /** Read the tenant realtime snapshot */
    retrieve(): Promise<PortalSnapshot>;
}
export declare class PortalMediaApi {
    private client;
    constructor(client: HttpClient);
    /** Read the tenant media snapshot */
    retrieve(): Promise<PortalSnapshot>;
}
export declare class PortalHomeApi {
    private client;
    constructor(client: HttpClient);
    /** Read the tenant portal home snapshot */
    retrieve(): Promise<PortalSnapshot>;
}
export declare class PortalGovernanceApi {
    private client;
    constructor(client: HttpClient);
    /** Read the tenant governance snapshot */
    retrieve(): Promise<PortalSnapshot>;
}
export declare class PortalDashboardApi {
    private client;
    constructor(client: HttpClient);
    /** Read the tenant dashboard snapshot */
    retrieve(): Promise<PortalSnapshot>;
}
export declare class PortalConversationSnapshotApi {
    private client;
    constructor(client: HttpClient);
    /** Read the tenant conversations snapshot */
    retrieve(): Promise<PortalSnapshot>;
}
export declare class PortalAutomationApi {
    private client;
    constructor(client: HttpClient);
    /** Read the tenant automation snapshot */
    retrieve(): Promise<PortalSnapshot>;
}
export declare class PortalAccessApi {
    private client;
    constructor(client: HttpClient);
    /** Read the tenant portal sign-in snapshot */
    retrieve(): Promise<PortalSnapshot>;
}
export declare class PortalApi {
    private client;
    readonly access: PortalAccessApi;
    readonly automation: PortalAutomationApi;
    readonly conversationSnapshot: PortalConversationSnapshotApi;
    readonly dashboard: PortalDashboardApi;
    readonly governance: PortalGovernanceApi;
    readonly home: PortalHomeApi;
    readonly media: PortalMediaApi;
    readonly realtime: PortalRealtimeApi;
    readonly workspace: PortalWorkspaceApi;
    constructor(client: HttpClient);
}
export declare function createPortalApi(client: HttpClient): PortalApi;
//# sourceMappingURL=portal.d.ts.map