import type { StringMap } from './string-map';

export interface AppendStreamFrameRequest {
  frameSeq: number;
  frameType: string;
  schemaRef?: string;
  encoding: string;
  payload: string;
  attributes?: StringMap;
}
