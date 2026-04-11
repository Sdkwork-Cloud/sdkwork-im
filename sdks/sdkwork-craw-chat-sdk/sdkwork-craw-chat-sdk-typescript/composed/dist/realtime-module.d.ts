import type { AckRealtimeEventsRequest, QueryParams, RealtimeAckState, RealtimeEventWindow, RealtimeSubscriptionSnapshot, SyncRealtimeSubscriptionsRequest } from './types.js';
import type { CrawChatSdkContext } from './sdk-context.js';
export declare class CrawChatRealtimeModule {
    private readonly context;
    constructor(context: CrawChatSdkContext);
    replaceSubscriptions(body: SyncRealtimeSubscriptionsRequest): Promise<RealtimeSubscriptionSnapshot>;
    pullEvents(params?: QueryParams): Promise<RealtimeEventWindow>;
    ackEvents(body: AckRealtimeEventsRequest): Promise<RealtimeAckState>;
}
//# sourceMappingURL=realtime-module.d.ts.map