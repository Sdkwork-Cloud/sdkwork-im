package types


type ConversationMember struct {
	TenantId string `json:"tenantId"`
	ConversationId string `json:"conversationId"`
	MemberId string `json:"memberId"`
	PrincipalId string `json:"principalId"`
	PrincipalKind string `json:"principalKind"`
	Role MembershipRole `json:"role"`
	State MembershipState `json:"state"`
	InvitedBy string `json:"invitedBy"`
	JoinedAt string `json:"joinedAt"`
	RemovedAt string `json:"removedAt"`
	Attributes StringMap `json:"attributes"`
}
