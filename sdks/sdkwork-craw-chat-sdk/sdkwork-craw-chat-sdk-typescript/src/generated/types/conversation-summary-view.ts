import type { ConversationAgentHandoffView } from './conversation-agent-handoff-view.js';
import type { SummarySenderView } from './summary-sender-view.js';

export interface ConversationSummaryView {
  tenantId: string;
  conversationId: string;
  messageCount: number;
  lastMessageId?: string;
  lastMessageSeq: number;
  lastSenderId?: string;
  lastSenderKind?: string;
  lastSender?: SummarySenderView;
  lastSummary?: string;
  lastMessageAt?: string;
  agentHandoff?: ConversationAgentHandoffView;
}
