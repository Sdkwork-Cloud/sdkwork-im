import type { RealtimeSubscriptionItemInput } from './realtime-subscription-item-input';

export interface SyncRealtimeSubscriptionsRequest {
  deviceId?: string;
  items?: RealtimeSubscriptionItemInput[];
}
