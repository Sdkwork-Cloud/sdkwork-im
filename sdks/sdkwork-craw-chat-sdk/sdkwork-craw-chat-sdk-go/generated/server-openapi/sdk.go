package backend

import (
    "github.com/sdkwork/craw-chat-backend-sdk/api"
    sdkhttp "github.com/sdkwork/craw-chat-backend-sdk/http"
)

type SdkworkBackendClient struct {
    http *sdkhttp.Client
    Auth *api.AuthApi
    Portal *api.PortalApi
    Session *api.SessionApi
    Presence *api.PresenceApi
    Realtime *api.RealtimeApi
    Device *api.DeviceApi
    Inbox *api.InboxApi
    Conversation *api.ConversationApi
    Message *api.MessageApi
    Media *api.MediaApi
    Stream *api.StreamApi
    Rtc *api.RtcApi
}

func NewSdkworkBackendClient(baseURL string) *SdkworkBackendClient {
    cfg := sdkhttp.NewDefaultConfig(baseURL)
    return NewSdkworkBackendClientWithConfig(cfg)
}

func NewSdkworkBackendClientWithConfig(config sdkhttp.Config) *SdkworkBackendClient {
    client := sdkhttp.NewClient(config)
    return &SdkworkBackendClient{
        http: client,
        Auth: api.NewAuthApi(client),
        Portal: api.NewPortalApi(client),
        Session: api.NewSessionApi(client),
        Presence: api.NewPresenceApi(client),
        Realtime: api.NewRealtimeApi(client),
        Device: api.NewDeviceApi(client),
        Inbox: api.NewInboxApi(client),
        Conversation: api.NewConversationApi(client),
        Message: api.NewMessageApi(client),
        Media: api.NewMediaApi(client),
        Stream: api.NewStreamApi(client),
        Rtc: api.NewRtcApi(client),
    }
}

func (c *SdkworkBackendClient) SetApiKey(apiKey string) *SdkworkBackendClient {
    c.http.SetApiKey(apiKey)
    return c
}

func (c *SdkworkBackendClient) SetAuthToken(token string) *SdkworkBackendClient {
    c.http.SetAuthToken(token)
    return c
}

func (c *SdkworkBackendClient) SetAccessToken(token string) *SdkworkBackendClient {
    c.http.SetAccessToken(token)
    return c
}

func (c *SdkworkBackendClient) SetHeader(key string, value string) *SdkworkBackendClient {
    c.http.SetHeader(key, value)
    return c
}

func (c *SdkworkBackendClient) Http() *sdkhttp.Client {
    return c.http
}
