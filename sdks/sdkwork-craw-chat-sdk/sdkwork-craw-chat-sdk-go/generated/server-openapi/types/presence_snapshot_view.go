package types


type PresenceSnapshotView struct {
	TenantId string `json:"tenantId"`
	PrincipalId string `json:"principalId"`
	CurrentDeviceId string `json:"currentDeviceId"`
	Devices []DevicePresenceView `json:"devices"`
}
