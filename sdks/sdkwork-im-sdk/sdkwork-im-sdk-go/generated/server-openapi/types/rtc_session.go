package types


type RtcSession struct {
	TenantId string `json:"tenantId"`
	RtcSessionId string `json:"rtcSessionId"`
	ConversationId string `json:"conversationId"`
	ProviderPluginId string `json:"providerPluginId"`
	ProviderSessionId string `json:"providerSessionId"`
	RtcMode string `json:"rtcMode"`
	State string `json:"state"`
	CreatedAt string `json:"createdAt"`
	UpdatedAt string `json:"updatedAt"`
}
