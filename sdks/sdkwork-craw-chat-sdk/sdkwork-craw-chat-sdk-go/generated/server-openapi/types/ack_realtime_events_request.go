package types


type AckRealtimeEventsRequest struct {
	DeviceId string `json:"deviceId"`
	AckedSeq int `json:"ackedSeq"`
}
