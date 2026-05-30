package app

import (
    "github.com/sdkwork/im-app-api-generated/api"
    sdkhttp "github.com/sdkwork/im-app-api-generated/http"
)

type SdkworkAppClient struct {
    http *sdkhttp.Client
    Portal *api.PortalApi
    Device *api.DeviceApi
    Presence *api.PresenceApi
    Realtime *api.RealtimeApi
    Social *api.SocialApi
    Chat *api.ChatApi
    Media *api.MediaApi
    Stream *api.StreamApi
    Rtc *api.RtcApi
    Notification *api.NotificationApi
    Automation *api.AutomationApi
}

func NewSdkworkAppClient(baseURL string) *SdkworkAppClient {
    cfg := sdkhttp.NewDefaultConfig(baseURL)
    return NewSdkworkAppClientWithConfig(cfg)
}

func NewSdkworkAppClientWithConfig(config sdkhttp.Config) *SdkworkAppClient {
    client := sdkhttp.NewClient(config)
    return &SdkworkAppClient{
        http: client,
        Portal: api.NewPortalApi(client),
        Device: api.NewDeviceApi(client),
        Presence: api.NewPresenceApi(client),
        Realtime: api.NewRealtimeApi(client),
        Social: api.NewSocialApi(client),
        Chat: api.NewChatApi(client),
        Media: api.NewMediaApi(client),
        Stream: api.NewStreamApi(client),
        Rtc: api.NewRtcApi(client),
        Notification: api.NewNotificationApi(client),
        Automation: api.NewAutomationApi(client),
    }
}


func (c *SdkworkAppClient) SetAuthToken(token string) *SdkworkAppClient {
    c.http.SetAuthToken(token)
    return c
}

func (c *SdkworkAppClient) SetAccessToken(token string) *SdkworkAppClient {
    c.http.SetAccessToken(token)
    return c
}

func (c *SdkworkAppClient) SetHeader(key string, value string) *SdkworkAppClient {
    c.http.SetHeader(key, value)
    return c
}

func (c *SdkworkAppClient) Http() *sdkhttp.Client {
    return c.http
}
