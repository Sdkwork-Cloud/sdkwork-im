package types


type PortalLoginResponse struct {
	AccessToken string `json:"accessToken"`
	RefreshToken string `json:"refreshToken"`
	ExpiresAt int `json:"expiresAt"`
	User PortalUserView `json:"user"`
	Workspace PortalWorkspaceView `json:"workspace"`
}
