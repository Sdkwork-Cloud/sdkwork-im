package types


type TransferConversationOwnerResult struct {
	EventId string `json:"eventId"`
	TransferredAt string `json:"transferredAt"`
	PreviousOwner ConversationMember `json:"previousOwner"`
	NewOwner ConversationMember `json:"newOwner"`
}
