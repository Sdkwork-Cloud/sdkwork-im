package types


type AttachMediaRequest struct {
	ConversationId string `json:"conversationId"`
	ClientMsgId string `json:"clientMsgId"`
	Summary string `json:"summary"`
	Text string `json:"text"`
	RenderHints StringMap `json:"renderHints"`
}
