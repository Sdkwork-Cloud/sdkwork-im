package types


type StreamFrameWindow struct {
	Items []StreamFrame `json:"items"`
	NextAfterFrameSeq int `json:"nextAfterFrameSeq"`
	HasMore bool `json:"hasMore"`
}
