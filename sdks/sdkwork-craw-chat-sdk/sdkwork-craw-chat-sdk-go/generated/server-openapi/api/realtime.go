package api

import (
    sdktypes "github.com/sdkwork/craw-chat-backend-sdk/types"
    sdkhttp "github.com/sdkwork/craw-chat-backend-sdk/http"
)

type RealtimeApi struct {
    client *sdkhttp.Client
}

func NewRealtimeApi(client *sdkhttp.Client) *RealtimeApi {
    return &RealtimeApi{client: client}
}

// Replace realtime subscriptions for the current device
func (a *RealtimeApi) SyncRealtimeSubscriptions(body sdktypes.SyncRealtimeSubscriptionsRequest) (sdktypes.RealtimeSubscriptionSnapshot, error) {
    raw, err := a.client.Post(BackendApiPath("/realtime/subscriptions/sync"), body, nil, nil, "application/json")
    if err != nil {
        var zero sdktypes.RealtimeSubscriptionSnapshot
        return zero, err
    }
    return decodeResult[sdktypes.RealtimeSubscriptionSnapshot](raw)
}

// Pull realtime events for the current device
func (a *RealtimeApi) ListRealtimeEvents(query map[string]interface{}) (sdktypes.RealtimeEventWindow, error) {
    raw, err := a.client.Get(BackendApiPath("/realtime/events"), query, nil)
    if err != nil {
        var zero sdktypes.RealtimeEventWindow
        return zero, err
    }
    return decodeResult[sdktypes.RealtimeEventWindow](raw)
}

// Ack realtime events for the current device
func (a *RealtimeApi) AckRealtimeEvents(body sdktypes.AckRealtimeEventsRequest) (sdktypes.RealtimeAckState, error) {
    raw, err := a.client.Post(BackendApiPath("/realtime/events/ack"), body, nil, nil, "application/json")
    if err != nil {
        var zero sdktypes.RealtimeAckState
        return zero, err
    }
    return decodeResult[sdktypes.RealtimeAckState](raw)
}
