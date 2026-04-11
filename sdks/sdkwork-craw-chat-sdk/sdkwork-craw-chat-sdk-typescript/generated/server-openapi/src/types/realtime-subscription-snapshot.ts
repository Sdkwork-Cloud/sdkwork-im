import type { RealtimeSubscription } from './realtime-subscription';

export interface RealtimeSubscriptionSnapshot {
  tenantId: string;
  principalId: string;
  deviceId: string;
  items: RealtimeSubscription[];
  syncedAt: string;
}
