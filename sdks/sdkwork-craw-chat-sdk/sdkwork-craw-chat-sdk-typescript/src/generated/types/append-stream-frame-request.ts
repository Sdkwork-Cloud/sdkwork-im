import type { StringMap } from './string-map.js';

export interface AppendStreamFrameRequest {
  frameSeq: number;
  frameType: string;
  schemaRef?: string;
  encoding: string;
  payload: string;
  attributes?: StringMap;
}
