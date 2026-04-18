package types


type ChangeConversationMemberRoleResult struct {
	EventId string `json:"eventId"`
	ChangedAt string `json:"changedAt"`
	PreviousMember ConversationMember `json:"previousMember"`
	UpdatedMember ConversationMember `json:"updatedMember"`
}
