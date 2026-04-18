package types


type SyncRealtimeSubscriptionsRequest struct {
	DeviceId string `json:"deviceId"`
	Items []RealtimeSubscriptionItemInput `json:"items"`
}
