package types


type UpdateReadCursorRequest struct {
	ReadSeq int `json:"readSeq"`
	LastReadMessageId string `json:"lastReadMessageId"`
}
