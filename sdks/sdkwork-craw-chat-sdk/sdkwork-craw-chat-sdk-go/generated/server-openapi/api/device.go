package api

import (
    "fmt"
    sdktypes "github.com/sdkwork/craw-chat-backend-sdk/types"
    sdkhttp "github.com/sdkwork/craw-chat-backend-sdk/http"
)

type DeviceApi struct {
    client *sdkhttp.Client
}

func NewDeviceApi(client *sdkhttp.Client) *DeviceApi {
    return &DeviceApi{client: client}
}

// Register the current device
func (a *DeviceApi) Register(body sdktypes.RegisterDeviceRequest) (sdktypes.RegisteredDeviceView, error) {
    raw, err := a.client.Post(BackendApiPath("/devices/register"), body, nil, nil, "application/json")
    if err != nil {
        var zero sdktypes.RegisteredDeviceView
        return zero, err
    }
    return decodeResult[sdktypes.RegisteredDeviceView](raw)
}

// Get device sync feed entries
func (a *DeviceApi) GetDeviceSyncFeed(deviceId string, query map[string]interface{}) (sdktypes.DeviceSyncFeedResponse, error) {
    raw, err := a.client.Get(BackendApiPath(fmt.Sprintf("/devices/%s/sync-feed", deviceId)), query, nil)
    if err != nil {
        var zero sdktypes.DeviceSyncFeedResponse
        return zero, err
    }
    return decodeResult[sdktypes.DeviceSyncFeedResponse](raw)
}
