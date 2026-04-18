package types


type ChangeConversationMemberRoleRequest struct {
	MemberId string `json:"memberId"`
	Role MembershipRole `json:"role"`
}
