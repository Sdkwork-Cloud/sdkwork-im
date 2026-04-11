import type { ContentPart } from './content-part';
import type { StringMap } from './string-map';

export interface PostMessageRequest {
  clientMsgId?: string;
  summary?: string;
  text?: string;
  parts?: ContentPart[];
  renderHints?: StringMap;
}
