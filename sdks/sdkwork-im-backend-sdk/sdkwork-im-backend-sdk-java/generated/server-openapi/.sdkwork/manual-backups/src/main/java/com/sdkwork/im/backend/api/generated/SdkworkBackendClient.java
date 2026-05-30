package com.sdkwork.im.backend.api.generated;

import com.sdkwork.common.core.Types;
import com.sdkwork.im.backend.api.generated.http.HttpClient;
import com.sdkwork.im.backend.api.generated.api.OpsApi;
import com.sdkwork.im.backend.api.generated.api.AuditApi;
import com.sdkwork.im.backend.api.generated.api.ProviderApi;
import com.sdkwork.im.backend.api.generated.api.IotApi;
import com.sdkwork.im.backend.api.generated.api.RtcApi;
import com.sdkwork.im.backend.api.generated.api.AutomationApi;

public class SdkworkBackendClient {
    private final HttpClient httpClient;
    private OpsApi ops;
    private AuditApi audit;
    private ProviderApi provider;
    private IotApi iot;
    private RtcApi rtc;
    private AutomationApi automation;

    public SdkworkBackendClient(String baseUrl) {
        this.httpClient = new HttpClient(baseUrl);
        this.ops = new OpsApi(httpClient);
        this.audit = new AuditApi(httpClient);
        this.provider = new ProviderApi(httpClient);
        this.iot = new IotApi(httpClient);
        this.rtc = new RtcApi(httpClient);
        this.automation = new AutomationApi(httpClient);
    }

    public SdkworkBackendClient(Types.SdkConfig config) {
        this.httpClient = new HttpClient(config);
        this.ops = new OpsApi(httpClient);
        this.audit = new AuditApi(httpClient);
        this.provider = new ProviderApi(httpClient);
        this.iot = new IotApi(httpClient);
        this.rtc = new RtcApi(httpClient);
        this.automation = new AutomationApi(httpClient);
    }

    public OpsApi getOps() {
        return this.ops;
    }

    public AuditApi getAudit() {
        return this.audit;
    }

    public ProviderApi getProvider() {
        return this.provider;
    }

    public IotApi getIot() {
        return this.iot;
    }

    public RtcApi getRtc() {
        return this.rtc;
    }

    public AutomationApi getAutomation() {
        return this.automation;
    }


    public SdkworkBackendClient setAuthToken(String token) {
        httpClient.setAuthToken(token);
        return this;
    }

    public SdkworkBackendClient setAccessToken(String token) {
        httpClient.setAccessToken(token);
        return this;
    }

    public SdkworkBackendClient setHeader(String key, String value) {
        httpClient.setHeader(key, value);
        return this;
    }

    public HttpClient getHttpClient() {
        return httpClient;
    }
}
