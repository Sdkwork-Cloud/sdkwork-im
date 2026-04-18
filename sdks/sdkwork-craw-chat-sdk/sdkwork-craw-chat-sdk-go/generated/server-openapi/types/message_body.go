package types


type MessageBody struct {
	Summary string `json:"summary"`
	Parts []ContentPart `json:"parts"`
	RenderHints StringMap `json:"renderHints"`
}
