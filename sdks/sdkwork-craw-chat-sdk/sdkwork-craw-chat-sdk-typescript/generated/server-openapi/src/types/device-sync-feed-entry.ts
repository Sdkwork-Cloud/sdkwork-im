export interface DeviceSyncFeedEntry {
  tenantId: string;
  principalId: string;
  deviceId: string;
  syncSeq: number;
  originEventId: string;
  originEventType: string;
  conversationId?: string;
  messageId?: string;
  messageSeq?: number;
  memberId?: string;
  readSeq?: number;
  lastReadMessageId?: string;
  actorId?: string;
  actorKind?: string;
  actorDeviceId?: string;
  summary?: string;
  payloadSchema?: string;
  payload?: string;
  occurredAt: string;
}
