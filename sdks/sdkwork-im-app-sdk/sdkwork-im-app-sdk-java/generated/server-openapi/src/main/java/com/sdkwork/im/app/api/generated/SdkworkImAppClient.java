package com.sdkwork.im.app.api.generated;

import com.sdkwork.common.core.Types;
import com.sdkwork.im.app.api.generated.http.HttpClient;
import com.sdkwork.im.app.api.generated.api.AutomationApi;
import com.sdkwork.im.app.api.generated.api.DeviceApi;
import com.sdkwork.im.app.api.generated.api.NotificationApi;
import com.sdkwork.im.app.api.generated.api.PortalApi;
import com.sdkwork.im.app.api.generated.api.ProviderApi;
import com.sdkwork.im.app.api.generated.api.IotApi;

public class SdkworkImAppClient {
    private final HttpClient httpClient;
    private AutomationApi automation;
    private DeviceApi device;
    private NotificationApi notification;
    private PortalApi portal;
    private ProviderApi provider;
    private IotApi iot;

    public SdkworkImAppClient(String baseUrl) {
        this.httpClient = new HttpClient(baseUrl);
        this.automation = new AutomationApi(httpClient);
        this.device = new DeviceApi(httpClient);
        this.notification = new NotificationApi(httpClient);
        this.portal = new PortalApi(httpClient);
        this.provider = new ProviderApi(httpClient);
        this.iot = new IotApi(httpClient);
    }

    public SdkworkImAppClient(Types.SdkConfig config) {
        this.httpClient = new HttpClient(config);
        this.automation = new AutomationApi(httpClient);
        this.device = new DeviceApi(httpClient);
        this.notification = new NotificationApi(httpClient);
        this.portal = new PortalApi(httpClient);
        this.provider = new ProviderApi(httpClient);
        this.iot = new IotApi(httpClient);
    }

    public AutomationApi getAutomation() {
        return this.automation;
    }

    public DeviceApi getDevice() {
        return this.device;
    }

    public NotificationApi getNotification() {
        return this.notification;
    }

    public PortalApi getPortal() {
        return this.portal;
    }

    public ProviderApi getProvider() {
        return this.provider;
    }

    public IotApi getIot() {
        return this.iot;
    }
    public SdkworkImAppClient setAuthToken(String token) {
        httpClient.setAuthToken(token);
        return this;
    }

    public SdkworkImAppClient setAccessToken(String token) {
        httpClient.setAccessToken(token);
        return this;
    }

    public SdkworkImAppClient setHeader(String key, String value) {
        httpClient.setHeader(key, value);
        return this;
    }

    public HttpClient getHttpClient() {
        return httpClient;
    }
}
