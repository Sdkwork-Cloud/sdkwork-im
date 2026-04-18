import type { ContentPart } from './content-part.js';
import type { StringMap } from './string-map.js';

export interface PostMessageRequest {
  clientMsgId?: string;
  summary?: string;
  text?: string;
  parts?: ContentPart[];
  renderHints?: StringMap;
}
