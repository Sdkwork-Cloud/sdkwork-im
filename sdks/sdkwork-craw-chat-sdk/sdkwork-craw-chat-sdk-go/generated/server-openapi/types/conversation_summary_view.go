package types


type ConversationSummaryView struct {
	TenantId string `json:"tenantId"`
	ConversationId string `json:"conversationId"`
	MessageCount int `json:"messageCount"`
	LastMessageId string `json:"lastMessageId"`
	LastMessageSeq int `json:"lastMessageSeq"`
	LastSenderId string `json:"lastSenderId"`
	LastSenderKind string `json:"lastSenderKind"`
	LastSender SummarySenderView `json:"lastSender"`
	LastSummary string `json:"lastSummary"`
	LastMessageAt string `json:"lastMessageAt"`
	AgentHandoff ConversationAgentHandoffView `json:"agentHandoff"`
}
