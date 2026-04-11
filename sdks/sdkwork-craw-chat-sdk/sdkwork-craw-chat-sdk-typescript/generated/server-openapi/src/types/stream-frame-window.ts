import type { StreamFrame } from './stream-frame';

export interface StreamFrameWindow {
  items: StreamFrame[];
  nextAfterFrameSeq?: number;
  hasMore: boolean;
}
