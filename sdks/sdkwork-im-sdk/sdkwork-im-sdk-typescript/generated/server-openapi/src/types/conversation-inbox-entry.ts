export interface ConversationInboxEntry {
  tenantId: string;
  conversationId: string;
  agentHandoff?: boolean;
  conversationType: string;
  lastActivityAt: string;
  lastMessageId?: string | null;
  lastSenderId?: string | null;
  messageCount: number;
  lastMessageSeq: number;
  lastSummary?: string | null;
  lastMessageAt?: string;
  unreadCount: number;
}
