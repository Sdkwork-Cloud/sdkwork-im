package types


type ContentPart struct {
	Kind string `json:"kind"`
	Text string `json:"text"`
	SchemaRef string `json:"schemaRef"`
	Encoding string `json:"encoding"`
	Payload string `json:"payload"`
	MediaAssetId string `json:"mediaAssetId"`
	Resource MediaResource `json:"resource"`
	SignalType string `json:"signalType"`
	StreamId string `json:"streamId"`
	StreamType string `json:"streamType"`
	State string `json:"state"`
}
