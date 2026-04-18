package types


type RealtimeEventWindow struct {
	DeviceId string `json:"deviceId"`
	Items []RealtimeEvent `json:"items"`
	NextAfterSeq int `json:"nextAfterSeq"`
	HasMore bool `json:"hasMore"`
	AckedThroughSeq int `json:"ackedThroughSeq"`
	TrimmedThroughSeq int `json:"trimmedThroughSeq"`
}
