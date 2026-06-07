package types


type DeviceSyncFeedEntry struct {
	TenantId string `json:"tenantId"`
	PrincipalId string `json:"principalId"`
	PrincipalKind string `json:"principalKind"`
	DeviceId string `json:"deviceId"`
	SyncSeq int `json:"syncSeq"`
	EventId string `json:"eventId"`
	OriginEventType string `json:"originEventType"`
	ActorId string `json:"actorId"`
	ConversationId string `json:"conversationId"`
	MessageId string `json:"messageId"`
	MessageSeq int `json:"messageSeq"`
	Payload string `json:"payload"`
	ReadSeq int `json:"readSeq"`
	Summary string `json:"summary"`
	OccurredAt string `json:"occurredAt"`
}
