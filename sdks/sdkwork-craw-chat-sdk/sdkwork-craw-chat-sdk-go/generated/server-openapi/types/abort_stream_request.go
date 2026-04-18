package types


type AbortStreamRequest struct {
	FrameSeq int `json:"frameSeq"`
	Reason string `json:"reason"`
}
