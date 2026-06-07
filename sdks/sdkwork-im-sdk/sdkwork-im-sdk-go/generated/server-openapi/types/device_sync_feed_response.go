package types


type DeviceSyncFeedResponse struct {
	Items []DeviceSyncFeedEntry `json:"items"`
	NextAfterSeq int `json:"nextAfterSeq"`
	HasMore bool `json:"hasMore"`
	TrimmedThroughSeq int `json:"trimmedThroughSeq"`
}
