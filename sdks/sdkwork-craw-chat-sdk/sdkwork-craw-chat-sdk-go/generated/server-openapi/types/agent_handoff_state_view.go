package types


type AgentHandoffStateView struct {
	TenantId string `json:"tenantId"`
	ConversationId string `json:"conversationId"`
	Status string `json:"status"`
	Source ChangeAgentHandoffStatusView `json:"source"`
	Target ChangeAgentHandoffStatusView `json:"target"`
	HandoffSessionId string `json:"handoffSessionId"`
	HandoffReason string `json:"handoffReason"`
	AcceptedAt string `json:"acceptedAt"`
	AcceptedBy ChangeAgentHandoffStatusView `json:"acceptedBy"`
	ResolvedAt string `json:"resolvedAt"`
	ResolvedBy ChangeAgentHandoffStatusView `json:"resolvedBy"`
	ClosedAt string `json:"closedAt"`
	ClosedBy ChangeAgentHandoffStatusView `json:"closedBy"`
}
