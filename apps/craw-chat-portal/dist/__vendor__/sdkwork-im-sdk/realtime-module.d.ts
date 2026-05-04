import type { AckRealtimeEventsRequest, ImRealtimeSubscriptionScopeOptions, QueryParams, RealtimeAckState, RealtimeEventWindow, RealtimeSubscriptionSnapshot, SyncRealtimeSubscriptionsRequest } from './types.js';
import type { ImSdkContext } from './sdk-context.js';
export declare class ImRealtimeModule {
    private readonly context;
    constructor(context: ImSdkContext);
    replaceSubscriptions(body: SyncRealtimeSubscriptionsRequest): Promise<RealtimeSubscriptionSnapshot>;
    replaceScopeSubscriptions(scopeType: string, scopeIds: string | number | Array<string | number>, eventTypes: string[] | undefined, options?: ImRealtimeSubscriptionScopeOptions): Promise<RealtimeSubscriptionSnapshot>;
    replaceConversationSubscriptions(conversationIds: string | number | Array<string | number>, eventTypes: string[] | undefined, options?: ImRealtimeSubscriptionScopeOptions): Promise<RealtimeSubscriptionSnapshot>;
    replaceRtcSubscriptions(rtcSessionIds: string | number | Array<string | number>, eventTypes?: string[], options?: ImRealtimeSubscriptionScopeOptions): Promise<RealtimeSubscriptionSnapshot>;
    catchUpEvents(params?: QueryParams): Promise<RealtimeEventWindow>;
    ackEvents(body: AckRealtimeEventsRequest): Promise<RealtimeAckState>;
}
//# sourceMappingURL=realtime-module.d.ts.map