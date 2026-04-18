package types


type InboxResponse struct {
	Items []ConversationInboxEntry `json:"items"`
}
