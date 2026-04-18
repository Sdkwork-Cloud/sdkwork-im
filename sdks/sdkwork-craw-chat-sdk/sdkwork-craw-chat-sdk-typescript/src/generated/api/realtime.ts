import { backendApiPath } from './paths.js';
import type { HttpClient } from '../http/client.js';
import type { QueryParams } from '../types/common.js';
import type { AckRealtimeEventsRequest, RealtimeAckState, RealtimeEventWindow, RealtimeSubscriptionSnapshot, SyncRealtimeSubscriptionsRequest } from '../types/index.js';


export class RealtimeApi {
  private client: HttpClient;
  
  constructor(client: HttpClient) { 
    this.client = client; 
  }

/** Replace realtime subscriptions for the current device */
  async syncRealtimeSubscriptions(body: SyncRealtimeSubscriptionsRequest): Promise<RealtimeSubscriptionSnapshot> {
    return this.client.post<RealtimeSubscriptionSnapshot>(backendApiPath(`/realtime/subscriptions/sync`), body, undefined, undefined, 'application/json');
  }

/** Pull realtime events for the current device */
  async listRealtimeEvents(params?: QueryParams): Promise<RealtimeEventWindow> {
    return this.client.get<RealtimeEventWindow>(backendApiPath(`/realtime/events`), params);
  }

/** Ack realtime events for the current device */
  async ackRealtimeEvents(body: AckRealtimeEventsRequest): Promise<RealtimeAckState> {
    return this.client.post<RealtimeAckState>(backendApiPath(`/realtime/events/ack`), body, undefined, undefined, 'application/json');
  }
}

export function createRealtimeApi(client: HttpClient): RealtimeApi {
  return new RealtimeApi(client);
}
