package types


type RtcSignalEvent struct {
	TenantId string `json:"tenantId"`
	RtcSessionId string `json:"rtcSessionId"`
	ConversationId string `json:"conversationId"`
	RtcMode string `json:"rtcMode"`
	SignalType string `json:"signalType"`
	SchemaRef string `json:"schemaRef"`
	Payload string `json:"payload"`
	Sender Sender `json:"sender"`
	SignalingStreamId string `json:"signalingStreamId"`
	OccurredAt string `json:"occurredAt"`
}
