import type {
  AckRealtimeEventsRequest,
  CrawChatRealtimeSubscriptionScopeOptions,
  QueryParams,
  RealtimeAckState,
  RealtimeEventWindow,
  RealtimeSubscriptionItemInput,
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

  replaceScopeSubscriptions(
    scopeType: string,
    scopeIds: string | number | Array<string | number>,
    eventTypes: string[] | undefined,
    options: CrawChatRealtimeSubscriptionScopeOptions = {},
  ): Promise<RealtimeSubscriptionSnapshot> {
    const items = normalizeScopeIds(scopeIds).map(
      (scopeId): RealtimeSubscriptionItemInput => ({
        scopeType,
        scopeId: String(scopeId),
        eventTypes,
      }),
    );

    return this.replaceSubscriptions({
      deviceId: options.deviceId,
      items,
    });
  }

  replaceConversationSubscriptions(
    conversationIds: string | number | Array<string | number>,
    eventTypes: string[] | undefined,
    options: CrawChatRealtimeSubscriptionScopeOptions = {},
  ): Promise<RealtimeSubscriptionSnapshot> {
    return this.replaceScopeSubscriptions(
      'conversation',
      conversationIds,
      eventTypes,
      options,
    );
  }

  replaceRtcSubscriptions(
    rtcSessionIds: string | number | Array<string | number>,
    eventTypes: string[] = ['rtc.signal'],
    options: CrawChatRealtimeSubscriptionScopeOptions = {},
  ): Promise<RealtimeSubscriptionSnapshot> {
    return this.replaceScopeSubscriptions(
      'rtc_session',
      rtcSessionIds,
      eventTypes,
      options,
    );
  }

  pullEvents(params?: QueryParams): Promise<RealtimeEventWindow> {
    return this.context.backendClient.realtime.listRealtimeEvents(params);
  }

  ackEvents(body: AckRealtimeEventsRequest): Promise<RealtimeAckState> {
    return this.context.backendClient.realtime.ackRealtimeEvents(body);
  }
}

function normalizeScopeIds(
  value: string | number | Array<string | number>,
): Array<string | number> {
  return Array.isArray(value) ? value : [value];
}
