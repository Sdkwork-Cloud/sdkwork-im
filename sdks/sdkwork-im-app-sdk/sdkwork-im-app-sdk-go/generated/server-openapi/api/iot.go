package api

import (
    sdktypes "github.com/sdkwork/im-app-api-generated/types"
    sdkhttp "github.com/sdkwork/im-app-api-generated/http"
)

type IotApi struct {
    client *sdkhttp.Client
}

func NewIotApi(client *sdkhttp.Client) *IotApi {
    return &IotApi{client: client}
}

// Retrieve IoT access provider health
func (a *IotApi) AccessProviderHealthRetrieve() (sdktypes.AccessProviderHealthRetrieveResponse, error) {
    raw, err := a.client.Get(AppApiPath("/iot/access/provider_health"), nil, nil)
    if err != nil {
        var zero sdktypes.AccessProviderHealthRetrieveResponse
        return zero, err
    }
    return decodeResult[sdktypes.AccessProviderHealthRetrieveResponse](raw)
}

// Retrieve IoT protocol provider health
func (a *IotApi) ProtocolProviderHealthRetrieve() (sdktypes.ProtocolProviderHealthRetrieveResponse, error) {
    raw, err := a.client.Get(AppApiPath("/iot/protocol/provider_health"), nil, nil)
    if err != nil {
        var zero sdktypes.ProtocolProviderHealthRetrieveResponse
        return zero, err
    }
    return decodeResult[sdktypes.ProtocolProviderHealthRetrieveResponse](raw)
}

// Ingest IoT protocol uplink
func (a *IotApi) ProtocolUplinkCreate() (sdktypes.ProtocolUplinkCreateResponse, error) {
    raw, err := a.client.Post(AppApiPath("/iot/protocol/uplink"), nil, nil, nil, "")
    if err != nil {
        var zero sdktypes.ProtocolUplinkCreateResponse
        return zero, err
    }
    return decodeResult[sdktypes.ProtocolUplinkCreateResponse](raw)
}

// Ingest IoT protocol downlink
func (a *IotApi) ProtocolDownlinkCreate() (sdktypes.ProtocolDownlinkCreateResponse, error) {
    raw, err := a.client.Post(AppApiPath("/iot/protocol/downlink"), nil, nil, nil, "")
    if err != nil {
        var zero sdktypes.ProtocolDownlinkCreateResponse
        return zero, err
    }
    return decodeResult[sdktypes.ProtocolDownlinkCreateResponse](raw)
}
