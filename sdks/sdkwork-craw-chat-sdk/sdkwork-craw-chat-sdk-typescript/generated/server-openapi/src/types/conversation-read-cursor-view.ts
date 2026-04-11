export interface ConversationReadCursorView {
  tenantId: string;
  conversationId: string;
  memberId: string;
  principalId: string;
  readSeq: number;
  lastReadMessageId?: string;
  updatedAt: string;
  unreadCount: number;
}
