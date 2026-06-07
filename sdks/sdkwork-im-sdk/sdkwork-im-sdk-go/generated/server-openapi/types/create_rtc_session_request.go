package types


type CreateRtcSessionRequest struct {
	ConversationId string `json:"conversationId"`
	MediaKind string `json:"mediaKind"`
}
