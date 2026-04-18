package api

import (
    sdktypes "github.com/sdkwork/craw-chat-backend-sdk/types"
    sdkhttp "github.com/sdkwork/craw-chat-backend-sdk/http"
)

type PortalApi struct {
    client *sdkhttp.Client
}

func NewPortalApi(client *sdkhttp.Client) *PortalApi {
    return &PortalApi{client: client}
}

// Read the tenant portal home snapshot
func (a *PortalApi) GetHome() (sdktypes.PortalSnapshot, error) {
    raw, err := a.client.Get(BackendApiPath("/portal/home"), nil, nil)
    if err != nil {
        var zero sdktypes.PortalSnapshot
        return zero, err
    }
    return decodeResult[sdktypes.PortalSnapshot](raw)
}

// Read the tenant portal sign-in snapshot
func (a *PortalApi) GetAuth() (sdktypes.PortalSnapshot, error) {
    raw, err := a.client.Get(BackendApiPath("/portal/auth"), nil, nil)
    if err != nil {
        var zero sdktypes.PortalSnapshot
        return zero, err
    }
    return decodeResult[sdktypes.PortalSnapshot](raw)
}

// Read the current tenant workspace snapshot
func (a *PortalApi) GetWorkspace() (sdktypes.PortalWorkspaceView, error) {
    raw, err := a.client.Get(BackendApiPath("/portal/workspace"), nil, nil)
    if err != nil {
        var zero sdktypes.PortalWorkspaceView
        return zero, err
    }
    return decodeResult[sdktypes.PortalWorkspaceView](raw)
}

// Read the tenant dashboard snapshot
func (a *PortalApi) GetDashboard() (sdktypes.PortalSnapshot, error) {
    raw, err := a.client.Get(BackendApiPath("/portal/dashboard"), nil, nil)
    if err != nil {
        var zero sdktypes.PortalSnapshot
        return zero, err
    }
    return decodeResult[sdktypes.PortalSnapshot](raw)
}

// Read the tenant conversations snapshot
func (a *PortalApi) GetConversations() (sdktypes.PortalSnapshot, error) {
    raw, err := a.client.Get(BackendApiPath("/portal/conversations"), nil, nil)
    if err != nil {
        var zero sdktypes.PortalSnapshot
        return zero, err
    }
    return decodeResult[sdktypes.PortalSnapshot](raw)
}

// Read the tenant realtime snapshot
func (a *PortalApi) GetRealtime() (sdktypes.PortalSnapshot, error) {
    raw, err := a.client.Get(BackendApiPath("/portal/realtime"), nil, nil)
    if err != nil {
        var zero sdktypes.PortalSnapshot
        return zero, err
    }
    return decodeResult[sdktypes.PortalSnapshot](raw)
}

// Read the tenant media snapshot
func (a *PortalApi) GetMedia() (sdktypes.PortalSnapshot, error) {
    raw, err := a.client.Get(BackendApiPath("/portal/media"), nil, nil)
    if err != nil {
        var zero sdktypes.PortalSnapshot
        return zero, err
    }
    return decodeResult[sdktypes.PortalSnapshot](raw)
}

// Read the tenant automation snapshot
func (a *PortalApi) GetAutomation() (sdktypes.PortalSnapshot, error) {
    raw, err := a.client.Get(BackendApiPath("/portal/automation"), nil, nil)
    if err != nil {
        var zero sdktypes.PortalSnapshot
        return zero, err
    }
    return decodeResult[sdktypes.PortalSnapshot](raw)
}

// Read the tenant governance snapshot
func (a *PortalApi) GetGovernance() (sdktypes.PortalSnapshot, error) {
    raw, err := a.client.Get(BackendApiPath("/portal/governance"), nil, nil)
    if err != nil {
        var zero sdktypes.PortalSnapshot
        return zero, err
    }
    return decodeResult[sdktypes.PortalSnapshot](raw)
}
