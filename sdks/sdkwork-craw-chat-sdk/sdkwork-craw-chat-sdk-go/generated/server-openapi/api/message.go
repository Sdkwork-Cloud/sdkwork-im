package api

import (
    "fmt"
    sdktypes "github.com/sdkwork/craw-chat-backend-sdk/types"
    sdkhttp "github.com/sdkwork/craw-chat-backend-sdk/http"
)

type MessageApi struct {
    client *sdkhttp.Client
}

func NewMessageApi(client *sdkhttp.Client) *MessageApi {
    return &MessageApi{client: client}
}

// Edit a posted message
func (a *MessageApi) Edit(messageId string, body sdktypes.EditMessageRequest) (sdktypes.MessageMutationResult, error) {
    raw, err := a.client.Post(BackendApiPath(fmt.Sprintf("/messages/%s/edit", messageId)), body, nil, nil, "application/json")
    if err != nil {
        var zero sdktypes.MessageMutationResult
        return zero, err
    }
    return decodeResult[sdktypes.MessageMutationResult](raw)
}

// Recall a posted message
func (a *MessageApi) Recall(messageId string) (sdktypes.MessageMutationResult, error) {
    raw, err := a.client.Post(BackendApiPath(fmt.Sprintf("/messages/%s/recall", messageId)), nil, nil, nil, "")
    if err != nil {
        var zero sdktypes.MessageMutationResult
        return zero, err
    }
    return decodeResult[sdktypes.MessageMutationResult](raw)
}
