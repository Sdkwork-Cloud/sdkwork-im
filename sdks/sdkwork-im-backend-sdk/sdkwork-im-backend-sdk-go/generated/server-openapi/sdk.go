package backend

import (
    "github.com/sdkwork/im-backend-api-generated/api"
    sdkhttp "github.com/sdkwork/im-backend-api-generated/http"
)

type SdkworkBackendClient struct {
    http *sdkhttp.Client
    Ops *api.OpsApi
    Audit *api.AuditApi
    Automation *api.AutomationApi
    Control *api.ControlApi
    Admin *api.AdminApi
}

func NewSdkworkBackendClient(baseURL string) *SdkworkBackendClient {
    cfg := sdkhttp.NewDefaultConfig(baseURL)
    return NewSdkworkBackendClientWithConfig(cfg)
}

func NewSdkworkBackendClientWithConfig(config sdkhttp.Config) *SdkworkBackendClient {
    client := sdkhttp.NewClient(config)
    return &SdkworkBackendClient{
        http: client,
        Ops: api.NewOpsApi(client),
        Audit: api.NewAuditApi(client),
        Automation: api.NewAutomationApi(client),
        Control: api.NewControlApi(client),
        Admin: api.NewAdminApi(client),
    }
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
