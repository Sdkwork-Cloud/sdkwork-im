export interface TimelineViewEntry {
  tenantId: string;
  conversationId: string;
  messageId: string;
  messageSeq: number;
  summary?: string;
}
