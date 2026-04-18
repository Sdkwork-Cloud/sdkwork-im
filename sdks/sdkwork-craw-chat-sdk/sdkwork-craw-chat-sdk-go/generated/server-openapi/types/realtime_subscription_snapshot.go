package types


type RealtimeSubscriptionSnapshot struct {
	TenantId string `json:"tenantId"`
	PrincipalId string `json:"principalId"`
	DeviceId string `json:"deviceId"`
	Items []RealtimeSubscription `json:"items"`
	SyncedAt string `json:"syncedAt"`
}
