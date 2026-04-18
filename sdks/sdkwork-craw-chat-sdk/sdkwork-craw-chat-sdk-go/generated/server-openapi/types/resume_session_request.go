package types


type ResumeSessionRequest struct {
	DeviceId string `json:"deviceId"`
	LastSeenSyncSeq int `json:"lastSeenSyncSeq"`
}
