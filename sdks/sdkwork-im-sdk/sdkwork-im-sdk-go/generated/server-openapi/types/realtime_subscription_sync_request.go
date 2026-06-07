package types


type RealtimeSubscriptionSyncRequest struct {
	DeviceId string `json:"deviceId"`
	Conversations []string `json:"conversations"`
}
