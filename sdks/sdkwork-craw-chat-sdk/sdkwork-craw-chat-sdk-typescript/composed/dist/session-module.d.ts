import type { PresenceDeviceRequest, PresenceSnapshotView, ResumeSessionRequest, SessionResumeView } from './types.js';
import type { CrawChatSdkContext } from './sdk-context.js';
export declare class CrawChatSessionModule {
    private readonly context;
    constructor(context: CrawChatSdkContext);
    resume(body: ResumeSessionRequest): Promise<SessionResumeView>;
    disconnectDevice(body: PresenceDeviceRequest): Promise<PresenceSnapshotView>;
}
//# sourceMappingURL=session-module.d.ts.map