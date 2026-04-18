import type { RealtimeEvent } from './realtime-event.js';

export interface RealtimeEventWindow {
  deviceId: string;
  items: RealtimeEvent[];
  nextAfterSeq?: number;
  hasMore: boolean;
  ackedThroughSeq: number;
  trimmedThroughSeq: number;
}
