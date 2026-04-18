package types


type PortalLoginRequest struct {
	TenantId string `json:"tenantId"`
	Login string `json:"login"`
	Password string `json:"password"`
	DeviceId string `json:"deviceId"`
	SessionId string `json:"sessionId"`
	ClientKind string `json:"clientKind"`
}
