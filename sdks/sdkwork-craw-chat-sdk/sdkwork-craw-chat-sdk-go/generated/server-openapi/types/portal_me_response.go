package types


type PortalMeResponse struct {
	TenantId string `json:"tenantId"`
	User PortalUserView `json:"user"`
	Workspace PortalWorkspaceView `json:"workspace"`
}
