package api

import (
    "fmt"
    sdktypes "github.com/sdkwork/craw-chat-backend-sdk/types"
    sdkhttp "github.com/sdkwork/craw-chat-backend-sdk/http"
)

type StreamApi struct {
    client *sdkhttp.Client
}

func NewStreamApi(client *sdkhttp.Client) *StreamApi {
    return &StreamApi{client: client}
}

// Open a stream session
func (a *StreamApi) Open(body sdktypes.OpenStreamRequest) (sdktypes.StreamSession, error) {
    raw, err := a.client.Post(BackendApiPath("/streams"), body, nil, nil, "application/json")
    if err != nil {
        var zero sdktypes.StreamSession
        return zero, err
    }
    return decodeResult[sdktypes.StreamSession](raw)
}

// List stream frames
func (a *StreamApi) ListStreamFrames(streamId string, query map[string]interface{}) (sdktypes.StreamFrameWindow, error) {
    raw, err := a.client.Get(BackendApiPath(fmt.Sprintf("/streams/%s/frames", streamId)), query, nil)
    if err != nil {
        var zero sdktypes.StreamFrameWindow
        return zero, err
    }
    return decodeResult[sdktypes.StreamFrameWindow](raw)
}

// Append a frame to a stream
func (a *StreamApi) AppendStreamFrame(streamId string, body sdktypes.AppendStreamFrameRequest) (sdktypes.StreamFrame, error) {
    raw, err := a.client.Post(BackendApiPath(fmt.Sprintf("/streams/%s/frames", streamId)), body, nil, nil, "application/json")
    if err != nil {
        var zero sdktypes.StreamFrame
        return zero, err
    }
    return decodeResult[sdktypes.StreamFrame](raw)
}

// Checkpoint a stream session
func (a *StreamApi) Checkpoint(streamId string, body sdktypes.CheckpointStreamRequest) (sdktypes.StreamSession, error) {
    raw, err := a.client.Post(BackendApiPath(fmt.Sprintf("/streams/%s/checkpoint", streamId)), body, nil, nil, "application/json")
    if err != nil {
        var zero sdktypes.StreamSession
        return zero, err
    }
    return decodeResult[sdktypes.StreamSession](raw)
}

// Complete a stream session
func (a *StreamApi) Complete(streamId string, body sdktypes.CompleteStreamRequest) (sdktypes.StreamSession, error) {
    raw, err := a.client.Post(BackendApiPath(fmt.Sprintf("/streams/%s/complete", streamId)), body, nil, nil, "application/json")
    if err != nil {
        var zero sdktypes.StreamSession
        return zero, err
    }
    return decodeResult[sdktypes.StreamSession](raw)
}

// Abort a stream session
func (a *StreamApi) Abort(streamId string, body sdktypes.AbortStreamRequest) (sdktypes.StreamSession, error) {
    raw, err := a.client.Post(BackendApiPath(fmt.Sprintf("/streams/%s/abort", streamId)), body, nil, nil, "application/json")
    if err != nil {
        var zero sdktypes.StreamSession
        return zero, err
    }
    return decodeResult[sdktypes.StreamSession](raw)
}
