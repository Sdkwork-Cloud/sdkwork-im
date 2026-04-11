import type { ContentPart } from './content-part';
import type { StringMap } from './string-map';

export interface EditMessageRequest {
  summary?: string;
  text?: string;
  parts?: ContentPart[];
  renderHints?: StringMap;
}
