import type {
  AckRealtimeEventsRequest,
  QueryParams,
  RealtimeAckState,
  RealtimeEventWindow,
  RealtimeSubscriptionSnapshot,
  SyncRealtimeSubscriptionsRequest,
} from './types.js';
import type { CrawChatSdkContext } from './sdk-context.js';

export class CrawChatRealtimeModule {
  constructor(private readonly context: CrawChatSdkContext) {}

  replaceSubscriptions(
    body: SyncRealtimeSubscriptionsRequest,
  ): Promise<RealtimeSubscriptionSnapshot> {
    return this.context.backendClient.realtime.syncRealtimeSubscriptions(body);
  }

  pullEvents(params?: QueryParams): Promise<RealtimeEventWindow> {
    return this.context.backendClient.realtime.listRealtimeEvents(params);
  }

  ackEvents(body: AckRealtimeEventsRequest): Promise<RealtimeAckState> {
    return this.context.backendClient.realtime.ackRealtimeEvents(body);
  }
}
