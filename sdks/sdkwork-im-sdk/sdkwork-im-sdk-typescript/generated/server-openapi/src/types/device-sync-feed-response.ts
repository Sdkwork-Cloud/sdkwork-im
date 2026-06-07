import type { DeviceSyncFeedEntry } from './device-sync-feed-entry';

export interface DeviceSyncFeedResponse {
  items: DeviceSyncFeedEntry[];
  nextAfterSeq?: number | null;
  hasMore: boolean;
  trimmedThroughSeq: number;
}
