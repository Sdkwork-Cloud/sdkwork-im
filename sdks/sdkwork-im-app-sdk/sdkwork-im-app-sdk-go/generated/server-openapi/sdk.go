package app

import (
    "github.com/sdkwork/im-app-api-generated/api"
    sdkhttp "github.com/sdkwork/im-app-api-generated/http"
)

type SdkworkAppClient struct {
    http *sdkhttp.Client
    Automation *api.AutomationApi
    Device *api.DeviceApi
    Notification *api.NotificationApi
    Portal *api.PortalApi
    Provider *api.ProviderApi
    Iot *api.IotApi
    Rtc *api.RtcApi
}

func NewSdkworkAppClient(baseURL string) *SdkworkAppClient {
    cfg := sdkhttp.NewDefaultConfig(baseURL)
    return NewSdkworkAppClientWithConfig(cfg)
}

func NewSdkworkAppClientWithConfig(config sdkhttp.Config) *SdkworkAppClient {
    client := sdkhttp.NewClient(config)
    return &SdkworkAppClient{
        http: client,
        Automation: api.NewAutomationApi(client),
        Device: api.NewDeviceApi(client),
        Notification: api.NewNotificationApi(client),
        Portal: api.NewPortalApi(client),
        Provider: api.NewProviderApi(client),
        Iot: api.NewIotApi(client),
        Rtc: api.NewRtcApi(client),
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
