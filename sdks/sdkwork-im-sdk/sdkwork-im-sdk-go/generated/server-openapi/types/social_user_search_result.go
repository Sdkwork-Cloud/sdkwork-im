package types


type SocialUserSearchResult struct {
	TenantId string `json:"tenantId"`
	UserId string `json:"userId"`
	DisplayName string `json:"displayName"`
	RelationshipState string `json:"relationshipState"`
	AvatarUrl string `json:"avatarUrl"`
	Email string `json:"email"`
	Phone string `json:"phone"`
	Metadata map[string]interface{} `json:"metadata"`
}
