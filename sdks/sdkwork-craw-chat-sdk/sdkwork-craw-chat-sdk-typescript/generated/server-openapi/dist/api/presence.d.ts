import type { HttpClient } from '../http/client';
import type { PresenceDeviceRequest, PresenceSnapshotView } from '../types';
export declare class PresenceApi {
    private client;
    constructor(client: HttpClient);
    /** Refresh device presence */
    heartbeat(body: PresenceDeviceRequest): Promise<PresenceSnapshotView>;
    /** Get current presence */
    getPresenceMe(): Promise<PresenceSnapshotView>;
}
export declare function createPresenceApi(client: HttpClient): PresenceApi;
//# sourceMappingURL=presence.d.ts.map