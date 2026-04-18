package api

import (
    sdktypes "github.com/sdkwork/craw-chat-backend-sdk/types"
    sdkhttp "github.com/sdkwork/craw-chat-backend-sdk/http"
)

type PresenceApi struct {
    client *sdkhttp.Client
}

func NewPresenceApi(client *sdkhttp.Client) *PresenceApi {
    return &PresenceApi{client: client}
}

// Refresh device presence
func (a *PresenceApi) Heartbeat(body sdktypes.PresenceDeviceRequest) (sdktypes.PresenceSnapshotView, error) {
    raw, err := a.client.Post(BackendApiPath("/presence/heartbeat"), body, nil, nil, "application/json")
    if err != nil {
        var zero sdktypes.PresenceSnapshotView
        return zero, err
    }
    return decodeResult[sdktypes.PresenceSnapshotView](raw)
}

// Get current presence
func (a *PresenceApi) GetPresenceMe() (sdktypes.PresenceSnapshotView, error) {
    raw, err := a.client.Get(BackendApiPath("/presence/me"), nil, nil)
    if err != nil {
        var zero sdktypes.PresenceSnapshotView
        return zero, err
    }
    return decodeResult[sdktypes.PresenceSnapshotView](raw)
}
