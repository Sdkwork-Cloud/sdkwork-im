export interface RealtimeAckState {
  tenantId: string;
  principalId: string;
  deviceId: string;
  ackedThroughSeq: number;
  trimmedThroughSeq: number;
  retainedEventCount: number;
  ackedAt: string;
}
