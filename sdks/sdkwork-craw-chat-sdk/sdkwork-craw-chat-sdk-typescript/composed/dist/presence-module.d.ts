import type { PresenceDeviceRequest, PresenceSnapshotView } from './types.js';
import type { CrawChatSdkContext } from './sdk-context.js';
export declare class CrawChatPresenceModule {
    private readonly context;
    constructor(context: CrawChatSdkContext);
    heartbeat(body: PresenceDeviceRequest): Promise<PresenceSnapshotView>;
    current(): Promise<PresenceSnapshotView>;
}
//# sourceMappingURL=presence-module.d.ts.map