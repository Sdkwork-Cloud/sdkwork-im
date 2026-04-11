import type { ContentPart } from './content-part';
import type { StringMap } from './string-map';

export interface MessageBody {
  summary?: string;
  parts: ContentPart[];
  renderHints: StringMap;
}
