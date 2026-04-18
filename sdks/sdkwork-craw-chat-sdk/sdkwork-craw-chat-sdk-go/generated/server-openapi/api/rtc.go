package api

import (
    "fmt"
    sdktypes "github.com/sdkwork/craw-chat-backend-sdk/types"
    sdkhttp "github.com/sdkwork/craw-chat-backend-sdk/http"
)

type RtcApi struct {
    client *sdkhttp.Client
}

func NewRtcApi(client *sdkhttp.Client) *RtcApi {
    return &RtcApi{client: client}
}

// Create an RTC session
func (a *RtcApi) CreateRtcSession(body sdktypes.CreateRtcSessionRequest) (sdktypes.RtcSession, error) {
    raw, err := a.client.Post(BackendApiPath("/rtc/sessions"), body, nil, nil, "application/json")
    if err != nil {
        var zero sdktypes.RtcSession
        return zero, err
    }
    return decodeResult[sdktypes.RtcSession](raw)
}

// Invite participants into an RTC session
func (a *RtcApi) InviteRtcSession(rtcSessionId string, body sdktypes.InviteRtcSessionRequest) (sdktypes.RtcSession, error) {
    raw, err := a.client.Post(BackendApiPath(fmt.Sprintf("/rtc/sessions/%s/invite", rtcSessionId)), body, nil, nil, "application/json")
    if err != nil {
        var zero sdktypes.RtcSession
        return zero, err
    }
    return decodeResult[sdktypes.RtcSession](raw)
}

// Accept an RTC session
func (a *RtcApi) AcceptRtcSession(rtcSessionId string, body sdktypes.UpdateRtcSessionRequest) (sdktypes.RtcSession, error) {
    raw, err := a.client.Post(BackendApiPath(fmt.Sprintf("/rtc/sessions/%s/accept", rtcSessionId)), body, nil, nil, "application/json")
    if err != nil {
        var zero sdktypes.RtcSession
        return zero, err
    }
    return decodeResult[sdktypes.RtcSession](raw)
}

// Reject an RTC session
func (a *RtcApi) RejectRtcSession(rtcSessionId string, body sdktypes.UpdateRtcSessionRequest) (sdktypes.RtcSession, error) {
    raw, err := a.client.Post(BackendApiPath(fmt.Sprintf("/rtc/sessions/%s/reject", rtcSessionId)), body, nil, nil, "application/json")
    if err != nil {
        var zero sdktypes.RtcSession
        return zero, err
    }
    return decodeResult[sdktypes.RtcSession](raw)
}

// End an RTC session
func (a *RtcApi) EndRtcSession(rtcSessionId string, body sdktypes.UpdateRtcSessionRequest) (sdktypes.RtcSession, error) {
    raw, err := a.client.Post(BackendApiPath(fmt.Sprintf("/rtc/sessions/%s/end", rtcSessionId)), body, nil, nil, "application/json")
    if err != nil {
        var zero sdktypes.RtcSession
        return zero, err
    }
    return decodeResult[sdktypes.RtcSession](raw)
}

// Post an RTC signaling event
func (a *RtcApi) PostRtcSignal(rtcSessionId string, body sdktypes.PostRtcSignalRequest) (sdktypes.RtcSignalEvent, error) {
    raw, err := a.client.Post(BackendApiPath(fmt.Sprintf("/rtc/sessions/%s/signals", rtcSessionId)), body, nil, nil, "application/json")
    if err != nil {
        var zero sdktypes.RtcSignalEvent
        return zero, err
    }
    return decodeResult[sdktypes.RtcSignalEvent](raw)
}

// Issue an RTC participant credential
func (a *RtcApi) IssueRtcParticipantCredential(rtcSessionId string, body sdktypes.IssueRtcParticipantCredentialRequest) (sdktypes.RtcParticipantCredential, error) {
    raw, err := a.client.Post(BackendApiPath(fmt.Sprintf("/rtc/sessions/%s/credentials", rtcSessionId)), body, nil, nil, "application/json")
    if err != nil {
        var zero sdktypes.RtcParticipantCredential
        return zero, err
    }
    return decodeResult[sdktypes.RtcParticipantCredential](raw)
}

// Get the RTC recording artifact
func (a *RtcApi) GetRtcRecordingArtifact(rtcSessionId string) (sdktypes.RtcRecordingArtifact, error) {
    raw, err := a.client.Get(BackendApiPath(fmt.Sprintf("/rtc/sessions/%s/artifacts/recording", rtcSessionId)), nil, nil)
    if err != nil {
        var zero sdktypes.RtcRecordingArtifact
        return zero, err
    }
    return decodeResult[sdktypes.RtcRecordingArtifact](raw)
}
