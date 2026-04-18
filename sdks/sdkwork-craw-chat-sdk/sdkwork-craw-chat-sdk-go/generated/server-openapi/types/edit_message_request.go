package types


type EditMessageRequest struct {
	Summary string `json:"summary"`
	Text string `json:"text"`
	Parts []ContentPart `json:"parts"`
	RenderHints StringMap `json:"renderHints"`
}
