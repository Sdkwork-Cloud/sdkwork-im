package api

import (
    sdktypes "github.com/sdkwork/craw-chat-backend-sdk/types"
    sdkhttp "github.com/sdkwork/craw-chat-backend-sdk/http"
)

type SessionApi struct {
    client *sdkhttp.Client
}

func NewSessionApi(client *sdkhttp.Client) *SessionApi {
    return &SessionApi{client: client}
}

// Resume the current app session
func (a *SessionApi) Resume(body sdktypes.ResumeSessionRequest) (sdktypes.SessionResumeView, error) {
    raw, err := a.client.Post(BackendApiPath("/sessions/resume"), body, nil, nil, "application/json")
    if err != nil {
        var zero sdktypes.SessionResumeView
        return zero, err
    }
    return decodeResult[sdktypes.SessionResumeView](raw)
}

// Disconnect the current app session device route
func (a *SessionApi) Disconnect(body sdktypes.PresenceDeviceRequest) (sdktypes.PresenceSnapshotView, error) {
    raw, err := a.client.Post(BackendApiPath("/sessions/disconnect"), body, nil, nil, "application/json")
    if err != nil {
        var zero sdktypes.PresenceSnapshotView
        return zero, err
    }
    return decodeResult[sdktypes.PresenceSnapshotView](raw)
}
