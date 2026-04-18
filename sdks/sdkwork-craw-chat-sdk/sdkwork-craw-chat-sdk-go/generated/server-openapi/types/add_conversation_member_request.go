package types


type AddConversationMemberRequest struct {
	PrincipalId string `json:"principalId"`
	PrincipalKind string `json:"principalKind"`
	Role MembershipRole `json:"role"`
}
