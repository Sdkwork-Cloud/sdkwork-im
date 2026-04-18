package types


type RealtimeEvent struct {
	TenantId string `json:"tenantId"`
	PrincipalId string `json:"principalId"`
	DeviceId string `json:"deviceId"`
	RealtimeSeq int `json:"realtimeSeq"`
	ScopeType string `json:"scopeType"`
	ScopeId string `json:"scopeId"`
	EventType string `json:"eventType"`
	DeliveryClass string `json:"deliveryClass"`
	Payload string `json:"payload"`
	OccurredAt string `json:"occurredAt"`
}
