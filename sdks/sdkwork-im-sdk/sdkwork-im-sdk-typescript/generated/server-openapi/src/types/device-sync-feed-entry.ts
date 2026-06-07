export interface DeviceSyncFeedEntry {
  tenantId: string;
  principalId: string;
  principalKind: string;
  deviceId?: string | null;
  syncSeq: number;
  eventId: string;
  originEventType: string;
  actorId?: string | null;
  conversationId?: string | null;
  messageId?: string | null;
  messageSeq?: number | null;
  payload?: string | null;
  readSeq?: number | null;
  summary?: string | null;
  occurredAt: string;
}
