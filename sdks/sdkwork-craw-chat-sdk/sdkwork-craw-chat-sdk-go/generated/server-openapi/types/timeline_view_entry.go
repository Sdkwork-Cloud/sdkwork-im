package types


type TimelineViewEntry struct {
	TenantId string `json:"tenantId"`
	ConversationId string `json:"conversationId"`
	MessageId string `json:"messageId"`
	MessageSeq int `json:"messageSeq"`
	Summary string `json:"summary"`
}
