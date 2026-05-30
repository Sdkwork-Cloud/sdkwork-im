package api

import (
    sdktypes "github.com/sdkwork/im-app-api-generated/types"
    sdkhttp "github.com/sdkwork/im-app-api-generated/http"
)

type RtcApi struct {
    client *sdkhttp.Client
}

func NewRtcApi(client *sdkhttp.Client) *RtcApi {
    return &RtcApi{client: client}
}

// Map RTC provider callback
func (a *RtcApi) ProviderCallbacksCreate() (sdktypes.ProviderCallbacksCreateResponse, error) {
    raw, err := a.client.Post(AppApiPath("/rtc/provider_callbacks"), nil, nil, nil, "")
    if err != nil {
        var zero sdktypes.ProviderCallbacksCreateResponse
        return zero, err
    }
    return decodeResult[sdktypes.ProviderCallbacksCreateResponse](raw)
}

// Retrieve RTC provider health
func (a *RtcApi) ProviderHealthRetrieve() (sdktypes.ProviderHealthRetrieveResponse, error) {
    raw, err := a.client.Get(AppApiPath("/rtc/provider_health"), nil, nil)
    if err != nil {
        var zero sdktypes.ProviderHealthRetrieveResponse
        return zero, err
    }
    return decodeResult[sdktypes.ProviderHealthRetrieveResponse](raw)
}
