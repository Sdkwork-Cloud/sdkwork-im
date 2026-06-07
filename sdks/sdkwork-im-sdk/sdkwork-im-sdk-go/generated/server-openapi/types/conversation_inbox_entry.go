package types


type ConversationInboxEntry struct {
	TenantId string `json:"tenantId"`
	ConversationId string `json:"conversationId"`
	AgentHandoff bool `json:"agentHandoff"`
	ConversationType string `json:"conversationType"`
	LastActivityAt string `json:"lastActivityAt"`
	LastMessageId string `json:"lastMessageId"`
	LastSenderId string `json:"lastSenderId"`
	MessageCount int `json:"messageCount"`
	LastMessageSeq int `json:"lastMessageSeq"`
	LastSummary string `json:"lastSummary"`
	LastMessageAt string `json:"lastMessageAt"`
	UnreadCount int `json:"unreadCount"`
}
