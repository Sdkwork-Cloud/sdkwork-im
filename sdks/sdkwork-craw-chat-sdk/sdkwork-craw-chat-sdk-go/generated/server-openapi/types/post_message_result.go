package types


type PostMessageResult struct {
	MessageId string `json:"messageId"`
	MessageSeq int `json:"messageSeq"`
	EventId string `json:"eventId"`
}
