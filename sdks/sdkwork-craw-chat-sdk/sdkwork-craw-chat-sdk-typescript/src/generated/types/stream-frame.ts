import type { Sender } from './sender.js';
import type { StringMap } from './string-map.js';

export interface StreamFrame {
  tenantId: string;
  streamId: string;
  streamType: string;
  scopeKind: string;
  scopeId: string;
  frameSeq: number;
  frameType: string;
  schemaRef?: string;
  encoding: string;
  payload: string;
  sender: Sender;
  attributes: StringMap;
  occurredAt: string;
}
