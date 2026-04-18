package types


type RegisteredDeviceView struct {
	TenantId string `json:"tenantId"`
	PrincipalId string `json:"principalId"`
	DeviceId string `json:"deviceId"`
	RegisteredAt string `json:"registeredAt"`
}
