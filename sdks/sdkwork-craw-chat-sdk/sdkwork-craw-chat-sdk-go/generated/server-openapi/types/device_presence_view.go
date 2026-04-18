package types


type DevicePresenceView struct {
	TenantId string `json:"tenantId"`
	PrincipalId string `json:"principalId"`
	DeviceId string `json:"deviceId"`
	Platform string `json:"platform"`
	SessionId string `json:"sessionId"`
	Status DevicePresenceStatus `json:"status"`
	LastSyncSeq int `json:"lastSyncSeq"`
	LastResumeAt string `json:"lastResumeAt"`
	LastSeenAt string `json:"lastSeenAt"`
}
