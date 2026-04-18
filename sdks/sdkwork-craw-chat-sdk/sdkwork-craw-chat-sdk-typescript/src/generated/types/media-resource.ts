import type { MediaResourceType } from './media-resource-type.js';
import type { StringMap } from './string-map.js';

export interface MediaResource {
  id?: number;
  uuid?: string;
  url?: string;
  bytes?: number[];
  localFile?: string;
  base64?: string;
  type?: MediaResourceType;
  mimeType?: string;
  size?: number;
  name?: string;
  extension?: string;
  tags?: StringMap;
  metadata?: StringMap;
  prompt?: string;
}
