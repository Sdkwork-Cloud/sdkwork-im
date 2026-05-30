package types


type DeviceTwinView struct {
	TenantId string `json:"tenantId"`
	DeviceId string `json:"deviceId"`
	DesiredStateJson string `json:"desiredStateJson"`
	ReportedStateJson string `json:"reportedStateJson"`
	UpdatedAt string `json:"updatedAt"`
}
