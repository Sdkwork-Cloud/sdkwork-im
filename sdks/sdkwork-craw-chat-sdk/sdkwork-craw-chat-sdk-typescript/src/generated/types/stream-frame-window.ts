import type { StreamFrame } from './stream-frame.js';

export interface StreamFrameWindow {
  items: StreamFrame[];
  nextAfterFrameSeq?: number;
  hasMore: boolean;
}
