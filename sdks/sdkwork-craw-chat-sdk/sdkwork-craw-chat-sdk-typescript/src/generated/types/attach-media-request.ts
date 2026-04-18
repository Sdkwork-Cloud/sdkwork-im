import type { StringMap } from './string-map.js';

export interface AttachMediaRequest {
  conversationId: string;
  clientMsgId?: string;
  summary?: string;
  text?: string;
  renderHints?: StringMap;
}
