package types


type ConversationAgentHandoffView struct {
	Status string `json:"status"`
	Source ConversationActorView `json:"source"`
	Target ConversationActorView `json:"target"`
	HandoffSessionId string `json:"handoffSessionId"`
	HandoffReason string `json:"handoffReason"`
	AcceptedAt string `json:"acceptedAt"`
	AcceptedBy ConversationActorView `json:"acceptedBy"`
	ResolvedAt string `json:"resolvedAt"`
	ResolvedBy ConversationActorView `json:"resolvedBy"`
	ClosedAt string `json:"closedAt"`
	ClosedBy ConversationActorView `json:"closedBy"`
}
