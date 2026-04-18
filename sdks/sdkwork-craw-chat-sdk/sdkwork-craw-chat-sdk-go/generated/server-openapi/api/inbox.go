package api

import (
    sdktypes "github.com/sdkwork/craw-chat-backend-sdk/types"
    sdkhttp "github.com/sdkwork/craw-chat-backend-sdk/http"
)

type InboxApi struct {
    client *sdkhttp.Client
}

func NewInboxApi(client *sdkhttp.Client) *InboxApi {
    return &InboxApi{client: client}
}

// Get inbox entries
func (a *InboxApi) GetInbox() (sdktypes.InboxResponse, error) {
    raw, err := a.client.Get(BackendApiPath("/inbox"), nil, nil)
    if err != nil {
        var zero sdktypes.InboxResponse
        return zero, err
    }
    return decodeResult[sdktypes.InboxResponse](raw)
}
