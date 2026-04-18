package types


type PortalUserView struct {
	Id string `json:"id"`
	Login string `json:"login"`
	Name string `json:"name"`
	Role string `json:"role"`
	Email string `json:"email"`
	ActorKind string `json:"actorKind"`
	ClientKind string `json:"clientKind"`
	Permissions []string `json:"permissions"`
}
