import type { StreamDurabilityClass } from './stream-durability-class';
import type { StreamSessionState } from './stream-session-state';

export interface StreamSession {
  tenantId: string;
  streamId: string;
  streamType: string;
  scopeKind: string;
  scopeId: string;
  durabilityClass: StreamDurabilityClass;
  orderingScope: string;
  schemaRef?: string;
  state: StreamSessionState;
  lastFrameSeq: number;
  lastCheckpointSeq?: number;
  resultMessageId?: string;
  openedAt: string;
  closedAt?: string;
  expiresAt?: string;
}
