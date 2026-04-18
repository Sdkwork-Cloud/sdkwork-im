package types


type RealtimeAckState struct {
	TenantId string `json:"tenantId"`
	PrincipalId string `json:"principalId"`
	DeviceId string `json:"deviceId"`
	AckedThroughSeq int `json:"ackedThroughSeq"`
	TrimmedThroughSeq int `json:"trimmedThroughSeq"`
	RetainedEventCount int `json:"retainedEventCount"`
	AckedAt string `json:"ackedAt"`
}
