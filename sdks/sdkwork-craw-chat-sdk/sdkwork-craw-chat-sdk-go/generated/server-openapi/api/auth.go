package api

import (
    sdktypes "github.com/sdkwork/craw-chat-backend-sdk/types"
    sdkhttp "github.com/sdkwork/craw-chat-backend-sdk/http"
)

type AuthApi struct {
    client *sdkhttp.Client
}

func NewAuthApi(client *sdkhttp.Client) *AuthApi {
    return &AuthApi{client: client}
}

// Sign in to the tenant portal
func (a *AuthApi) Login(body sdktypes.PortalLoginRequest) (sdktypes.PortalLoginResponse, error) {
    raw, err := a.client.Post(BackendApiPath("/auth/login"), body, nil, nil, "application/json")
    if err != nil {
        var zero sdktypes.PortalLoginResponse
        return zero, err
    }
    return decodeResult[sdktypes.PortalLoginResponse](raw)
}

// Read the current portal session
func (a *AuthApi) Me() (sdktypes.PortalMeResponse, error) {
    raw, err := a.client.Get(BackendApiPath("/auth/me"), nil, nil)
    if err != nil {
        var zero sdktypes.PortalMeResponse
        return zero, err
    }
    return decodeResult[sdktypes.PortalMeResponse](raw)
}
