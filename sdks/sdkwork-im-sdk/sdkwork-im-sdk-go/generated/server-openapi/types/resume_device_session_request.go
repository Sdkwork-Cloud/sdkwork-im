package types


type ResumeDeviceSessionRequest struct {
	DeviceId string `json:"deviceId"`
	LastSeenSyncSeq int `json:"lastSeenSyncSeq"`
}
