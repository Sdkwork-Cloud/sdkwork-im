import type { ConversationAgentHandoffView } from './conversation-agent-handoff-view.js';

export interface ConversationInboxEntry {
  tenantId: string;
  principalId: string;
  memberId: string;
  conversationId: string;
  conversationType: string;
  messageCount: number;
  lastMessageId?: string;
  lastMessageSeq: number;
  lastSenderId?: string;
  lastSenderKind?: string;
  lastSummary?: string;
  unreadCount: number;
  lastActivityAt: string;
  agentHandoff?: ConversationAgentHandoffView;
}
