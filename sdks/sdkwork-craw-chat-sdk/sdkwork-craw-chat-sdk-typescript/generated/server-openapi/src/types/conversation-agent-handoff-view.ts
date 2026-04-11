import type { ConversationActorView } from './conversation-actor-view';

export interface ConversationAgentHandoffView {
  status: string;
  source: ConversationActorView;
  target: ConversationActorView;
  handoffSessionId: string;
  handoffReason?: string;
  acceptedAt?: string;
  acceptedBy?: ConversationActorView;
  resolvedAt?: string;
  resolvedBy?: ConversationActorView;
  closedAt?: string;
  closedBy?: ConversationActorView;
}
