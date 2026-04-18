package types


type ConversationInboxEntry struct {
	TenantId string `json:"tenantId"`
	PrincipalId string `json:"principalId"`
	MemberId string `json:"memberId"`
	ConversationId string `json:"conversationId"`
	ConversationType string `json:"conversationType"`
	MessageCount int `json:"messageCount"`
	LastMessageId string `json:"lastMessageId"`
	LastMessageSeq int `json:"lastMessageSeq"`
	LastSenderId string `json:"lastSenderId"`
	LastSenderKind string `json:"lastSenderKind"`
	LastSummary string `json:"lastSummary"`
	UnreadCount int `json:"unreadCount"`
	LastActivityAt string `json:"lastActivityAt"`
	AgentHandoff ConversationAgentHandoffView `json:"agentHandoff"`
}
