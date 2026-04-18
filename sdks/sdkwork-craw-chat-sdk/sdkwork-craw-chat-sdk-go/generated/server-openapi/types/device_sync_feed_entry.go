package types


type DeviceSyncFeedEntry struct {
	TenantId string `json:"tenantId"`
	PrincipalId string `json:"principalId"`
	DeviceId string `json:"deviceId"`
	SyncSeq int `json:"syncSeq"`
	OriginEventId string `json:"originEventId"`
	OriginEventType string `json:"originEventType"`
	ConversationId string `json:"conversationId"`
	MessageId string `json:"messageId"`
	MessageSeq int `json:"messageSeq"`
	MemberId string `json:"memberId"`
	ReadSeq int `json:"readSeq"`
	LastReadMessageId string `json:"lastReadMessageId"`
	ActorId string `json:"actorId"`
	ActorKind string `json:"actorKind"`
	ActorDeviceId string `json:"actorDeviceId"`
	Summary string `json:"summary"`
	PayloadSchema string `json:"payloadSchema"`
	Payload string `json:"payload"`
	OccurredAt string `json:"occurredAt"`
}
