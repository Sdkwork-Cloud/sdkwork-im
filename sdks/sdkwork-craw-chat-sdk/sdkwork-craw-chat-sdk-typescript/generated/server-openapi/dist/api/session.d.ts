import type { HttpClient } from '../http/client';
import type { PresenceDeviceRequest, PresenceSnapshotView, ResumeSessionRequest, SessionResumeView } from '../types';
export declare class SessionApi {
    private client;
    constructor(client: HttpClient);
    /** Resume the current app session */
    resume(body: ResumeSessionRequest): Promise<SessionResumeView>;
    /** Disconnect the current app session device route */
    disconnect(body: PresenceDeviceRequest): Promise<PresenceSnapshotView>;
}
export declare function createSessionApi(client: HttpClient): SessionApi;
//# sourceMappingURL=session.d.ts.map