import type { ContentPart } from './content-part.js';
import type { StringMap } from './string-map.js';

export interface MessageBody {
  summary?: string;
  parts: ContentPart[];
  renderHints: StringMap;
}
