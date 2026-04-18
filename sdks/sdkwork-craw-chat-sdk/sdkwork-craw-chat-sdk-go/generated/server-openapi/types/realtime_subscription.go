package types


type RealtimeSubscription struct {
	ScopeType string `json:"scopeType"`
	ScopeId string `json:"scopeId"`
	EventTypes []string `json:"eventTypes"`
	SubscribedAt string `json:"subscribedAt"`
}
