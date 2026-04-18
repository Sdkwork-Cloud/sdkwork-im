package types


type PostMessageRequest struct {
	ClientMsgId string `json:"clientMsgId"`
	Summary string `json:"summary"`
	Text string `json:"text"`
	Parts []ContentPart `json:"parts"`
	RenderHints StringMap `json:"renderHints"`
}
