package types


type ConversationReadCursorView struct {
	TenantId string `json:"tenantId"`
	ConversationId string `json:"conversationId"`
	MemberId string `json:"memberId"`
	PrincipalId string `json:"principalId"`
	ReadSeq int `json:"readSeq"`
	LastReadMessageId string `json:"lastReadMessageId"`
	UpdatedAt string `json:"updatedAt"`
	UnreadCount int `json:"unreadCount"`
}
