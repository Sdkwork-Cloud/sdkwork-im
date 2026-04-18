import type { RealtimeSubscriptionItemInput } from './realtime-subscription-item-input.js';

export interface SyncRealtimeSubscriptionsRequest {
  deviceId?: string;
  items?: RealtimeSubscriptionItemInput[];
}
