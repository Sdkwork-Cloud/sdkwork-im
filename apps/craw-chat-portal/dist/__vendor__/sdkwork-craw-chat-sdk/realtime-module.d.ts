import type { AckRealtimeEventsRequest, CrawChatRealtimeSubscriptionScopeOptions, QueryParams, RealtimeAckState, RealtimeEventWindow, RealtimeSubscriptionSnapshot, SyncRealtimeSubscriptionsRequest } from './types.js';
import type { CrawChatSdkContext } from './sdk-context.js';
export declare class CrawChatRealtimeModule {
    private readonly context;
    constructor(context: CrawChatSdkContext);
    replaceSubscriptions(body: SyncRealtimeSubscriptionsRequest): Promise<RealtimeSubscriptionSnapshot>;
    replaceScopeSubscriptions(scopeType: string, scopeIds: string | number | Array<string | number>, eventTypes: string[] | undefined, options?: CrawChatRealtimeSubscriptionScopeOptions): Promise<RealtimeSubscriptionSnapshot>;
    replaceConversationSubscriptions(conversationIds: string | number | Array<string | number>, eventTypes: string[] | undefined, options?: CrawChatRealtimeSubscriptionScopeOptions): Promise<RealtimeSubscriptionSnapshot>;
    replaceRtcSubscriptions(rtcSessionIds: string | number | Array<string | number>, eventTypes?: string[], options?: CrawChatRealtimeSubscriptionScopeOptions): Promise<RealtimeSubscriptionSnapshot>;
    pullEvents(params?: QueryParams): Promise<RealtimeEventWindow>;
    ackEvents(body: AckRealtimeEventsRequest): Promise<RealtimeAckState>;
}
//# sourceMappingURL=realtime-module.d.ts.map