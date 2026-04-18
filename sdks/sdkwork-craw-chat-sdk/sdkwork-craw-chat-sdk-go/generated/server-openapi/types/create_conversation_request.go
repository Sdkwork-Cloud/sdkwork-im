package types


type CreateConversationRequest struct {
	ConversationId string `json:"conversationId"`
	ConversationType string `json:"conversationType"`
}
