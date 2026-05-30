import type { StringMap } from './string-map';

export interface AppendAgentResponseDeltaRequest {
  frameSeq: number;
  frameType: string;
  schemaRef?: string;
  encoding: string;
  payload: string;
  attributes?: StringMap;
}
