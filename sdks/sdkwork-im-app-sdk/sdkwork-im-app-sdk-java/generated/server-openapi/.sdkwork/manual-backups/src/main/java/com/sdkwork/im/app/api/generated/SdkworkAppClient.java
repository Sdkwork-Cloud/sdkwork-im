package com.sdkwork.im.app.api.generated;

import com.sdkwork.common.core.Types;
import com.sdkwork.im.app.api.generated.http.HttpClient;
import com.sdkwork.im.app.api.generated.api.PortalApi;
import com.sdkwork.im.app.api.generated.api.DeviceApi;
import com.sdkwork.im.app.api.generated.api.PresenceApi;
import com.sdkwork.im.app.api.generated.api.RealtimeApi;
import com.sdkwork.im.app.api.generated.api.SocialApi;
import com.sdkwork.im.app.api.generated.api.ChatApi;
import com.sdkwork.im.app.api.generated.api.MediaApi;
import com.sdkwork.im.app.api.generated.api.StreamApi;
import com.sdkwork.im.app.api.generated.api.RtcApi;
import com.sdkwork.im.app.api.generated.api.NotificationApi;
import com.sdkwork.im.app.api.generated.api.AutomationApi;

public class SdkworkAppClient {
    private final HttpClient httpClient;
    private PortalApi portal;
    private DeviceApi device;
    private PresenceApi presence;
    private RealtimeApi realtime;
    private SocialApi social;
    private ChatApi chat;
    private MediaApi media;
    private StreamApi stream;
    private RtcApi rtc;
    private NotificationApi notification;
    private AutomationApi automation;

    public SdkworkAppClient(String baseUrl) {
        this.httpClient = new HttpClient(baseUrl);
        this.portal = new PortalApi(httpClient);
        this.device = new DeviceApi(httpClient);
        this.presence = new PresenceApi(httpClient);
        this.realtime = new RealtimeApi(httpClient);
        this.social = new SocialApi(httpClient);
        this.chat = new ChatApi(httpClient);
        this.media = new MediaApi(httpClient);
        this.stream = new StreamApi(httpClient);
        this.rtc = new RtcApi(httpClient);
        this.notification = new NotificationApi(httpClient);
        this.automation = new AutomationApi(httpClient);
    }

    public SdkworkAppClient(Types.SdkConfig config) {
        this.httpClient = new HttpClient(config);
        this.portal = new PortalApi(httpClient);
        this.device = new DeviceApi(httpClient);
        this.presence = new PresenceApi(httpClient);
        this.realtime = new RealtimeApi(httpClient);
        this.social = new SocialApi(httpClient);
        this.chat = new ChatApi(httpClient);
        this.media = new MediaApi(httpClient);
        this.stream = new StreamApi(httpClient);
        this.rtc = new RtcApi(httpClient);
        this.notification = new NotificationApi(httpClient);
        this.automation = new AutomationApi(httpClient);
    }

    public PortalApi getPortal() {
        return this.portal;
    }

    public DeviceApi getDevice() {
        return this.device;
    }

    public PresenceApi getPresence() {
        return this.presence;
    }

    public RealtimeApi getRealtime() {
        return this.realtime;
    }

    public SocialApi getSocial() {
        return this.social;
    }

    public ChatApi getChat() {
        return this.chat;
    }

    public MediaApi getMedia() {
        return this.media;
    }

    public StreamApi getStream() {
        return this.stream;
    }

    public RtcApi getRtc() {
        return this.rtc;
    }

    public NotificationApi getNotification() {
        return this.notification;
    }

    public AutomationApi getAutomation() {
        return this.automation;
    }


    public SdkworkAppClient setAuthToken(String token) {
        httpClient.setAuthToken(token);
        return this;
    }

    public SdkworkAppClient setAccessToken(String token) {
        httpClient.setAccessToken(token);
        return this;
    }

    public SdkworkAppClient setHeader(String key, String value) {
        httpClient.setHeader(key, value);
        return this;
    }

    public HttpClient getHttpClient() {
        return httpClient;
    }
}
