import type { HttpClient } from '../http/client';
import type { QueryParams } from '../types/common';
import type { AckRealtimeEventsRequest, RealtimeAckState, RealtimeEventWindow, RealtimeSubscriptionSnapshot, SyncRealtimeSubscriptionsRequest } from '../types';
export declare class RealtimeApi {
    private client;
    constructor(client: HttpClient);
    /** Replace realtime subscriptions for the current device */
    syncRealtimeSubscriptions(body: SyncRealtimeSubscriptionsRequest): Promise<RealtimeSubscriptionSnapshot>;
    /** Pull realtime events for the current device */
    listRealtimeEvents(params?: QueryParams): Promise<RealtimeEventWindow>;
    /** Ack realtime events for the current device */
    ackRealtimeEvents(body: AckRealtimeEventsRequest): Promise<RealtimeAckState>;
}
export declare function createRealtimeApi(client: HttpClient): RealtimeApi;
//# sourceMappingURL=realtime.d.ts.map