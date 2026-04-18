package types


type RtcSession struct {
	TenantId string `json:"tenantId"`
	RtcSessionId string `json:"rtcSessionId"`
	ConversationId string `json:"conversationId"`
	RtcMode string `json:"rtcMode"`
	InitiatorId string `json:"initiatorId"`
	ProviderPluginId string `json:"providerPluginId"`
	ProviderSessionId string `json:"providerSessionId"`
	AccessEndpoint string `json:"accessEndpoint"`
	ProviderRegion string `json:"providerRegion"`
	State RtcSessionState `json:"state"`
	SignalingStreamId string `json:"signalingStreamId"`
	ArtifactMessageId string `json:"artifactMessageId"`
	StartedAt string `json:"startedAt"`
	EndedAt string `json:"endedAt"`
}
