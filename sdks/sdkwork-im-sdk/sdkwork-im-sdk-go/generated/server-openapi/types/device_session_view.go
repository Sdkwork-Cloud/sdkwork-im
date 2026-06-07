package types


type DeviceSessionView struct {
	TenantId string `json:"tenantId"`
	PrincipalId string `json:"principalId"`
	PrincipalKind string `json:"principalKind"`
	DeviceId string `json:"deviceId"`
	ResumedAt string `json:"resumedAt"`
}
