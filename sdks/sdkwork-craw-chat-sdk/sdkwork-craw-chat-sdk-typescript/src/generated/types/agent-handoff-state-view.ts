import type { ChangeAgentHandoffStatusView } from './change-agent-handoff-status-view.js';

export interface AgentHandoffStateView {
  tenantId: string;
  conversationId: string;
  status: string;
  source: ChangeAgentHandoffStatusView;
  target: ChangeAgentHandoffStatusView;
  handoffSessionId: string;
  handoffReason?: string;
  acceptedAt?: string;
  acceptedBy?: ChangeAgentHandoffStatusView;
  resolvedAt?: string;
  resolvedBy?: ChangeAgentHandoffStatusView;
  closedAt?: string;
  closedBy?: ChangeAgentHandoffStatusView;
}
