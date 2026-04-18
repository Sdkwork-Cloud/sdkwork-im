package types


type CompleteStreamRequest struct {
	FrameSeq int `json:"frameSeq"`
	ResultMessageId string `json:"resultMessageId"`
}
