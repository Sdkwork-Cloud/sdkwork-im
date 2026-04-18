package types


type OpenStreamRequest struct {
	StreamId string `json:"streamId"`
	StreamType string `json:"streamType"`
	ScopeKind string `json:"scopeKind"`
	ScopeId string `json:"scopeId"`
	DurabilityClass string `json:"durabilityClass"`
	SchemaRef string `json:"schemaRef"`
}
