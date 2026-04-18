package api

import (
    "fmt"
    sdktypes "github.com/sdkwork/craw-chat-backend-sdk/types"
    sdkhttp "github.com/sdkwork/craw-chat-backend-sdk/http"
)

type MediaApi struct {
    client *sdkhttp.Client
}

func NewMediaApi(client *sdkhttp.Client) *MediaApi {
    return &MediaApi{client: client}
}

// Create a media upload record
func (a *MediaApi) CreateMediaUpload(body sdktypes.CreateUploadRequest) (sdktypes.MediaAsset, error) {
    raw, err := a.client.Post(BackendApiPath("/media/uploads"), body, nil, nil, "application/json")
    if err != nil {
        var zero sdktypes.MediaAsset
        return zero, err
    }
    return decodeResult[sdktypes.MediaAsset](raw)
}

// Complete a media upload
func (a *MediaApi) CompleteMediaUpload(mediaAssetId string, body sdktypes.CompleteUploadRequest) (sdktypes.MediaAsset, error) {
    raw, err := a.client.Post(BackendApiPath(fmt.Sprintf("/media/uploads/%s/complete", mediaAssetId)), body, nil, nil, "application/json")
    if err != nil {
        var zero sdktypes.MediaAsset
        return zero, err
    }
    return decodeResult[sdktypes.MediaAsset](raw)
}

// Issue a signed media download URL
func (a *MediaApi) GetMediaDownloadUrl(mediaAssetId string, query map[string]interface{}) (sdktypes.MediaDownloadUrlResponse, error) {
    raw, err := a.client.Get(BackendApiPath(fmt.Sprintf("/media/%s/download-url", mediaAssetId)), query, nil)
    if err != nil {
        var zero sdktypes.MediaDownloadUrlResponse
        return zero, err
    }
    return decodeResult[sdktypes.MediaDownloadUrlResponse](raw)
}

// Get a media asset by id
func (a *MediaApi) GetMediaAsset(mediaAssetId string) (sdktypes.MediaAsset, error) {
    raw, err := a.client.Get(BackendApiPath(fmt.Sprintf("/media/%s", mediaAssetId)), nil, nil)
    if err != nil {
        var zero sdktypes.MediaAsset
        return zero, err
    }
    return decodeResult[sdktypes.MediaAsset](raw)
}

// Attach a ready media asset as a conversation message
func (a *MediaApi) AttachMediaAsset(mediaAssetId string, body sdktypes.AttachMediaRequest) (sdktypes.PostMessageResult, error) {
    raw, err := a.client.Post(BackendApiPath(fmt.Sprintf("/media/%s/attach", mediaAssetId)), body, nil, nil, "application/json")
    if err != nil {
        var zero sdktypes.PostMessageResult
        return zero, err
    }
    return decodeResult[sdktypes.PostMessageResult](raw)
}
